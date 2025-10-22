import pygame
from car import Car, AICar
from track import Track
import time
import random

def race_mode(screen, track_name, num_opponents, max_laps, car_type, tire_type, settings, high_score):
    track = Track(track_name, 1000, 700)
    player = Car(track.start_positions[0][0], track.start_positions[0][1], car_type, tire_type)
    cars = [player]
    for i in range(num_opponents):
        x, y = track.start_positions[i + 1]
        cars.append(AICar(x, y, settings["ai_difficulty"], track, tire_type, name=f"AI_{i+1}"))

    font = pygame.font.SysFont("arial", 24)
    clock = pygame.time.Clock()
    start_time = time.time()
    running = True
    paused = False
    race_finished = False
    standings = []

    while running:
        keys = pygame.key.get_pressed()
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_ESCAPE:
                    paused = not paused
                if event.key == pygame.K_r and race_finished:
                    return high_score

        if paused:
            pause_text = font.render("Paused" if settings["language"] == "en" else "Pauza", True, (255, 0, 0))
            screen.blit(pause_text, (screen.get_width() // 2 - pause_text.get_width() // 2, screen.get_height() // 2))
            pygame.display.flip()
            clock.tick(60)
            continue

        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))

        if not race_finished:
            for car in cars:
                car.update(keys if car == player else {}, settings["controls"], track, cars, settings["car_collisions"], settings["fuel_consumption"], settings["weather"])
                car.draw(screen)

        for wall in track.walls:
            pygame.draw.rect(screen, (100, 100, 100), wall)
        for checkpoint in track.checkpoints:
            pygame.draw.rect(screen, (0, 255, 0), checkpoint)
        pygame.draw.rect(screen, (200, 200, 0), track.pit_lane)
        for drs_zone in track.drs_zones:
            pygame.draw.rect(screen, (0, 0, 255), drs_zone)

        if settings["minimap_enabled"]:
            minimap_scale = 0.2
            minimap_surface = pygame.Surface((track.width * minimap_scale, track.height * minimap_scale))
            minimap_surface.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
            for wall in track.walls:
                scaled_wall = pygame.Rect(wall.x * minimap_scale, wall.y * minimap_scale, wall.width * minimap_scale, wall.height * minimap_scale)
                pygame.draw.rect(minimap_surface, (100, 100, 100), scaled_wall)
            for car in cars:
                scaled_pos = (car.pos.x * minimap_scale, car.pos.y * minimap_scale)
                pygame.draw.circle(minimap_surface, car.color, scaled_pos, 5)
            screen.blit(minimap_surface, (10, 10))

        if max_laps is not None:
            lap_text = font.render(f"Lap: {player.laps + 1}/{max_laps}" if settings["language"] == "en" else f"Okrążenie: {player.laps + 1}/{max_laps}", True, (0, 0, 0))
        else:
            lap_text = font.render("Time Attack" if settings["language"] == "en" else "Time Attack", True, (0, 0, 0))
        screen.blit(lap_text, (10, screen.get_height() - 100))
        time_text = font.render(f"Time: {time.time() - start_time:.2f}s", True, (0, 0, 0))
        screen.blit(time_text, (10, screen.get_height() - 70))
        fuel_text = font.render(f"Fuel: {player.fuel:.1f}%" if settings["language"] == "en" else f"Paliwo: {player.fuel:.1f}%", True, (0, 0, 0))
        screen.blit(fuel_text, (10, screen.get_height() - 40))
        tire_text = font.render(f"Tire Wear: {player.tire_wear:.1f}%" if settings["language"] == "en" else f"Zużycie opon: {player.tire_wear:.1f}%", True, (0, 0, 0))
        screen.blit(tire_text, (10, screen.get_height() - 10))

        if player.laps >= max_laps if max_laps is not None else False or player.fuel <= 0:
            if not race_finished:
                race_finished = True
                standings = sorted(cars, key=lambda x: (-x.laps, min(x.lap_times) if x.lap_times else float('inf')))
                if max_laps is None and player.lap_times:
                    new_high_score = min(player.lap_times)
                    high_score = min(high_score, new_high_score) if high_score > 0 else new_high_score

        if race_finished:
            for i, car in enumerate(standings[:5]):
                standing_text = font.render(f"{i+1}. {car.name}: {min(car.lap_times):.2f}s" if car.lap_times else f"{i+1}. {car.name}: DNF", True, (0, 0, 0))
                screen.blit(standing_text, (screen.get_width() // 2 - standing_text.get_width() // 2, 200 + i * 30))
            restart_text = font.render("Press R to Restart" if settings["language"] == "en" else "Naciśnij R, aby zrestartować", True, (255, 0, 0))
            screen.blit(restart_text, (screen.get_width() // 2 - restart_text.get_width() // 2, screen.get_height() - 50))

        pygame.display.flip()
        clock.tick(60)

    return high_score

def practice_mode(screen, track_name, duration, car_type, tire_type, settings):
    track = Track(track_name, 1000, 700)
    player = Car(track.start_positions[0][0], track.start_positions[0][1], car_type, tire_type)
    font = pygame.font.SysFont("arial", 24)
    clock = pygame.time.Clock()
    start_time = time.time()
    running = True
    paused = False

    while running:
        keys = pygame.key.get_pressed()
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_ESCAPE:
                    paused = not paused
                if event.key == pygame.K_r and time.time() - start_time > duration * 60:
                    return

        if paused:
            pause_text = font.render("Paused" if settings["language"] == "en" else "Pauza", True, (255, 0, 0))
            screen.blit(pause_text, (screen.get_width() // 2 - pause_text.get_width() // 2, screen.get_height() // 2))
            pygame.display.flip()
            clock.tick(60)
            continue

        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        player.update(keys, settings["controls"], track, [player], settings["car_collisions"], settings["fuel_consumption"], settings["weather"])
        player.draw(screen)

        for wall in track.walls:
            pygame.draw.rect(screen, (100, 100, 100), wall)
        for checkpoint in track.checkpoints:
            pygame.draw.rect(screen, (0, 255, 0), checkpoint)
        pygame.draw.rect(screen, (200, 200, 0), track.pit_lane)
        for drs_zone in track.drs_zones:
            pygame.draw.rect(screen, (0, 0, 255), drs_zone)

        time_text = font.render(f"Time Left: {max(0, duration * 60 - (time.time() - start_time)):.2f}s", True, (0, 0, 0))
        screen.blit(time_text, (10, screen.get_height() - 70))
        fuel_text = font.render(f"Fuel: {player.fuel:.1f}%" if settings["language"] == "en" else f"Paliwo: {player.fuel:.1f}%", True, (0, 0, 0))
        screen.blit(fuel_text, (10, screen.get_height() - 40))
        tire_text = font.render(f"Tire Wear: {player.tire_wear:.1f}%" if settings["language"] == "en" else f"Zużycie opon: {player.tire_wear:.1f}%", True, (0, 0, 0))
        screen.blit(tire_text, (10, screen.get_height() - 10))

        pygame.display.flip()
        clock.tick(60)

        if time.time() - start_time > duration * 60:
            return

def championship_mode(screen, font, settings, championship_standings):
    tracks = ["Monaco", "Silverstone", "Spa", "Monza", "Suzuka", "Interlagos"]
    opponents, laps, practice_enabled, practice_duration, qualifying_enabled, qualifying_duration, car_type, tire_type, settings = menu.championship_setup_menu(screen, font, settings)
    if opponents is None:
        return None

    championship_standings = {f"AI_{i+1}": 0 for i in range(opponents)}
    championship_standings["Player"] = 0
    high_score = 0

    for track_name in tracks:
        if practice_enabled:
            practice_mode(screen, track_name, practice_duration, car_type, tire_type, settings)
        if qualifying_enabled:
            practice_mode(screen, track_name, qualifying_duration, car_type, tire_type, settings)
        high_score = race_mode(screen, track_name, opponents, laps, car_type, tire_type, settings, high_score)

        # Simulate race results for standings
        cars = [Car(0, 0, car_type, tire_type, name="Player")] + [AICar(0, 0, settings["ai_difficulty"], Track(track_name, 1000, 700), tire_type, name=f"AI_{i+1}") for i in range(opponents)]
        for car in cars:
            car.laps = random.randint(laps - 2, laps)
            car.lap_times = [random.uniform(50, 70) for _ in range(car.laps)] if car.laps > 0 else []
        standings = sorted(cars, key=lambda x: (-x.laps, min(x.lap_times) if x.lap_times else float('inf')))
        points = [25, 18, 15, 12, 10, 8, 6, 4, 2, 1] + [0] * (len(standings) - 10)
        for i, car in enumerate(standings):
            championship_standings[car.name] += points[i]

        # Display race results
        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        for i, car in enumerate(standings[:5]):
            text = font.render(f"{i+1}. {car.name}: {min(car.lap_times):.2f}s" if car.lap_times else f"{i+1}. {car.name}: DNF", True, (0, 0, 0))
            screen.blit(text, (screen.get_width() // 2 - text.get_width() // 2, 200 + i * 30))
        pygame.display.flip()
        pygame.time.wait(3000)

    # Display final championship standings
    screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
    sorted_standings = sorted(championship_standings.items(), key=lambda x: x[1], reverse=True)
    for i, (name, points) in enumerate(sorted_standings[:5]):
        text = font.render(f"{i+1}. {name}: {points} pts" if settings["language"] == "en" else f"{i+1}. {name}: {points} pkt", True, (0, 0, 0))
        screen.blit(text, (screen.get_width() // 2 - text.get_width() // 2, 200 + i * 30))
    pygame.display.flip()
    pygame.time.wait(5000)

    return high_score, championship_standings
