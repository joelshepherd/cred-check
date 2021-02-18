import * as api from "../api.ts";
import { sessionContext } from "../context/session.tsx";
import { None, React, Some, Option } from "../deps.ts";

export default function Auth(): React.ReactElement {
  const session = React.useContext(sessionContext);
  const [state, setState] = React.useState({
    name: "",
    username: "",
  });
  const [action, setAction] = React.useState<"login" | "signup">("login");
  const [error, setError] = React.useState<Option<string>>(None);

  const handleLogin = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    api.login({ username: state.username }).then((result) =>
      result.match({
        ok: ({ token }) => {
          setError(None);
          session.setToken(Some(token));
        },
        err: () => setError(Some("Incorrect username or password")),
      })
    );
  };

  const handleSignup = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    api.signup(state).then((result) =>
      result.match({
        ok: ({ token }) => {
          setError(None);
          session.setToken(Some(token));
        },
        err: () => setError(Some("Invalid details")),
      })
    );
  };

  const handleLogout = () => session.setToken(None);

  return (
    <div>
      <h1>Auth</h1>
      {error.match({
        some: (err) => <p>{err}</p>,
        none: () => null,
      })}
      {session.token.match({
        some: () => <button onClick={handleLogout}>Logout</button>,
        none: () =>
          action === "login" ? (
            <form onSubmit={handleLogin}>
              <label>
                Username{" "}
                <input
                  type="text"
                  value={state.username}
                  onChange={(e) =>
                    setState({ ...state, username: e.target.value })
                  }
                />
              </label>{" "}
              <button type="submit">Login</button>{" "}
              <a onClick={() => setAction("signup")}>or sign up</a>
            </form>
          ) : (
            <form onSubmit={handleSignup}>
              <label>
                Name{" "}
                <input
                  type="text"
                  value={state.name}
                  onChange={(e) => setState({ ...state, name: e.target.value })}
                />
              </label>{" "}
              <label>
                Username{" "}
                <input
                  type="text"
                  value={state.username}
                  onChange={(e) =>
                    setState({ ...state, username: e.target.value })
                  }
                />
              </label>{" "}
              <button type="submit">Sign up</button>{" "}
              <a onClick={() => setAction("login")}>or login</a>
            </form>
          ),
      })}
    </div>
  );
}
