import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { form_api } from "../../api";
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

  useEffect(() => {
    setDist(null);
    setError(null);
  }, [start, end]);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(
      e.target as HTMLFormElement,
      1,
      (d: number) => {
        setDist(d);
        setSavedDist(parseFloat(d.toFixed(2)));
      },
      setError,
    );
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
                  onChange={(e) => setStart(e.target.value)}
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
