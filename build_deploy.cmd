cargo install wasm-pack
wasm-pack build examples/balls --target web

git add -f examples/balls/pkg
git stash

git fetch --all
git checkout --track origin/pages
git rm examples/balls/pkg -r
git add .
git commit -m "cleanup"
git stash pop
git commit -m "updated"