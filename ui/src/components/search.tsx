import { Option, React } from "../deps.ts";

interface Props {
  initialState: Option<string>;
  onSearch: (query: string) => void;
}

export default function Search(props: Props): React.ReactElement {
  const [state, setState] = React.useState(props.initialState.unwrapOr(""));
  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    props.onSearch(state);
  };

  return (
    <form onSubmit={handleSubmit}>
      <label>
        Search{" "}
        <input
          type="search"
          value={state}
          onChange={(e) => setState(e.target.value)}
        />
      </label>{" "}
      <button type="submit">Go</button>
    </form>
  );
}
