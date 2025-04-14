import { useState, useEffect } from "react";
import { ships, fuels, isCompatible, getEngine } from "../consts";
import { useSessionStorage } from "usehooks-ts";

export function ShipFuelSelect({
    onMassChange,
    onTankChange,
    onEfficiencyChange,
}: {
    onMassChange: (mass: number) => void;
    onTankChange: (tank: number) => void;
    onEfficiencyChange: (efficiency: number) => void;
}) {
  const [ship, setShip] = useSessionStorage<keyof typeof ships>("ship", "Val");
  const [fuelType, setFuelType] = useState<keyof typeof fuels>("SOF-40");

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
            isCompatible(getEngine(ships[ship].type).fuel, name as keyof typeof fuels),
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
