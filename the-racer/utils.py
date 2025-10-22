import json
import os
import pygame

DEFAULT_SETTINGS = {
    "resolution": [0, 0],
    "fullscreen": True,
    "weather": "sunny",
    "theme": "default",  # New: theme option
    "controls": {
        "up": pygame.K_UP,
        "down": pygame.K_DOWN,
        "left": pygame.K_LEFT,
        "right": pygame.K_RIGHT,
        "pit_stop": pygame.K_p,
        "drs": pygame.K_d
    },
    "music_volume": 0.5,
    "sfx_volume": 0.5,
    "language": "en",
    "ai_difficulty": "medium",
    "minimap_enabled": True,
    "default_track": "Monaco",
    "default_opponents": 1,
    "default_laps": 3,
    "car_type": "balanced",
    "car_collisions": True,
    "fuel_consumption": "medium",
    "practice_enabled": True,
    "practice_duration": 3,
    "qualifying_enabled": True,
    "qualifying_duration": 2
}

def load_game():
    home = os.path.expanduser("~")
    save_dir = os.path.join(home, '.f1game', 'saves')
    if not os.path.exists(save_dir):
        os.makedirs(save_dir)
    save_file = os.path.join(save_dir, 'save.json')
    if os.path.exists(save_file):
        with open(save_file, 'r') as f:
            data = json.load(f)
        settings = data.get('settings', DEFAULT_SETTINGS)
        for key, value in DEFAULT_SETTINGS.items():
            if key not in settings:
                settings[key] = value
            elif key == "controls" and isinstance(settings[key], dict):
                for control_key, control_value in DEFAULT_SETTINGS["controls"].items():
                    if control_key not in settings["controls"]:
                        settings["controls"][control_key] = control_value
        return (
            data.get('high_score', 0),
            data.get('championship_standings', {}),
            settings
        )
    return 0, {}, DEFAULT_SETTINGS

def save_game(high_score, championship_standings, settings):
    home = os.path.expanduser("~")
    save_dir = os.path.join(home, '.f1game', 'saves')
    if not os.path.exists(save_dir):
        os.makedirs(save_dir)
    save_file = os.path.join(save_dir, 'save.json')
    data = {
        'high_score': high_score,
        'championship_standings': championship_standings,
        'settings': settings
    }
    with open(save_file, 'w') as f:
        json.dump(data, f)

def save_game_state(game_state):
    home = os.path.expanduser("~")
    save_dir = os.path.join(home, '.f1game', 'saves')
    if not os.path.exists(save_dir):
        os.makedirs(save_dir)
    save_file = os.path.join(save_dir, 'game_state.json')
    with open(save_file, 'w') as f:
        json.dump(game_state, f)

def load_game_state():
    home = os.path.expanduser("~")
    save_dir = os.path.join(home, '.f1game', 'saves')
    save_file = os.path.join(save_dir, 'game_state.json')
    if os.path.exists(save_file):
        with open(save_file, 'r') as f:
            return json.load(f)
    return None
