import pygame
import tkinter as tk
from tkinter import filedialog

pygame.init()

# global variable
font = pygame.font.SysFont('Monospace', 16)


class Game:
    grid_width = 1
    image_width = 32
    header_width = 32
    block_width = image_width+grid_width

    def __init__(self, rows, columns):
        self.rows = rows
        self.columns = columns
        self.width = self.image_width+(columns + 1) * self.block_width
        self.height = self.header_width + \
            self.image_width+(rows+1)*self.block_width
        self.screen = pygame.display.set_mode((self.width, self.height))
        self.screen.fill((255, 255, 255))
        self.draw_grid()
        self.cells = [[Cell(self.screen, (i, j), self.index_to_position((i, j)), 0)
                       for j in range(self.columns)] for i in range(self.rows)]
        self.back_button = Button(self.screen, (10, 10), 100, 30, 'Back')
        pygame.display.set_caption('Create game')
        pygame.display.update()

    def draw_grid(self):
        for row in range(self.rows+1):
            pygame.draw.line(self.screen, (0, 0, 0),
                             (self.image_width, self.header_width+self.image_width+row*self.block_width), (self.width-self.block_width, self.header_width+self.image_width+row*self.block_width), self.grid_width)

        for column in range(self.columns+1):
            pygame.draw.line(self.screen, (0, 0, 0), (self.image_width+column*self.block_width,
                                                      self.header_width+self.image_width), (self.image_width+column*self.block_width, self.height-self.block_width), self.grid_width)

    def change_cell(self, position):
        cell_index = self.position_to_index(position)
        if cell_index[0] < 0 or cell_index[0] >= self.rows or cell_index[1] < 0 or cell_index[1] >= self.columns:
            return
        cell = self.cells[cell_index[0]][cell_index[1]]
        image_number = (cell.image_number + 1) % 3
        cell.image_number = image_number
        is_valid = self.validate_cell(cell, image_number)
        cell.set_image(image_number, self.image_width, is_valid)
        self.update_tents_qty(cell_index)
        pygame.display.update()

    def update_tents_qty(self, cell_index):
        tents_qty_in_row = len(
            [i for i in self.cells[cell_index[0]] if i.image_number == 2])
        tents_qty_in_column = len(
            [i for i in range(self.rows) if self.cells[i][cell_index[1]].image_number == 2])
        text_row = font.render(str(tents_qty_in_row),
                               True,
                               (0, 0, 0),
                               (255, 255, 255))
        text_row_rect = text_row.get_rect()
        text_row_rect.center = (self.width-self.image_width/2,
                                self.header_width+(cell_index[0]+1)*self.block_width+self.image_width/2)
        text_column = font.render(str(tents_qty_in_column),
                                  True,
                                  (0, 0, 0),
                                  (255, 255, 255))
        text_column_rect = text_column.get_rect()
        text_column_rect.center = (
            (cell_index[1]+1)*self.block_width+self.image_width/2, self.height - self.image_width/2)
        self.screen.blit(text_row, text_row_rect)
        self.screen.blit(text_column, text_column_rect)

    def position_to_index(self, position):
        return (position[1]-self.header_width) // self.block_width - 1, position[0] // self.block_width - 1

    def index_to_position(self, index):
        return ((index[1]+1)*self.block_width, (index[0]+1) * self.block_width+self.header_width)

    def validate_cell(self, cell, image_number):
        is_valid = True
        adjacent_cells = self.get_adjacent_cells(cell)
        if image_number == 2:  # tent
            for adjacent_cell in adjacent_cells:
                if adjacent_cell.image_number == 2:
                    adjacent_cell.set_image(2, self.image_width, False)
                    is_valid = False
        else:
            for adjacent_cell in adjacent_cells:
                if adjacent_cell.image_number == 2:
                    if self.validate_cell(adjacent_cell, 2):
                        adjacent_cell.set_image(2, self.image_width, True)
        return is_valid

    def get_adjacent_cells(self, cell):
        adjacent_cells = []
        indices_row = (0,)
        if cell.index[0] > 0:
            indices_row += (-1,)
        if cell.index[0] < self.rows - 1:
            indices_row += (1,)
        indices_column = (0,)
        if cell.index[1] > 0:
            indices_column += (-1,)
        if cell.index[1] < self.columns - 1:
            indices_column += (1,)
        for i in indices_row:
            for j in indices_column:
                if i != 0 or j != 0:
                    adjacent_cells.append(
                        self.cells[cell.index[0] + i][cell.index[1] + j])
        return adjacent_cells


