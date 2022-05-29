#!/bin/sh
DIRNAME=$(dirname "$(realpath "$0")")
(cd "$DIRNAME/../www" && npm run dev)