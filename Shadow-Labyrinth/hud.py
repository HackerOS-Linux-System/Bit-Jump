import pygame

FONT = pygame.font.SysFont(None, 24)
WHITE = (255, 255, 255)

def draw_hud(screen, player, score, level):
    health_text = FONT.render(f"Health: {player.health}", True, WHITE)
    battery_text = FONT.render(f"Battery: {int(player.battery)}%", True, WHITE)
    torches_text = FONT.render(f"Torches: {player.torches}", True, WHITE)
    score_text = FONT.render(f"Score: {score}", True, WHITE)
    level_text = FONT.render(f"Level: {level}", True, WHITE)
    screen.blit(health_text, (10, 10))
    screen.blit(battery_text, (10, 40))
    screen.blit(torches_text, (10, 70))
    screen.blit(score_text, (10, 100))
    screen.blit(level_text, (10, 130))
