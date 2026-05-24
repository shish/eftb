import _fuels from "./fuels_data";

export type FuelName = keyof typeof _fuels;
export type Fuel = { efficiency: number, group: string };
export const fuels: { [key in FuelName]: Fuel } = _fuels;

export function isCompatible(fuel1: FuelName, fuel2: FuelName) {
  return fuels[fuel1].group === fuels[fuel2].group;
}
