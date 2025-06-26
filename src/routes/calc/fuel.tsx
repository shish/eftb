import { createFileRoute } from "@tanstack/react-router";
import { useState, useEffect } from "react";
import { useSessionStorage } from "usehooks-ts";
import { ShipFuelSelect } from "../../components/ShipFuelSelect";

export const Route = createFileRoute("/calc/fuel")({
  component: FuelCalculator,
});

function FuelCalculator() {
  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [dist, setDist] = useSessionStorage<number>("dist", 100);
  const [effi, setEffi] = useState<number>(0.4);

  const [tank, setTank] = useState<number>(0);

  useEffect(() => {
    setTank(parseInt(((dist / (effi * 1e7)) * mass).toFixed(0)));
  }, [mass, dist, effi]);

  return (
    <section>
      <h2>How much fuel will I need?</h2>
      <p>How much fuel will it take to jump a given distance</p>
      <table className="form">
        <tbody>
          <tr>
            <th>Ship / Fuel</th>
            <td>
              <ShipFuelSelect
                onMassChange={setMass}
                onTankChange={setTank}
                onEfficiencyChange={setEffi}
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
                onChange={(e) => setMass(e.target.valueAsNumber)}
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
                onChange={(e) => setDist(e.target.valueAsNumber)}
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
                value={effi}
                onChange={(e) => setEffi(e.target.valueAsNumber)}
              />
            </td>
            <td>(The number in the fuel type)</td>
          </tr>
          <tr>
            <th>Fuel needed</th>
            <td>{tank.toLocaleString()} units</td>
          </tr>
        </tbody>
      </table>
    </section>
  );
}
