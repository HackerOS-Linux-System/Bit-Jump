import pygame
import random

TILE_SIZE = 40
WALL_COLOR = (255, 255, 255)

class Wall(pygame.sprite.Sprite):
    def __init__(self, x, y):
        super().__init__()
        self.image = pygame.Surface((TILE_SIZE, TILE_SIZE))
        self.image.fill(WALL_COLOR)
        self.rect = self.image.get_rect(topleft=(x * TILE_SIZE, y * TILE_SIZE))

def generate_maze(width, height):
    maze = [[1] * width for _ in range(height)]
    def carve(x, y):
        maze[y][x] = 0
        dirs = [(0, -2), (0, 2), (-2, 0), (2, 0)]
        random.shuffle(dirs)
        for dx, dy in dirs:
            nx, ny = x + dx, y + dy
            if 0 <= nx < width and 0 <= ny < height and maze[ny][nx] == 1:
                maze[y + dy // 2][x + dx // 2] = 0
                carve(nx, ny)
    carve(1, 1)
    maze[1][1] = 0
    maze[height - 2][width - 2] = 0

    walls = pygame.sprite.Group()
    for y in range(height):
        for x in range(width):
            if maze[y][x] == 1:
                walls.add(Wall(x, y))
    return walls
