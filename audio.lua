-- audio.lua: Zarządzanie dźwiękiem

sounds = {
    jump = nil,
    collect = nil,
    hack = nil,
    speed = nil,
    double_jump = nil,
    background = nil
}

function initAudio()
-- Sprawdza, czy plik istnieje przed załadowaniem
local function tryLoadSource(filename, type)
if love.filesystem.getInfo(filename) then
    return love.audio.newSource(filename, type)
    else
        print("Warning: Could not load " .. filename .. ". File does not exist.")
        return nil
        end
        end

        sounds.jump = tryLoadSource("jump.wav", "static")
        sounds.collect = tryLoadSource("collect.wav", "static")
        sounds.hack = tryLoadSource("hack.wav", "static")
        sounds.speed = tryLoadSource("speed.wav", "static")
        sounds.double_jump = tryLoadSource("double_jump.wav", "static")
        sounds.background = tryLoadSource("background.mp3", "stream")

        if sounds.background then
            sounds.background:setLooping(true)
            end
            updateAudioVolumes()
            if settings.musicOn and sounds.background then
                love.audio.play(sounds.background)
                end
                end

                function updateAudioVolumes()
                for _, sound in pairs(sounds) do
                    if sound then
                        sound:setVolume(settings.soundVolume)
                        end
                        end
                        end

                        function playSound(name)
                        if sounds[name] and settings.soundVolume > 0 then
                            love.audio.play(sounds[name])
                            end
                            end
