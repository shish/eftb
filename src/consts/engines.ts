import engines_data from "./engines_data";
import type { FuelName } from "./fuels";

export type EngineName = keyof typeof engines_data;
export type Engine = {
  mass: number;
  fuel: FuelName;
  //types: ShipType[];
}
export type Engines = { [key in EngineName]: Engine };
export const engines: Engines = engines_data;
