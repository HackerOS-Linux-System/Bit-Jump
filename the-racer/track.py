import pygame

class Track:
    def __init__(self, name, width=1280, height=720):
        self.name = name
        self.width = width
        self.height = height
        self.start_positions = []
        self.walls = []
        self.checkpoints = []
        self.pit_lane = None
        self.drs_zones = []
        self.track_surface = []
        self.grass = []

        if name == "Monaco":
            # Monaco: Tight street circuit with smooth corners
            self.walls = [
                pygame.Rect(50, 50, 1180, 20),  # Top boundary
                pygame.Rect(50, 670, 1180, 20),  # Bottom boundary
                pygame.Rect(50, 50, 20, 620),  # Left boundary
                pygame.Rect(1210, 50, 20, 620),  # Right boundary
                pygame.Rect(250, 150, 20, 400),  # Inner walls
                pygame.Rect(250, 550, 400, 20),
                pygame.Rect(650, 150, 20, 400),
                pygame.Rect(650, 150, 400, 20),
                pygame.Rect(1050, 150, 20, 400),
            ]
            self.track_surface = [pygame.Rect(70, 70, 1140, 580)]  # Gray track
            self.grass = [
                pygame.Rect(70, 70, 180, 580),  # Left grass
                pygame.Rect(1050, 70, 160, 580),  # Right grass
                pygame.Rect(250, 70, 800, 80),  # Top grass
                pygame.Rect(250, 550, 800, 100),  # Bottom grass
            ]
            self.checkpoints = [
                pygame.Rect(100, 600, 50, 50),  # Start/finish
                pygame.Rect(200, 550, 50, 50),  # Sainte Devote
                pygame.Rect(300, 500, 50, 50),  # Casino
                pygame.Rect(400, 450, 50, 50),  # Mirabeau
                pygame.Rect(500, 400, 50, 50),  # Hairpin
                pygame.Rect(600, 350, 50, 50),  # Portier
                pygame.Rect(700, 300, 50, 50),  # Tunnel
                pygame.Rect(800, 250, 50, 50),  # Nouvelle Chicane
                pygame.Rect(900, 300, 50, 50),  # Tabac
                pygame.Rect(1000, 350, 50, 50),  # Swimming Pool
                pygame.Rect(1100, 400, 50, 50),  # La Rascasse
                pygame.Rect(1000, 500, 50, 50),  # Anthony Noghes
            ]
            self.start_positions = [(150 + i * 60, 630 + (i % 2) * 20) for i in range(20)]
            self.pit_lane = pygame.Rect(100, 590, 200, 50)
            self.drs_zones = [pygame.Rect(900, 590, 300, 50)]  # Main straight

        elif name == "Silverstone":
            # Silverstone: Wide, flowing track
            self.walls = [
                pygame.Rect(50, 50, 1180, 20),
                pygame.Rect(50, 670, 1180, 20),
                pygame.Rect(50, 50, 20, 620),
                pygame.Rect(1210, 50, 20, 620),
                pygame.Rect(250, 150, 20, 450),
                pygame.Rect(250, 600, 450, 20),
                pygame.Rect(700, 150, 20, 450),
                pygame.Rect(700, 150, 450, 20),
            ]
            self.track_surface = [pygame.Rect(70, 70, 1140, 580)]
            self.grass = [
                pygame.Rect(70, 70, 180, 580),
                pygame.Rect(1050, 70, 160, 580),
                pygame.Rect(250, 70, 800, 80),
                pygame.Rect(250, 550, 800, 100),
            ]
            self.checkpoints = [
                pygame.Rect(100, 600, 50, 50),  # Start/finish
                pygame.Rect(250, 550, 50, 50),  # Copse
                pygame.Rect(350, 500, 50, 50),  # Maggotts
                pygame.Rect(450, 450, 50, 50),  # Becketts
                pygame.Rect(550, 400, 50, 50),  # Chapel
                pygame.Rect(700, 350, 50, 50),  # Hangar Straight
                pygame.Rect(850, 300, 50, 50),  # Stowe
                pygame.Rect(950, 350, 50, 50),  # Vale
                pygame.Rect(1050, 400, 50, 50),  # Club
                pygame.Rect(900, 500, 50, 50),  # Abbey
            ]
            self.start_positions = [(150 + i * 60, 630 + (i % 2) * 20) for i in range(20)]
            self.pit_lane = pygame.Rect(100, 590, 200, 50)
            self.drs_zones = [
                pygame.Rect(900, 590, 300, 50),  # Main straight
                pygame.Rect(700, 350, 300, 50),  # Hangar Straight
            ]

        elif name == "Spa":
            # Spa: High-speed with technical sections
            self.walls = [
                pygame.Rect(50, 50, 1180, 20),
                pygame.Rect(50, 670, 1180, 20),
                pygame.Rect(50, 50, 20, 620),
                pygame.Rect(1210, 50, 20, 620),
                pygame.Rect(250, 150, 20, 450),
                pygame.Rect(250, 600, 450, 20),
                pygame.Rect(700, 150, 20, 450),
                pygame.Rect(700, 150, 450, 20),
            ]
            self.track_surface = [pygame.Rect(70, 70, 1140, 580)]
            self.grass = [
                pygame.Rect(70, 70, 180, 580),
                pygame.Rect(1050, 70, 160, 580),
                pygame.Rect(250, 70, 800, 80),
                pygame.Rect(250, 550, 800, 100),
            ]
            self.checkpoints = [
                pygame.Rect(100, 600, 50, 50),  # Start/finish
                pygame.Rect(200, 550, 50, 50),  # Eau Rouge
                pygame.Rect(300, 500, 50, 50),  # Raidillon
                pygame.Rect(450, 450, 50, 50),  # Kemmel Straight
                pygame.Rect(600, 400, 50, 50),  # Les Combes
                pygame.Rect(750, 350, 50, 50),  # Malmedy
                pygame.Rect(900, 300, 50, 50),  # Pouhon
                pygame.Rect(1050, 350, 50, 50),  # Blanchimont
                pygame.Rect(900, 500, 50, 50),  # Bus Stop
                pygame.Rect(700, 550, 50, 50),  # La Source
            ]
            self.start_positions = [(150 + i * 60, 630 + (i % 2) * 20) for i in range(20)]
            self.pit_lane = pygame.Rect(100, 590, 200, 50)
            self.drs_zones = [
                pygame.Rect(900, 590, 300, 50),  # Main straight
                pygame.Rect(450, 450, 300, 50),  # Kemmel Straight
            ]

        elif name == "Monza":
            # Monza: High-speed with chicanes
            self.walls = [
                pygame.Rect(50, 50, 1180, 20),
                pygame.Rect(50, 670, 1180, 20),
                pygame.Rect(50, 50, 20, 620),
                pygame.Rect(1210, 50, 20, 620),
                pygame.Rect(250, 150, 20, 400),
                pygame.Rect(250, 550, 400, 20),
                pygame.Rect(650, 150, 20, 400),
                pygame.Rect(800, 150, 20, 400),
            ]
            self.track_surface = [pygame.Rect(70, 70, 1140, 580)]
            self.grass = [
                pygame.Rect(70, 70, 180, 580),
                pygame.Rect(1050, 70, 160, 580),
                pygame.Rect(250, 70, 800, 80),
                pygame.Rect(250, 550, 800, 100),
            ]
            self.checkpoints = [
                pygame.Rect(100, 600, 50, 50),  # Start/finish
                pygame.Rect(200, 550, 50, 50),  # Curva Grande
                pygame.Rect(300, 500, 50, 50),  # Della Roggia
                pygame.Rect(450, 450, 50, 50),  # Lesmo 1
                pygame.Rect(600, 400, 50, 50),  # Lesmo 2
                pygame.Rect(750, 350, 50, 50),  # Ascari
                pygame.Rect(900, 300, 50, 50),  # Parabolica
                pygame.Rect(1050, 350, 50, 50),  # Back straight
                pygame.Rect(900, 500, 50, 50),  # Rettifilo
            ]
            self.start_positions = [(150 + i * 60, 630 + (i % 2) * 20) for i in range(20)]
            self.pit_lane = pygame.Rect(100, 590, 200, 50)
            self.drs_zones = [
                pygame.Rect(900, 590, 300, 50),  # Main straight
                pygame.Rect(900, 350, 300, 50),  # Back straight
            ]

        elif name == "Suzuka":
            # Suzuka: Flowing S-curves and technical turns
            self.walls = [
                pygame.Rect(50, 50, 1180, 20),
                pygame.Rect(50, 670, 1180, 20),
                pygame.Rect(50, 50, 20, 620),
                pygame.Rect(1210, 50, 20, 620),
                pygame.Rect(250, 150, 20, 400),
                pygame.Rect(250, 550, 400, 20),
                pygame.Rect(650, 150, 20, 400),
                pygame.Rect(800, 150, 20, 400),
            ]
            self.track_surface = [pygame.Rect(70, 70, 1140, 580)]
            self.grass = [
                pygame.Rect(70, 70, 180, 580),
                pygame.Rect(1050, 70, 160, 580),
                pygame.Rect(250, 70, 800, 80),
                pygame.Rect(250, 550, 800, 100),
            ]
            self.checkpoints = [
                pygame.Rect(100, 600, 50, 50),  # Start/finish
                pygame.Rect(200, 550, 50, 50),  # Turn 1
                pygame.Rect(300, 500, 50, 50),  # S-curves 1
                pygame.Rect(400, 450, 50, 50),  # S-curves 2
                pygame.Rect(500, 400, 50, 50),  # S-curves 3
                pygame.Rect(600, 350, 50, 50),  # Degner
                pygame.Rect(750, 300, 50, 50),  # Hairpin
                pygame.Rect(900, 350, 50, 50),  # Spoon
                pygame.Rect(1050, 400, 50, 50),  # 130R
                pygame.Rect(900, 500, 50, 50),  # Casio Triangle
            ]
            self.start_positions = [(150 + i * 60, 630 + (i % 2) * 20) for i in range(20)]
            self.pit_lane = pygame.Rect(100, 590, 200, 50)
            self.drs_zones = [pygame.Rect(900, 590, 300, 50)]  # Main straight

        elif name == "Interlagos":
            # Interlagos: Compact with tight corners
            self.walls = [
                pygame.Rect(50, 50, 1180, 20),
                pygame.Rect(50, 670, 1180, 20),
                pygame.Rect(50, 50, 20, 620),
                pygame.Rect(1210, 50, 20, 620),
                pygame.Rect(250, 150, 20, 400),
                pygame.Rect(250, 550, 400, 20),
                pygame.Rect(650, 150, 20, 400),
                pygame.Rect(800, 150, 20, 400),
            ]
            self.track_surface = [pygame.Rect(70, 70, 1140, 580)]
            self.grass = [
                pygame.Rect(70, 70, 180, 580),
                pygame.Rect(1050, 70, 160, 580),
                pygame.Rect(250, 70, 800, 80),
                pygame.Rect(250, 550, 800, 100),
            ]
            self.checkpoints = [
                pygame.Rect(100, 600, 50, 50),  # Start/finish
                pygame.Rect(200, 550, 50, 50),  # Senna S
                pygame.Rect(300, 500, 50, 50),  # Curva do Sol
                pygame.Rect(450, 450, 50, 50),  # Descida do Lago
                pygame.Rect(600, 400, 50, 50),  # Ferradura
                pygame.Rect(750, 350, 50, 50),  # Pinheirinho
                pygame.Rect(900, 400, 50, 50),  # Bico de Pato
                pygame.Rect(1050, 450, 50, 50),  # Junção
                pygame.Rect(900, 500, 50, 50),  # Subida dos Boxes
            ]
            self.start_positions = [(150 + i * 60, 630 + (i % 2) * 20) for i in range(20)]
            self.pit_lane = pygame.Rect(100, 590, 200, 50)
            self.drs_zones = [
                pygame.Rect(900, 590, 300, 50),  # Main straight
                pygame.Rect(900, 400, 200, 50),  # Reta Oposta
            ]

    def check_pitstop(self, rect):
        return self.pit_lane.colliderect(rect) if self.pit_lane else False

    def check_drs_zone(self, rect):
        return any(drs_zone.colliderect(rect) for drs_zone in self.drs_zones)

    def check_collision(self, rect):
        return any(wall.colliderect(rect) for wall in self.walls)

    def check_checkpoint(self, rect, car):
        if car.checkpoint < len(self.checkpoints) and self.checkpoints[car.checkpoint].colliderect(rect):
            return True
        return False

    def draw(self, surface):
        # Draw grass (green)
        for grass in self.grass:
            pygame.draw.rect(surface, (0, 100, 0), grass)
        # Draw track surface (gray)
        for track in self.track_surface:
            pygame.draw.rect(surface, (100, 100, 100), track)
        # Draw walls (black)
        for wall in self.walls:
            pygame.draw.rect(surface, (0, 0, 0), wall)
        # Draw pit lane (white)
        if self.pit_lane:
            pygame.draw.rect(surface, (255, 255, 255), self.pit_lane)
        # Draw DRS zones (blue)
        for drs_zone in self.drs_zones:
            pygame.draw.rect(surface, (0, 0, 255), drs_zone)
        # Draw checkpoints (semi-transparent for visibility)
        for checkpoint in self.checkpoints:
            pygame.draw.rect(surface, (255, 255, 0, 100), checkpoint)
