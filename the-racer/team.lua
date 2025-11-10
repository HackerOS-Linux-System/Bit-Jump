function createTeam(name)
    return {
        name = name,
        drivers = {
            { name = "Driver A", speed = 100, reliability = 90 },
            { name = "Driver B", speed = 95, reliability = 95 }
        },
        carPerformance = 100,
        budget = 1000000,
        upgrades = {
            aero = 1,
            engine = 1,
            tires = 1
        }
    }
end

function upgradeDriver(driverIndex, stat)
    local cost = 100000
    if playerTeam.budget >= cost then
        if stat == "speed" then
            playerTeam.drivers[driverIndex].speed = playerTeam.drivers[driverIndex].speed + 5
        elseif stat == "reliability" then
            playerTeam.drivers[driverIndex].reliability = playerTeam.drivers[driverIndex].reliability + 5
        end
        playerTeam.budget = playerTeam.budget - cost
    end
end

function upgradeCar(upgradeType)
    local cost = 200000
    if playerTeam.budget >= cost then
        if upgradeType == "performance" then
            playerTeam.carPerformance = playerTeam.carPerformance + 10
        end
        -- Można dodać więcej typów ulepszeń
        playerTeam.budget = playerTeam.budget - cost
    end
end

function generateOpponents(num)
    opponents = {}
    for i = 1, num do
        table.insert(opponents, {
            name = "Team " .. i,
            drivers = {
                { name = "Opp Driver " .. i .. "A", speed = 90 + math.random(20), reliability = 80 + math.random(20) },
                { name = "Opp Driver " .. i .. "B", speed = 90 + math.random(20), reliability = 80 + math.random(20) }
            },
            carPerformance = 90 + math.random(20)
        })
    end
end

function awardPrizes()
    -- Nagrody na podstawie pozycji
    local playerPositions = {}
    for i, car in ipairs(positions) do
        if car.team == playerTeam.name then
            table.insert(playerPositions, i)
        end
    end
    table.sort(playerPositions)
    local prizes = {500000, 300000, 200000, 100000, 50000}  -- Dla top 5
    for i = 1, math.min(2, #playerPositions) do
        local pos = playerPositions[i]
        if pos <= 5 then
            playerTeam.budget = playerTeam.budget + (prizes[pos] or 0)
        end
    end
end
