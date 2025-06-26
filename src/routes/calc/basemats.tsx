import { createFileRoute } from "@tanstack/react-router";
import { useState, useEffect } from "react";
import { useSessionStorage } from "usehooks-ts";
import { items, posboms, BaseBom, ItemName, StructureName } from "../../consts";

export const Route = createFileRoute("/calc/basemats")({
  component: CargoCalculator,
});

function CargoCalculator() {
  const [baseBom, setBaseBom] = useSessionStorage<BaseBom>("baseBom", {});
  const [itemsBom, setItemsBom] = useState<Record<string, number>>({});
  const [cargoMass, setCargoMass] = useSessionStorage<number>("cargoMass", 0);
  const [cargoVolume, setCargoVolume] = useSessionStorage<number>(
    "cargoVolume",
    0,
  );

  useEffect(() => {
    const myItemsBom: { [key: string]: number } = {};
    for (const [pos, posCount] of Object.entries(baseBom) as [
      StructureName,
      number,
    ][]) {
      for (const [item, itemCount] of Object.entries(posboms[pos])) {
        if (!myItemsBom[item]) myItemsBom[item] = 0;
        myItemsBom[item] += itemCount * posCount;
      }
    }
    setItemsBom(myItemsBom);
  }, [baseBom]);

  useEffect(() => {
    let mass = 0;
    let volume = 0;
    for (const [item, itemCount] of Object.entries(itemsBom) as [
      ItemName,
      number,
    ][]) {
      if (!items[item]) continue;
      mass += items[item].mass * itemCount;
      volume += items[item].volume * itemCount;
    }
    setCargoMass(mass);
    setCargoVolume(volume);
  }, [itemsBom, setCargoMass, setCargoVolume]);

  return (
    <section>
      <h2>How much does this stuff weigh?</h2>
      <table className="form">
        <tbody>
          {(Object.keys(posboms) as StructureName[]).map((posName) => (
            <tr key={posName}>
              <th>{posName}</th>
              <td>
                <input
                  type="number"
                  value={baseBom[posName] || 0}
                  onChange={(e) => {
                    const newBaseBom = { ...baseBom };
                    newBaseBom[posName] = e.target.valueAsNumber;
                    setBaseBom(newBaseBom);
                  }}
                />
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <p>Materials:</p>
      <table className="form">
        <tbody>
          {Object.entries(itemsBom)
            .filter(([_, count]) => count > 0)
            .map(([itemName, count]) => (
              <tr key={itemName}>
                <th>{itemName}</th>
                <td>{count}</td>
              </tr>
            ))}
        </tbody>
      </table>
      <p>Total Mass: {cargoMass} kg</p>
      <p>Total Volume: {cargoVolume} m³</p>
      <table className="form">
        <tbody></tbody>
      </table>
    </section>
  );
}
