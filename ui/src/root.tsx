import { TokenProvider } from "./context/token.tsx";
import { React } from "./deps.ts";
import Router from "./screens/router.tsx";

export default function Root() {
  return (
    <TokenProvider>
      <Router />
    </TokenProvider>
  );
}
