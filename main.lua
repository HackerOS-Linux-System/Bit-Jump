-- main.lua: Główny plik gry, ładuje moduły i obsługuje główne pętle

require("conf")
require("player")
require("levels")
require("menu")
require("settings")
require("utils")
require("audio")

function love.load()
love.window.setTitle("Hacker Game")
initAudio() -- Z audio.lua
applySettings() -- Z settings.lua

state = 'menu'
gameOver = false
gameWon = false
currentLevel = 1
score = 0
lives = 3 -- Nowy system żyć
highScore = loadHighScore() -- Z utils.lua
highestLevel = loadHighestLevel() -- Z utils.lua
gameMode = 'normal'
timeElapsed = 0
timeLimit = 0
achievements = loadAchievements() -- Z utils.lua

loadLevel(currentLevel) -- Z levels.lua
end

function love.update(dt)
if state == 'game' then
    updateGame(dt) -- Z levels.lua
    updateAudioVolumes(dt) -- Z audio.lua, poprawiono z updateAudio
    end
    end

    function love.draw()
    if state == 'game' then
        drawGame() -- Z levels.lua
        elseif state == 'menu' then
            drawMenu() -- Z menu.lua
            elseif state == 'level_select' then
                drawLevelSelect() -- Z menu.lua
                elseif state == 'game_modes' then
                    drawGameModes() -- Z menu.lua
                    elseif state == 'settings' then
                        drawSettings() -- Z settings.lua
                        elseif state == 'pause' then
                            drawPause() -- Z menu.lua
                            elseif state == 'achievements' then
                                drawAchievements() -- Z menu.lua
                                end
                                end

                                function love.keypressed(key)
                                handleKeyPressed(key) -- Z menu.lua, settings.lua, player.lua
                                end
