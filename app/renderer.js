const { ipcRenderer } = require('electron');

function launchGame(game) {
  const gamePaths = {
    'starblaster': '/usr/share/HackerOS/Scripts/HackerOS-Games/starblaster',
    'bit-jump': '/usr/share/HackerOS/Scripts/HackerOS-Games/bit-jump.love',
    'the-racer': '/usr/share/HackerOS/Scripts/HackerOS-Games/the-racer'
  };

  const launchCommands = {
    'starblaster': gamePaths['starblaster'],
    'bit-jump': `love ${gamePaths['bit-jump']}`,
    'the-racer': gamePaths['the-racer']
  };

  ipcRenderer.send('launch-game', launchCommands[game]);
}
