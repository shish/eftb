import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { form_api } from "../../api";
import { ships, fuels } from "../../consts";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/fuel")({
  component: FuelCalculator,
});

function FuelCalculator() {
  const [ship, setShip] = useSessionStorage<string>("ship", "Val");
  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [dist, setDist] = useSessionStorage<number>("dist", 100);
  const [fuelType, setFuelType] = useState<string>("SOF-40");

  const [fuel, setFuel] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  useEffect(() => {
    setFuel(null);
    setError(null);
  }, [ship, mass, dist, fuelType]);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(e.target as HTMLFormElement, 1, setFuel, setError);
  }

  return (
    <section>
      <h2>How much fuel will I need?</h2>
      <p>How much fuel will it take to jump a given distance</p>
      <form action="/api/fuel" method="get" onSubmit={submit}>
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
              <td>(Just a shortcut to set mass &amp; fuel type)</td>
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
            </tr>
            <tr>
              <th>Dist (ly)</th>
              <td>
                <input
                  name="dist"
                  type="number"
                  min="1"
                  required={true}
                  value={dist}
                  onChange={(e) => setDist(parseInt(e.target.value))}
                />
              </td>
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
                {fuel && `${fuel.toFixed(2)} units`}
                {error && error.message}
              </td>
            </tr>
          </tbody>
        </table>
      </form>
    </section>
  );
}
