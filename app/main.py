import sys
import os
import subprocess
from PySide6.QtWidgets import QApplication, QWidget, QVBoxLayout, QHBoxLayout, QLabel, QPushButton, QGridLayout, QMainWindow
from PySide6.QtGui import QFont, QPixmap, QIcon, QFontDatabase, QLinearGradient, QBrush, QColor, QPainter, QPalette, QMovie
from PySide6.QtCore import Qt, QPropertyAnimation, QSequentialAnimationGroup, QRect, QTimer, QAbstractAnimation, QEasingCurve, QUrl, QPoint, QElapsedTimer

# Use system icons directory
IMAGES_DIR = '/usr/share/HackerOS/ICONS'

# Game paths and commands (hardcoded as in original)
GAME_PATHS = {
    'starblaster': '/usr/share/HackerOS/Scripts/HackerOS-Games/starblaster',
    'bit-jump': '/usr/share/HackerOS/Scripts/HackerOS-Games/bit-jump.love',
    'the-racer': '/usr/share/HackerOS/Scripts/HackerOS-Games/the-racer'
}
LAUNCH_COMMANDS = {
    'starblaster': [GAME_PATHS['starblaster']],
    'bit-jump': ['love', GAME_PATHS['bit-jump']],
    'the-racer': [GAME_PATHS['the-racer']]
}

class ParticleWidget(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setAttribute(Qt.WA_TransparentForMouseEvents)
        self.particles = [
            {'cx_start': 0.10, 'cx_end': 0.15, 'cx_dur': 4, 'cy_start': 0.15, 'cy_end': 0.85, 'cy_dur': 4, 'r': 5, 'color': QColor(57, 255, 20, 178)},
            {'cx_start': 0.30, 'cx_end': 0.35, 'cx_dur': 5, 'cy_start': 0.50, 'cy_end': 0.95, 'cy_dur': 5, 'r': 4, 'color': QColor(0, 183, 235, 178)},
            {'cx_start': 0.60, 'cx_end': 0.55, 'cx_dur': 4.5, 'cy_start': 0.25, 'cy_end': 0.75, 'cy_dur': 4.5, 'r': 6, 'color': QColor(255, 7, 58, 178)},
            {'cx_start': 0.80, 'cx_end': 0.85, 'cx_dur': 5.5, 'cy_start': 0.40, 'cy_end': 0.90, 'cy_dur': 5.5, 'r': 3, 'color': QColor(255, 255, 255, 153)},
            {'cx_start': 0.20, 'cx_end': 0.25, 'cx_dur': 4.8, 'cy_start': 0.70, 'cy_end': 0.20, 'cy_dur': 4.8, 'r': 4, 'color': QColor(57, 255, 20, 178)}
        ]
        self.elapsed = QElapsedTimer()
        self.elapsed.start()
        self.timer = QTimer(self)
        self.timer.timeout.connect(self.update)
        self.timer.start(16)  # ~60 FPS

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.Antialiasing)
        w, h = self.width(), self.height()
        for p in self.particles:
            # cx
            dur_cx = p['cx_dur'] * 1000
            t_cx = self.elapsed.elapsed() % dur_cx
            frac_cx = t_cx / dur_cx
            cx = p['cx_start'] + frac_cx * (p['cx_end'] - p['cx_start'])
            cx_abs = cx * w
            # cy
            dur_cy = p['cy_dur'] * 1000
            t_cy = self.elapsed.elapsed() % dur_cy
            frac_cy = t_cy / dur_cy
            cy = p['cy_start'] + frac_cy * (p['cy_end'] - p['cy_start'])
            cy_abs = cy * h
            painter.setBrush(QBrush(p['color']))
            painter.setPen(Qt.NoPen)
            painter.drawEllipse(int(cx_abs - p['r']), int(cy_abs - p['r']), int(p['r']*2), int(p['r']*2))

