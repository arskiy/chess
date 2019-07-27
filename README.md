# Chess - chess implemented in Rust.

[![Chess](https://img.shields.io/badge/Chess-v0.3.0-green.svg)]()

From my research, this is the first ever Rust implementation of a Chess AI. This was made in 10 days as a Rust exercise.

## Installation
#### Requirements:
- SDL2
- SDL2_image

If you are on Linux or BSD, simply
`git clone https://github.com/Arsukeey/chess.git`
`cd chess`
`sudo ./install.sh`
and then simply run `chess` on your terminal.

If you are on Windows or Mac, 
download the "portable" or "windows" (according to your OS) [release](https://github.com/Arsukeey/chess/releases), download the sprites and put them in a folder called `sprites`, creating a path like this
```
- chess
|
|- - chess.exe
|- -
   | - sprites
     | - b_white.png
     | - ...
```

## Gameplay
- *To play*, simply drag and drop the pieces. If the movement isn't valid, the game won't let you play that move.
- *To castle*, Drag the king to the rook.

### Screenshots:
[![Image](https://imgur.com/BThE07k.png)]
[![Image](https://imgur.com/kOuPjfU.png)]
