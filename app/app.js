function launchGame(game) {
  // Simulate launching games (since running system binaries directly in browser isn't possible)
  const gamePaths = {
    'starblaster': '/usr/share/HackerOS/Scripts/HackerOS-Games/starblaster',
    'bit-jump': '/usr/share/HackerOS/Scripts/HackerOS-Games/bit-jump.love',
    'the-racer': '/usr/share/HackerOS/Scripts/HackerOS-Games/the-racer'
  };

  const launchCommands = {
    'starblaster': `${gamePaths['starblaster']}`,
    'bit-jump': `love ${gamePaths['bit-jump']}`,
    'the-racer': `${gamePaths['the-racer']}`
  };

  // For demo purposes, show an alert with the command that would be executed
  alert(`Would launch: ${launchCommands[game]}`);
  
  // Note: In a real application, you would need a backend (e.g., Node.js with child_process)
  // to execute these commands on the server-side. Example (not executable in browser):
  /*
  const { exec } = require('child_process');
  exec(launchCommands[game], (error, stdout, stderr) => {
    if (error) {
      console.error(`Error launching ${game}: ${error}`);
      return;
    }
    console.log(`Launched ${game}: ${stdout}`);
  });
  */
}
