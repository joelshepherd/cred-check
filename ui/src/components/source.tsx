import { Source as SourceModel } from "../api.ts";
import { React } from "../deps.ts";

interface Props {
  source: SourceModel;
}

export default function Source({ source }: Props): React.ReactElement {
  return (
    <>
      <h2>{source.title}</h2>
      <p>
        <a href={source.url}>{source.url}</a>
      </p>
    </>
  );
}
