mkdir -p dist
cp public/* dist
deno bundle --unstable --watch --config tsconfig.json src/main.tsx dist/main.js
