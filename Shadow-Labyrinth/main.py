import pygame
import menu
import game_modes
from utils import load_game, save_game
import os

pygame.init()
pygame.mixer.init()

def main():
    high_score, championship_standings, settings = load_game()
    flags = pygame.FULLSCREEN if settings["fullscreen"] else 0
    screen = pygame.display.set_mode(settings["resolution"], flags)
    pygame.display.set_caption("F1 Manager & Racer")
    font = pygame.font.SysFont("arial", 36)
    if os.path.exists("music.mp3"):
        pygame.mixer.music.load("music.mp3")
        pygame.mixer.music.set_volume(settings["music_volume"])
        pygame.mixer.music.play(-1)

    while True:
        choice, settings = menu.main_menu(screen, font, settings, high_score)
        flags = pygame.FULLSCREEN if settings["fullscreen"] else 0
        screen = pygame.display.set_mode(settings["resolution"], flags)
        if choice == "race_setup":
            track_name, num_opponents, max_laps, car_type, tire_type, settings = menu.race_setup_menu(screen, font, settings)
            if track_name:
                high_score = game_modes.race_mode(screen, track_name, num_opponents, max_laps, car_type, tire_type, settings, high_score)
                save_game(high_score, championship_standings, settings)
        elif choice == "championship":
            result = game_modes.championship_mode(screen, font, settings, championship_standings)
            if result:
                high_score, championship_standings = result
                save_game(high_score, championship_standings, settings)
        elif choice == "practice":
            track_name, duration, car_type, tire_type, settings = menu.practice_setup_menu(screen, font, settings)
            if track_name:
                game_modes.practice_mode(screen, track_name, duration, car_type, tire_type, settings)
        elif choice == "time_attack":
            track_name, car_type, tire_type, settings = menu.time_attack_setup_menu(screen, font, settings)
            if track_name:
                high_score = game_modes.race_mode(screen, track_name, 0, None, car_type, tire_type, settings, high_score)
                save_game(high_score, championship_standings, settings)
        elif choice == "settings":
            settings = menu.settings_menu(screen, font, settings)
            save_game(high_score, championship_standings, settings)

if __name__ == "__main__":
    main()
