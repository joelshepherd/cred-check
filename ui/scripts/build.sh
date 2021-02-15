mkdir -p dist
deno bundle -c tsconfig.json src/main.tsx > dist/main.js
cp public/* dist
