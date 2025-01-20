export type Ship = {
  mass: number;
  fuel: number;
  fuel_type: string;
};
export type Ships = {
  [key: string]: Ship;
};
export const ships: Ships = {
  Explorer: { mass: 5000000, fuel: 280, fuel_type: "uSOF-30" },
  Forager: { mass: 11000000, fuel: 63, fuel_type: "uSOF-30" },
  Juav: { mass: 14000000, fuel: 182, fuel_type: "uSOF-30" },
  Microptero: { mass: 17000000, fuel: 245, fuel_type: "SOF-40" },
  Val: { mass: 28000000, fuel: 539, fuel_type: "SOF-40" },
  Flegel: { mass: 145000000, fuel: 2990, fuel_type: "SOF-40" },
  Anser: { mass: 285000000, fuel: 7090, fuel_type: "SOF-40" },
  Dremar: { mass: 70250000, fuel: 1100, fuel_type: "SOF-40" },
  Axte: { mass: 807000000, fuel: 220500, fuel_type: "SOF-40" },
  Caruda: { mass: 1400000000, fuel: 38900, fuel_type: "SOF-40" },
  Grus: { mass: 2425000000, fuel: 66200, fuel_type: "SOF-40" },
} as const;

export const fuels = {
  "uSOF-30": 0.3,
  "SOF-40": 0.4,
  "EU-40": 0.4,
  "SOF-80": 0.8,
  "EU-90": 0.9,
};
