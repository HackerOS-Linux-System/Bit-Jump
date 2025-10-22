import pygame
from pygame.math import Vector2
import math
import random
import time

class Car:
    def __init__(self, x, y, car_type, tire_type, color=(255, 0, 0), name="Player"):
        self.pos = Vector2(x, y)
        self.vel = Vector2(0, 0)
        self.angle = 0
        self.speed = 0
        self.car_type = car_type
        self.tire_type = tire_type
        self.base_max_speed = 5.5 if car_type == "speed" else 5.0 if car_type == "balanced" else 4.5  # Adjusted for consistency
        self.max_speed = self.base_max_speed
        self.acceleration = 0.2 if car_type == "accel" else 0.15 if car_type == "balanced" else 0.1  # Slower acceleration for control
        self.brake = 0.3 if car_type == "accel" else 0.25 if car_type == "balanced" else 0.2
        self.turn_speed = 3.0 if car_type == "speed" else 2.5 if car_type == "balanced" else 2.0  # Reduced for smoother turns
        self.slip_factor = 0.95 if car_type == "speed" else 0.9 if car_type == "balanced" else 0.85  # Increased grip
        self.rect = pygame.Rect(x - 20, y - 10, 40, 20)
        self.color = color
        self.laps = 0
        self.checkpoint = 0
        self.lap_times = []
        self.current_lap_start = time.time()
        self.slip_effect = 0
        self.fuel = 100.0
        self.damage_aero = 0.0
        self.damage_engine = 0.0
        self.tire_wear = 0.0
        self.in_pit = False
        self.pit_timer = 0
        self.name = name
        self.drs_active = False

    def update(self, keys, controls, track, cars, car_collisions, fuel_consumption, weather):
        if self.in_pit:
            self.pit_timer -= 1
            if self.pit_timer <= 0:
                self.in_pit = False
                self.fuel = 100.0
                self.damage_aero = 0.0
                self.damage_engine = 0.0
                self.tire_wear = 0.0
                self.tire_type = "medium"
                self.max_speed = self.base_max_speed
            return

        # Define default controls to prevent KeyError
        default_controls = {
            "up": pygame.K_UP,
            "down": pygame.K_DOWN,
            "left": pygame.K_LEFT,
            "right": pygame.K_RIGHT,
            "pit_stop": pygame.K_p,
            "drs": pygame.K_d
        }
        effective_controls = {key: controls.get(key, default_controls[key]) for key in default_controls}

        fuel_rate = (0.035 if fuel_consumption == "high" else 0.025 if fuel_consumption == "medium" else 0.015) * (0.8 if weather == "rain" else 1.0)
        tire_wear_rate = (0.06 if self.tire_type == "soft" else 0.04 if self.tire_type == "medium" else 0.02) * (1.2 if weather == "rain" else 1.0)
        grip_modifier = 1.1 if self.tire_type == "soft" else 1.0 if self.tire_type == "medium" else 0.9

        if keys[effective_controls["up"]]:
            self.speed = min(self.speed + self.acceleration * grip_modifier, self.max_speed * (1 - self.damage_engine / 200) * (1 - self.tire_wear / 100))
            self.fuel -= fuel_rate
            self.tire_wear += tire_wear_rate
        if keys[effective_controls["down"]]:
            self.speed = max(self.speed - self.brake, 0)
        if keys[effective_controls["left"]]:
            speed_ratio = self.speed / self.max_speed if self.max_speed > 0 else 0
            self.angle += self.turn_speed * (1 - speed_ratio * 0.3) * (0.7 if weather == "rain" else 1.0) * (0.8 if self.tire_wear > 50 else 1.0) * grip_modifier
        if keys[effective_controls["right"]]:
            speed_ratio = self.speed / self.max_speed if self.max_speed > 0 else 0
            self.angle -= self.turn_speed * (1 - speed_ratio * 0.3) * (0.7 if weather == "rain" else 1.0) * (0.8 if self.tire_wear > 50 else 1.0) * grip_modifier
        if keys[effective_controls["pit_stop"]] and track.check_pitstop(self.rect):
            self.in_pit = True
            self.pit_timer = 360
            self.speed = 0
        if keys[effective_controls["drs"]] and track.check_drs_zone(self.rect) and self.speed > self.max_speed * 0.8:
            self.drs_active = True
            self.max_speed *= 1.15
        else:
            self.drs_active = False

        self.vel = Vector2(0, -self.speed).rotate(-self.angle)
        self.vel *= self.slip_factor * (0.65 if weather == "rain" else 1.0) * grip_modifier
        self.pos += self.vel
        self.rect.center = self.pos

        self.pos.x = max(20, min(self.pos.x, track.width - 20))
        self.pos.y = max(10, min(self.pos.y, track.height - 10))
        self.rect.center = self.pos

        if car_collisions:
            for other_car in cars:
                if other_car != self and self.rect.colliderect(other_car.rect):
                    self.speed *= 0.55
                    self.slip_effect = 20
                    self.damage_aero += 1.5
                    self.damage_engine += 0.7
                    self.tire_wear += 7.0

        self.max_speed = max(self.base_max_speed * (1 - self.damage_aero / 200), self.base_max_speed * 0.2)
        if track.check_collision(self.rect):
            self.speed *= 0.35
            self.slip_effect = 20
            self.damage_aero += 2.5
            self.damage_engine += 1.5
            self.tire_wear += 12.0

        if track.check_checkpoint(self.rect, self):
            self.checkpoint += 1
            if self.checkpoint == len(track.checkpoints):
                self.laps += 1
                lap_time = time.time() - self.current_lap_start
                self.lap_times.append(lap_time)
                self.current_lap_start = time.time()
                self.checkpoint = 0

        if self.slip_effect > 0:
            self.slip_effect -= 1

    def draw(self, surface):
        car_surface = pygame.Surface((40, 20), pygame.SRCALPHA)
        color = (self.color[0], max(0, self.color[1] - self.slip_effect * 10), max(0, self.color[2] - self.slip_effect * 5)) if self.slip_effect > 0 else self.color
        pygame.draw.rect(car_surface, color, (0, 0, 40, 20))
        if self.drs_active:
            pygame.draw.rect(car_surface, (0, 255, 0), (30, 5, 10, 10))
        rotated = pygame.transform.rotate(car_surface, self.angle)
        rect = rotated.get_rect(center=self.pos)
        surface.blit(rotated, rect)

