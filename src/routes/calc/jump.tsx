import { createFileRoute } from "@tanstack/react-router";
import { useContext, useEffect } from "react";
import { useSessionStorage } from "usehooks-ts";
import { ShipFuelSelect } from "../../components/ShipFuelSelect";
import { getEngine, items, Ship, ShipName, ships } from "../../consts";
import { Fuel, FuelName, fuels, isCompatible } from "../../consts/fuels";
import { SettingsContext } from "../../providers/settings";

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
              onChange={(e) => setMass(e.target.valueAsNumber)}
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
              onChange={(e) => setTank(e.target.valueAsNumber)}
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
              onChange={(e) => setEffi(e.target.valueAsNumber)}
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

type SummaryMode = "dist" | "effi" | "ceff";
type FittingsMode = "empty" | "engine" | "custom";
type CargoMode = "empty" | "fuel" | "smartgate" | "custom";

function SummaryTable() {
  const sorted_ships = Object.entries(ships) as [ShipName, Ship][];
  sorted_ships.sort((a, b) => a[1].mass - b[1].mass);
  const dfuels = (Object.entries(fuels) as [FuelName, Fuel][]).filter(
    ([fuelName, _]) => fuelName !== "EU-40",
  );

  const [mode, setMode] = useSessionStorage<SummaryMode>(
    "jumpSummaryMode",
    "dist",
  );

  const [fittingsMode, setFittingsMode] = useSessionStorage<FittingsMode>(
    "jumpSummaryFittingsMode",
    "engine",
  );
  const [fittingsMass, setFittingsMass] = useSessionStorage("fittingsMass", 0);
  useEffect(() => {
    if (fittingsMode === "empty") setFittingsMass(0);
  }, [fittingsMode, setFittingsMass]);

  const [cargoMode, setCargoMode] = useSessionStorage<CargoMode>(
    "jumpSummaryCargoMode",
    "empty",
  );
  const [cargoMass, setCargoMass] = useSessionStorage("cargoMass", 0);
  useEffect(() => {
    if (cargoMode === "empty") setCargoMass(0);
    if (cargoMode === "smartgate") setCargoMass(3055000000);
  }, [cargoMode, setCargoMass]);

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
                <option value="ceff">Travel Cost (lux per ly)</option>
              </select>
            </td>
          </tr>
          <tr>
            <th>Fittings</th>
            <td className={fittingsMode == "custom" ? "pair" : undefined}>
              <select
                value={fittingsMode}
                onChange={(e) =>
                  setFittingsMode(e.target.value as FittingsMode)
                }
              >
                <option value="empty">Empty</option>
                <option value="engine">Engine</option>
                <option value="custom">Custom</option>
              </select>
              <input
                type={fittingsMode == "custom" ? "number" : "hidden"}
                value={fittingsMass}
                onChange={(e) => setFittingsMass(e.target.valueAsNumber)}
              />
            </td>
          </tr>
          <tr>
            <th>Cargo</th>
            <td className={cargoMode == "custom" ? "pair" : undefined}>
              <select
                value={cargoMode}
                onChange={(e) => setCargoMode(e.target.value as CargoMode)}
              >
                <option value="empty">Empty</option>
                <option value="fuel">Full of Fuel</option>
                <option value="smartgate">Materials for one Smart Gate</option>
                <option value="custom">Custom</option>
              </select>
              <input
                type={cargoMode == "custom" ? "number" : "hidden"}
                value={cargoMass}
                onChange={(e) => setCargoMass(e.target.valueAsNumber)}
              />
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
                  fittingsMass={
                    fittingsMode === "engine"
                      ? getEngine(ship.type).mass
                      : fittingsMass
                  }
                  cargoMass={
                    cargoMode === "fuel"
                      ? ship.tank * items["D1 Fuel"].mass
                      : cargoMass
                  }
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
  fittingsMass,
  cargoMass,
}: {
  ship: Ship;
  fuelName: FuelName;
  efficiency: number;
  mode: SummaryMode;
  fittingsMass: number;
  cargoMass: number;
}) {
  const { fuelCosts } = useContext(SettingsContext);
  const totalMass = ship.mass + fittingsMass + cargoMass;
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
    case "ceff":
      return (
        <td>
          {parseInt(
            (
              (ship.tank * fuelCosts[fuelName]) /
              jumpRange(totalMass, ship.tank, efficiency)
            ).toFixed(0),
          ).toLocaleString()}
        </td>
      );
  }
}
