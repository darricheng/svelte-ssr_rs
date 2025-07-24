import { render as renderer } from "svelte/server";
import App from "./App.svelte";

export function render(property) {
  const { head, body } = renderer(App, {
    // `property` is a string, so we convert it to an object
    // NOTE: need to not pass this in if in Rust `None` was passed
    // to `ssr.render_to_string(None)`.
    // props: { some: JSON.parse(property) },
  });
  return JSON.stringify({ head, body });
}