class AICar(Car):
    def __init__(self, x, y, difficulty, track, tire_type, name="AI"):
        color = (random.randint(0, 255), random.randint(0, 255), random.randint(0, 255))
        super().__init__(x, y, random.choice(["speed", "balanced", "accel"]), tire_type, color, name)
        speed_modifier = 1.3 if track.name == "Monza" else 0.85 if track.name == "Suzuka" else 0.95
        self.base_max_speed = self.base_max_speed * (0.8 if difficulty == "easy" else 1.0 if difficulty == "medium" else 1.25) * speed_modifier
        self.max_speed = self.base_max_speed
        self.turn_speed = self.turn_speed * (0.8 if difficulty == "easy" else 1.0 if difficulty == "medium" else 1.25)
        self.path = [(cp.x + cp.width // 2, cp.y + cp.height // 2) for cp in track.checkpoints]
        self.target_idx = 0
        self.pit_strategy = random.randint(1, 3)
        self.drs_chance = 0.6 if difficulty == "hard" else 0.4 if difficulty == "medium" else 0.2
        self.tire_strategy = tire_type
        self.ai_difficulty = difficulty

    def update(self, keys, controls, track, cars, car_collisions, fuel_consumption, weather):
        if self.in_pit:
            self.pit_timer -= 1
            if self.pit_timer <= 0:
                self.in_pit = False
                self.fuel = 100.0
                self.damage_aero = 0.0
                self.damage_engine = 0.0
                self.tire_wear = 0.0
                self.tire_type = self.tire_strategy
                self.max_speed = self.base_max_speed
            return

        if self.fuel < 15 or self.damage_engine > 60 or self.tire_wear > 85 or (self.laps == self.pit_strategy and track.check_pitstop(self.rect)):
            self.in_pit = True
            self.pit_timer = 360
            self.speed = 0
            return

        target = Vector2(self.path[self.target_idx])
        direction = target - self.pos
        distance = direction.length()
        if distance < 25:
            self.target_idx = (self.target_idx + 1) % len(self.path)
            target = Vector2(self.path[self.target_idx])
            direction = target - self.pos
            distance = direction.length()

        current_angle = self.angle
        target_angle = -math.degrees(math.atan2(direction.y, direction.x))
        angle_diff = (target_angle - current_angle) % 360
        if angle_diff > 180:
            angle_diff -= 360
        self.angle += min(max(angle_diff * 0.2, -self.turn_speed), self.turn_speed)

        speed_factor = min(1.0, distance / 70.0) * (0.65 if weather == "rain" else 1.0) * (0.8 if self.tire_wear > 50 else 1.0)
        self.speed = min(self.speed + self.acceleration * speed_factor, self.max_speed * speed_factor)
        fuel_rate = (0.017 if fuel_consumption == "high" else 0.012 if fuel_consumption == "medium" else 0.007) * (0.8 if weather == "rain" else 1.0)
        self.fuel -= fuel_rate
        self.tire_wear += (0.03 if self.tire_type == "soft" else 0.02 if self.tire_type == "medium" else 0.01) * (1.2 if weather == "rain" else 1.0)

        if random.random() < self.drs_chance and track.check_drs_zone(self.rect) and self.speed > self.max_speed * 0.8:
            self.drs_active = True
            self.max_speed = self.base_max_speed * 1.15
        else:
            self.drs_active = False
            self.max_speed = self.base_max_speed

        for other_car in cars:
            if other_car != self:
                dist_to_car = (self.pos - other_car.pos).length()
                if dist_to_car < 60:
                    self.speed *= 0.55
                    self.slip_effect = 15
                    self.damage_aero += 0.7
                    self.damage_engine += 0.4
                    self.tire_wear += 6.0
                    if dist_to_car > 0:
                        direction += (self.pos - other_car.pos).normalize() * 20

        self.vel = Vector2(0, -self.speed).rotate(-self.angle)
        self.pos += self.vel
        self.rect.center = self.pos

        if track.check_collision(self.rect):
            self.speed *= 0.35
            self.slip_effect = 20
            self.damage_aero += 2.5
            self.damage_engine += 1.5
            self.tire_wear += 12.0
            self.target_idx = (self.target_idx + 1) % len(self.path)

        if track.check_checkpoint(self.rect, self):
            self.checkpoint += 1
            if self.checkpoint == len(track.checkpoints):
                self.laps += 1
                lap_time = time.time() - self.current_lap_start
                self.lap_times.append(lap_time)
                self.current_lap_start = time.time()
                self.checkpoint = 0
