import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { form_api } from "../../api";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/path")({
  component: PathFinder,
});

type PathOptimize = "fuel" | "distance" | "hops";
type ConnType = "npc_gate" | "smart_gate" | "jump";
type PathStep = {
  from: {
    name: string;
    id: string;
    x: number;
    y: number;
    z: number;
  };
  conn_type: ConnType;
  distance: number;
  to: {
    name: string;
    id: string;
    x: number;
    y: number;
    z: number;
  };
};

function CopyFormattedButton(props: { path: PathStep[] }) {
  function copyFormatted() {
    const text =
      `${props.path[0].from.name} → ${props.path[props.path.length - 1].to.name}\n\n` +
      props.path
        .map(
          (p) =>
            `<a href="showinfo:5//${p.to.id}">${p.to.name}</a> (${p.conn_type}, ${p.distance.toFixed()}ly)`,
        )
        .join("\n");

    navigator.clipboard.writeText(text).catch(() => alert("Failed to copy :("));
  }

  return (
    <>
      <input
        type="button"
        value="Copy with EVE-Links"
        onClick={copyFormatted}
      />
      (If you paste with EVE-Links into an in-game notepad, you get clickable
      links)
    </>
  );
}

function TextPath(props: { path: PathStep[] }) {
  return (
    <>
      <ul>
        {props.path.map((p) => (
          <li key={p.from.id}>
            {p.from.name} &rarr; {p.to.name} ({p.conn_type},{" "}
            {p.distance.toFixed(2)} ly)
          </li>
        ))}
      </ul>
      {props.path.length} hops (
      {props.path.filter((c) => c.conn_type == "jump").length} jumps),{" "}
      {props.path.reduce((a, b) => a + b.distance, 0).toFixed(2)} ly travelled (
      {props.path
        .reduce((a, b) => a + (b.conn_type == "jump" ? b.distance : 0), 0)
        .toFixed(2)}{" "}
      ly jumped)
    </>
  );
}

function SvgPath(props: { path: PathStep[] }) {
  function jump_color(conn_type: ConnType): string {
    switch (conn_type) {
      case "npc_gate":
        return "green";
      case "smart_gate":
        return "blue";
      case "jump":
        return "orange";
    }
  }

  let path_bounds = {
    min_x: Infinity,
    min_y: Infinity,
    max_x: -Infinity,
    max_y: -Infinity,
    min_z: Infinity,
    max_z: -Infinity,
  };
  props.path.forEach((p) => {
    path_bounds.min_x = Math.min(path_bounds.min_x, p.from.x, p.to.x);
    path_bounds.min_y = Math.min(path_bounds.min_y, p.from.y, p.to.y);
    path_bounds.max_x = Math.max(path_bounds.max_x, p.from.x, p.to.x);
    path_bounds.max_y = Math.max(path_bounds.max_y, p.from.y, p.to.y);
    path_bounds.min_z = Math.min(path_bounds.min_z, p.from.z, p.to.z);
    path_bounds.max_z = Math.max(path_bounds.max_z, p.from.z, p.to.z);
  });
  const content_width = path_bounds.max_x - path_bounds.min_x;
  const content_height = path_bounds.max_z - path_bounds.min_z;
  const margin = Math.max(content_width, content_height) * 0.1;
  path_bounds = {
    min_x: path_bounds.min_x - margin,
    max_x: path_bounds.max_x + margin,
    min_y: path_bounds.min_y - margin,
    max_y: path_bounds.max_y + margin,
    min_z: path_bounds.min_z - margin,
    max_z: path_bounds.max_z + margin,
  };

  const width = path_bounds.max_x - path_bounds.min_x;
  const height = path_bounds.max_z - path_bounds.min_z;
  const scale = 1 / Math.max(width, height);

  const path2d: PathStep[] = props.path.map((n) => ({
    from: {
      id: n.from.id,
      name: n.from.name,
      x: (n.from.x - path_bounds.min_x) * scale,
      y: 1 - (n.from.z - path_bounds.min_z) * scale,
      z: 0,
    },
    conn_type: n.conn_type,
    distance: n.distance,
    to: {
      id: n.to.id,
      name: n.to.name,
      x: (n.to.x - path_bounds.min_x) * scale,
      y: 1 - (n.to.z - path_bounds.min_z) * scale,
      z: 0,
    },
  }));

  return (
    <>
      <svg
        width="100%"
        viewBox="0 0 1024 1024"
        xmlns="http://www.w3.org/2000/svg"
      >
        <rect x="0" y="0" width="1024" height="1024" fill="black" />
        {path2d.map((p, i) => {
          const x1 = p.from.x * 1024;
          const y1 = p.from.y * 1024;
          const x2 = p.to.x * 1024;
          const y2 = p.to.y * 1024;
          return (
            <g key={i}>
              <line
                x1={x1}
                y1={y1}
                x2={x2}
                y2={y2}
                stroke={jump_color(p.conn_type)}
                strokeWidth={2}
              />
              <circle cx={x1} cy={y1} r={5} fill="blue" />
              <circle cx={x2} cy={y2} r={5} fill="blue" />
              {i == 0 ? (
                <text x={x1} y={y1} dy={-10} textAnchor="middle" fill="white">
                  {p.from.name}
                </text>
              ) : null}
              <text x={x2} y={y2} dy={-10} textAnchor="middle" fill="white">
                {p.to.name}
              </text>
            </g>
          );
        })}
      </svg>
    </>
  );
}

function PathFinder() {
  const [start, setStart] = useSessionStorage<string>("start", "E.G1G.6GD");
  const [end, setEnd] = useSessionStorage<string>("end", "Nod");
  const [jump, setJump] = useSessionStorage<number>("jump", 80);
  const [optimize, setOptimize] = useSessionStorage<PathOptimize>(
    "optimize",
    "fuel",
  );
  const [useSmartGates, setUseSmartGates] = useSessionStorage<boolean>(
    "useSmartGates",
    true,
  );

  const [path, setPath] = useState<null | PathStep[]>(null);
  const [error, setError] = useState<null | Error>(null);

  /*
  useEffect(() => {
    setPath(null);
    setError(null);
  }, [start, end, jump, optimize]);
*/
  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(e.target as HTMLFormElement, 2, setPath, setError);
  }

  return (
    <section>
      <h2>How do I get there?</h2>
      <form action="/api/path" method="get" onSubmit={submit}>
        <table className="form">
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
                  list="starDataList"
                  autoComplete="off"
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
                  onChange={(e) => setJump(e.target.valueAsNumber)}
                />
              </td>
            </tr>
            <tr>
              <th>Optimize for</th>
              <td>
                <select
                  name="optimize"
                  value={optimize}
                  onChange={(e) => setOptimize(e.target.value as PathOptimize)}
                >
                  <option value="fuel">Fuel (Prefer gates)</option>
                  <option value="distance">Distance (Prefer jumps)</option>
                  <option value="hops">Hops (Minimise clicks)</option>
                </select>
              </td>
            </tr>
            <tr>
              <th>Use smart gates</th>
              <td>
                <input
                  name="use_smart_gates"
                  type="checkbox"
                  checked={useSmartGates}
                  onChange={(e) => setUseSmartGates(e.target.checked)}
                />
              </td>
            </tr>
            <tr>
              <td>
                <input type="submit" value="Calculate" />
                {path && <CopyFormattedButton path={path} />}
              </td>
              <td>
                {path && <TextPath path={path} />}
                {error && error.message}
              </td>
            </tr>
          </tbody>
        </table>
      </form>
      {path && <SvgPath path={path} />}
    </section>
  );
}
