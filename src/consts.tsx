export const fuels = {
  "uSOF-20": 0.2,
  "SOF-40": 0.4,
  "EU-40": 0.4,
  "SOF-80": 0.8,
  "EU-90": 0.9,
};

export type Ship = {
  mass: number;
  tank: number;
  fuel: keyof typeof fuels;
};
export type Ships = {
  [key: string]: Ship;
};
export const ships: Ships = {
  // Cycle 3 has mass and fuel in show-info \o/
  Anser: { mass: 285_000_000, tank: 7050, fuel: "SOF-40" },
  Axte: { mass: 800_000_000, tank: 22030, fuel: "SOF-40" },
  Baile: { mass: 488_000_000, tank: 12200, fuel: "SOF-40" },
  Caruda: { mass: 1_425_000_000, tank: 49870, fuel: "SOF-40" },
  Drema: { mass: 69_000_000, tank: 1110, fuel: "SOF-40" },
  Explorer: { mass: 4_500_000, tank: 230, fuel: "uSOF-20" },
  Flegel: { mass: 143_000_000, tank: 2860, fuel: "SOF-40" },
  Forager: { mass: 8_000_000, tank: 120, fuel: "uSOF-20" },
  Grus: { mass: 2_383_000_000, tank: 71340, fuel: "SOF-40" },
  Harpia: { mass: 63_000_000, tank: 1020, fuel: "SOF-40" },
  Juav: { mass: 14_000_000, tank: 360, fuel: "uSOF-20" },
  Klinge: { mass: 800_000_000, tank: 21970, fuel: "SOF-40" },
  Microptero: { mass: 20_500_000, tank: 240, fuel: "SOF-40" },
  Pici: { mass: 26_000_000, tank: 330, fuel: "SOF-40" },
  Raubtier: { mass: 46_000_000, tank: 690, fuel: "SOF-40" },
  Rebus_K: { mass: 1_474_000_000, tank: 41_620, fuel: "SOF-40" },
  Samoskyd_1: { mass: 25_000_000, tank: 300, fuel: "SOF-40" },
  Strix: { mass: 96_000_000, tank: 1550, fuel: "SOF-40" },
  Ungher: { mass: 75_000_000, tank: 1400, fuel: "SOF-40" },
  Val: { mass: 28_000_000, tank: 550, fuel: "SOF-40" },
} as const;

export function isCompatible(fuel1: keyof typeof fuels, fuel2: keyof typeof fuels) {
  const fuel1_is_basic = fuel1 === "uSOF-20";
  const fuel2_is_basic = fuel2 === "uSOF-20";
  return fuel1_is_basic === fuel2_is_basic;
}