class Cell:

    def __init__(self, screen, index, position, image_number):
        self.screen = screen
        self.index = index
        self.position = position
        self.image_number = image_number

    def set_image(self, image_number, image_width, is_valid):
        if image_number == 0:
            self.draw_background(self.position, (255, 255, 255), image_width)
        else:
            color = (255, 255, 255) if is_valid else (255, 0, 0)
            self.draw_background(self.position, color, image_width)
            if image_number == 1:
                image = pygame.image.load(r'src/tree.png')
            else:
                image = pygame.image.load(r'src/tent.png')
            self.screen.blit(image, self.position)

    def draw_background(self, position, color, image_width):
        pygame.draw.rect(self.screen, color, position +
                         (image_width, image_width))


class Menu:
    screen_width = 300
    screen_height = 300

    def __init__(self):
        self.screen = pygame.display.set_mode(
            (self.screen_width, self.screen_height))
        self.screen.fill((255, 255, 255))
        self.input_field_rows = InputField(self.screen, (50, 10), 30, 30, '8')
        self.input_field_columns = InputField(
            self.screen, (100, 10), 30, 30, '8')
        self.button = Button(self.screen, (50, 50), 130, 30, 'Create game')
        self.upload_file_button = Button(
            self.screen, (50, 100), 130, 30, 'Load game from file')
        pygame.display.set_caption('Tents Puzzle')
        pygame.display.update()


class InputField:

    def __init__(self, screen, position, width, height, content):
        self.screen = screen
        self.position = position
        self.width = width
        self.height = height
        self.content = content
        self.rect = pygame.draw.rect(
            self.screen, (0, 0, 0), self.position+(self.width, self.height), 1)
        self.draw_field()

    def draw_field(self):
        pygame.draw.rect(
            self.screen, (255, 255, 255), (self.position[0]+1, self.position[1]+1, self.width-2, self.height-2))
        content = font.render(self.content,
                              True,
                              (0, 0, 0),
                              (255, 255, 255))
        content_rect = content.get_rect()
        content_rect.center = self.rect.center  # center the number
        self.screen.blit(content, content_rect)


class Button:
    def __init__(self, screen, position, width, height, text):
        self.screen = screen
        self.position = position
        self.width = width
        self.height = height
        self.text = text
        self.rect = pygame.draw.rect(
            self.screen, (128, 128, 128), self.position+(self.width, self.height))
        self.draw_button()

    def draw_button(self):
        text = font.render(self.text,
                           True,
                           (0, 0, 0),
                           (128, 128, 128))
        text_rect = text.get_rect()
        text_rect.center = self.rect.center  # center the text
        self.screen.blit(text, text_rect)


def upload_file():
    root = tk.Tk()
    button = tk.Button(root, text='Open', command=load_game)
    button.pack()
    root.mainloop()


def load_game():
    file = filedialog.askopenfilename()
    with open(file) as f:
        for l in f:
            print(l)


def main():
    menu = Menu()
    key_numbers = [pygame.K_0, pygame.K_1, pygame.K_2, pygame.K_3, pygame.K_4,
                   pygame.K_5, pygame.K_6, pygame.K_7, pygame.K_8, pygame.K_9]
    run_menu = True
    run_game = False
    while run_menu or run_game:
        while run_menu:
            # Handle events
            for event in pygame.event.get():
                if event.type == pygame.MOUSEBUTTONUP:
                    position = pygame.mouse.get_pos()
                    if menu.input_field_rows.rect.collidepoint(position):
                        edit_rows = True
                    else:
                        edit_rows = False
                    if menu.input_field_columns.rect.collidepoint(position):
                        edit_columns = True
                    else:
                        edit_columns = False
                    if menu.button.rect.collidepoint(position):
                        rows = int(menu.input_field_rows.content)
                        columns = int(menu.input_field_columns.content)
                        game = Game(rows, columns)
                        run_menu = False
                        run_game = True
                    if menu.upload_file_button.rect.collidepoint(position):
                        upload_file()
                elif event.type == pygame.KEYDOWN and (edit_rows or edit_columns):
                    input_field = menu.input_field_rows if edit_rows else menu.input_field_columns
                    if event.key in key_numbers:
                        input_field.content += chr(event.key)
                    elif event.key == pygame.K_BACKSPACE:
                        if len(input_field.content) > 0:
                            input_field.content = input_field.content[:-1]
                    input_field.draw_field()
                    pygame.display.update()
                elif event.type == pygame.QUIT:
                    run_menu = False

        while run_game:
            # Handle events
            for event in pygame.event.get():
                if event.type == pygame.MOUSEBUTTONUP:
                    position = pygame.mouse.get_pos()
                    if game.back_button.rect.collidepoint(position):
                        menu = Menu()
                        run_game = False
                        run_menu = True
                    else:
                        game.change_cell(position)
                elif event.type == pygame.QUIT:
                    run_game = False

    pygame.quit()


main()
