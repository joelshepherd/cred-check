import { React, Option, None, Some } from "../deps.ts";

export const tokenContext = React.createContext<Option<string>>(None);

interface Props {
  children: React.ReactChild;
}

export function TokenProvider(props: Props) {
  const stored = window.localStorage.getItem("token");
  const token = stored ? Some(stored) : None;

  return (
    <tokenContext.Provider value={token}>
      {props.children}
    </tokenContext.Provider>
  );
}
