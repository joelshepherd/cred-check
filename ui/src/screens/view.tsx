import {
  addOpinion,
  addSupporter,
  findSource,
  SourceResponse,
} from "../api.ts";
import Opinions from "../components/opinions.tsx";
import Search from "../components/search.tsx";
import Source2 from "../components/source.tsx";
import { React } from "../deps.ts";
import { push } from "../history.ts";

interface Props {
  url?: string;
}

export default function View(props: Props) {
  const [state, setState] = React.useState<SourceResponse | null>(null);

  const handleSearch = (query: string) => {
    push(`/view/${query}`);
  };

  const handleOpinion = (position: boolean) => (body: string) => {
    if (state)
      addOpinion({
        body,
        position,
        source_id: state.source.id,
      }).then((opinion) =>
        setState({
          ...state,
          opinions: state.opinions.concat(opinion),
        })
      );
  };

  const handleSupporter = (opinionId: number) => {
    addSupporter({ opinion_id: opinionId }).then(() => {
      const opinion = state?.opinions.find(
        (opinion) => opinion.id === opinionId
      );
      if (state && opinion) {
        setState({
          ...state,
          votes: [
            state.votes[0] + (opinion.position ? 1 : 0),
            state.votes[1] + (opinion.position ? 0 : 1),
          ],
        });
      }
    });
  };

  React.useEffect(() => {
    if (props.url) findSource(props.url).then(setState);
  }, [props.url]);

  return (
    <div>
      <h1>View</h1>
      <Search initialState={props.url} onSearch={handleSearch} />
      {state && (
        <>
          <Source2 source={state.source} />
          <h3>True ({state.votes[0]} votes)</h3>
          <Opinions
            opinions={state.opinions.filter(
              (opinion) => opinion.position === true
            )}
            onOpinion={handleOpinion(true)}
            onSupporter={handleSupporter}
          />
          <h3>False ({state.votes[1]} votes)</h3>
          <Opinions
            opinions={state.opinions.filter(
              (opinion) => opinion.position === false
            )}
            onOpinion={handleOpinion(false)}
            onSupporter={handleSupporter}
          />
        </>
      )}
    </div>
  );
}
