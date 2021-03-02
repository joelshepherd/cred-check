import * as api from "../api.ts";
import Opinions from "../components/opinions.tsx";
import Search from "../components/search.tsx";
import Source from "../components/source.tsx";
import { None, Option, React, Some } from "../deps.ts";
import { push } from "../history.ts";

interface Props {
  url: Option<string>;
}

export default function View(props: Props): React.ReactElement {
  const [state, setState] = React.useState<Option<api.SourceReply>>(None);
  const [lastVote, setLastVote] = React.useState<Option<boolean>>(None);

  React.useEffect(() => {
    props.url.match({
      some: (url) =>
        api.findSource(url).then((result) => setState(result.ok())),
      none: () => setState(None),
    });
  }, [props.url]);

  const handleSearch = (query: string) => push(`/search/${query}`);

  // Should only be able to be called if URL is set
  const handleCreateSource = () =>
    api
      .createSource(props.url.unwrap())
      .then((result) => setState(result.ok()));

  const handleOpinion = (position: boolean) => (body: string) =>
    state.match({
      some: (state) =>
        api
          .createOpinion({
            body,
            position,
            source_id: state.id,
          })
          .then((result) =>
            result.match({
              ok: (opinion) => {
                setState(
                  Some({
                    ...state,
                    opinions: state.opinions.concat(opinion),
                  })
                );
              },
              err: () => {},
            })
          ),
      none: () => {},
    });

  const handleVote = (opinionId: number) => {
    api.createVote({ opinion_id: opinionId }).then((result) => {
      if (result.isOk()) {
        state.match({
          some: (state) => {
            const opinion = state.opinions.find(
              (opinion) => opinion.id === opinionId
            );
            if (!opinion) return;

            const thisVote = opinion.position;

            setState(
              Some({
                ...state,
                votes: [
                  state.votes[0] +
                    (thisVote ? 1 : 0) -
                    lastVote.match({
                      some: (lastVote) => (lastVote ? 1 : 0),
                      none: 0,
                    }),
                  state.votes[1] +
                    (thisVote ? 0 : 1) -
                    lastVote.match({
                      some: (lastVote) => (lastVote ? 0 : 1),
                      none: 0,
                    }),
                ],
              })
            );

            // TODO: Pull last vote from the server instead
            setLastVote(Some(thisVote));
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
            <Source source={state} />
            <h3>True ({state.votes[0]} votes)</h3>
            <Opinions
              opinions={state.opinions.filter(
                (opinion) => opinion.position === true
              )}
              onOpinion={handleOpinion(true)}
              onVote={handleVote}
            />
            <h3>False ({state.votes[1]} votes)</h3>
            <Opinions
              opinions={state.opinions.filter(
                (opinion) => opinion.position === false
              )}
              onOpinion={handleOpinion(false)}
              onVote={handleVote}
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
