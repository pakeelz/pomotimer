#!/bin/bash
set -e

## BE CAREFUL TO USE THIS SCRIPT
release_version="$1"
title="$2"
message="$3"

if [ -z "$release_version" ] && [ -z "$title" ] && [ -z "$message"]; then
  "Usage: ./relase.sh [version] [title] [message]"
  "Example: ./release.sh v0.0.1  myapps 'new release'"
fi

if [ -n "$(which gh)" ]; then
  cp ./target/release/pomotimer .
  tar -czvf pomotimer-linux-x86_64.tar.gz pomotimer
  gh release create "$release_version" --title "$title" --notes "$message" pomotimer-linux-x86_64.tar.gz

  rm pomotimer
  rm pomotimer-linux-x86_64.tar.gz
fi
