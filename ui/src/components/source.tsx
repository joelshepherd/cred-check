import { SourceReply } from "../api.ts";
import { React } from "../deps.ts";

interface Props {
  source: SourceReply;
}

export default function Source({ source }: Props): React.ReactElement {
  return (
    <>
      <h2>{source.title}</h2>
      <p>
        <a href={source.canonical_url}>{source.canonical_url}</a>
      </p>
    </>
  );
}
