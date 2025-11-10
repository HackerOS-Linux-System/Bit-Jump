GAME_STATES = {
    MAIN_MENU = "main_menu",
    TEAM_SETUP = "team_setup",
    DRIVER_MANAGEMENT = "driver_management",
    CAR_UPGRADES = "car_upgrades",
    RACE_SETUP = "race_setup",
    RACE = "race",
    RESULTS = "results",
    OPTIONS = "options"
}

currentState = GAME_STATES.MAIN_MENU

playerTeam = createTeam("Your Team")

opponents = {}
raceLaps = 50
currentLap = 0
positions = {}
tireTypes = { "Soft", "Medium", "Hard" }
playerTireChoice = "Medium"
playerPitStops = 1
raceEvents = {}

function initGame()
    generateOpponents(8)
end

function updateGame(dt)
    if currentState == GAME_STATES.RACE then
        if currentLap < raceLaps then
            simulateLap()
        else
            currentState = GAME_STATES.RESULTS
        end
    end
end

function drawGame()
    love.graphics.setColor(1, 1, 1)
    
    if currentState == GAME_STATES.MAIN_MENU then
        drawMainMenu()
    elseif currentState == GAME_STATES.TEAM_SETUP then
        drawTeamSetup()
    elseif currentState == GAME_STATES.DRIVER_MANAGEMENT then
        drawDriverManagement()
    elseif currentState == GAME_STATES.CAR_UPGRADES then
        drawCarUpgrades()
    elseif currentState == GAME_STATES.RACE_SETUP then
        drawRaceSetup()
    elseif currentState == GAME_STATES.RACE then
        drawRace()
    elseif currentState == GAME_STATES.RESULTS then
        drawResults()
    elseif currentState == GAME_STATES.OPTIONS then
        drawOptions()
    end
end

function handleKeyPress(key)
    if key == "q" then
        love.event.quit()
    end
    
    if currentState == GAME_STATES.MAIN_MENU then
        if key == "1" then currentState = GAME_STATES.TEAM_SETUP
        elseif key == "2" then currentState = GAME_STATES.DRIVER_MANAGEMENT
        elseif key == "3" then currentState = GAME_STATES.CAR_UPGRADES
        elseif key == "4" then currentState = GAME_STATES.RACE_SETUP
        elseif key == "5" then currentState = GAME_STATES.OPTIONS
        end
    elseif currentState == GAME_STATES.TEAM_SETUP then
        -- Tutaj obsługa klawiszy dla setupu zespołu
        if key == "m" then currentState = GAME_STATES.MAIN_MENU end
    elseif currentState == GAME_STATES.DRIVER_MANAGEMENT then
        if key == "1" then upgradeDriver(1, "speed")
        elseif key == "2" then upgradeDriver(1, "reliability")
        elseif key == "3" then upgradeDriver(2, "speed")
        elseif key == "4" then upgradeDriver(2, "reliability")
        elseif key == "m" then currentState = GAME_STATES.MAIN_MENU end
    elseif currentState == GAME_STATES.CAR_UPGRADES then
        if key == "1" then upgradeCar("performance")
        elseif key == "m" then currentState = GAME_STATES.MAIN_MENU end
    elseif currentState == GAME_STATES.RACE_SETUP then
        if key == "1" then playerTireChoice = "Soft"
        elseif key == "2" then playerTireChoice = "Medium"
        elseif key == "3" then playerTireChoice = "Hard"
        elseif key == "+" then playerPitStops = playerPitStops + 1
        elseif key == "-" and playerPitStops > 0 then playerPitStops = playerPitStops - 1
        elseif key == "r" then
            startRace()
            currentState = GAME_STATES.RACE
        elseif key == "m" then currentState = GAME_STATES.MAIN_MENU end
    elseif currentState == GAME_STATES.RESULTS then
        if key == "m" then
            awardPrizes()
            currentState = GAME_STATES.MAIN_MENU
        end
    elseif currentState == GAME_STATES.OPTIONS then
        if key == "m" then currentState = GAME_STATES.MAIN_MENU end
    end
end
