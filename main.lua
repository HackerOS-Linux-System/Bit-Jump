require("player")
require("levels")
require("menu")
require("settings")
require("utils")
function love.load()
love.window.setTitle("Bit Jump")
applySettings()
logo = nil -- Initialize logo as nil
if love.filesystem.getInfo("Bit-Jump.png") then -- Check if file exists
    logo = love.graphics.newImage("Bit-Jump.png")
    else
        print("Warning: Bit-Jump.png not found. Using default placeholder text.")
        end
        state = 'menu'
        gameOver = false
        gameWon = false
        currentLevel = 1
        score = 0
        lives = 3
        highScore = loadHighScore()
        highestLevel = loadHighestLevel()
        gameMode = 'normal'
        timeElapsed = 0
        timeLimit = 0
        achievements = loadAchievements()
        loadLevel(currentLevel)
        end
        function love.update(dt)
        if state == 'game' then
            updateGame(dt)
            end
            end
            function love.draw()
            if state == 'game' then
                drawGame()
                elseif state == 'menu' then
                    drawMenu()
                    elseif state == 'level_select' then
                        drawLevelSelect()
                        elseif state == 'game_modes' then
                            drawGameModes()
                            elseif state == 'settings' then
                                drawSettings()
                                elseif state == 'pause' then
                                    drawPause()
                                    elseif state == 'achievements' then
                                        drawAchievements()
                                        elseif state == 'credits' then
                                            drawCredits() -- Nowa sekcja
                                            elseif state == 'themes' then
                                                drawThemes() -- Nowa sekcja dla themes
                                                end
                                                end
                                                function love.keypressed(key)
                                                handleKeyPressed(key)
                                                end
