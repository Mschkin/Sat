import pygame
import subprocess
import numpy as np

pygame.init()

# global variable
font = pygame.font.Font(None, 24)
button_font = pygame.font.Font(None, 35)
BLACK = (0, 0, 0)
RED = (255, 0, 0)
GREY = (105, 105, 105)
WHITE = (255, 255, 255)
GREEN = (0, 255, 0)
EMPTY_NUMBER = 0
TREE_NUMBER = 1
TENT_NUMBER = 2


class Game:
    grid_width = 1
    image_width = 32
    header_width = 32
    block_width = image_width + grid_width

    def __init__(self, rows, columns, play_mode=False, cells_content=[]):
        self.rows = rows
        self.columns = columns
        self.tents_qty_in_rows = [0]*self.rows
        self.tents_qty_in_columns = [0]*self.columns
        self.play_mode = play_mode
        self.width = self.image_width+(columns + 1) * self.block_width
        self.height = self.header_width + \
            self.image_width+(rows+1)*self.block_width
        self.screen = pygame.display.set_mode((self.width, self.height))
        self.screen.fill(WHITE)
        self.draw_grid()
        if not play_mode:
            self.cells = [[Cell(self.screen, (i, j), self.index_to_position((i, j)), EMPTY_NUMBER, self.image_width)
                           for j in range(self.columns)] for i in range(self.rows)]
            for i in range(self.rows):
                self.draw_tents_qty_in_row(i, 0)
            for j in range(self.columns):
                self.draw_tents_qty_in_column(j, 0)
            self.solve_or_create_button = Button(
                self.screen, (self.width-self.image_width-80, self.image_width/2), 80, 30, 'Create')
        else:
            self.cells = [[Cell(self.screen, (i, j), self.index_to_position((i, j)), cells_content[i][j], self.image_width)
                           for j in range(self.columns)] for i in range(self.rows)]
            for index_row, row in enumerate(cells_content[:-1]):
                self.tents_qty_in_rows[index_row] = row[-1]
                self.draw_tents_qty_in_row(index_row, row[-1])
            for index_column, number in enumerate(cells_content[-1]):
                self.tents_qty_in_columns[index_column] = number
                self.draw_tents_qty_in_column(index_column, number)
            self.reset_button = Button(
                self.screen, (self.width/2-40, self.image_width/2), 80, 30, 'Reset')
            self.solve_or_create_button = Button(
                self.screen, (self.width-self.image_width-80, self.image_width/2), 80, 30, 'Solve')
        self.back_button = Button(
            self.screen, (self.image_width, self.image_width / 2), 80, 30, 'Back')
        pygame.display.set_caption(
            'Create game') if not play_mode else pygame.display.set_caption('Play game')
        pygame.display.update()

    def draw_grid(self):
        for row in range(self.rows+1):
            pygame.draw.line(self.screen, BLACK,
                             (self.image_width, self.header_width+self.image_width+row*self.block_width), (self.width-self.block_width, self.header_width+self.image_width+row*self.block_width), self.grid_width)

        for column in range(self.columns+1):
            pygame.draw.line(self.screen, BLACK, (self.image_width+column*self.block_width,
                                                  self.header_width+self.image_width), (self.image_width+column*self.block_width, self.height-self.block_width), self.grid_width)

    def change_cell(self, position):
        cell_index = self.position_to_index(position)
        # position outside of board
        if cell_index[0] < 0 or cell_index[0] >= self.rows or cell_index[1] < 0 or cell_index[1] >= self.columns:
            return
        cell = self.cells[cell_index[0]][cell_index[1]]
        if self.play_mode:
            if cell.image_number == TREE_NUMBER:
                return
            else:
                image_number = EMPTY_NUMBER if cell.image_number == TENT_NUMBER else TENT_NUMBER
        else:
            image_number = (cell.image_number + 1) % 3
        cell.image_number = image_number
        cell.is_valid = self.validate_cell(cell)
        cell.draw_image()
        adjacent_cells = self.get_adjacent_cells(cell)
        for adjacent_cell in adjacent_cells:
            adjacent_cell.is_valid = self.validate_cell(adjacent_cell)
            adjacent_cell.draw_image()
        for cell_in_row in self.cells[cell.index[0]]:
            if cell_in_row not in adjacent_cells and cell_in_row != cell:
                cell_in_row.is_valid = self.validate_cell(cell_in_row)
                cell_in_row.draw_image()
        for cells in self.cells:
            cell_in_column = cells[cell_index[1]]
            if cell_in_column not in adjacent_cells and cell_in_column != cell:
                cell_in_column.is_valid = self.validate_cell(
                    cell_in_column)
                cell_in_column.draw_image()
        if not self.play_mode:
            tents_qty = self.get_current_tents_qty(cell_index)
            self.tents_qty_in_rows[cell_index[0]] = tents_qty[0]
            self.tents_qty_in_columns[cell_index[1]] = tents_qty[1]
            self.draw_tents_qty_in_row(cell_index[0], tents_qty[0])
            self.draw_tents_qty_in_column(
                cell_index[1], tents_qty[1])
        pygame.display.update()

    def get_current_tents_qty(self, cell_index):
        tents_qty_in_row = len(
            [i for i in self.cells[cell_index[0]] if i.image_number == TENT_NUMBER])
        tents_qty_in_column = len(
            [i for i in range(self.rows) if self.cells[i][cell_index[1]].image_number == TENT_NUMBER])
        return tents_qty_in_row, tents_qty_in_column

    def draw_tents_qty_in_row(self, index_row, number):
        number = font.render(str(number),
                             True,
                             BLACK)
        number_rect = number.get_rect()
        number_rect.center = (self.width-self.image_width/2,
                              self.header_width+(index_row+1)*self.block_width+self.image_width/2)
        # clear old number
        pygame.draw.rect(self.screen, WHITE, (self.width-self.image_width,
                                              self.header_width + (index_row + 1)*self.block_width, self.image_width, self.image_width))
        # draw new number
        self.screen.blit(number, number_rect)

    def draw_tents_qty_in_column(self, index_column, number):
        number = font.render(str(number),
                             True,
                             BLACK)
        number_rect = number.get_rect()
        number_rect.center = (
            (index_column + 1)*self.block_width + self.image_width / 2, self.height - self.image_width / 2)
        # clear old number
        pygame.draw.rect(self.screen, WHITE, ((index_column + 1)*self.block_width,
                                              self.height - self.image_width, self.image_width, self.image_width))
        # draw new number
        self.screen.blit(number, number_rect)

    def position_to_index(self, position):
        return (position[1]-self.header_width) // self.block_width - 1, position[0] // self.block_width - 1

    def index_to_position(self, index):
        return ((index[1]+1)*self.block_width, (index[0]+1) * self.block_width+self.header_width)

    def validate_cell(self, cell):
        adjacent_cells = self.get_adjacent_cells(cell)
        if cell.image_number == TENT_NUMBER:
            for adjacent_cell in adjacent_cells:
                if adjacent_cell.image_number == TENT_NUMBER:
                    return False
        if self.play_mode:
            tents_qty = self.get_current_tents_qty(cell.index)
            if tents_qty[0] > self.tents_qty_in_rows[cell.index[0]] or tents_qty[1] > self.tents_qty_in_columns[cell.index[1]]:
                return False
        return True

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

    def create_puzzel(self):
        if len(self.get_solution()) == 0:
            self.render_message('UNSAT', RED)
            pygame.display.update()
            return
        for cells in self.cells:
            for cell in cells:
                if cell.image_number == TENT_NUMBER:
                    cell.image_number = EMPTY_NUMBER
                    cell.draw_image()
        self.save_puzzle_and_play()
        pygame.display.update()

    def generate_random_puzzle(self):
        possible_tents = list(range(self.rows * self.columns))
        while len(possible_tents) > 0:
            next_tent_number = possible_tents[np.random.randint(
                len(possible_tents))]
            next_cell = self.cells[next_tent_number //
                                   self.columns][next_tent_number % self.columns]
            possible_trees = []
            for cell in self.get_adjacent_cells(next_cell):
                if cell.index[0] * self.columns + cell.index[1] in possible_tents:
                    possible_tents.remove(
                        cell.index[0] * self.columns + cell.index[1])
                if cell.index[0] == next_cell.index[0] or cell.index[1] == next_cell.index[1]:
                    if cell.image_number != TREE_NUMBER:
                        possible_trees.append(cell)
            if len(possible_trees) > 0:
                possible_trees[np.random.randint(
                    len(possible_trees))].image_number = TREE_NUMBER
                next_cell.image_number = TENT_NUMBER
            possible_tents.remove(next_tent_number)
        for row in range(self.rows):
            self.tents_qty_in_rows[row] = len(
                [i for i in self.cells[row] if i.image_number == TENT_NUMBER])
            self.draw_tents_qty_in_row(row, self.tents_qty_in_rows[row])
        for column in range(self.columns):
            self.tents_qty_in_columns[column] = len([i for i in range(
                self.rows) if self.cells[i][column].image_number == TENT_NUMBER])
            self.draw_tents_qty_in_column(
                column, self.tents_qty_in_columns[column])
        for cells in self.cells:
            for cell in cells:
                if cell.image_number == TREE_NUMBER:
                    cell.draw_image()
        self.save_puzzle_and_play()
        pygame.display.update()

    def save_puzzle_and_play(self):
        content = f'{self.rows} {self.columns}\n'
        for row, cells in enumerate(self.cells):
            content += ' '.join(['.' if cell.image_number == EMPTY_NUMBER or cell.image_number ==
                                 TENT_NUMBER else 'T' for cell in cells]) + f' {self.tents_qty_in_rows[row]}\n'
        content += ' '.join([str(n) for n in self.tents_qty_in_columns])
        with open('src/tents.txt', 'w') as file:
            file.write(content)
        self.play_mode = True
        self.solve_or_create_button = Button(
            self.screen, (self.width - self.image_width - 80, self.image_width / 2), 80, 30, 'Solve')
        self.reset_button = Button(
            self.screen, (self.width/2-40, self.image_width/2), 80, 30, 'Reset')

    def solve_puzzle(self):
        solution = self.get_solution()
        if len(solution) > 0:
            self.reset_game()
            for index in solution:
                cell = self.cells[index[0]][index[1]]
                cell.image_number = TENT_NUMBER
                cell.draw_image()
        else:
            self.render_message('UNSAT', RED)
        if self.validate_solution():
            self.render_message('SAT', GREEN)
            print('SAT')
        pygame.display.update()

    def get_solution(self):
        try:
            solution = subprocess.run(
                ['target/release/tents'], capture_output=True)
            return eval(solution.stdout[:-1])
        except:
            return []

    def render_message(self, text, color):
        text = button_font.render(text, True, color)
        rect = text.get_rect()
        rect.center = (self.width/2, self.height/2)
        self.screen.blit(text, rect)

    def reset_game(self):
        # remove tents and redraw unvalid cells
        for cells in self.cells:
            for cell in cells:
                if cell.image_number == TENT_NUMBER:
                    cell.image_number = EMPTY_NUMBER
                    if not cell.is_valid:
                        cell.is_valid = True
                    cell.draw_image()
                elif not cell.is_valid:
                    cell.is_valid = True
                    cell.draw_image()

    def validate_solution(self):
        for row in range(self.rows):
            if len([i for i in self.cells[row] if i.image_number == TENT_NUMBER]) != self.tents_qty_in_rows[row]:
                return False
        for column in range(self.columns):
            if len([i for i in range(self.rows) if self.cells[i][column].image_number == TENT_NUMBER]) != self.tents_qty_in_columns[column]:
                return False
        return True


