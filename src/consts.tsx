export type Ship = {
  mass: number;
  fuel: number;
  fuel_type: string;
};
export type Ships = {
  [key: string]: Ship;
};
export const ships: Ships = {
  Juav: { mass: 14000000, fuel: 182, fuel_type: "uSOF-30" },
  Val: { mass: 28000000, fuel: 539, fuel_type: "SOF-40" },
  Flegel: { mass: 145000000, fuel: 2990, fuel_type: "SOF-40" },
};

export const fuels = {
  "uSOF-30": 0.3,
  "SOF-40": 0.4,
  "EU-40": 0.4,
  "SOF-80": 0.8,
  "EU-90": 0.9,
};