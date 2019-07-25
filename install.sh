#! /bin/sh

cargo build --release


sudo install -o root -m 751 ./target/release/chess /usr/share/

mkdir -p /usr/share/chess.d/sprites
chmod -R 755 ./src/sprites/
sudo cp -r ./src/sprites/ /usr/share/chess.d/