class Cell:
    is_valid = True

    def __init__(self, screen, index, position, image_number, image_width):
        self.screen = screen
        self.index = index
        self.position = position
        self.image_number = image_number
        self.image_width = image_width
        self.draw_image()

    def draw_image(self):
        color = WHITE if self.is_valid else RED
        self.draw_background(self.position, color)
        if self.image_number != EMPTY_NUMBER:
            if self.image_number == TREE_NUMBER:
                image = pygame.image.load(r'src/tree.png')
            else:
                image = pygame.image.load(r'src/tent.png')
            self.screen.blit(image, self.position)

    def draw_background(self, position, color):
        pygame.draw.rect(self.screen, color, position +
                         (self.image_width, self.image_width))


class Menu:
    screen_width = 360
    screen_height = 300

    def __init__(self):
        self.screen = pygame.display.set_mode(
            (self.screen_width, self.screen_height))
        self.screen.fill(BLACK)
        self.screen.blit(font.render('Please enter the puzzle size:',
                                     True,
                                     (100, 100, 100)), (50, 20))
        self.input_field_rows = InputField(self.screen, (50, 50), 30, 30, '8')
        self.input_field_columns = InputField(
            self.screen, (100, 50), 30, 30, '8')
        self.screen.blit(font.render('X',
                                     True,
                                     (100, 100, 100)), (85, 57))
        self.generate_manu_button = MenuButton(
            self.screen, (50, 100), 'Create game manually')
        self.generate_auto_button = MenuButton(
            self.screen, (50, 140), 'Generate random game')
        self.load_game_button = MenuButton(
            self.screen, (50, 180), 'Load game from file')
        self.input_field_file = InputField(
            self.screen, (50, 230), 230, 30, 'src/tents.txt')
        pygame.display.set_caption('Tents Puzzle')
        pygame.display.update()

    def load_game(self, file_path):
        with open(file_path) as file:
            file = list(file)
            with open('src/tents.txt', 'w') as new_file:
                new_file.write(''.join(file))
            contents = [[n.rstrip() for n in line.split(' ')]
                        for line in file if line.strip() != '']
        rows, columns = contents[0]
        contents_new = []
        for content in contents[1:]:
            content_new = []
            for value in content:
                if '.' in value or 'T' in value:
                    content_new += list(value)
                else:
                    content_new.append(value)
            contents_new.append(content_new)
        cells_content = [[0 if n == '.' else (
            1 if n == 'T' else int(n)) for n in row] for row in contents_new]
        return Game(int(rows), int(columns), True, cells_content)


