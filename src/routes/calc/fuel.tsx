import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { form_api } from "../../api";
import { ships, fuels, isCompatible } from "../../consts";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/fuel")({
  component: FuelCalculator,
});

function FuelCalculator() {
  const [ship, setShip] = useSessionStorage<keyof typeof ships>("ship", "Val");
  const [fuelType, setFuelType] = useState<keyof typeof fuels>("SOF-40");

  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [dist, setDist] = useSessionStorage<number>("dist", 100);
  const [efficiency, setEfficiency] = useState<number>(fuels[fuelType]);

  const [fuel, setFuel] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  useEffect(() => {
    setFuel(null);
    setError(null);
  }, [ship, mass, dist, efficiency]);

  useEffect(() => {
    const shipData = ships[ship];
    setMass(shipData.mass);
    setFuel(shipData.fuel);
    setFuelType(shipData.fuel_type);
  }, [ship]);

  useEffect(() => {
    setEfficiency(fuels[fuelType]);
  }, [fuelType]);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(e.target as HTMLFormElement, 1, setFuel, setError);
  }

  return (
    <section>
      <h2>How much fuel will I need?</h2>
      <p>How much fuel will it take to jump a given distance</p>
      <form action="/api/fuel" method="get" onSubmit={submit}>
        <table className="form">
          <tbody>
            <tr>
              <th>Ship / Fuel</th>
              <td className="pair">
                <select
                  value={ship}
                  onChange={(e) => {
                    setShip(e.target.value);
                  }}
                >
                  {Object.keys(ships).map((ship) => (
                    <option key={ship} value={ship}>
                      {ship}
                    </option>
                  ))}
                </select>
                <select
                  value={fuelType}
                  onChange={(e) => {
                    setFuelType(e.target.value as keyof typeof fuels);
                  }}
                >
                  {Object.keys(fuels)
                    .filter((name) =>
                      isCompatible(
                        ships[ship].fuel_type,
                        name as keyof typeof fuels,
                      ),
                    )
                    .map((name) => (
                      <option key={name} value={name}>
                        {name}
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
              <th>Fuel multiplier</th>
              <td>
                <input
                  name="efficiency"
                  type="number"
                  required={true}
                  value={efficiency}
                  onChange={(e) => setEfficiency(parseFloat(e.target.value))}
                />
              </td>
              <td>(The number in the fuel type)</td>
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