class GameCard(QWidget):
    def __init__(self, game_name, image_file, color_class, parent=None):
        super().__init__(parent)
        self.game_name = game_name
        layout = QVBoxLayout(self)
        layout.setContentsMargins(40, 40, 40, 40)  # p-10

        # Image
        self.image_label = QLabel()
        image_path = os.path.join(IMAGES_DIR, image_file)
        pixmap = QPixmap(image_path)
        self.image_label.setPixmap(pixmap.scaledToHeight(320, Qt.SmoothTransformation))  # h-80 approximate
        self.image_label.setAlignment(Qt.AlignCenter)
        layout.addWidget(self.image_label)

        # Animate pixel-bounce
        self.bounce_anim = QPropertyAnimation(self.image_label, b'pos')
        self.bounce_anim.setDuration(1400)
        self.bounce_anim.setLoopCount(-1)
        self.bounce_anim.setEasingCurve(QEasingCurve.InOutSine)
        start_pos = self.image_label.pos()
        self.bounce_anim.setKeyValueAt(0, start_pos)
        self.bounce_anim.setKeyValueAt(0.5, start_pos + QPoint(0, -25))
        self.bounce_anim.setKeyValueAt(1, start_pos)
        self.bounce_anim.start()

        # Title
        title = QLabel(game_name.replace('-', ' ').title())
        title.setFont(QFont('Press Start 2P', 24))  # text-4xl
        title.setAlignment(Qt.AlignCenter)
        title.setStyleSheet("color: white;")
        layout.addWidget(title)

        # Button
        button = QPushButton('Launch')
        button.setFont(QFont('Press Start 2P', 12))
        neon_color = self.get_neon_color(color_class)
        dark_color = self.get_dark_color(color_class)
        button.setStyleSheet(f"""
            QPushButton {{
                background-color: {neon_color};
                color: black;
                padding: 16px;
                border-radius: 8px;
            }}
            QPushButton:hover {{
                background-color: {dark_color};
            }}
        """)
        button.clicked.connect(self.launch_game)
        layout.addWidget(button)

        # Card style
        shadow_color = self.get_shadow_color(color_class)
        self.setStyleSheet(f"""
            background-color: #1a2533;
            border-radius: 16px;
            border: 4px solid #ffffff55;
            box-shadow: 0 0 30px rgba(255, 255, 255, 0.5), {shadow_color};
        """)

    def get_neon_color(self, color_class):
        if color_class == 'green': return '#39ff14'
        if color_class == 'blue': return '#00b7eb'
        if color_class == 'red': return '#ff073a'
        return '#39ff14'

    def get_dark_color(self, color_class):
        if color_class == 'green': return '#2ecc40'
        if color_class == 'blue': return '#0099c9'
        if color_class == 'red': return '#d90429'
        return '#2ecc40'

    def get_shadow_color(self, color_class):
        if color_class == 'green': return '0 0 40px rgba(57, 255, 20, 0.7), 0 0 80px rgba(57, 255, 20, 0.5)'
        if color_class == 'blue': return '0 0 40px rgba(0, 183, 235, 0.7), 0 0 80px rgba(0, 183, 235, 0.5)'
        if color_class == 'red': return '0 0 40px rgba(255, 7, 58, 0.7), 0 0 80px rgba(255, 7, 58, 0.5)'
        return '0 0 40px rgba(57, 255, 20, 0.7), 0 0 80px rgba(57, 255, 20, 0.5)'

    def launch_game(self):
        command = LAUNCH_COMMANDS.get(self.game_name)
        if command:
            try:
                subprocess.Popen(command)
            except Exception as e:
                print(f"Error launching {self.game_name}: {e}")

class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("HackerOS Games")
        self.setGeometry(100, 100, 1280, 900)
        self.setWindowIcon(QIcon(os.path.join(IMAGES_DIR, 'HackerOS-Games.png')))

        # Central widget
        central = QWidget()
        self.setCentralWidget(central)
        main_layout = QVBoxLayout(central)
        main_layout.setContentsMargins(40, 40, 40, 40)  # p-10

        # Background gradient
        palette = central.palette()
        gradient = QLinearGradient(0, 0, self.width(), self.height())
        gradient.setColorAt(0, QColor(8, 12, 20))
        gradient.setColorAt(1, QColor(22, 32, 47))
        palette.setBrush(QPalette.Window, QBrush(gradient))
        central.setPalette(palette)
        central.setAutoFillBackground(True)

        # Title
        title = QLabel("HackerOS Games")
        title.setFont(QFont('Press Start 2P', 48))  # text-6xl
        title.setAlignment(Qt.AlignCenter)
        title.setStyleSheet("color: #39ff14; text-shadow: 0 0 10px #39ff14, 0 0 25px #39ff14, 0 0 50px #39ff14;")
        main_layout.addWidget(title)

        # Grid for games
        grid = QGridLayout()
        grid.setSpacing(48)  # gap-12

        # Starblaster
        starblaster = GameCard('starblaster', 'starblaster.png', 'green')
        grid.addWidget(starblaster, 0, 0)

        # Bit-Jump
        bitjump = GameCard('bit-jump', 'Bit-Jump.png', 'blue')
        grid.addWidget(bitjump, 0, 1)

        # The-Racer
        racer = GameCard('the-racer', 'The-Racer.png', 'red')
        grid.addWidget(racer, 0, 2)

        main_layout.addLayout(grid)

        # Logo
        self.logo = QLabel(central)
        logo_pixmap = QPixmap(os.path.join(IMAGES_DIR, 'HackerOS-Games.png')).scaled(80, 80, Qt.KeepAspectRatio)
        self.logo.setPixmap(logo_pixmap)
        self.logo.setFixedSize(80, 80)
        self.logo.move(self.width() - 100, 24)  # top-6 right-6
        self.logo.raise_()

        # Particles
        self.particles = ParticleWidget(central)
        self.particles.setGeometry(0, 0, self.width(), self.height())
        self.particles.lower()

    def resizeEvent(self, event):
        self.particles.setGeometry(0, 0, self.width(), self.height())
        self.logo.move(self.width() - 100, 24)
        super().resizeEvent(event)

if __name__ == '__main__':
    app = QApplication(sys.argv)
    # Assume 'Press Start 2P' font is installed system-wide; no need to load from file
    window = MainWindow()
    window.show()
    sys.exit(app.exec())
