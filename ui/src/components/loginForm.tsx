import { React } from "../deps.ts";

export interface State {
  username: string;
}

interface Props {
  initialState?: State;
  onSubmit: (state: State) => void;
}

export default function LoginForm({
  initialState = { username: "" },
  onSubmit,
}: Props): React.ReactElement {
  const [state, setState] = React.useState(initialState);

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onSubmit(state);
    setState(initialState);
  };

  return (
    <form onSubmit={handleSubmit}>
      <label>
        Username{" "}
        <input
          type="text"
          value={state.username}
          onChange={(e) => setState({ ...state, username: e.target.value })}
        />
      </label>{" "}
      <button type="submit">Login</button>
    </form>
  );
}
