import { createLazyFileRoute } from "@tanstack/react-router";

export const Route = createLazyFileRoute("/about")({
  component: About,
});

function About() {
  return (
    <section>
      <h2>About</h2>
      <p>
        <a href="https://ko-fi.com/shish2k">Buy me a coffee</a>?
      </p>
    </section>
  );
}
