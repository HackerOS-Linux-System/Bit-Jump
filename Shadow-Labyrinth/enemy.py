import pygame
import math
import random

TILE_SIZE = 40
ENEMY_SPEED = 2
HEARING_RANGE = 250
SIGHT_RANGE = 180
PATROL_SPEED = 1

class Enemy(pygame.sprite.Sprite):
    def __init__(self, color, walls, damage=10):
        super().__init__()
        self.image = pygame.Surface((TILE_SIZE // 2, TILE_SIZE // 2))
        self.image.fill(color)
        self.rect = self.image.get_rect(center=self.random_position(walls))
        self.target = None
        self.damage = damage

    def random_position(self, walls):
        pos = (random.randint(TILE_SIZE, 800 - TILE_SIZE), random.randint(TILE_SIZE, 600 - TILE_SIZE))
        while any(self.rect.colliderect(wall.rect) for wall in walls):
            pos = (random.randint(TILE_SIZE, 800 - TILE_SIZE), random.randint(TILE_SIZE, 600 - TILE_SIZE))
        return pos

    def move_towards(self, target_pos, walls, speed=ENEMY_SPEED):
        dx = target_pos[0] - self.rect.centerx
        dy = target_pos[1] - self.rect.centery
        dist = math.hypot(dx, dy)
        if dist > 0:
            dx, dy = dx / dist * speed, dy / dist * speed
            new_rect = self.rect.move(dx, dy)
            if not any(new_rect.colliderect(wall.rect) for wall in walls):
                self.rect = new_rect

    def is_in_light(self, light_positions):
        for pos, radius in light_positions:
            dist = math.hypot(self.rect.centerx - pos[0], self.rect.centery - pos[1])
            if dist < radius:
                return True
        return False

class SoundEnemy(Enemy):
    def __init__(self, walls):
        super().__init__((255, 0, 0), walls, damage=15)
        self.heard_sounds = []  # List of (pos, strength)

    def update(self, player, walls, sound_pos, light_positions):
        if sound_pos:
            dist = math.hypot(self.rect.centerx - sound_pos[0], self.rect.centery - sound_pos[1])
            if dist < HEARING_RANGE:
                strength = HEARING_RANGE - dist
                self.heard_sounds.append((sound_pos, strength))
        if self.heard_sounds:
            self.heard_sounds = [(pos, str - 1) for pos, str in self.heard_sounds if str > 0]
            if self.heard_sounds:
                strongest = max(self.heard_sounds, key=lambda x: x[1])
                self.move_towards(strongest[0], walls)
        elif self.is_in_light(light_positions):
            self.move_towards(player.rect.center, walls)

class LightEnemy(Enemy):
    def __init__(self, walls):
        super().__init__((0, 0, 255), walls, damage=20)

    def update(self, player, walls, sound_pos, light_positions):
        if self.is_in_light(light_positions):
            self.move_towards(player.rect.center, walls, speed=ENEMY_SPEED * 1.2)
        elif sound_pos and random.random() < 0.5:
            dist = math.hypot(self.rect.centerx - sound_pos[0], self.rect.centery - sound_pos[1])
            if dist < HEARING_RANGE / 2:
                self.move_towards(sound_pos, walls)

class PatrolEnemy(Enemy):
    def __init__(self, walls):
        super().__init__((0, 255, 0), walls, damage=10)
        self.patrol_points = [self.random_position(walls) for _ in range(4)]
        self.current_patrol = 0

    def update(self, player, walls, sound_pos, light_positions):
        if self.is_in_light(light_positions):
            self.move_towards(player.rect.center, walls)
        elif sound_pos:
            dist = math.hypot(self.rect.centerx - sound_pos[0], self.rect.centery - sound_pos[1])
            if dist < HEARING_RANGE:
                self.move_towards(sound_pos, walls)
        else:
            # Patrol
            target = self.patrol_points[self.current_patrol]
            self.move_towards(target, walls, speed=PATROL_SPEED)
            if math.hypot(self.rect.centerx - target[0], self.rect.centery - target[1]) < 10:
                self.current_patrol = (self.current_patrol + 1) % len(self.patrol_points)
