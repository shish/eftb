import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { form_api } from "../../api";
import { useSessionStorage } from "usehooks-ts";
import { ShipFuelSelect } from "../../components/shipfuel";

export const Route = createFileRoute("/calc/fuel")({
  component: FuelCalculator,
});

function FuelCalculator() {
  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [dist, setDist] = useSessionStorage<number>("dist", 100);
  const [efficiency, setEfficiency] = useState<number>(0.4);

  const [tank, setTank] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  useEffect(() => {
    setTank(null);
    setError(null);
  }, [mass, dist, efficiency]);

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    form_api(e.target as HTMLFormElement, 1, setTank, setError);
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
              <td>
                <ShipFuelSelect
                  onMassChange={setMass}
                  onTankChange={setTank}
                  onEfficiencyChange={setEfficiency}
                />
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
                {tank && `${tank.toFixed(2)} units`}
                {error && error.message}
              </td>
            </tr>
          </tbody>
        </table>
      </form>
    </section>
  );
}
