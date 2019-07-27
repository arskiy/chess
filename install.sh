#! /bin/sh

wget https://github.com/Arsukeey/chess/releases/download/v0.2/chess -O /usr/bin/chess
chmod 751 /usr/bin/chess

mkdir -p /usr/share/chess.d/sprites
chmod -R 755 ./src/sprites/
cp -r ./src/sprites/ /usr/share/chess.d/
