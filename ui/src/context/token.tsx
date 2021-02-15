import { React } from "../deps.ts";

export const tokenContext = React.createContext("");

interface Props {
  children: React.ReactChild;
}

export function TokenProvider(props: Props) {
  const token = window.localStorage.getItem("token") ?? "";

  return (
    <tokenContext.Provider value={token}>
      {props.children}
    </tokenContext.Provider>
  );
}
