import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { form_api } from "../../api";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/path")({
  component: PathFinder,
});

type ConnType = "npc_gate" | "smart_gate" | "jump";
type PathStep = {
  from: {
    name: string;
    id: string;
  };
  conn_type: ConnType;
  distance: number;
  to: {
    name: string;
    id: string;
  };
};

function PathFinder() {
  const [start, setStart] = useSessionStorage<string>("start", "E.G1G.6GD");
  const [end, setEnd] = useSessionStorage<string>("end", "Nod");
  const [jump, setJump] = useSessionStorage<number>("jump", 80);
  const [optimize, setOptimize] = useSessionStorage<"fuel" | "distance">(
    "optimize",
    "fuel",
  );

  const [path, setPath] = useState<null | PathStep[]>(null);
  const [error, setError] = useState<null | Error>(null);

  useEffect(() => {
    setPath(null);
    setError(null);
  }, [start, end, jump, optimize]);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(e.target as HTMLFormElement, 2, setPath, setError);
  }
  function copyFormatted() {
    if (path) {
      const text =
        `${start} → ${end}\n\n` +
        path
          .map(
            (p) =>
              `<a href="showinfo:5//${p.to.id}">${p.to.name}</a> (${p.conn_type}, ${p.distance.toFixed()}ly)`,
          )
          .join("\n");

      navigator.clipboard
        .writeText(text)
        .catch(() => alert("Failed to copy :("));
    }
  }

  return (
    <section>
      <h2>How do I get there?</h2>
      <form action="/api/path" method="get" onSubmit={submit}>
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
              <th>Jump distance (ly)</th>
              <td>
                <input
                  name="jump"
                  type="number"
                  required={true}
                  value={jump}
                  min={1}
                  max={500}
                  onChange={(e) => setJump(parseInt(e.target.value))}
                />
              </td>
            </tr>
            <tr>
              <th>Optimize for</th>
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
                {path && (
                  <>
                    <input
                      type="button"
                      value="Copy with EVE-Links"
                      onClick={copyFormatted}
                    />
                    (If you paste with EVE-Links into an in-game notepad, you
                    get clickable links)
                  </>
                )}
              </td>
              <td>
                {path && (
                  <>
                    <ul>
                      {path.map((p) => (
                        <>
                          <li key={p.from.id}>
                            {p.from.name} &rarr; {p.to.name} ({p.conn_type},{" "}
                            {p.distance.toFixed(2)} ly)
                          </li>
                          {"\n"}
                        </>
                      ))}
                    </ul>
                    {path.length} jumps,{" "}
                    {path.reduce((a, b) => a + b.distance, 0).toFixed(2)} ly
                    travelled,{" "}
                    {path
                      .reduce(
                        (a, b) => a + (b.conn_type == "jump" ? b.distance : 0),
                        0,
                      )
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
