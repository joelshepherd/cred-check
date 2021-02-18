import { None, React, Some } from "../deps.ts";
import { subscribe, unsubscribe } from "../history.ts";
import Auth from "./auth.tsx";
import View from "./view.tsx";

export default function Router(): React.ReactElement {
  const [path, setPath] = React.useState(window.location.pathname);
  React.useEffect(() => {
    subscribe(setPath);
    return () => unsubscribe(setPath);
  }, []);

  if (path.startsWith("/auth")) {
    return <Auth />;
  }

  if (path.startsWith("/search")) {
    const source = path.slice(8);
    return <View url={Some(source)} />;
  }

  return <View url={None} />;
}
