import { React } from "../deps.ts";

export interface State {
  name: string;
  username: string;
}

interface Props {
  initialState?: State;
  onSubmit: (state: State) => void;
}

export default function SignupForm({
  initialState = { name: "", username: "" },
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
          onChange={(e) => setState({ ...state, username: e.target.value })}
        />
      </label>{" "}
      <button type="submit">Sign up</button>
    </form>
  );
}
