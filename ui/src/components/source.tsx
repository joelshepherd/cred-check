import { Source } from "../api.ts";
import { React } from "../deps.ts";

interface Props {
  source: Source;
}

export default function Source2({ source }: Props) {
  return (
    <>
      <h2>{source.title}</h2>
      <p>
        <a href={source.url}>{source.url}</a>
      </p>
    </>
  );
}
