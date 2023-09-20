# Rusty Corks

## Features

### Mandatory

- Menu démarrer/quitter
- Obstacles qui bougent
- Un thread qui lit les touches du clavier
- Spawn voiture joueur et réussir à déplacer haut et bas
- Score qui s'incrémente

### Optional

- (Touche pause)
- (Personnel Best)

## Architecture

* Car (Trait)
* Game
  * Score
  * Player
    * PlayerCar
  * BotManager
      * CarBot[5]
