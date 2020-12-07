import pygame
pygame.init()


class Game:
    grid_width = 1
    image_size = 32
    menu_size = 32
    block_size = image_size+grid_width
    font = pygame.font.SysFont('Monospace', 16)

    def __init__(self, rows, columns):
        self.rows = rows
        self.columns = columns
        self.width = self.image_size+(columns + 1) * self.block_size
        self.height = self.menu_size+self.image_size+(rows+1)*self.block_size
        self.screen = pygame.display.set_mode((self.width, self.height))
        self.screen.fill((255, 255, 255))
        self.draw_grid()
        self.cells = [[Cell(self.screen, (i, j), self.index_to_position((i, j)), 0)
                       for j in range(self.columns)] for i in range(self.rows)]
        pygame.display.set_caption('Tents Puzzle')
        pygame.display.update()

    def draw_grid(self):
        for row in range(self.rows+1):
            pygame.draw.line(self.screen, (0, 0, 0),
                             (self.image_size, self.menu_size+self.image_size+row*self.block_size), (self.width-self.block_size, self.menu_size+self.image_size+row*self.block_size), self.grid_width)

        for column in range(self.columns+1):
            pygame.draw.line(self.screen, (0, 0, 0), (self.image_size+column*self.block_size,
                                                      self.menu_size+self.image_size), (self.image_size+column*self.block_size, self.height-self.block_size), self.grid_width)

    def change_cell(self, position):
        cell_index = self.position_to_index(position)
        if cell_index[0] < 0 or cell_index[0] >= self.rows or cell_index[1] < 0 or cell_index[1] >= self.columns:
            return
        cell = self.cells[cell_index[0]][cell_index[1]]
        image_number = (cell.image_number + 1) % 3
        cell.image_number = image_number
        is_valid = self.validate_cell(cell, image_number)
        cell.set_image(image_number, self.image_size, is_valid)
        self.update_tents_qty(cell_index)
        pygame.display.update()

    def update_tents_qty(self, cell_index):
        tents_qty_in_row = len(
            [i for i in self.cells[cell_index[0]] if i.image_number == 2])
        tents_qty_in_column = len(
            [i for i in range(self.rows) if self.cells[i][cell_index[1]].image_number == 2])
        text_row = self.font.render(str(tents_qty_in_row),
                                    True,
                                    (0, 0, 0),
                                    (255, 255, 255))
        text_row_rect = text_row.get_rect()
        text_row_rect.center = (self.width-self.image_size/2,
                                self.menu_size+(cell_index[0]+1)*self.block_size+self.image_size/2)
        text_column = self.font.render(str(tents_qty_in_column),
                                       True,
                                       (0, 0, 0),
                                       (255, 255, 255))
        text_column_rect = text_column.get_rect()
        text_column_rect.center = (
            (cell_index[1]+1)*self.block_size+self.image_size/2, self.height - self.image_size/2)
        self.screen.blit(text_row, text_row_rect)
        self.screen.blit(text_column, text_column_rect)

    def position_to_index(self, position):
        return (position[1]-self.menu_size) // self.block_size - 1, position[0] // self.block_size - 1

    def index_to_position(self, index):
        return ((index[1]+1)*self.block_size, (index[0]+1) * (
            self.block_size)+self.menu_size)

    def validate_cell(self, cell, image_number):
        is_valid = True
        adjacent_cells = self.get_adjacent_cells(cell)
        if image_number == 2:  # tent
            for adjacent_cell in adjacent_cells:
                if adjacent_cell.image_number == 2:
                    adjacent_cell.set_image(2, self.image_size, False)
                    is_valid = False
        else:
            for adjacent_cell in adjacent_cells:
                if adjacent_cell.image_number == 2:
                    if self.validate_cell(adjacent_cell, 2):
                        adjacent_cell.set_image(2, self.image_size, True)
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

    def set_image(self, image_number, image_size, is_valid):
        if image_number == 0:
            self.draw_background(self.position, (255, 255, 255), image_size)
        else:
            color = (255, 255, 255) if is_valid else (255, 0, 0)
            self.draw_background(self.position, color, image_size)
            if image_number == 1:
                image = pygame.image.load(r'src/tree.png')
            else:
                image = pygame.image.load(r'src/tent.png')
            self.screen.blit(image, self.position)

    def draw_background(self, position, color, image_size):
        pygame.draw.rect(self.screen, color, position +
                         (image_size, image_size))


def main():
    game = Game(8, 10)
    running = True
    while running:

        # Handle events
        for event in pygame.event.get():
            if event.type == pygame.MOUSEBUTTONUP:
                position = pygame.mouse.get_pos()
                game.change_cell(position)

            if event.type == pygame.QUIT:
                running = False

    pygame.quit()


main()
