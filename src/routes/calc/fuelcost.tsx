import { createFileRoute, Link } from "@tanstack/react-router";
import { useContext } from "react";
import { ships, getEngine, Ship, ShipName } from "../../consts";
import { FuelName, Fuel, fuels, isCompatible } from "../../consts/fuels";
import { SettingsContext } from "../../providers/settings";

export const Route = createFileRoute("/calc/fuelcost")({
  component: FuelCostCalculator,
});

function FuelCostCalculator() {
  return (
    <section>
      <h2>Cost for One Full Tank</h2>
      <p>
        Based on prices from the <Link to="/calc/settings">Settings</Link> page
      </p>
      <SummaryTable />
    </section>
  );
}

function SummaryTable() {
  const sorted_ships = Object.entries(ships) as [ShipName, Ship][];
  sorted_ships.sort((a, b) => a[1].mass - b[1].mass);
  const dfuels = Object.entries(fuels) as [FuelName, Fuel][];

  return (
    <>
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
              {dfuels.map(([fuelName, _]) => (
                <SummaryCell key={fuelName} fuelName={fuelName} ship={ship} />
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </>
  );
}

function SummaryCell({ ship, fuelName }: { ship: Ship; fuelName: FuelName }) {
  const { fuelCosts } = useContext(SettingsContext);
  if (!isCompatible(fuelName, getEngine(ship.type).fuel)) {
    return <td>-</td>;
  }
  return <td>{(ship.tank * fuelCosts[fuelName]).toLocaleString()}</td>;
}
