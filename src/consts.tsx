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
  cargo: number;
};
export type Ships = {
  [key: string]: Ship;
};
export const ships: Ships = {
  // Cycle 3 has mass and fuel in show-info \o/
  Anser: { mass: 281_681_000, tank: 7_050, fuel: "SOF-40", cargo: 36_400 },
  Axte: { mass: 800_711_000, tank: 22_030, fuel: "SOF-40", cargo: 33_800 },
  Baile: { mass: 487_820_000, tank: 12_200, fuel: "SOF-40", cargo: 20_800 },
  Caruda: { mass: 1_424_833_000, tank: 49_870, fuel: "SOF-40", cargo: 54_600 },
  Dremar: { mass: 68_221_000, tank: 1_110, fuel: "SOF-40", cargo: 3_120 },
  Explorer: { mass: 4_517_000, tank: 230, fuel: "uSOF-20", cargo: 520 },
  Flegel: { mass: 142_121_000, tank: 2_860, fuel: "SOF-40", cargo: 31_200 },
  Forager: { mass: 7_642_000, tank: 120, fuel: "uSOF-20", cargo: 1_040 },
  Grus: { mass: 2_383_202_000, tank: 71_340, fuel: "SOF-40", cargo: 286_000 },
  Harpia: { mass: 62_359_000, tank: 1_020, fuel: "SOF-40", cargo: 3_120 },
  Juav: { mass: 12_928_000, tank: 360, fuel: "uSOF-20", cargo: 520 },
  Klinge: { mass: 798_858_000, tank: 21_970, fuel: "SOF-40", cargo: 31_200 },
  Microptero: { mass: 20_464_000, tank: 240, fuel: "SOF-40", cargo: 1_040 },
  Pici: { mass: 25_921_000, tank: 330, fuel: "SOF-40", cargo: 3_120 },
  Raubtier: { mass: 45_402_000, tank: 690, fuel: "SOF-40", cargo: 2_080 },
  Rebus_K: { mass: 1_474_255_000, tank: 41_620, fuel: "SOF-40", cargo: 312_000 },
  Samoskyd_1: { mass: 24_552_000, tank: 300, fuel: "SOF-40", cargo: 5_720 },
  Strix: { mass: 95_376_000, tank: 1_550, fuel: "SOF-40", cargo: 4_160 },
  Ungher: { mass: 74_389_000, tank: 1_400, fuel: "SOF-40", cargo: 3_120 },
  Val: { mass: 27_210_000, tank: 550, fuel: "SOF-40", cargo: 6_240 },
} as const;

export function isCompatible(fuel1: keyof typeof fuels, fuel2: keyof typeof fuels) {
  const fuel1_is_basic = fuel1 === "uSOF-20";
  const fuel2_is_basic = fuel2 === "uSOF-20";
  return fuel1_is_basic === fuel2_is_basic;
}
