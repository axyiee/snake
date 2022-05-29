#!/bin/sh
DIRNAME=$(dirname "$(realpath "$0")")
(cd "$DIRNAME/.." && cargo run -p snake-lint-tool)