import { createFileRoute } from "@tanstack/react-router";
import { FormEvent, useEffect, useState } from "react";
import { useSessionStorage } from "usehooks-ts";
import { form_api } from "../../api";
import { SystemInput } from "../../components/SystemInput";

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
        <table className="form">
          <tbody>
            <tr>
              <th>System 1</th>
              <td>
                <SystemInput
                  name="start"
                  value={start}
                  onChange={(s) => setStart(s)}
                />
              </td>
            </tr>
            <tr>
              <th>System 2</th>
              <td>
                <SystemInput
                  name="end"
                  value={end}
                  onChange={(s) => setEnd(s)}
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
