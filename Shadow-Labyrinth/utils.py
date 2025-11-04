import pygame

LEVELS = [(20, 15), (25, 20), (30, 25), (35, 30), (40, 35)]  # Width, height per level

def draw_text(screen, text, position, color, size=30):
    font = pygame.font.SysFont(None, size)
    text_surf = font.render(text, True, color)
    text_rect = text_surf.get_rect(center=position)
    screen.blit(text_surf, text_rect)
