import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent } from "react";
import { ships, fuels } from "../../consts";
import { api } from "../../api";

export const Route = createFileRoute("/calc/jump")({
  component: JumpCapacityCalculator,
});

function JumpCapacityCalculator() {
  const [ship, setShip] = useState("Val");
  const [mass, setMass] = useState(28000000);
  const [fuel, setFuel] = useState(539);
  const [fuelType, setFuelType] = useState("SOF-40");

  const [dist, setDist] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    api(1, e.target as HTMLFormElement, setDist, setError);
  }

  return (
    <section>
      <h2>How far can I jump?</h2>
      <form action="/api/jump" method="get" onSubmit={submit}>
        <table>
          <tbody>
            <tr>
              <th>Ship</th>
              <td>
                <select
                  value={ship}
                  onChange={(e) => {
                    const ship = ships[e.target.value];
                    setShip(e.target.value);
                    setMass(ship.mass);
                    setFuel(ship.fuel);
                    setFuelType(ship.fuel_type);
                  }}
                >
                  {Object.keys(ships).map((ship) => (
                    <option key={ship} value={ship}>
                      {ship}
                    </option>
                  ))}
                </select>
              </td>
              <td>(Just a shortcut to set mass &amp; fuel)</td>
            </tr>
            <tr>
              <th>Mass (kg)</th>
              <td>
                <input
                  name="mass"
                  type="number"
                  min="1"
                  required={true}
                  value={mass}
                  onChange={(e) => setMass(parseInt(e.target.value))}
                />
              </td>
              <td>(Right-click ship &rarr; Show Info)</td>
            </tr>
            <tr>
              <th>Fuel level</th>
              <td>
                <input
                  name="fuel"
                  type="number"
                  min="1"
                  required={true}
                  value={fuel}
                  onChange={(e) => setFuel(parseInt(e.target.value))}
                />
              </td>
              <td>(The number in the orange rectangle)</td>
            </tr>
            <tr>
              <th>Fuel Type</th>
              <td>
                <select name="efficiency">
                  {Object.entries(fuels).map(([name, value]) => (
                    <option
                      key={name}
                      value={value}
                      selected={name == fuelType}
                    >
                      {name}
                    </option>
                  ))}
                </select>
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
