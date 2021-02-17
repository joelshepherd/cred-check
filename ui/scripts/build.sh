mkdir -p dist
deno bundle --config tsconfig.json src/main.tsx > dist/main.js
cp public/* dist
