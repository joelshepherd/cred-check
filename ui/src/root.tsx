import Nav from "./components/nav.tsx";
import { SessionProvider } from "./context/session.tsx";
import { React } from "./deps.ts";
import Router from "./screens/router.tsx";

export default function Root(): React.ReactElement {
  return (
    <SessionProvider>
      <>
        <Nav />
        <Router />
      </>
    </SessionProvider>
  );
}
