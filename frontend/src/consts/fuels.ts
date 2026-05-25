import _fuels from "./fuels_data";

export type FuelType = "Hydrogen" | "Crude";
export type FuelName = keyof typeof _fuels;
export type Fuel = { efficiency: number; fuelType: FuelType };
export const fuels: { [key in FuelName]: Fuel } = _fuels;
