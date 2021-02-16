import { Opinion as OpinionModel } from "../api.ts";
import { tokenContext } from "../context/token.tsx";
import { React } from "../deps.ts";

interface Props {
  opinion: OpinionModel;
  onVote: () => void;
}

export default function Opinion({ opinion, onVote }: Props) {
  const token = React.useContext(tokenContext);

  return (
    <p>
      {opinion.body}
      {token.isSome() && <button onClick={onVote}>Support</button>}
    </p>
  );
}