class InputField:

    def __init__(self, screen, position, width, height, content):
        self.screen = screen
        self.position = position
        self.width = width
        self.height = height
        self.content = content
        self.rect = pygame.draw.rect(
            self.screen, BLACK, self.position+(self.width, self.height), 1)
        self.draw_field()

    def draw_field(self):
        self.rect = pygame.draw.rect(
            self.screen, WHITE, self.position+(self.width, self.height))
        content = font.render(self.content,
                              True,
                              BLACK)
        content_rect = content.get_rect()
        content_rect.center = self.rect.center  # center the number
        self.screen.blit(content, content_rect)


class MenuButton:
    hovered = False

    def __init__(self, screen, position, text):
        self.screen = screen
        self.position = position
        self.text = text
        self.draw_button()

    def draw_button(self):
        text = button_font.render(self.text,
                                  True,
                                  self.get_color())
        self.rect = text.get_rect()
        self.rect.topleft = self.position
        self.screen.blit(text, self.rect)

    def get_color(self):
        return (150, 150, 150) if self.hovered else(100, 100, 100)


class Button:
    def __init__(self, screen, position, width, height, text):
        self.screen = screen
        self.position = position
        self.width = width
        self.height = height
        self.text = text
        self.rect = pygame.draw.rect(
            self.screen, GREY, self.position+(self.width, self.height))
        self.draw_button()

    def draw_button(self):
        text = font.render(self.text,
                           True,
                           WHITE)
        text_rect = text.get_rect()
        text_rect.center = self.rect.center  # center the text
        self.screen.blit(text, text_rect)


