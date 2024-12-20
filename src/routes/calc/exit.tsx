import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { form_api } from "../../api";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/exit")({
  component: ExitFinder,
});

type Exit = [string, string, number];

function ExitFinder() {
  const [start, setStart] = useSessionStorage<string>("start", "E.G1G.6GD");
  const [jump, setJump] = useSessionStorage<number>("jump", 80);

  const [exits, setExits] = useState<null | Exit[]>(null);
  const [error, setError] = useState<null | Error>(null);

  useEffect(() => {
    setExits(null);
    setError(null);
  }, [start, jump]);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(e.target as HTMLFormElement, 1, setExits, setError);
  }

  return (
    <section>
      <h2>How do I get out of here?</h2>
      <p>Show the places you can jump to from a given constellation</p>
      <form action="/api/exit" method="get" onSubmit={submit}>
        <table>
          <tbody>
            <tr>
              <th>Solar System</th>
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
              <th>Jump distance (ly)</th>
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
              <td>
                <input type="submit" value="Calculate" />
              </td>
              <td>
                {exits && (
                  <ul>
                    {exits.map((exit) => (
                      <li key={exit[0]}>
                        {exit[0]} &rarr; {exit[1]} ({exit[2].toFixed(2)} ly)
                      </li>
                    ))}
                  </ul>
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
