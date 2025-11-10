function startRace()
    currentLap = 0
    positions = {}
    raceEvents = {}
    -- Dodaj kierowców gracza
    for _, driver in ipairs(playerTeam.drivers) do
        table.insert(positions, { team = playerTeam.name, driver = driver.name, position = 0, tire = playerTireChoice, lapsOnTire = 0, out = false })
    end
    -- Dodaj przeciwników
    for _, opp in ipairs(opponents) do
        for _, driver in ipairs(opp.drivers) do
            table.insert(positions, { team = opp.name, driver = driver.name, position = 0, tire = "Medium", lapsOnTire = 0, out = false })
        end
    end
    -- Losowa siatka startowa
    for i = #positions, 2, -1 do
        local j = math.random(i)
        positions[i], positions[j] = positions[j], positions[i]
    end
end

function simulateLap()
    currentLap = currentLap + 1
    for _, car in ipairs(positions) do
        if not car.out then
            local baseSpeed = getCarSpeed(car)
            local tireMod = getTireModifier(car.tire)
            local wearMod = 1 - (car.lapsOnTire / 30) * 0.15  -- Ulepszony zużycie
            local speed = baseSpeed * tireMod * wearMod * (0.9 + math.random() * 0.2)
            
            car.position = car.position + speed
            car.lapsOnTire = car.lapsOnTire + 1
            
            -- Losowe wydarzenia
            if math.random(100) > 98 then
                table.insert(raceEvents, car.driver .. " crashed!")
                car.out = true
            elseif math.random(100) > 95 then
                table.insert(raceEvents, car.driver .. " has engine failure!")
                car.out = true
            end
        end
    end
    
    -- Sortuj pozycje
    table.sort(positions, function(a, b) return a.position > b.position end)
    
    -- AI pit stops
    for _, car in ipairs(positions) do
        if not car.out and car.lapsOnTire > 20 and math.random() > 0.4 then
            car.tire = tireTypes[math.random(#tireTypes)]
            car.lapsOnTire = 0
            car.position = car.position - 300  -- Kara za pit
            table.insert(raceEvents, car.driver .. " pitted for " .. car.tire)
        end
    end
    
    -- Symuluj pit stops gracza (prosta logika)
    if currentLap == math.floor(raceLaps / (playerPitStops + 1)) then
        for _, car in ipairs(positions) do
            if car.team == playerTeam.name and not car.out then
                car.tire = tireTypes[math.random(#tireTypes)]
                car.lapsOnTire = 0
                car.position = car.position - 300
                table.insert(raceEvents, car.driver .. " player pit stop")
            end
        end
    end
end

function getCarSpeed(car)
    if car.team == playerTeam.name then
        for _, driver in ipairs(playerTeam.drivers) do
            if driver.name == car.driver then
                return driver.speed + playerTeam.carPerformance
            end
        end
    else
        for _, opp in ipairs(opponents) do
            if opp.name == car.team then
                for _, driver in ipairs(opp.drivers) do
                    if driver.name == car.driver then
                        return driver.speed + opp.carPerformance
                    end
                end
            end
        end
    end
    return 100
end

function getTireModifier(tire)
    if tire == "Soft" then return 1.3
    elseif tire == "Medium" then return 1.0
    elseif tire == "Hard" then return 0.8
    end
    return 1.0
end
