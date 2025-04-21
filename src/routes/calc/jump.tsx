import { createFileRoute } from "@tanstack/react-router";
import { useEffect, useState } from "react";
import {
  ships,
  fuels,
  isCompatible,
  getEngine,
  Ship,
  ShipName,
  FuelName,
  Fuel,
} from "../../consts";
import { useSessionStorage } from "usehooks-ts";
import { ShipFuelSelect } from "../../components/shipfuel";

export const Route = createFileRoute("/calc/jump")({
  component: JumpCapacityCalculator,
});

function jumpRange(mass: number, tank: number, efficiency: number): number {
  return parseInt(((tank / mass) * efficiency * 1e7).toFixed(0));
}

function JumpCapacityCalculator() {
  return (
    <section>
      <h2>How far can I jump?</h2>
      <Calculator />
      <hr />
      <h2>Summary</h2>
      <p>
        These numbers are for empty ships with only an engine and no cargo. To
        get an accurate jump distance, you need to know the total mass, which
        can be found with right-click &rarr; Show Info.
      </p>
      <SummaryTable />
    </section>
  );
}

function Calculator() {
  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [tank, setTank] = useSessionStorage<number>("tank", 539);
  const [effi, setEffi] = useSessionStorage<number>("efficiency", 0.4);

  const [jump, setJump] = useSessionStorage<number>("jump", 0);

  useEffect(() => {
    setJump(jumpRange(mass, tank, effi));
  }, [setJump, mass, tank, effi]);

  return (
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
              value={tank}
              onChange={(e) => setTank(parseInt(e.target.value))}
            />
          </td>
          <td>(The number in the orange rectangle)</td>
        </tr>
        <tr>
          <th>Fuel multiplier</th>
          <td>
            <input
              name="efficiency"
              type="number"
              required={true}
              value={effi}
              onChange={(e) => setEffi(parseFloat(e.target.value))}
            />
          </td>
          <td>(The number in the fuel type)</td>
        </tr>
        <tr>
          <th>Jump range</th>
          <td>{jump} ly</td>
        </tr>
      </tbody>
    </table>
  );
}

type SummaryMode = "dist" | "effi";

function SummaryTable() {
  const sorted_ships = Object.entries(ships) as [ShipName, Ship][];
  sorted_ships.sort((a, b) => a[1].mass - b[1].mass);
  const dfuels = (Object.entries(fuels) as [FuelName, Fuel][]).filter(
    ([fuelName, _]) => fuelName !== "EU-40",
  );

  const [mode, setMode] = useState<SummaryMode>("dist");

  return (
    <>
      <table className="form">
        <tbody>
          <tr>
            <th>Mode</th>
            <td>
              <select
                value={mode}
                onChange={(e) => setMode(e.target.value as SummaryMode)}
              >
                <option value="dist">Distance (ly)</option>
                <option value="effi">Fuel Efficiency (ly per fuel unit)</option>
              </select>
            </td>
          </tr>
        </tbody>
      </table>

      <p></p>

      <table className="jumpSummary">
        <thead>
          <tr>
            <th>Ship</th>
            {dfuels.map(([fuelType]) => (
              <th key={fuelType}>{fuelType}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {sorted_ships.map(([shipName, ship]) => (
            <tr key={shipName}>
              <th>{shipName}</th>
              {dfuels.map(([fuelName, efficiency]) => (
                <SummaryCell
                  key={fuelName}
                  fuelName={fuelName}
                  efficiency={efficiency}
                  ship={ship}
                  mode={mode}
                />
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}

function SummaryCell({
  ship,
  fuelName,
  efficiency,
  mode,
}: {
  ship: Ship;
  fuelName: FuelName;
  efficiency: number;
  mode: SummaryMode;
}) {
  const totalMass = ship.mass + getEngine(ship.type).mass;
  if (!isCompatible(fuelName, getEngine(ship.type).fuel)) {
    return <td>-</td>;
  }
  switch (mode) {
    case "dist":
      return <td>{jumpRange(totalMass, ship.tank, efficiency)}</td>;
    case "effi":
      return (
        <td>
          {(jumpRange(totalMass, ship.tank, efficiency) / ship.tank).toFixed(3)}
        </td>
      );
  }
}
