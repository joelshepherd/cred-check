import { React, Option, None, Some } from "../deps.ts";

interface Session {
  authenticated: boolean;
  token: Option<string>;
  setToken: (token: Option<string>) => void;
}

export const sessionContext = React.createContext<Session>({} as Session);

interface Props {
  children: React.ReactChild;
}

export function SessionProvider(props: Props): React.ReactElement {
  const stored = window.localStorage.getItem("token");

  const [token, setToken] = React.useState(stored ? Some(stored) : None);

  const value: Session = {
    authenticated: token.isSome(),
    token,
    setToken: (token) => {
      window.localStorage.setItem("token", token.unwrapOr(""));
      setToken(token);
    },
  };

  return (
    <sessionContext.Provider value={value}>
      {props.children}
    </sessionContext.Provider>
  );
}
