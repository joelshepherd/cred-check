import { sessionContext } from "../context/session.tsx";
import { None, React, Some } from "../deps.ts";

export default function Auth() {
  const session = React.useContext(sessionContext);
  const [state, setState] = React.useState({
    username: "",
  });

  const handleLogin = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    session.setToken(state.username.length > 0 ? Some(state.username) : None);
  };

  const handleLogout = () => session.setToken(None);

  return (
    <div>
      <h1>Auth</h1>
      {session.token.match({
        some: () => <button onClick={handleLogout}>Logout</button>,
        none: () => (
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
            <button type="submit">Login</button>
          </form>
        ),
      })}
    </div>
  );
}
