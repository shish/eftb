import { createFileRoute } from "@tanstack/react-router";
import { useState, FormEvent, useEffect } from "react";
import { ships, fuels } from "../../consts";
import { form_api } from "../../api";
import { useSessionStorage } from "usehooks-ts";

export const Route = createFileRoute("/calc/jump")({
  component: JumpCapacityCalculator,
});

function isCompatible(fuel1: keyof typeof fuels, fuel2: keyof typeof fuels) {
    const fuel1_is_basic = fuel1 === "uSOF-20";
    const fuel2_is_basic = fuel2 === "uSOF-20";
    return fuel1_is_basic === fuel2_is_basic;
}

function jumpRange(ship: keyof typeof ships, efficiency: number) {
    const mass = ships[ship].mass;
    const fuel = ships[ship].fuel;
    const jumpRange = (fuel / mass) * efficiency * 1e7;
    return jumpRange.toFixed(2);
}

function SummaryTable() {
    return (
        <table className="jumpSummary">
            <thead>
                <tr>
                    <th>Ship</th>
                    {Object
                        .entries(fuels)
                        .filter(([fuelName, _]) => fuelName !== "EU-40")
                        .map(([fuelType]) => (
                        <th key={fuelType}>{fuelType}</th>
                    ))}
                </tr>
            </thead>
            <tbody>
                {Object
                    .entries(ships)
                    .toSorted((a, b) => a[1].mass - b[1].mass)
                    .map(([shipName, ship]) => (
                    <tr key={shipName}>
                        <th>{shipName}</th>
                        {Object
                            .entries(fuels)
                            .filter(([fuelName, _]) => fuelName !== "EU-40")
                            .map(([fuelName, efficiency]) => (
                            <td key={fuelName}>{
                                isCompatible(fuelName, ship.fuel_type)
                                    ? jumpRange(shipName, efficiency)
                                    : "-"
                            }</td>
                        ))}
                    </tr>
                ))}
            </tbody>
        </table>
    );
}

function JumpCapacityCalculator() {
  const [ship, setShip] = useSessionStorage<keyof typeof ships>("ship", "Val");
  const [fuelType, setFuelType] = useSessionStorage<keyof typeof fuels>("fuelType", "SOF-40");

  const [mass, setMass] = useSessionStorage<number>("mass", 28000000);
  const [fuel, setFuel] = useSessionStorage<number>("fuel", 539);
  const [efficiency, setEfficiency] = useSessionStorage<number>("efficiency", 40);

  const [_, setSavedJump] = useSessionStorage<number>("jump", 0);
  const [jump, setJump] = useState<null | number>(null);
  const [error, setError] = useState<null | Error>(null);

  useEffect(() => {
    setJump(null);
    setError(null);
  }, [ship, mass, fuel, fuelType]);

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
                      setFuelType(e.target.value);
                  }}
                >
                  {Object
                      .entries(fuels)
                      .filter(([name, _]) => isCompatible(ships[ship].fuel_type, name))
                      .map(([name, value]) => (
                    <option key={name} value={value}>
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
                  min="1"
                  required={true}
                  value={efficiency}
                  onChange={(e) => setEfficiency(parseInt(e.target.value))}
                />
              </td>
              <td>(The number in the fuel type)</td>
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
      <hr/>
      <h3>Summary</h3>
      <p>These numbers are for empty ships with no fittings and no cargo.
          To get an accurate jump distance, you need to know the total mass,
          which can be found with right-click &rarr; Show Info.</p>
      <SummaryTable />
    </section>
  );
}
