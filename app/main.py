import typer
import subprocess
import os
from typing import Optional
from rich.console import Console
from rich.table import Table

# PySide6 imports for GUI
from PySide6.QtWidgets import (
    QApplication, QMainWindow, QWidget, QVBoxLayout, QHBoxLayout,
    QPushButton, QLabel, QGridLayout, QFrame, QMessageBox
)
from PySide6.QtGui import QFont, QColor, QPalette, QIcon
from PySide6.QtCore import Qt, QSize

# Initialize Typer app
app = typer.Typer(
    name="hackeros-games",
    help="HackerOS Games Launcher - Launch your favorite HackerOS games from the command line.",
    add_completion=False,
    rich_markup_mode="rich",
)

# Define game configurations
GAMES = {
    "starblaster": {
        "description": "Blast through stars in this exciting space shooter!",
        "path": "/usr/share/HackerOS/Scripts/HackerOS-Games/starblaster",
        "command": lambda path: [path],
        "color": "#00FF00",  # Green for GUI
        "theme_color": QColor(0, 255, 0),
    },
    "bit-jump": {
        "description": "Jump through bits in this platformer adventure!",
        "path": "/usr/share/HackerOS/Scripts/Bin/Bit-Jump.hacker",
        "command": lambda path: ["hackerc", "run", path],
        "color": "#0000FF",  # Blue for GUI
        "theme_color": QColor(0, 0, 255),
    },
    "the-racer": {
        "description": "Race through circuits in this high-speed thriller!",
        "path": "/usr/share/HackerOS/Scripts/HackerOS-Games/the-racer",
        "command": lambda path: [path],
        "color": "#FF0000",  # Red for GUI
        "theme_color": QColor(255, 0, 0),
    },
}

# Rich console for pretty output in CLI
console = Console()

def launch_game(game_name: str):
    """
    Launch the specified game using subprocess.
    """
    if game_name not in GAMES:
        console.print(f"[bold red]Error:[/bold red] Game '{game_name}' not found.")
        raise typer.Exit(code=1)
    game = GAMES[game_name]
    path = game["path"]
    if not os.path.exists(path):
        console.print(f"[bold red]Error:[/bold red] Game path '{path}' does not exist.")
        raise typer.Exit(code=1)
    command = game["command"](path)
    try:
        console.print(f"[bold {game['color']}]Launching {game_name.replace('-', ' ').title()}...[/bold {game['color']}]")
        subprocess.Popen(command)
    except Exception as e:
        console.print(f"[bold red]Error launching {game_name}: {e}[/bold red]")
        raise typer.Exit(code=1)

@app.command(help="Launch Starblaster game.")
def starblaster():
    launch_game("starblaster")

@app.command(help="Launch Bit-Jump game.")
def bit_jump():  # Use underscore for CLI hyphen
    launch_game("bit-jump")

@app.command(help="Launch The-Racer game.")
def the_racer():  # Use underscore for CLI hyphen
    launch_game("the-racer")

@app.command(help="List all available games.")
def list_games():
    table = Table(title="Available HackerOS Games", show_header=True, header_style="bold magenta")
    table.add_column("Game Name", style="cyan", no_wrap=True)
    table.add_column("Description", style="green")
    table.add_column("Command", style="yellow")
    for game_name, game in GAMES.items():
        table.add_row(
            f"[bold {game['color']}] {game_name.replace('-', ' ').title()} [/bold {game['color']}]",
            game["description"],
            f"hackeros-games {game_name}",
        )
    console.print(table)

@app.command(help="Show detailed information about a specific game.")
def info(game_name: str = typer.Argument(..., help="The name of the game (e.g., starblaster, bit-jump, the-racer)")):
    if game_name not in GAMES:
        console.print(f"[bold red]Error:[/bold red] Game '{game_name}' not found.")
        raise typer.Exit(code=1)
    game = GAMES[game_name]
    console.print(f"[bold underline]Game Information: {game_name.replace('-', ' ').title()}[/bold underline]")
    console.print(f"[bold]Description:[/bold] {game['description']}")
    console.print(f"[bold]Path:[/bold] {game['path']}")
    console.print(f"[bold]Launch Command:[/bold] hackeros-games {game_name}")
    console.print(f"[bold]Color Theme:[/bold] [ {game['color']} ]{game['color'].capitalize()}[/ {game['color']} ]")

