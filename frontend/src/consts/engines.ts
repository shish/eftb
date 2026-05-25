import engines_data from "./engines_data";
import type { FuelType } from "./fuels";
import type { ShipType } from "./ships";

export type EngineName = keyof typeof engines_data;
export type Engine = {
  mass: number;
  fuelType: FuelType;
  canFitShipGroups: readonly ShipType[];
};
export const engines: { [key in EngineName]: Engine } = engines_data;
