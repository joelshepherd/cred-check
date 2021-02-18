import { Opinion as OpinionModel } from "../api.ts";
import { sessionContext } from "../context/session.tsx";
import { React } from "../deps.ts";
import Opinion from "./opinion.tsx";
import OpinionForm, { State } from "./opinionForm.tsx";

interface Props {
  opinions: OpinionModel[];
  onOpinion: (body: string) => void;
  onVote: (opinionId: number) => void;
}

export default function Opinions({
  opinions,
  onOpinion,
  onVote,
}: Props): React.ReactElement {
  const session = React.useContext(sessionContext);

  const handleVote = (opinionId: number) => () => onVote(opinionId);
  const handleSubmit = (state: State) => onOpinion(state.body);

  return (
    <>
      <ul>
        {opinions.map((opinion) => (
          <li key={opinion.id}>
            <Opinion onVote={handleVote(opinion.id)} opinion={opinion} />
          </li>
        ))}
      </ul>

      {session.authenticated && <OpinionForm onSubmit={handleSubmit} />}
    </>
  );
}