# GUI Class Definition
class HackerOSGamesLauncher(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("HackerOS Games Launcher")
        self.setGeometry(100, 100, 800, 600)
        self.setStyleSheet("""
            QMainWindow {
                background-color: #1E1E1E;
            }
            QLabel {
                color: #FFFFFF;
                font-size: 14px;
            }
            QPushButton {
                background-color: #333333;
                color: #FFFFFF;
                border: 1px solid #555555;
                border-radius: 5px;
                padding: 10px;
                font-size: 16px;
                font-weight: bold;
            }
            QPushButton:hover {
                background-color: #444444;
            }
            QFrame {
                background-color: #2A2A2A;
                border: 1px solid #444444;
                border-radius: 8px;
            }
        """)

        # Central widget
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        main_layout = QVBoxLayout()
        central_widget.setLayout(main_layout)

        # Title label
        title_label = QLabel("Welcome to HackerOS Games!")
        title_label.setFont(QFont("Arial", 24, QFont.Bold))
        title_label.setAlignment(Qt.AlignCenter)
        title_label.setStyleSheet("color: #00FF00;")
        main_layout.addWidget(title_label)

        # Grid for games
        grid_layout = QGridLayout()
        main_layout.addLayout(grid_layout)

        # Add game cards
        row = 0
        col = 0
        for game_name, game in GAMES.items():
            card = self.create_game_card(game_name, game)
            grid_layout.addWidget(card, row, col)
            col += 1
            if col > 1:  # 2 columns
                col = 0
                row += 1

        # Footer
        footer_label = QLabel("Choose a game to launch")
        footer_label.setAlignment(Qt.AlignCenter)
        footer_label.setStyleSheet("color: #AAAAAA; font-size: 12px;")
        main_layout.addWidget(footer_label)

    def create_game_card(self, game_name: str, game: dict):
        card = QFrame()
        card_layout = QVBoxLayout()
        card.setLayout(card_layout)
        card.setFixedSize(350, 200)

        # Game title
        title = QLabel(game_name.replace('-', ' ').title())
        title.setFont(QFont("Arial", 18, QFont.Bold))
        title.setAlignment(Qt.AlignCenter)
        title.setStyleSheet(f"color: {game['color']};")
        card_layout.addWidget(title)

        # Description
        desc = QLabel(game["description"])
        desc.setWordWrap(True)
        desc.setAlignment(Qt.AlignCenter)
        card_layout.addWidget(desc)

        # Launch button
        launch_btn = QPushButton("Launch")
        launch_btn.setStyleSheet(f"""
            background-color: {game['color']};
            color: #000000;
            font-weight: bold;
        """)
        launch_btn.setFixedHeight(40)
        launch_btn.clicked.connect(lambda: self.launch_game_gui(game_name))
        card_layout.addWidget(launch_btn)

        return card

    def launch_game_gui(self, game_name: str):
        if game_name not in GAMES:
            QMessageBox.warning(self, "Error", f"Game '{game_name}' not found.")
            return
        game = GAMES[game_name]
        path = game["path"]
        if not os.path.exists(path):
            QMessageBox.warning(self, "Error", f"Game path '{path}' does not exist.")
            return
        command = game["command"](path)
        try:
            subprocess.Popen(command)
            QMessageBox.information(self, "Launching", f"Launching {game_name.replace('-', ' ').title()}...")
        except Exception as e:
            QMessageBox.critical(self, "Error", f"Error launching {game_name}: {e}")

@app.callback(invoke_without_command=True)
def main(ctx: typer.Context):
    if ctx.invoked_subcommand is None:
        # Launch GUI instead of CLI output
        qt_app = QApplication([])
        window = HackerOSGamesLauncher()
        window.show()
        qt_app.exec()
    else:
        # For subcommands, proceed as usual
        pass

if __name__ == "__main__":
    app()
