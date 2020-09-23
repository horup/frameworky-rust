cargo install wasm-pack
wasm-pack build examples/balls --target web
git add -f examples/balls/pkg
git stash
git checkout --track origin/pages
git stash pop
git commit -m "updated"