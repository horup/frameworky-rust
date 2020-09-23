#!/bin/sh
git fetch --all
git add -f examples/balls/pkg
git stash
git checkout --track origin/pages
git stash pop
git commit -m "updated"