export type FuelName = "D1" | "D2" | "EU-40" | "SOF-40" | "SOF-80" | "EU-90";
export type Fuel = number;
export const fuels: { [key in FuelName]: Fuel } = {
  D1: 0.1,
  D2: 0.15,
  "EU-40": 0.4,
  "SOF-40": 0.4,
  "SOF-80": 0.8,
  "EU-90": 0.9,
};
const groups = {
  "EU-90": "Crude Fuel",
  "SOF-80": "Crude Fuel",
  "EU-40": "Crude Fuel",
  "SOF-40": "Crude Fuel",
  D2: "Hydrogen Fuel",
  D1: "Hydrogen Fuel",
};

export function isCompatible(fuel1: FuelName, fuel2: FuelName) {
  return groups[fuel1] === groups[fuel2];
}
