import { createFileRoute } from "@tanstack/react-router";
import { useEffect } from "react";
import { ships, fuels, isCompatible } from "../../consts";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/jump")({
  component: JumpCapacityCalculator,
});

function JumpCapacityCalculator() {
  return (
      <section>
        <h2>How far can I jump?</h2>
        <Calculator />
        <hr />
        <h2>Summary</h2>
        <p>
          These numbers are for empty ships with no fittings and no cargo. To
          get an accurate jump distance, you need to know the total mass, which
          can be found with right-click &rarr; Show Info.
        </p>
        <SummaryTable />
      </section>
  );
}

function jumpRange(mass: number, fuel: number, efficiency: number): number {
  return parseInt(((fuel / mass) * efficiency * 1e7).toFixed(0));
}

const sorted_ships = Object.entries(ships);
sorted_ships.sort((a, b) => a[1].mass - b[1].mass);

function SummaryTable() {
  return (
    <table className="jumpSummary">
      <thead>
        <tr>
          <th>Ship</th>
          {Object.entries(fuels)
            .filter(([fuelName, _]) => fuelName !== "EU-40")
            .map(([fuelType]) => (
              <th key={fuelType}>{fuelType}</th>
            ))}
        </tr>
      </thead>
      <tbody>
        {sorted_ships
          .map(([shipName, ship]) => (
            <tr key={shipName}>
              <th>{shipName}</th>
              {Object.entries(fuels)
                .filter(([fuelName, _]) => fuelName !== "EU-40")
                .map(([fuelName, efficiency]) => (
                  <td key={fuelName}>
                    {isCompatible(
                      fuelName as keyof typeof fuels,
                      ship.fuel_type,
                    )
                      ? jumpRange(ships[shipName].mass, ships[shipName].fuel, efficiency)
                      : "-"}
                  </td>
                ))}
            </tr>
          ))}
      </tbody>
    </table>
  );
}

function Calculator() {
  const [ship, setShip] = useSessionStorage<keyof typeof ships>("ship", "Val");
  const [fuelType, setFuelType] = useSessionStorage<keyof typeof fuels>(
    "fuelType",
    "SOF-40",
  );

  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [fuel, setFuel] = useSessionStorage<number>("fuel", 539);
  const [efficiency, setEfficiency] = useSessionStorage<number>(
    "efficiency",
    0.4,
  );

  const [_, setSavedJump] = useSessionStorage<number>("jump", 0);

  useEffect(() => {
    const shipData = ships[ship];
    setMass(shipData.mass);
    setFuel(shipData.fuel);
    setFuelType(shipData.fuel_type);
  }, [ship]);

  useEffect(() => {
    setEfficiency(fuels[fuelType]);
  }, [fuelType]);

  useEffect(() => {
    setSavedJump(jumpRange(mass, fuel, efficiency));
  }, [mass, fuel, efficiency]);

  return (
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
          <td></td>
          <td>{jumpRange(mass, fuel, efficiency)} ly</td>
        </tr>
      </tbody>
    </table>
  );
}
