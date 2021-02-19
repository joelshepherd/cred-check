import { React } from "../deps.ts";
import { push } from "../history.ts";

interface Props extends React.HTMLAttributes<HTMLAnchorElement> {
  href: string;
  children: React.ReactChild;
}

export default function Link({
  children,
  ...props
}: Props): React.ReactElement {
  const handleClick = React.useCallback(
    (e: React.MouseEvent<HTMLAnchorElement>) => {
      e.preventDefault();
      push(props.href);
    },
    [props.href]
  );

  return (
    <a {...props} onClick={handleClick}>
      {children}
    </a>
  );
}
