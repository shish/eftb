import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent } from "react";
import { api } from "../../api";

export const Route = createFileRoute("/calc/path")({
  component: PathFinder,
});

type ConnType = "npc_gate" | "smart_gate" | "jump";
type PathStep = [string, string, ConnType, number];

function PathFinder() {
  const [start, setStart] = useState("E.G1G.6GD");
  const [end, setEnd] = useState("Nod");
  const [jump, setJump] = useState(120);
  const [optimize, setOptimize] = useState<"fuel" | "distance">("fuel");

  const [path, setPath] = useState<null | PathStep[]>(null);
  const [error, setError] = useState<null | Error>(null);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    api(e.target as HTMLFormElement, setPath, setError);
  }

  return (
    <section>
      <h2>How do I get there?</h2>
      <form action="/api/path" method="get" onSubmit={submit}>
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
              <td>Jump distance (ly)</td>
              <td>
                <input
                  name="jump"
                  type="number"
                  required={true}
                  value={jump}
                  onChange={(e) => setJump(parseInt(e.target.value))}
                />
              </td>
            </tr>
            <tr>
              <td>Optimize for</td>
              <td>
                <select
                  name="optimize"
                  value={optimize}
                  onChange={(e) =>
                    setOptimize(e.target.value as "fuel" | "distance")
                  }
                >
                  <option value="fuel">Fuel (Prefer gates)</option>
                  <option value="distance">Distance (Prefer jumps)</option>
                </select>
              </td>
            </tr>
            <tr>
              <td>
                <input type="submit" value="Calculate" />
              </td>
              <td>
                {path && (
                  <>
                    <ul>
                      {path.map((p) => (
                        <li key={p[0]}>
                          {p[0]} &rarr; {p[1]} ({p[2]}, {p[3].toFixed(2)} ly)
                        </li>
                      ))}
                    </ul>
                    {path.length} jumps,{" "}
                    {path.reduce((a, b) => a + b[3], 0).toFixed(2)} ly
                    travelled,{" "}
                    {path
                      .reduce((a, b) => a + (b[2] == "jump" ? b[3] : 0), 0)
                      .toFixed(2)}{" "}
                    ly jumped
                  </>
                )}
                {error && error.message}
              </td>
            </tr>
          </tbody>
        </table>
      </form>
    </section>
  );
}
