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

export function isCompatible(fuel1: FuelName, fuel2: FuelName) {
  const fuel1_is_basic = fuel1 === "D1" || fuel1 === "D2";
  const fuel2_is_basic = fuel2 === "D1" || fuel2 === "D2";
  return fuel1_is_basic === fuel2_is_basic;
}
