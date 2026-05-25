import { useEffect, useState } from "react";
import { useSessionStorage } from "usehooks-ts";
import { getEngine, type ShipName, ships } from "../consts/ships";
import { type FuelName, fuels } from "../consts/fuels";

export function ShipFuelSelect({
  onMassChange,
  onTankChange,
  onEfficiencyChange,
}: {
  onMassChange: (mass: number) => void;
  onTankChange: (tank: number) => void;
  onEfficiencyChange: (efficiency: number) => void;
}) {
  const [ship, setShip] = useSessionStorage<ShipName>("ship", "Wend");
  const [fuelName, setFuelName] = useState<FuelName>("SOF-40");

  useEffect(() => {
    const shipData = ships[ship];
    const engData = getEngine(shipData.type);
    for (const name of Object.keys(fuels) as FuelName[]) {
      if (fuels[name].fuelType === shipData.fuelType) {
        setFuelName(name);
        break;
      }
    }
    onMassChange(shipData.mass + engData.mass);
    onTankChange(shipData.tank);
  }, [ship, onMassChange, onTankChange]);

  useEffect(() => {
    onEfficiencyChange(fuels[fuelName].efficiency);
  }, [fuelName, onEfficiencyChange]);

  return (
    <div className="pair">
      <select
        value={ship}
        onChange={(e) => {
          setShip(e.target.value as ShipName);
        }}
      >
        {Object.keys(ships).map((ship) => (
          <option key={ship} value={ship}>
            {ship}
          </option>
        ))}
      </select>
      <select
        value={fuelName}
        onChange={(e) => {
          setFuelName(e.target.value as FuelName);
        }}
      >
        {Object.keys(fuels)
          .filter((name) => fuels[name as FuelName].fuelType === ships[ship].fuelType)
          .map((name) => (
            <option key={name} value={name}>
              {name}
            </option>
          ))}
      </select>
    </div>
  );
}
