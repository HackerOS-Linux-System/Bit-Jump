-- conf.lua: Konfiguracja Love2D

function love.conf(t)
t.window.width = 800
t.window.height = 600
t.window.resizable = true
t.window.fullscreen = false
t.window.title = "Hacker Game"
t.modules.audio = true
t.modules.sound = true
end
