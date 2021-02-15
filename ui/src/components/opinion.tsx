import { Opinion as OpinionModel } from "../api.ts";
import { React } from "../deps.ts";

interface Props {
  opinion: OpinionModel;
  onSupporter: () => void;
}

export default function Opinion({ opinion, onSupporter }: Props) {
  return (
    <p>
      {opinion.body} <button onClick={onSupporter}>Support</button>
    </p>
  );
}
