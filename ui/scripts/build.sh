mkdir -p dist
cp public/* dist
deno bundle --config tsconfig.json src/main.tsx dist/main.js
