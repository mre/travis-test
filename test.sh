#!/usr/bin/env bash
set -ev

CHANGES=$(git diff HEAD^ --no-ext-diff --unified=0 --exit-code -a --no-prefix README.md | egrep "^\+\*")

echo $CHANGES
