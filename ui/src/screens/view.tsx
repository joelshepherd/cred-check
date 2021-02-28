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
  const [source, setSource] = React.useState<Option<api.SourceReply>>(None);

  React.useEffect(() => {
    props.url.match({
      some: (url) =>
        api.findSource(url).then((result) => setSource(result.ok())),
      none: () => setSource(None),
    });
  }, [props.url]);

  const handleSearch = (query: string) => push(`/search/${query}`);

  // Should only be able to be called if URL is set
  const handleCreateSource = () =>
    api
      .createSource(props.url.unwrap())
      .then((result) => setSource(result.ok()));

  const handleOpinion = (position: boolean) => (body: string) =>
    source.match({
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
                // setSource(
                //   Some({
                //     ...source,
                //     opinions: source.opinions.concat(opinion),
                //   })
                // );
                handleVote(opinion.id);
              },
              err: () => {},
            })
          ),
      none: () => {},
    });

  const handleVote = (opinionId: number) => {
    api.createVote({ opinion_id: opinionId }).then((result) => {
      if (result.isOk()) {
        source.match({
          some: (state) => {
            // const opinion = source.opinions.find(
            //   (opinion) => opinion.id === opinionId
            // );
            // if (opinion) {
            //   setSource(
            //     Some({
            //       ...source,
            //       votes: [
            //         source.votes[0] + (opinion.position ? 1 : 0),
            //         source.votes[1] + (opinion.position ? 0 : 1),
            //       ],
            //     })
            //   );
            // }
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
      {source.match({
        some: (source) => (
          <>
            <Source source={source} />
            <h3>True ({0} votes)</h3>
            {/* <Opinions
              opinions={source.opinions.filter(
                (opinion) => opinion.position === true
              )}
              onOpinion={handleOpinion(true)}
              onVote={handleVote}
            /> */}
            <h3>False ({1} votes)</h3>
            {/* <Opinions
              opinions={source.opinions.filter(
                (opinion) => opinion.position === false
              )}
              onOpinion={handleOpinion(false)}
              onVote={handleVote}
            /> */}
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
