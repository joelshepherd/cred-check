import { Opinion as OpinionModel } from "../api.ts";
import { tokenContext } from "../context/token.tsx";
import { React } from "../deps.ts";
import Opinion from "./opinion.tsx";

interface Props {
  opinions: OpinionModel[];
  onOpinion: (body: string) => void;
  onSupporter: (opinionId: number) => void;
}

export default function Opinions({ opinions, onOpinion, onSupporter }: Props) {
  const token = React.useContext(tokenContext);
  const [body, setBody] = React.useState("");

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    onOpinion(body);
  };

  const handleSupporter = (opinionId: number) => () => onSupporter(opinionId);

  return (
    <>
      <ul>
        {opinions.map((opinion) => (
          <li key={opinion.id}>
            <Opinion
              onSupporter={handleSupporter(opinion.id)}
              opinion={opinion}
            />
          </li>
        ))}
      </ul>

      {token && (
        <form onSubmit={handleSubmit}>
          <label>
            Body{" "}
            <textarea
              name="body"
              value={body}
              onChange={(e) => setBody(e.target.value)}
            />
          </label>{" "}
          <button type="submit">Submit</button>
        </form>
      )}
    </>
  );
}
