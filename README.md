# Azul game state generator
Generate an azul game state for algorithm testing

## How to use
To generate a fresh game state use:
```bash
azul-game new
```

To generate a new game state use:
```bash
azul-game play --state "/path/to/game_state.json" --move "/path/to/move.json"
```

> For both of these you can also specify `--output "/new/game_state.json"` to choose the path of the new game state, defaults to ./game_state.json and `--display` to display the output

To display a gamestate do
```bash
azul-game display --state "/path/to/game_state.json"
```

To automatically make bots play do
```bash
azul-game simulate --bot "/path/to/bot" --bot "/path/to/bot" --bot "/path/to/bot" [-d]
```
To play against a bot do
```bash
azul-game play --interactive --bot "/path/to/bot"
```

## Game states
The game state is a json file containing all the game and player data in the following structure
```json
{
  "gameState": {
    // The plates the player can pick from
    "availablePlates": [
      {
        "blue": 0,
				"yellow": 1,
				"red": 2,
				"black": 0,
				"white": 0,
      },
      // ...
    ],
    // The discarted tiles
    "discardPile": {
      "blue": 2,
			"yellow": 1,
			"red": 2,
			"black": 0,
			"white": 0,
      "penaltyTaken": false,
    }
  },
  "playerStates": [
    // The bot knows its state from here
    {
      "playerID": 0,
      "points": 12,
      // Representing populated spots, might change to list of colors
      "scoreGrid": [
        "10000",
        "01000",
        "00100",
        "00010",
        "00001",
      ],
      "prepGrid": [
        { },
        { "green": 1 },
        { },
        { "red": 2},
        { "blue": 3},
      ],
    }
    // ...
  ]
}
```

A bot should output a move that is also a JSON File in this format:
```json
{
  "from": 0 // plate with index 0, -1 for bin
  "color": "green",
  // To what row of the prepGrid and how many
  "to": {
    "0": 1,
    "3": 2,
  }
}
```

## Making Bots
To make a bot you just need to follow the JSON convention and for automatic execution the following CLI structure

```bash
bot --state "/path/to/state" --id 0 --output "/path/to/output.json"
```

It should also print `OK` after writing the file
