import { useEffect, useState } from "react";
import { useSessionStorage } from "usehooks-ts";
import { getEngine, ShipName, ships } from "../consts";
import { FuelName, fuels, isCompatible } from "../consts/fuels";

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
  const [fuelType, setFuelType] = useState<FuelName>("SOF-40");

  useEffect(() => {
    const shipData = ships[ship];
    const engData = getEngine(shipData.type);
    setFuelType(engData.fuel);
    onMassChange(shipData.mass + engData.mass);
    onTankChange(shipData.tank);
  }, [ship]);

  useEffect(() => {
    onEfficiencyChange(fuels[fuelType]);
  }, [fuelType]);

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
        value={fuelType}
        onChange={(e) => {
          setFuelType(e.target.value as FuelName);
        }}
      >
        {Object.keys(fuels)
          .filter((name) =>
            isCompatible(getEngine(ships[ship].type).fuel, name as FuelName),
          )
          .map((name) => (
            <option key={name} value={name}>
              {name}
            </option>
          ))}
      </select>
    </div>
  );
}
