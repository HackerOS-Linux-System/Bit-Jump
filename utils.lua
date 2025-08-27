-- utils.lua: Funkcje pomocnicze

function checkCollision(a, b)
return a.x < b.x + b.width and
a.x + a.width > b.x and
a.y < b.y + b.height and
a.y + a.height > b.y
end

function checkCircleCollision(a, circle)
local closestX = math.max(a.x, math.min(circle.x, a.x + a.width))
local closestY = math.max(a.y, math.min(circle.y, a.y + a.height))
local dx = closestX - circle.x
local dy = closestY - circle.y
return (dx * dx + dy * dy) < (circle.radius ^ 2)
end

function createParticles(x, y, count, color)
for i = 1, count do
    table.insert(player.particles, {
        x = x,
        y = y,
        vx = math.random(-100, 100),
                 vy = math.random(-100, 100),
                 life = math.random(0.2, 0.5),
                 color = color
    })
    end
    end

    function saveHighScore(score)
    love.filesystem.write("highscore.txt", tostring(score))
    end

    function loadHighScore()
    local data = love.filesystem.read("highscore.txt")
    return data and tonumber(data) or 0
    end

    function saveHighestLevel(level)
    love.filesystem.write("highestlevel.txt", tostring(level))
    end

    function loadHighestLevel()
    local data = love.filesystem.read("highestlevel.txt")
    return data and tonumber(data) or 1
    end

    function saveAchievements()
    local data = ""
    for k, v in pairs(achievements) do
        data = data .. k .. "=" .. tostring(v) .. "\n"
        end
        love.filesystem.write("achievements.txt", data)
        end

        function loadAchievements()
        local ach = {hacker = false, data_collector = false, endless_runner = false, game_winner = false}
        local data = love.filesystem.read("achievements.txt")
        if data then
            for line in data:gmatch("[^\r\n]+") do
                local k, v = line:match("(%w+)=(%w+)")
                if k and v then
                    ach[k] = (v == "true")
                    end
                    end
                    end
                    return ach
                    end

                    function unlockAchievement(name)
                    if not achievements[name] then
                        achievements[name] = true
                        saveAchievements()
                        end
                        end
