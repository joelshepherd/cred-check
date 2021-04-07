import * as api from "../api.ts";
import LoginForm, { State as LoginState } from "../components/loginForm.tsx";
import SignupForm, { State as SignupState } from "../components/signupForm.tsx";
import { sessionContext } from "../context/session.tsx";
import { None, Option, React, Some } from "../deps.ts";

export default function Auth(): React.ReactElement {
  const session = React.useContext(sessionContext);
  const [action, setAction] = React.useState<"login" | "signup">("login");
  const [error, setError] = React.useState<Option<string>>(None);

  const handleLogin = (state: LoginState) =>
    api.login({ username: state.username }).then((res) =>
      res.match({
        ok: ({ token }) => {
          setError(None);
          session.setToken(Some(token));
        },
        err: () => setError(Some("Incorrect username or password")),
      })
    );

  const handleSignup = async (state: SignupState) => {
    const res = await api.signup(state);
    res.match({
      ok: ({ token }) => {
        setError(None);
        session.setToken(Some(token));
      },
      err: () => setError(Some("Invalid details")),
    });
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
        none: () => {
          switch (action) {
            case "login":
              return (
                <>
                  <LoginForm onSubmit={handleLogin} />
                  {" or "}
                  <a onClick={() => setAction("signup")}>sign up</a>
                </>
              );
            case "signup":
              return (
                <>
                  <SignupForm onSubmit={handleSignup} />
                  {" or "}
                  <a onClick={() => setAction("login")}>login</a>
                </>
              );
          }
        },
      })}
    </div>
  );
}
