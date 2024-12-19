import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent } from "react";
import { api } from "../../api";

export const Route = createFileRoute("/calc/dist")({
  component: DistanceBetweenSystems,
});

function DistanceBetweenSystems() {
  const [start, setStart] = useState("E.G1G.6GD");
  const [end, setEnd] = useState("Nod");

  const [dist, setDist] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    api(1, e.target as HTMLFormElement, setDist, setError);
  }

  return (
    <section>
      <h2>How far is it?</h2>
      <form action="/api/dist" method="get" onSubmit={submit}>
        <table>
          <tbody>
            <tr>
              <td>System 1</td>
              <td>
                <input
                  name="start"
                  type="text"
                  required={true}
                  value={start}
                  onChange={(e) => setStart(e.target.value)}
                />
              </td>
            </tr>
            <tr>
              <td>System 2</td>
              <td>
                <input
                  name="end"
                  type="text"
                  required={true}
                  value={end}
                  onChange={(e) => setEnd(e.target.value)}
                />
              </td>
            </tr>
            <tr>
              <td>
                <input type="submit" value="Calculate" />
              </td>
              <td>
                {dist && `${dist.toFixed(2)} ly`}
                {error && error.message}
              </td>
            </tr>
          </tbody>
        </table>
      </form>
    </section>
  );
}
