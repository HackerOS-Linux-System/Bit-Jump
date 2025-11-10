require("game_states")
require("team")
require("race")
require("ui")

function love.load()
    love.window.setTitle("The Racer")
    love.window.setMode(800, 600)
    love.graphics.setBackgroundColor(0.1, 0.1, 0.1)
    
    initGame()
end

function love.update(dt)
    updateGame(dt)
end

function love.draw()
    drawGame()
end

function love.keypressed(key)
    handleKeyPress(key)
end
