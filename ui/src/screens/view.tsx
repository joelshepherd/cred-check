import * as api from "../api.ts";
import Opinions from "../components/opinions.tsx";
import Search from "../components/search.tsx";
import Source2 from "../components/source.tsx";
import { None, Option, React, Some } from "../deps.ts";
import { push } from "../history.ts";

interface Props {
  url: Option<string>;
}

export default function View(props: Props) {
  const [state, setState] = React.useState<Option<api.FindSource>>(None);

  React.useEffect(() => {
    props.url.match({
      some: (url) =>
        api.findSource(url).then((result) => setState(result.ok())),
      none: () => setState(None),
    });
  }, [props.url]);

  const handleSearch = (query: string) => push(`/view/${query}`);

  // Should only be able to be called if URL is set
  const handleCreateSource = () =>
    api
      .createSource(props.url.unwrap())
      .then((result) => setState(result.ok()));

  const handleOpinion = (position: boolean) => (body: string) =>
    state.match({
      some: (state) =>
        api
          .addOpinion({
            body,
            position,
            source_id: state.source.id,
          })
          .then((result) =>
            result.match({
              ok: (opinion) =>
                setState(
                  Some({
                    ...state,
                    opinions: state.opinions.concat(opinion),
                  })
                ),
              err: () => {},
            })
          ),
      none: () => {},
    });

  const handleSupporter = (opinionId: number) => {
    api.addSupporter({ opinion_id: opinionId }).then((result) => {
      if (result.isOk()) {
        state.match({
          some: (state) => {
            const opinion = state.opinions.find(
              (opinion) => opinion.id === opinionId
            );

            if (opinion) {
              setState(
                Some({
                  ...state,
                  votes: [
                    state.votes[0] + (opinion.position ? 1 : 0),
                    state.votes[1] + (opinion.position ? 0 : 1),
                  ],
                })
              );
            }
          },
          none: () => {},
        });
      }
    });
  };

  return (
    <div>
      <h1>View</h1>
      <Search initialState={props.url} onSearch={handleSearch} />
      {state.match({
        some: (state) => (
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
        ),
        none: () =>
          props.url.match({
            some: () => (
              <>
                <h2>Not found</h2>
                <button onClick={handleCreateSource}>Add it</button>
              </>
            ),
            none: null,
          }),
      })}
    </div>
  );
}
