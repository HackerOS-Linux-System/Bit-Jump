import pygame
import random

TILE_SIZE = 40
PLAYER_SPEED = 3
FLASHLIGHT_RADIUS = 150
TORCH_RADIUS = 100
TORCH_DURATION = 300  # frames
SPEED_BOOST_DURATION = 600  # frames
HEALTH_PACK_HEAL = 50

class Player(pygame.sprite.Sprite):
    def __init__(self, x, y):
        super().__init__()
        self.image = pygame.Surface((TILE_SIZE // 2, TILE_SIZE // 2))
        self.image.fill((255, 255, 0))  # Yellow
        self.rect = self.image.get_rect(center=(x, y))
        self.health = 100
        self.battery = 100
        self.torches = 0
        self.active_torches = []  # List of (pos, timer)
        self.moving = False
        self.flashlight_on = True
        self.speed_boost = 0
        self.speed_timer = 0
        self.base_speed = PLAYER_SPEED

    def update(self, walls, items):
        self.moving = False
        keys = pygame.key.get_pressed()
        dx, dy = 0, 0
        current_speed = self.base_speed + self.speed_boost
        if keys[pygame.K_LEFT] or keys[pygame.K_a]:
            dx = -current_speed
        if keys[pygame.K_RIGHT] or keys[pygame.K_d]:
            dx = current_speed
        if keys[pygame.K_UP] or keys[pygame.K_w]:
            dy = -current_speed
        if keys[pygame.K_DOWN] or keys[pygame.K_s]:
            dy = current_speed

        if dx or dy:
            self.move(dx, dy, walls)
            self.moving = True

        # Flashlight drain
        if self.flashlight_on and self.battery > 0:
            self.battery -= 0.05
            if self.battery < 0:
                self.battery = 0
                self.flashlight_on = False

        # Toggle flashlight
        if keys[pygame.K_f]:
            self.flashlight_on = not self.flashlight_on

        # Place torch
        if keys[pygame.K_t] and self.torches > 0:
            self.active_torches.append((self.rect.center, TORCH_DURATION))
            self.torches -= 1

        # Update torches
        self.active_torches = [(pos, timer - 1) for pos, timer in self.active_torches if timer > 0]

        # Update speed boost
        if self.speed_timer > 0:
            self.speed_timer -= 1
        else:
            self.speed_boost = 0

    def move(self, dx, dy, walls):
        new_rect = self.rect.move(dx, dy)
        if not any(new_rect.colliderect(wall.rect) for wall in walls):
            self.rect = new_rect

    def get_sound_position(self):
        return (self.rect.centerx + random.randint(-20, 20), self.rect.centery + random.randint(-20, 20))

    def get_light_positions(self):
        positions = []
        if self.flashlight_on:
            positions.append((self.rect.centerx, self.rect.centery, FLASHLIGHT_RADIUS))
        for pos, _ in self.active_torches:
            positions.append((pos[0], pos[1], TORCH_RADIUS))
        return positions

    def collect_item(self, item):
        points = 10
        if item.type == 'battery':
            self.battery = min(100, self.battery + 50)
        elif item.type == 'torch':
            self.torches += 1
        elif item.type == 'health':
            self.health = min(100, self.health + HEALTH_PACK_HEAL)
            points = 20
        elif item.type == 'speed':
            self.speed_boost = 2
            self.speed_timer = SPEED_BOOST_DURATION
            points = 15
        return points

    def take_damage(self, amount):
        self.health -= amount
        if self.health < 0:
            self.health = 0
