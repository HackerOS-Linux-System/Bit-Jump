import pygame
import sys
from player import Player
from enemy import SoundEnemy, LightEnemy, PatrolEnemy
from maze import generate_maze
from lighting import draw_lighting
from items import Item
from hud import draw_hud
from utils import draw_text, LEVELS

# Initialization
pygame.init()

# Constants
SCREEN_WIDTH = 800
SCREEN_HEIGHT = 600
TILE_SIZE = 40
FPS = 60
MAX_LEVEL = 5

# Colors
BLACK = (0, 0, 0)
WHITE = (255, 255, 255)

# Screen
screen = pygame.display.set_mode((SCREEN_WIDTH, SCREEN_HEIGHT))
pygame.display.set_caption("Shadow Labyrinth")

# Main game function
def main():
    clock = pygame.time.Clock()
    current_level = 1
    score = 0
    paused = False

    while current_level <= MAX_LEVEL:
        maze_width, maze_height = LEVELS[current_level - 1]
        walls = generate_maze(maze_width, maze_height)
        player = Player(TILE_SIZE * 2, TILE_SIZE * 2)
        enemies = pygame.sprite.Group()
        num_sound = current_level * 2
        num_light = current_level
        num_patrol = current_level
        for _ in range(num_sound):
            enemies.add(SoundEnemy(walls))
        for _ in range(num_light):
            enemies.add(LightEnemy(walls))
        for _ in range(num_patrol):
            enemies.add(PatrolEnemy(walls))

        items = pygame.sprite.Group()
        for _ in range(10 + current_level * 2):
            items.add(Item('battery', walls))
        for _ in range(5 + current_level):
            items.add(Item('torch', walls))
        for _ in range(3 + current_level):
            items.add(Item('health', walls))
        for _ in range(2 + current_level):
            items.add(Item('speed', walls))

        # Exit position
        exit_rect = pygame.Rect((maze_width - 2) * TILE_SIZE, (maze_height - 2) * TILE_SIZE, TILE_SIZE, TILE_SIZE)

        running = True
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                    sys.exit()
                if event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_p:
                        paused = not paused

            if paused:
                draw_text(screen, "Paused", (SCREEN_WIDTH // 2, SCREEN_HEIGHT // 2), WHITE, 50)
                pygame.display.flip()
                clock.tick(10)
                continue

            # Player update
            player.update(walls, items)

            # Collect items
            collected = pygame.sprite.spritecollide(player, items, True)
            for item in collected:
                score += player.collect_item(item)

            # Enemies update
            sound_pos = player.get_sound_position() if player.moving else None
            light_positions = player.get_light_positions()
            for enemy in enemies:
                enemy.update(player, walls, sound_pos, light_positions)

            # Enemy collisions
            collided_enemies = pygame.sprite.spritecollide(player, enemies, False)
            for enemy in collided_enemies:
                player.take_damage(enemy.damage)
                if player.health <= 0:
                    draw_text(screen, "Game Over! Score: " + str(score), (SCREEN_WIDTH // 2, SCREEN_HEIGHT // 2), WHITE, 40)
                    pygame.display.flip()
                    pygame.time.wait(3000)
                    running = False
                    sys.exit()

            # Check exit
            if player.rect.colliderect(exit_rect):
                score += 100 * current_level
                current_level += 1
                if current_level > MAX_LEVEL:
                    draw_text(screen, "Victory! Total Score: " + str(score), (SCREEN_WIDTH // 2, SCREEN_HEIGHT // 2), WHITE, 40)
                    pygame.display.flip()
                    pygame.time.wait(3000)
                    running = False
                    sys.exit()
                else:
                    draw_text(screen, "Level Complete! Score: " + str(score), (SCREEN_WIDTH // 2, SCREEN_HEIGHT // 2), WHITE, 40)
                    pygame.display.flip()
                    pygame.time.wait(2000)
                    break

            # Drawing
            screen.fill(BLACK)
            walls.draw(screen)
            items.draw(screen)
            enemies.draw(screen)
            screen.blit(player.image, player.rect)
            pygame.draw.rect(screen, (0, 255, 0), exit_rect)

            # Lighting
            draw_lighting(screen, player, walls, enemies, items, light_positions)

            # HUD
            draw_hud(screen, player, score, current_level)

            pygame.display.flip()
            clock.tick(FPS)

    pygame.quit()
    sys.exit()

if __name__ == "__main__":
    main()
