import { None, React, Some } from "../deps.ts";
import { subscribe, unsubscribe } from "../history.ts";
import View from "./view.tsx";

export default function Router() {
  const [path, setPath] = React.useState(window.location.pathname);
  React.useEffect(() => {
    subscribe(setPath);
    return () => unsubscribe(setPath);
  }, []);

  if (path.startsWith("/auth/")) {
    return <p>auth</p>;
  }

  if (path.startsWith("/view/")) {
    const source = path.slice(6);
    return <View url={Some(source)} />;
  }

  return <View url={None} />;
}
