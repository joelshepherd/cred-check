import { React } from "../deps.ts";

export interface State {
  body: string;
}

interface Props {
  initialState?: State;
  onSubmit: (state: State) => void;
}

export default function OpinionForm({
  initialState = { body: "" },
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
        Body
        <textarea
          name="body"
          value={state.body}
          onChange={(e) => setState({ ...state, body: e.target.value })}
        />
      </label>
      <button type="submit">Submit</button>
    </form>
  );
}
