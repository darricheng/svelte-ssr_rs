import { render as renderer } from "svelte/server";
import App from "./App.svelte";

export function render(property) {
  const { head, body } = renderer(App, {
    // `property` is a string, so we convert it to an object
    props: { some: JSON.parse(property) },
  });
  return JSON.stringify({ head, body });
}
