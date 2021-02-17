import { React } from "../deps.ts";
import { push } from "../history.ts";

export default function Nav() {
  const handleClick = (path: string) => () => push(path);

  return (
    <nav>
      <ul>
        <li>
          <a onClick={handleClick("/")}>View</a>
        </li>
        <li>
          <a onClick={handleClick("/auth")}>Auth</a>
        </li>
      </ul>
    </nav>
  );
}
