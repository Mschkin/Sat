import pygame
pygame.init()


def draw_tree(screen, position):
    image_tree = pygame.image.load(r'src/tree.png')
    screen.blit(image_tree, position)


def draw_tent(screen, position):
    image_tree = pygame.image.load(r'src/tent.png')
    screen.blit(image_tree, position)


def draw_empty(screen, position):
    pygame.draw.rect(screen, (255, 255, 255), position + (24, 24))


class Game:
    grid_width = 1
    image_size = 24
    block_size = image_size+grid_width
    font = pygame.font.SysFont('Monospace', 16)

    def __init__(self, rows, columns):
        self.rows = rows
        self.columns = columns
        self.width = (columns + 1) * self.block_size
        self.height = (rows+1)*self.block_size
        self.screen = pygame.display.set_mode((self.width, self.height))
        self.screen.fill((255, 255, 255))
        self.draw_grid()
        self.cells = [[Cell(self.screen, (self.grid_width+i*self.block_size, self.grid_width+j * (
            self.block_size)), 2) for i in range(self.columns)] for j in range(self.rows)]
        pygame.display.set_caption('Tents Puzzle')
        pygame.display.update()

    def draw_grid(self):
        for row in range(self.rows+1):
            pygame.draw.line(self.screen, (0, 0, 0),
                             (0, row*self.block_size), (self.width-self.image_size, row*self.block_size), self.grid_width)

        for column in range(self.columns+1):
            pygame.draw.line(self.screen, (0, 0, 0), (column*self.block_size,
                                                      0), (column*self.block_size, self.height-self.image_size), self.grid_width)

    def change_cell(self, position):
        cell_index = self.position_to_index(position)
        if cell_index[0] >= self.rows or cell_index[1] >= self.columns:
            return
        self.cells[cell_index[0]][cell_index[1]].set_next_image()
        self.update_tents_qty(position)
        pygame.display.update()

    def update_tents_qty(self, position):
        cell_index = self.position_to_index(position)
        tents_qty_in_row = len(
            [i for i in self.cells[cell_index[0]] if i.image_number == 1])
        tents_qty_in_column = len(
            [i for i in range(self.rows) if self.cells[i][cell_index[1]].image_number == 1])
        text_row = self.font.render(str(tents_qty_in_row),
                                    True,
                                    (0, 0, 0),
                                    (255, 255, 255))
        text_row_rect = text_row.get_rect()
        text_row_rect.center = (self.width-self.image_size+self.image_size/2,
                                cell_index[0]*self.block_size+self.image_size/2)
        text_column = self.font.render(str(tents_qty_in_column),
                                       True,
                                       (0, 0, 0),
                                       (255, 255, 255))
        text_column_rect = text_column.get_rect()
        text_column_rect.center = (
            cell_index[1]*self.block_size+self.image_size/2, self.height - self.image_size+self.image_size/2)
        self.screen.blit(text_row, text_row_rect)
        self.screen.blit(text_column, text_column_rect)

    def position_to_index(self, position):
        return position[1]//self.block_size, position[0] // self.block_size


class Cell:
    draw_functions = [draw_tree, draw_tent, draw_empty]

    def __init__(self, screen, position, image_number):
        self.screen = screen
        self.position = position
        self.image_number = image_number

    def set_next_image(self):
        self.image_number = (self.image_number + 1) % 3
        self.draw_functions[self.image_number](self.screen, self.position)


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
