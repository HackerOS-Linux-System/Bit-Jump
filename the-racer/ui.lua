function drawMainMenu()
    love.graphics.printf("The Racer - Main Menu", 0, 100, 800, "center")
    love.graphics.printf("1. Team Setup", 0, 150, 800, "center")
    love.graphics.printf("2. Driver Management", 0, 200, 800, "center")
    love.graphics.printf("3. Car Upgrades", 0, 250, 800, "center")
    love.graphics.printf("4. Race Setup", 0, 300, 800, "center")
    love.graphics.printf("5. Options", 0, 350, 800, "center")
    love.graphics.printf("Q to Quit", 0, 400, 800, "center")
end

function drawTeamSetup()
    love.graphics.printf("Team Setup", 0, 50, 800, "center")
    love.graphics.printf("Team Name: " .. playerTeam.name, 100, 100, 600)
    love.graphics.printf("Budget: $" .. playerTeam.budget, 100, 150, 600)
    love.graphics.printf("Press M to Menu", 0, 500, 800, "center")
end

function drawDriverManagement()
    love.graphics.printf("Driver Management", 0, 50, 800, "center")
    for i, driver in ipairs(playerTeam.drivers) do
        love.graphics.printf("Driver " .. i .. ": " .. driver.name .. " (Speed: " .. driver.speed .. ", Rel: " .. driver.reliability .. ")", 100, 100 + (i-1)*50, 600)
        love.graphics.printf("Press " .. (i*2-1) .. " to upgrade Speed ($100k), " .. (i*2) .. " for Reliability ($100k)", 100, 130 + (i-1)*50, 600)
    end
    love.graphics.printf("Press M to Menu", 0, 500, 800, "center")
end

function drawCarUpgrades()
    love.graphics.printf("Car Upgrades", 0, 50, 800, "center")
    love.graphics.printf("Car Performance: " .. playerTeam.carPerformance, 100, 100, 600)
    love.graphics.printf("Press 1 to upgrade Performance ($200k)", 100, 150, 600)
    love.graphics.printf("Press M to Menu", 0, 500, 800, "center")
end

function drawRaceSetup()
    love.graphics.printf("Race Setup", 0, 50, 800, "center")
    love.graphics.printf("Choose Starting Tires: 1-Soft, 2-Medium, 3-Hard (Current: " .. playerTireChoice .. ")", 100, 100, 600)
    love.graphics.printf("Planned Pit Stops: " .. playerPitStops .. " (+/- to change)", 100, 150, 600)
    love.graphics.printf("Press R to Start Race", 0, 500, 800, "center")
    love.graphics.printf("Press M to Menu", 0, 550, 800, "center")
end

function drawRace()
    love.graphics.printf("Race - Lap " .. currentLap .. "/" .. raceLaps, 0, 50, 800, "center")
    for i, car in ipairs(positions) do
        local status = car.out and "OUT" or "Tire: " .. car.tire .. " (Laps: " .. car.lapsOnTire .. ")"
        love.graphics.printf(i .. ". " .. car.driver .. " (" .. car.team .. ") - " .. status, 100, 100 + (i-1)*25, 600)
    end
    -- Prosty tor
    love.graphics.rectangle("line", 600, 100, 150, 400)
    for i, car in ipairs(positions) do
        if not car.out then
            local y = 100 + (400 / #positions) * (i - 1)
            love.graphics.circle("fill", 675, y, 8)
        end
    end
end

function drawResults()
    love.graphics.printf("Race Results", 0, 50, 800, "center")
    for i, car in ipairs(positions) do
        love.graphics.printf(i .. ". " .. car.driver .. " (" .. car.team .. ")", 100, 100 + (i-1)*25, 600)
    end
    love.graphics.printf("Events:", 100, 400, 600)
    for i, event in ipairs(raceEvents) do
        love.graphics.printf(event, 100, 420 + (i-1)*20, 600)
    end
    love.graphics.printf("Press M to Menu", 0, 550, 800, "center")
end

function drawOptions()
    love.graphics.printf("Options", 0, 50, 800, "center")
    love.graphics.printf("No options yet. Press M to Menu", 0, 200, 800, "center")
end
