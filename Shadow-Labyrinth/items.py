import pygame
import random

TILE_SIZE = 40

class Item(pygame.sprite.Sprite):
    def __init__(self, item_type, walls):
        super().__init__()
        self.type = item_type
        colors = {
            'battery': (0, 255, 255),  # Cyan
            'torch': (255, 165, 0),    # Orange
            'health': (255, 0, 0),     # Red
            'speed': (0, 255, 0)       # Green
        }
        self.image = pygame.Surface((TILE_SIZE // 4, TILE_SIZE // 4))
        self.image.fill(colors[item_type])
        self.rect = self.image.get_rect(center=self.random_position(walls))

    def random_position(self, walls):
        pos = (random.randint(TILE_SIZE, 800 - TILE_SIZE), random.randint(TILE_SIZE, 600 - TILE_SIZE))
        temp_rect = self.rect.copy()
        temp_rect.center = pos
        while any(temp_rect.colliderect(wall.rect) for wall in walls):
            pos = (random.randint(TILE_SIZE, 800 - TILE_SIZE), random.randint(TILE_SIZE, 600 - TILE_SIZE))
            temp_rect.center = pos
        return pos
