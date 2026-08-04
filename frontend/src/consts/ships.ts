import type { Engine, EngineName } from "./engines";
import { engines } from "./engines";
import type { FuelType } from "./fuels";
import _ships from "./ships_data";

export type ShipType =
  | "Shuttle"
  | "Corvette"
  | "Destroyer"
  | "Frigate"
  | "Cruiser"
  | "Combat Battlecruiser"
  | "Battleship";

export type ShipName = keyof typeof _ships;
export type Ship = {
  mass: number;
  tank: number;
  type: ShipType;
  heat: number;
  fuelType: FuelType;
  cargo: number;
};
export type Ships = { [key in ShipName]: Ship };

export const ships: Ships = _ships;

export function getEngine(type: ShipType): Engine {
  for (const engineNameS of Object.keys(engines)) {
    const engineName = engineNameS as EngineName;
    if (engines[engineName].canFitShipGroups.includes(type)) {
      return engines[engineName];
    }
  }
  throw new Error(`No engine found for type ${type}`);
}
