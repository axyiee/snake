#!/bin/sh
DIRNAME=$(dirname "$(realpath "$0")")
(cd "$DIRNAME/.." && trunk serve)