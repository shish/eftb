import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { ships, fuels } from "../../consts";
import { form_api } from "../../api";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/jump")({
  component: JumpCapacityCalculator,
});

function JumpCapacityCalculator() {
  const [ship, setShip] = useSessionStorage<string>("ship", "Val");
  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [fuel, setFuel] = useSessionStorage<number>("fuel", 539);
  const [fuelType, setFuelType] = useSessionStorage<string>(
    "fuelType",
    "SOF-40",
  );

  const [_, setSavedJump] = useSessionStorage<number>("jump", 0);
  const [jump, setJump] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  useEffect(() => {
    setJump(null);
    setError(null);
  }, [ship, mass, fuel, fuelType]);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(
      e.target as HTMLFormElement,
      1,
      (x: number) => {
        setJump(x);
        setSavedJump(parseFloat(x.toFixed(2)));
      },
      setError,
    );
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
                {jump && `${jump.toFixed(2)} ly`}
                {error && error.message}
              </td>
            </tr>
          </tbody>
        </table>
      </form>
    </section>
  );
}