def main():
    menu = Menu()
    key_numbers = [pygame.K_0, pygame.K_1, pygame.K_2, pygame.K_3, pygame.K_4,
                   pygame.K_5, pygame.K_6, pygame.K_7, pygame.K_8, pygame.K_9]
    run_menu = True
    run_game = False
    while run_menu or run_game:
        while run_menu:
            for button in [menu.generate_manu_button, menu.generate_auto_button, menu.load_game_button]:
                change_hover = False
                if button.rect.collidepoint(pygame.mouse.get_pos()):  # on hover
                    if not button.hovered:
                        button.hovered = True
                        change_hover = True
                else:
                    if button.hovered:
                        button.hovered = False
                        change_hover = True
                if change_hover:
                    button.draw_button()
                    pygame.display.update()
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
                    if menu.input_field_file.rect.collidepoint(position):
                        edit_file_path = True
                    else:
                        edit_file_path = False
                    if menu.generate_manu_button.rect.collidepoint(position):
                        rows = int(menu.input_field_rows.content)
                        columns = int(menu.input_field_columns.content)
                        game = Game(rows, columns)
                        run_menu = False
                        run_game = True
                    if menu.generate_auto_button.rect.collidepoint(position):
                        rows = int(menu.input_field_rows.content)
                        columns = int(menu.input_field_columns.content)
                        game = Game(rows, columns)
                        game.generate_random_puzzle()
                        run_menu = False
                        run_game = True
                    elif menu.load_game_button.rect.collidepoint(position):
                        file_path = menu.input_field_file.content
                        game = menu.load_game(file_path)
                        run_menu = False
                        run_game = True
                elif event.type == pygame.KEYDOWN and (edit_rows or edit_columns or edit_file_path):
                    if edit_rows:
                        input_field = menu.input_field_rows
                    elif edit_columns:
                        input_field = menu.input_field_columns
                    else:
                        input_field = menu.input_field_file
                    if event.key in key_numbers:
                        input_field.content += chr(event.key)
                    elif event.key == pygame.K_BACKSPACE:
                        if len(input_field.content) > 0:
                            input_field.content = input_field.content[:-1]
                    else:  # none numberic inputs only allowed for input_field_file
                        if edit_file_path:
                            input_field.content += chr(event.key)
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
                    elif game.solve_or_create_button.rect.collidepoint(position):
                        if game.play_mode:
                            game.solve_puzzle()
                        else:  # create
                            game.create_puzzel()
                    elif game.play_mode and game.reset_button.rect.collidepoint(position):
                        game.reset_game()
                        pygame.display.update()
                    else:
                        game.change_cell(position)
                elif event.type == pygame.QUIT:
                    run_game = False

    pygame.quit()


main()
