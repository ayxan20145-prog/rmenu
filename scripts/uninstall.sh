#!/bin/sh
set -e

NAME="rmenu"

echo "Removing $NAME..."

sudo rm -f "/usr/local/bin/$NAME"

echo "Done!"
