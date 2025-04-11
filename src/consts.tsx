export type Ship = {
  mass: number;
  fuel: number;
  fuel_type: string;
};
export type Ships = {
  [key: string]: Ship;
};
export const ships: Ships = {
  // Confirmed in cycle 3
  Explorer: { mass: 4_500_000, fuel: 230, fuel_type: "uSOF-20" },
  Forager: { mass: 8_000_000, fuel: 120, fuel_type: "uSOF-20" },
  Juav: { mass: 14_000_000, fuel: 360, fuel_type: "uSOF-20" },
  // Unconfirmed
  Microptero: { mass: 17_000_000, fuel: 245, fuel_type: "SOF-40" },
  Val: { mass: 28_000_000, fuel: 539, fuel_type: "SOF-40" },
  Flegel: { mass: 145_000_000, fuel: 2990, fuel_type: "SOF-40" },
  Anser: { mass: 285_000_000, fuel: 7090, fuel_type: "SOF-40" },
} as const;

export const fuels = {
  "uSOF-20": 0.2,
  "SOF-40": 0.4,
  "EU-40": 0.4,
  "SOF-80": 0.8,
  "EU-90": 0.9,
};
