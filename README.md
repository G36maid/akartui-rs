# akartui-rs

a rust base tio akari (light up ) game

## app design

- menu // horizontal list
    - newgame // a radom game
    - archive // select a game from the archive
    - settings // configure the game
    - help // show help
    - exit // exit the game

- game //play a game
    - layout
        - top :information like time and puzzle id
        - middle : puzzle
        - bottom : controls and helps
    - games information
        - id 1-950
        - board sizes
            - row
            - col
        - board cell type
            - wall
            - target
                - number 0 - 4
            - empty
        - cell state
            - is wall
            - light
            - dark
        - player do in cell
            - put lightbulb
            - put flag
            - remove lightbulb
            - remove flag

- archive //todo!

- settings //todo!
