export const fuels = {
  "uSOF-20": 0.2,
  "SOF-40": 0.4,
  "EU-40": 0.4,
  "SOF-80": 0.8,
  "EU-90": 0.9,
};

export type Ship = {
  mass: number;
  fuel: number;
  fuel_type: keyof typeof fuels;
};
export type Ships = {
  [key: string]: Ship;
};
export const ships: Ships = {
  // Cycle 3 has mass and fuel in show-info \o/
  Anser: { mass: 285_000_000, fuel: 7050, fuel_type: "SOF-40" },
  Axte: { mass: 800_000_000, fuel: 22030, fuel_type: "SOF-40" },
  Baile: { mass: 488_000_000, fuel: 12200, fuel_type: "SOF-40" },
  Caruda: { mass: 1_425_000_000, fuel: 49870, fuel_type: "SOF-40" },
  Drema: { mass: 69_000_000, fuel: 1110, fuel_type: "SOF-40" },
  Explorer: { mass: 4_500_000, fuel: 230, fuel_type: "uSOF-20" },
  Flegel: { mass: 143_000_000, fuel: 2860, fuel_type: "SOF-40" },
  Forager: { mass: 8_000_000, fuel: 120, fuel_type: "uSOF-20" },
  Grus: { mass: 2_383_000_000, fuel: 71340, fuel_type: "SOF-40" },
  Harpia: { mass: 63_000_000, fuel: 1020, fuel_type: "SOF-40" },
  Juav: { mass: 14_000_000, fuel: 360, fuel_type: "uSOF-20" },
  Klinge: { mass: 800_000_000, fuel: 21970, fuel_type: "SOF-40" },
  Microptero: { mass: 20_500_000, fuel: 240, fuel_type: "SOF-40" },
  Pici: { mass: 26_000_000, fuel: 330, fuel_type: "SOF-40" },
  Raubtier: { mass: 46_000_000, fuel: 690, fuel_type: "SOF-40" },
  Rebus_K: { mass: 1_474_000_000, fuel: 41_620, fuel_type: "SOF-40" },
  Samoskyd_1: { mass: 25_000_000, fuel: 300, fuel_type: "SOF-40" },
  Strix: { mass: 96_000_000, fuel: 1550, fuel_type: "SOF-40" },
  Ungher: { mass: 75_000_000, fuel: 1400, fuel_type: "SOF-40" },
  Val: { mass: 28_000_000, fuel: 550, fuel_type: "SOF-40" },
} as const;


export function isCompatible(fuel1: keyof typeof fuels, fuel2: keyof typeof fuels) {
  const fuel1_is_basic = fuel1 === "uSOF-20";
  const fuel2_is_basic = fuel2 === "uSOF-20";
  return fuel1_is_basic === fuel2_is_basic;
}
