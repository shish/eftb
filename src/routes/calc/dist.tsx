import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent } from "react";
import { api } from "../../api";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/dist")({
  component: DistanceBetweenSystems,
});

function DistanceBetweenSystems() {
  const [start, setStart] = useSessionStorage<string>("start", "E.G1G.6GD");
  const [end, setEnd] = useSessionStorage<string>("end", "Nod");

  const [_, setSavedDist] = useSessionStorage<null | number>("dist", null);
  const [dist, setDist] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    api(
      1,
      e.target as HTMLFormElement,
      (d: number) => {
        setDist(d);
        setSavedDist(parseFloat(d.toFixed(2)));
      },
      setError,
    );
  }
  function reset() {
    setDist(null);
    setError(null);
  }

  return (
    <section>
      <h2>How far is it?</h2>
      <form action="/api/dist" method="get" onSubmit={submit}>
        <table>
          <tbody>
            <tr>
              <th>System 1</th>
              <td>
                <input
                  name="start"
                  type="text"
                  required={true}
                  value={start}
                  onChange={(e) => {
                    reset();
                    setStart(e.target.value);
                  }}
                />
              </td>
            </tr>
            <tr>
              <th>System 2</th>
              <td>
                <input
                  name="end"
                  type="text"
                  required={true}
                  value={end}
                  onChange={(e) => {
                    reset();
                    setEnd(e.target.value);
                  }}
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
