// Currently incompatible with deno bundle
export { default as React } from "https://cdn.skypack.dev/react";
export { default as ReactDOM } from "https://cdn.skypack.dev/react-dom";

export type { Result } from "https://deno.land/x/monads/lib/result/result.deno.ts";
export { Err, Ok } from "https://deno.land/x/monads/lib/result/result.deno.ts";

export type { Option } from "https://deno.land/x/monads/lib/option/option.deno.ts";
export {
  None,
  Some,
} from "https://deno.land/x/monads/lib/option/option.deno.ts";
