import pygame
import math

def main_menu(screen, font, settings, high_score):
    options = ["Single Race", "Championship", "Practice", "Time Attack", "Settings", "Exit"]
    selected = 0
    translations = {
        "en": {
            "title": "F1 Manager & Racer",
            "options": options,
            "high_score": "Best Lap: {:.2f}s"
        },
        "pl": {
            "title": "Menadżer i Kierowca F1",
            "options": ["Wyścig Pojedynczy", "Mistrzostwa", "Praktyki", "Time Attack", "Ustawienia", "Wyjście"],
            "high_score": "Najlepsze okrążenie: {:.2f}s"
        }
    }
    lang = settings["language"]
    hover_time = 0
    last_time = pygame.time.get_ticks()

    while True:
        current_time = pygame.time.get_ticks()
        dt = (current_time - last_time) / 1000.0
        last_time = current_time
        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        title = font.render(translations[lang]["title"], True, (0, 0, 0))
        screen.blit(title, (screen.get_width() // 2 - title.get_width() // 2, 100))
        score_text = font.render(translations[lang]["high_score"].format(high_score), True, (0, 0, 0))
        screen.blit(score_text, (screen.get_width() // 2 - score_text.get_width() // 2, 150))

        for i, option in enumerate(translations[lang]["options"]):
            color = (255, 0, 0) if i == selected else (0, 0, 0)
            text = font.render(option, True, color)
            text_rect = text.get_rect(center=(screen.get_width() // 2, 200 + i * 50))
            if i == selected:
                hover_time += dt
                scale = 1.0 + 0.1 * abs(math.sin(hover_time * 2))
                text = pygame.transform.scale(text, (int(text.get_width() * scale), int(text.get_height() * scale)))
                text_rect = text.get_rect(center=text_rect.center)
            screen.blit(text, text_rect)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    selected = (selected - 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_DOWN:
                    selected = (selected + 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_RETURN:
                    if selected == 0:
                        return "race_setup", settings
                    elif selected == 1:
                        return "championship", settings
                    elif selected == 2:
                        return "practice", settings
                    elif selected == 3:
                        return "time_attack", settings
                    elif selected == 4:
                        return "settings", settings
                    elif selected == 5:
                        pygame.quit()
                        exit()

        pygame.display.flip()
        pygame.time.Clock().tick(60)

def settings_menu(screen, font, settings):
    options = [
        "Resolution", "Fullscreen", "Controls", "Music Volume", "SFX Volume",
        "Language", "AI Difficulty", "Minimap", "Car Collisions",
        "Fuel Consumption", "Weather", "Back"
    ]
    resolutions = [[800, 600], [1024, 768], [1280, 720], [1920, 1080]]
    difficulties = ["easy", "medium", "hard"]
    languages = ["en", "pl"]
    fuel_consumptions = ["low", "medium", "high"]
    weathers = ["sunny", "rain"]
    selected = 0
    translations = {
        "en": {
            "options": options,
            "minimap": ["On", "Off"],
            "collisions": ["On", "Off"],
            "fuel": ["Low", "Medium", "High"],
            "weather": ["Sunny", "Rain"],
            "fullscreen": ["On", "Off"]
        },
        "pl": {
            "options": [
                "Rozdzielczość", "Pełny ekran", "Sterowanie", "Głośność muzyki",
                "Głośność efektów", "Język", "Poziom trudności AI", "Minimapa",
                "Kolizje samochodów", "Zużycie paliwa", "Pogoda", "Wróć"
            ],
            "minimap": ["Wł.", "Wył."],
            "collisions": ["Wł.", "Wył."],
            "fuel": ["Niskie", "Średnie", "Wysokie"],
            "weather": ["Słonecznie", "Deszcz"],
            "fullscreen": ["Wł.", "Wył."]
        }
    }
    lang = settings["language"]
    hover_time = 0
    last_time = pygame.time.get_ticks()

    while True:
        current_time = pygame.time.get_ticks()
        dt = (current_time - last_time) / 1000.0
        last_time = current_time
        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        for i, option in enumerate(translations[lang]["options"]):
            color = (255, 0, 0) if i == selected else (0, 0, 0)
            text = font.render(option, True, color)
            text_rect = text.get_rect(topleft=(100, 100 + i * 50))
            if i == selected:
                hover_time += dt
                scale = 1.0 + 0.1 * abs(math.sin(hover_time * 2))
                text = pygame.transform.scale(text, (int(text.get_width() * scale), int(text.get_height() * scale)))
                text_rect = text.get_rect(center=text_rect.center)
            screen.blit(text, text_rect)
            if i == 0:
                res_text = font.render(f"{settings['resolution'][0]}x{settings['resolution'][1]}" if not settings["fullscreen"] else "Fullscreen", True, (0, 0, 0))
                screen.blit(res_text, (400, 100))
            elif i == 1:
                fs_text = font.render(translations[lang]["fullscreen"][0 if settings['fullscreen'] else 1], True, (0, 0, 0))
                screen.blit(fs_text, (400, 150))
            elif i == 2:
                controls_text = font.render("Press Enter to edit" if lang == "en" else "Naciśnij Enter, aby edytować", True, (0, 0, 0))
                screen.blit(controls_text, (400, 200))
            elif i == 3:
                volume_text = font.render(f"{int(settings['music_volume'] * 100)}%", True, (0, 0, 0))
                screen.blit(volume_text, (400, 250))
            elif i == 4:
                sfx_text = font.render(f"{int(settings['sfx_volume'] * 100)}%", True, (0, 0, 0))
                screen.blit(sfx_text, (400, 300))
            elif i == 5:
                lang_text = font.render(settings['language'].upper(), True, (0, 0, 0))
                screen.blit(lang_text, (400, 350))
            elif i == 6:
                diff_text = font.render(settings['ai_difficulty'].capitalize(), True, (0, 0, 0))
                screen.blit(diff_text, (400, 400))
            elif i == 7:
                minimap_text = font.render(translations[lang]["minimap"][0 if settings['minimap_enabled'] else 1], True, (0, 0, 0))
                screen.blit(minimap_text, (400, 450))
            elif i == 8:
                collision_text = font.render(translations[lang]["collisions"][0 if settings['car_collisions'] else 1], True, (0, 0, 0))
                screen.blit(collision_text, (400, 500))
            elif i == 9:
                fuel_text = font.render(translations[lang]["fuel"][fuel_consumptions.index(settings['fuel_consumption'])], True, (0, 0, 0))
                screen.blit(fuel_text, (400, 550))
            elif i == 10:
                weather_text = font.render(translations[lang]["weather"][weathers.index(settings['weather'])], True, (0, 0, 0))
                screen.blit(weather_text, (400, 600))

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    selected = (selected - 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_DOWN:
                    selected = (selected + 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_RETURN:
                    if selected == 0 and not settings["fullscreen"]:
                        idx = resolutions.index(settings["resolution"]) if settings["resolution"] in resolutions else 0
                        settings["resolution"] = resolutions[(idx + 1) % len(resolutions)]
                    elif selected == 1:
                        settings["fullscreen"] = not settings["fullscreen"]
                        if settings["fullscreen"]:
                            settings["resolution"] = [0, 0]
                        else:
                            settings["resolution"] = [1280, 720]
                    elif selected == 2:
                        settings["controls"] = controls_menu(screen, font, settings["controls"], lang)
                    elif selected == 5:
                        idx = languages.index(settings["language"])
                        settings["language"] = languages[(idx + 1) % len(languages)]
                    elif selected == 6:
                        idx = difficulties.index(settings["ai_difficulty"])
                        settings["ai_difficulty"] = difficulties[(idx + 1) % len(difficulties)]
                    elif selected == 7:
                        settings["minimap_enabled"] = not settings["minimap_enabled"]
                    elif selected == 8:
                        settings["car_collisions"] = not settings["car_collisions"]
                    elif selected == 9:
                        idx = fuel_consumptions.index(settings["fuel_consumption"])
                        settings["fuel_consumption"] = fuel_consumptions[(idx + 1) % len(fuel_consumptions)]
                    elif selected == 10:
                        idx = weathers.index(settings["weather"])
                        settings["weather"] = weathers[(idx + 1) % len(weathers)]
                    elif selected == 11:
                        return settings
                if event.key == pygame.K_LEFT:
                    if selected == 3:
                        settings["music_volume"] = max(settings["music_volume"] - 0.1, 0.0)
                        pygame.mixer.music.set_volume(settings["music_volume"])
                    elif selected == 4:
                        settings["sfx_volume"] = max(settings["sfx_volume"] - 0.1, 0.0)
                if event.key == pygame.K_RIGHT:
                    if selected == 3:
                        settings["music_volume"] = min(settings["music_volume"] + 0.1, 1.0)
                        pygame.mixer.music.set_volume(settings["music_volume"])
                    elif selected == 4:
                        settings["sfx_volume"] = min(settings["sfx_volume"] + 0.1, 1.0)
                if event.key == pygame.K_ESCAPE:
                    return settings

        pygame.display.flip()
        pygame.time.Clock().tick(60)

def controls_menu(screen, font, controls, lang):
    keys = ["up", "down", "left", "right", "pit_stop", "drs"]
    selected = 0
    translations = {
        "en": {"prompt": "Press key for {}...", "controls": ["Up", "Down", "Left", "Right", "Pit Stop", "DRS"]},
        "pl": {"prompt": "Naciśnij klawisz dla {}...", "controls": ["Góra", "Dół", "Lewo", "Prawo", "Pit Stop", "DRS"]}
    }
    while True:
        screen.fill((255, 255, 255))
        for i, k in enumerate(keys):
            color = (255, 0, 0) if i == selected else (0, 0, 0)
            display_name = translations[lang]["controls"][i]
            text = font.render(f"{display_name}: {pygame.key.name(controls.get(k, pygame.K_UNKNOWN))}", True, color)
            screen.blit(text, (100, 100 + i * 50))
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    selected = (selected - 1) % len(keys)
                if event.key == pygame.K_DOWN:
                    selected = (selected + 1) % len(keys)
                if event.key == pygame.K_RETURN:
                    editing = keys[selected]
                    edit_text = font.render(translations[lang]["prompt"].format(translations[lang]["controls"][selected]), True, (0, 0, 255))
                    screen.blit(edit_text, (100, screen.get_height() - 50))
                    pygame.display.flip()
                    waiting = True
                    while waiting:
                        for ev in pygame.event.get():
                            if ev.type == pygame.KEYDOWN:
                                controls[editing] = ev.key
                                waiting = False
                if event.key == pygame.K_ESCAPE:
                    return controls
        pygame.display.flip()
        pygame.time.Clock().tick(60)

def race_setup_menu(screen, font, settings):
    tracks = ["Monaco", "Silverstone", "Spa", "Monza", "Suzuka", "Interlagos"]
    opponents = list(range(0, 20))
    laps = [1, 3, 5, 7, 10, 15]
    car_types = ["speed", "balanced", "accel"]
    tire_types = ["soft", "medium", "hard"]
    selected = 0
    track_idx = tracks.index(settings["default_track"]) if settings["default_track"] in tracks else 0
    opponent_idx = opponents.index(settings["default_opponents"]) if settings["default_opponents"] in opponents else 1
    lap_idx = laps.index(settings.get("default_laps", 3)) if settings.get("default_laps", 3) in laps else 1
    car_idx = car_types.index(settings["car_type"]) if settings["car_type"] in car_types else 1
    tire_idx = 1
    translations = {
        "en": {
            "title": "Race Setup",
            "track": "Track: {}",
            "opponents": "Opponents: {}",
            "laps": "Laps: {}",
            "car_type": "Car Type: {}",
            "tire_type": "Tire Type: {}",
            "start": "Start Race",
            "back": "Back"
        },
        "pl": {
            "title": "Ustawienia wyścigu",
            "track": "Tor: {}",
            "opponents": "Przeciwnicy: {}",
            "laps": "Okrążenia: {}",
            "car_type": "Typ samochodu: {}",
            "tire_type": "Typ opon: {}",
            "start": "Rozpocznij wyścig",
            "back": "Wróć"
        }
    }
    lang = settings["language"]
    hover_time = 0
    last_time = pygame.time.get_ticks()

    while True:
        current_time = pygame.time.get_ticks()
        dt = (current_time - last_time) / 1000.0
        last_time = current_time
        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        title = font.render(translations[lang]["title"], True, (0, 0, 0))
        screen.blit(title, (screen.get_width() // 2 - title.get_width() // 2, 100))
        options = [
            translations[lang]["track"].format(tracks[track_idx]),
            translations[lang]["opponents"].format(opponents[opponent_idx]),
            translations[lang]["laps"].format(laps[lap_idx]),
            translations[lang]["car_type"].format(car_types[car_idx]),
            translations[lang]["tire_type"].format(tire_types[tire_idx]),
            translations[lang]["start"],
            translations[lang]["back"]
        ]
        for i, option in enumerate(options):
            color = (255, 0, 0) if i == selected else (0, 0, 0)
            text = font.render(option, True, color)
            text_rect = text.get_rect(center=(screen.get_width() // 2, 200 + i * 50))
            if i == selected:
                hover_time += dt
                scale = 1.0 + 0.1 * abs(math.sin(hover_time * 2))
                text = pygame.transform.scale(text, (int(text.get_width() * scale), int(text.get_height() * scale)))
                text_rect = text.get_rect(center=text_rect.center)
            screen.blit(text, text_rect)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    selected = (selected - 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_DOWN:
                    selected = (selected + 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_LEFT:
                    if selected == 0:
                        track_idx = (track_idx - 1) % len(tracks)
                    elif selected == 1:
                        opponent_idx = (opponent_idx - 1) % len(opponents)
                    elif selected == 2:
                        lap_idx = (lap_idx - 1) % len(laps)
                    elif selected == 3:
                        car_idx = (car_idx - 1) % len(car_types)
                    elif selected == 4:
                        tire_idx = (tire_idx - 1) % len(tire_types)
                if event.key == pygame.K_RIGHT:
                    if selected == 0:
                        track_idx = (track_idx + 1) % len(tracks)
                    elif selected == 1:
                        opponent_idx = (opponent_idx + 1) % len(opponents)
                    elif selected == 2:
                        lap_idx = (lap_idx + 1) % len(laps)
                    elif selected == 3:
                        car_idx = (car_idx + 1) % len(car_types)
                    elif selected == 4:
                        tire_idx = (tire_idx + 1) % len(tire_types)
                if event.key == pygame.K_RETURN:
                    if selected == 5:
                        settings["default_track"] = tracks[track_idx]
                        settings["default_opponents"] = opponents[opponent_idx]
                        settings["default_laps"] = laps[lap_idx]
                        settings["car_type"] = car_types[car_idx]
                        return tracks[track_idx], opponents[opponent_idx], laps[lap_idx], car_types[car_idx], tire_types[tire_idx], settings
                    elif selected == 6:
                        return None, None, None, None, None, settings
                if event.key == pygame.K_ESCAPE:
                    return None, None, None, None, None, settings

        pygame.display.flip()
        pygame.time.Clock().tick(60)

def practice_setup_menu(screen, font, settings):
    tracks = ["Monaco", "Silverstone", "Spa", "Monza", "Suzuka", "Interlagos"]
    durations = [1, 3, 5, 10]
    car_types = ["speed", "balanced", "accel"]
    tire_types = ["soft", "medium", "hard"]
    selected = 0
    track_idx = tracks.index(settings["default_track"]) if settings["default_track"] in tracks else 0
    duration_idx = durations.index(settings.get("practice_duration", 3)) if settings.get("practice_duration", 3) in durations else 1
    car_idx = car_types.index(settings["car_type"]) if settings["car_type"] in car_types else 1
    tire_idx = 1
    translations = {
        "en": {
            "title": "Practice Setup",
            "track": "Track: {}",
            "duration": "Duration: {} min",
            "car_type": "Car Type: {}",
            "tire_type": "Tire Type: {}",
            "start": "Start Practice",
            "back": "Back"
        },
        "pl": {
            "title": "Ustawienia Praktyki",
            "track": "Tor: {}",
            "duration": "Czas trwania: {} min",
            "car_type": "Typ samochodu: {}",
            "tire_type": "Typ opon: {}",
            "start": "Rozpocznij praktykę",
            "back": "Wróć"
        }
    }
    lang = settings["language"]
    hover_time = 0
    last_time = pygame.time.get_ticks()

    while True:
        current_time = pygame.time.get_ticks()
        dt = (current_time - last_time) / 1000.0
        last_time = current_time
        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        title = font.render(translations[lang]["title"], True, (0, 0, 0))
        screen.blit(title, (screen.get_width() // 2 - title.get_width() // 2, 100))
        options = [
            translations[lang]["track"].format(tracks[track_idx]),
            translations[lang]["duration"].format(durations[duration_idx]),
            translations[lang]["car_type"].format(car_types[car_idx]),
            translations[lang]["tire_type"].format(tire_types[tire_idx]),
            translations[lang]["start"],
            translations[lang]["back"]
        ]
        for i, option in enumerate(options):
            color = (255, 0, 0) if i == selected else (0, 0, 0)
            text = font.render(option, True, color)
            text_rect = text.get_rect(center=(screen.get_width() // 2, 200 + i * 50))
            if i == selected:
                hover_time += dt
                scale = 1.0 + 0.1 * abs(math.sin(hover_time * 2))
                text = pygame.transform.scale(text, (int(text.get_width() * scale), int(text.get_height() * scale)))
                text_rect = text.get_rect(center=text_rect.center)
            screen.blit(text, text_rect)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    selected = (selected - 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_DOWN:
                    selected = (selected + 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_LEFT:
                    if selected == 0:
                        track_idx = (track_idx - 1) % len(tracks)
                    elif selected == 1:
                        duration_idx = (duration_idx - 1) % len(durations)
                    elif selected == 2:
                        car_idx = (car_idx - 1) % len(car_types)
                    elif selected == 3:
                        tire_idx = (tire_idx - 1) % len(tire_types)
                if event.key == pygame.K_RIGHT:
                    if selected == 0:
                        track_idx = (track_idx + 1) % len(tracks)
                    elif selected == 1:
                        duration_idx = (duration_idx + 1) % len(durations)
                    elif selected == 2:
                        car_idx = (car_idx + 1) % len(car_types)
                    elif selected == 3:
                        tire_idx = (tire_idx + 1) % len(tire_types)
                if event.key == pygame.K_RETURN:
                    if selected == 4:
                        settings["default_track"] = tracks[track_idx]
                        settings["practice_duration"] = durations[duration_idx]
                        settings["car_type"] = car_types[car_idx]
                        return tracks[track_idx], durations[duration_idx], car_types[car_idx], tire_types[tire_idx], settings
                    elif selected == 5:
                        return None, None, None, None, settings
                if event.key == pygame.K_ESCAPE:
                    return None, None, None, None, settings

        pygame.display.flip()
        pygame.time.Clock().tick(60)

def championship_setup_menu(screen, font, settings):
    opponents = list(range(1, 20))
    laps = [1, 3, 5, 7, 10, 15]
    durations = [1, 2, 3, 5]
    car_types = ["speed", "balanced", "accel"]
    tire_types = ["soft", "medium", "hard"]
    selected = 0
    opponent_idx = opponents.index(settings["default_opponents"]) if settings["default_opponents"] in opponents else 0
    lap_idx = laps.index(settings.get("default_laps", 3)) if settings.get("default_laps", 3) in laps else 1
    practice_idx = 1 if settings["practice_enabled"] else 0
    practice_duration_idx = durations.index(settings.get("practice_duration", 3)) if settings.get("practice_duration", 3) in durations else 1
    qualifying_idx = 1 if settings["qualifying_enabled"] else 0
    qualifying_duration_idx = durations.index(settings.get("qualifying_duration", 2)) if settings.get("qualifying_duration", 2) in durations else 0
    car_idx = car_types.index(settings["car_type"]) if settings["car_type"] in car_types else 1
    tire_idx = 1
    translations = {
        "en": {
            "title": "Championship Setup",
            "opponents": "Opponents: {}",
            "laps": "Laps per Race: {}",
            "practice": "Practice: {}",
            "practice_duration": "Practice Duration: {} min",
            "qualifying": "Qualifying: {}",
            "qualifying_duration": "Qualifying Duration: {} min",
            "car_type": "Car Type: {}",
            "tire_type": "Tire Type: {}",
            "start": "Start Championship",
            "back": "Back",
            "on_off": ["Off", "On"]
        },
        "pl": {
            "title": "Ustawienia Mistrzostw",
            "opponents": "Przeciwnicy: {}",
            "laps": "Okrążenia na wyścig: {}",
            "practice": "Praktyki: {}",
            "practice_duration": "Czas praktyk: {} min",
            "qualifying": "Kwalifikacje: {}",
            "qualifying_duration": "Czas kwalifikacji: {} min",
            "car_type": "Typ samochodu: {}",
            "tire_type": "Typ opon: {}",
            "start": "Rozpocznij mistrzostwa",
            "back": "Wróć",
            "on_off": ["Wył.", "Wł."]
        }
    }
    lang = settings["language"]
    hover_time = 0
    last_time = pygame.time.get_ticks()

    while True:
        current_time = pygame.time.get_ticks()
        dt = (current_time - last_time) / 1000.0
        last_time = current_time
        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        title = font.render(translations[lang]["title"], True, (0, 0, 0))
        screen.blit(title, (screen.get_width() // 2 - title.get_width() // 2, 100))
        options = [
            translations[lang]["opponents"].format(opponents[opponent_idx]),
            translations[lang]["laps"].format(laps[lap_idx]),
            translations[lang]["practice"].format(translations[lang]["on_off"][practice_idx]),
            translations[lang]["practice_duration"].format(durations[practice_duration_idx]),
            translations[lang]["qualifying"].format(translations[lang]["on_off"][qualifying_idx]),
            translations[lang]["qualifying_duration"].format(durations[qualifying_duration_idx]),
            translations[lang]["car_type"].format(car_types[car_idx]),
            translations[lang]["tire_type"].format(tire_types[tire_idx]),
            translations[lang]["start"],
            translations[lang]["back"]
        ]
        for i, option in enumerate(options):
            color = (255, 0, 0) if i == selected else (0, 0, 0)
            text = font.render(option, True, color)
            text_rect = text.get_rect(center=(screen.get_width() // 2, 200 + i * 50))
            if i == selected:
                hover_time += dt
                scale = 1.0 + 0.1 * abs(math.sin(hover_time * 2))
                text = pygame.transform.scale(text, (int(text.get_width() * scale), int(text.get_height() * scale)))
                text_rect = text.get_rect(center=text_rect.center)
            screen.blit(text, text_rect)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    selected = (selected - 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_DOWN:
                    selected = (selected + 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_LEFT:
                    if selected == 0:
                        opponent_idx = (opponent_idx - 1) % len(opponents)
                    elif selected == 1:
                        lap_idx = (lap_idx - 1) % len(laps)
                    elif selected == 2:
                        practice_idx = (practice_idx - 1) % 2
                    elif selected == 3:
                        practice_duration_idx = (practice_duration_idx - 1) % len(durations)
                    elif selected == 4:
                        qualifying_idx = (qualifying_idx - 1) % 2
                    elif selected == 5:
                        qualifying_duration_idx = (qualifying_duration_idx - 1) % len(durations)
                    elif selected == 6:
                        car_idx = (car_idx - 1) % len(car_types)
                    elif selected == 7:
                        tire_idx = (tire_idx - 1) % len(tire_types)
                if event.key == pygame.K_RIGHT:
                    if selected == 0:
                        opponent_idx = (opponent_idx + 1) % len(opponents)
                    elif selected == 1:
                        lap_idx = (lap_idx + 1) % len(laps)
                    elif selected == 2:
                        practice_idx = (practice_idx + 1) % 2
                    elif selected == 3:
                        practice_duration_idx = (practice_duration_idx + 1) % len(durations)
                    elif selected == 4:
                        qualifying_idx = (qualifying_idx + 1) % 2
                    elif selected == 5:
                        qualifying_duration_idx = (qualifying_duration_idx + 1) % len(durations)
                    elif selected == 6:
                        car_idx = (car_idx + 1) % len(car_types)
                    elif selected == 7:
                        tire_idx = (tire_idx + 1) % len(tire_types)
                if event.key == pygame.K_RETURN:
                    if selected == 8:
                        settings["default_opponents"] = opponents[opponent_idx]
                        settings["default_laps"] = laps[lap_idx]
                        settings["practice_enabled"] = bool(practice_idx)
                        settings["practice_duration"] = durations[practice_duration_idx]
                        settings["qualifying_enabled"] = bool(qualifying_idx)
                        settings["qualifying_duration"] = durations[qualifying_duration_idx]
                        settings["car_type"] = car_types[car_idx]
                        return (
                            opponents[opponent_idx],
                            laps[lap_idx],
                            bool(practice_idx),
                            durations[practice_duration_idx],
                            bool(qualifying_idx),
                            durations[qualifying_duration_idx],
                            car_types[car_idx],
                            tire_types[tire_idx],
                            settings
                        )
                    elif selected == 9:
                        return None, None, None, None, None, None, None, None, settings
                if event.key == pygame.K_ESCAPE:
                    return None, None, None, None, None, None, None, None, settings

        pygame.display.flip()
        pygame.time.Clock().tick(60)

def time_attack_setup_menu(screen, font, settings):
    tracks = ["Monaco", "Silverstone", "Spa", "Monza", "Suzuka", "Interlagos"]
    car_types = ["speed", "balanced", "accel"]
    tire_types = ["soft", "medium", "hard"]
    selected = 0
    track_idx = tracks.index(settings["default_track"]) if settings["default_track"] in tracks else 0
    car_idx = car_types.index(settings["car_type"]) if settings["car_type"] in car_types else 1
    tire_idx = 1
    translations = {
        "en": {
            "title": "Time Attack Setup",
            "track": "Track: {}",
            "car_type": "Car Type: {}",
            "tire_type": "Tire Type: {}",
            "start": "Start Time Attack",
            "back": "Back"
        },
        "pl": {
            "title": "Ustawienia Time Attack",
            "track": "Tor: {}",
            "car_type": "Typ samochodu: {}",
            "tire_type": "Typ opon: {}",
            "start": "Rozpocznij Time Attack",
            "back": "Wróć"
        }
    }
    lang = settings["language"]
    hover_time = 0
    last_time = pygame.time.get_ticks()

    while True:
        current_time = pygame.time.get_ticks()
        dt = (current_time - last_time) / 1000.0
        last_time = current_time
        screen.fill((255, 255, 255) if settings["weather"] == "sunny" else (150, 150, 200))
        title = font.render(translations[lang]["title"], True, (0, 0, 0))
        screen.blit(title, (screen.get_width() // 2 - title.get_width() // 2, 100))
        options = [
            translations[lang]["track"].format(tracks[track_idx]),
            translations[lang]["car_type"].format(car_types[car_idx]),
            translations[lang]["tire_type"].format(tire_types[tire_idx]),
            translations[lang]["start"],
            translations[lang]["back"]
        ]
        for i, option in enumerate(options):
            color = (255, 0, 0) if i == selected else (0, 0, 0)
            text = font.render(option, True, color)
            text_rect = text.get_rect(center=(screen.get_width() // 2, 200 + i * 50))
            if i == selected:
                hover_time += dt
                scale = 1.0 + 0.1 * abs(math.sin(hover_time * 2))
                text = pygame.transform.scale(text, (int(text.get_width() * scale), int(text.get_height() * scale)))
                text_rect = text.get_rect(center=text_rect.center)
            screen.blit(text, text_rect)

        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                pygame.quit()
                exit()
            if event.type == pygame.KEYDOWN:
                if event.key == pygame.K_UP:
                    selected = (selected - 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_DOWN:
                    selected = (selected + 1) % len(options)
                    hover_time = 0
                if event.key == pygame.K_LEFT:
                    if selected == 0:
                        track_idx = (track_idx - 1) % len(tracks)
                    elif selected == 1:
                        car_idx = (car_idx - 1) % len(car_types)
                    elif selected == 2:
                        tire_idx = (tire_idx - 1) % len(tire_types)
                if event.key == pygame.K_RIGHT:
                    if selected == 0:
                        track_idx = (track_idx + 1) % len(tracks)
                    elif selected == 1:
                        car_idx = (car_idx + 1) % len(car_types)
                    elif selected == 2:
                        tire_idx = (tire_idx + 1) % len(tire_types)
                if event.key == pygame.K_RETURN:
                    if selected == 3:
                        settings["default_track"] = tracks[track_idx]
                        settings["car_type"] = car_types[car_idx]
                        return tracks[track_idx], car_types[car_idx], tire_types[tire_idx], settings
                    elif selected == 4:
                        return None, None, None, settings
                if event.key == pygame.K_ESCAPE:
                    return None, None, None, settings

        pygame.display.flip()
        pygame.time.Clock().tick(60)
