import pygame

BLACK = (0, 0, 0)

def draw_lighting(screen, player, walls, enemies, items, light_positions):
    dark_surface = pygame.Surface(screen.get_size()).convert_alpha()
    dark_surface.fill((0, 0, 0, 220))  # Slightly less dark for ambiance

    for pos_x, pos_y, radius in light_positions:
        light_surface = pygame.Surface((radius * 2, radius * 2)).convert_alpha()
        light_surface.fill((0, 0, 0, 0))
        pygame.draw.circle(light_surface, (255, 255, 0, 100), (radius, radius), radius)  # Yellow tint
        for r in range(radius, 0, -5):
            alpha = int(255 * (1 - r / radius))
            pygame.draw.circle(light_surface, (255, 255, 255, alpha), (radius, radius), r)

        light_pos = (pos_x - radius, pos_y - radius)
        dark_surface.blit(light_surface, light_pos, special_flags=pygame.BLEND_RGBA_SUB)

    temp_surface = pygame.Surface(screen.get_size())
    temp_surface.fill(BLACK)
    walls.draw(temp_surface)
    enemies.draw(temp_surface)
    items.draw(temp_surface)
    temp_surface.blit(player.image, player.rect)
    temp_surface.blit(dark_surface, (0, 0))
    screen.blit(temp_surface, (0, 0))
