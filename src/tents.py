import pygame
pygame.init()

SIZE = [500, 500]
screen = pygame.display.set_mode(SIZE)
image_tree = pygame.image.load(r'src/tree.png')
screen.blit(image_tree, (0, 0))
pygame.display.update()


class Tents:
    def __init__(self):
        self.color_list = [(255, 0, 0), (0, 255, 0)]
        self.color_left = 0
        self.color_right = 1

    def change_cell(self, pos):
        if pos[0] < 250:
            self.color_left = (self.color_left + 1) % 2
            pygame.draw.rect(
                screen, self.color_list[self.color_left], (0, 0, 250, 250), 0)
        else:
            self.color_right = (self.color_right + 1) % 2
            pygame.draw.rect(
                screen, self.color_list[self.color_right], (250, 0, 250, 250), 0)
        pygame.display.update()


def main():
    tents = Tents()
    running = True
    while running:

        # Handle events
        for event in pygame.event.get():
            if event.type == pygame.MOUSEBUTTONUP:
                pos = pygame.mouse.get_pos()
                tents.change_cell(pos)

            if event.type == pygame.QUIT:
                running = False

    # Done! Time to quit.
    pygame.quit()


main()
