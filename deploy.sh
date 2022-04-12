
#!/usr/bin/env sh

# abort on errors
set -e

# build
wasm-pack build
yarn run build

# navigate into the build output directory
cd dist

git init
git add -A
git commit -m "deploy"
git push -f git@github.com:felixnaredi/starrock.git main:deploy
cd -
