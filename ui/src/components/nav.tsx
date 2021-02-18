import { React } from "../deps.ts";
import Link from "./link.tsx";

export default function Nav(): React.ReactElement {
  return (
    <nav>
      <h1>Fact Checker</h1>
      <ul>
        <li>
          <Link href="/">Search</Link>
        </li>
        <li>
          <Link href="/auth">Auth</Link>
        </li>
      </ul>
    </nav>
  );
}
