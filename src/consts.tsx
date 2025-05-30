export type FuelName = "uSOF-20" | "SOF-40" | "EU-40" | "SOF-80" | "EU-90";
export type Fuel = number;
export const fuels: { [key in FuelName]: Fuel } = {
  "uSOF-20": 0.2,
  "SOF-40": 0.4,
  "EU-40": 0.4,
  "SOF-80": 0.8,
  "EU-90": 0.9,
};

export type ShipType =
  | "Shuttle"
  | "Corvette"
  | "Destroyer"
  | "Frigate"
  | "Cruiser"
  | "Combat Battlecruiser"
  | "Battleship";

export type EngineName = "uSOF XS" | "GSC XS" | "GSC S" | "GSC M" | "GSC L";
export type Engine = {
  mass: number;
  fuel: FuelName;
  types: ShipType[];
};
export const engines: { [key in EngineName]: Engine } = {
  "uSOF XS": { mass: 53_694, fuel: "uSOF-20", types: ["Shuttle"] },
  "GSC XS": { mass: 99_758, fuel: "uSOF-20", types: ["Corvette"] },
  "GSC S": { mass: 208_650, fuel: "SOF-40", types: ["Frigate", "Destroyer"] },
  "GSC M": {
    mass: 417_280,
    fuel: "SOF-40",
    types: ["Destroyer", "Cruiser", "Combat Battlecruiser"],
  },
  "GSC L": {
    mass: 834_540,
    fuel: "SOF-40",
    types: ["Combat Battlecruiser", "Battleship"],
  },
};

export type ShipName =
  | "Anser"
  | "Axte"
  | "Baile"
  | "Caruda"
  | "Dremar"
  | "Explorer"
  | "Flegel"
  | "Forager"
  | "Grus"
  | "Harpia"
  | "Juav"
  | "Klinge"
  | "Microptero"
  | "Pici"
  | "Raubtier"
  | "Rebus-K"
  | "Samoskyd-1"
  | "Strix"
  | "Ungher"
  | "Val";
export type Ship = {
  mass: number;
  tank: number;
  type: ShipType;
  cargo: number;
};
export const ships: { [key in ShipName]: Ship } = {
  // Cycle 3 has mass and fuel in show-info \o/
  Anser: { mass: 281_681_000, tank: 7_050, type: "Cruiser", cargo: 36_400 },
  Axte: {
    mass: 800_711_000,
    tank: 22_030,
    type: "Combat Battlecruiser",
    cargo: 33_800,
  },
  Baile: { mass: 487_820_000, tank: 12_200, type: "Cruiser", cargo: 20_800 },
  Caruda: {
    mass: 1_424_833_000,
    tank: 49_870,
    type: "Battleship",
    cargo: 54_600,
  },
  Dremar: { mass: 68_221_000, tank: 1_110, type: "Destroyer", cargo: 3_120 },
  Explorer: { mass: 4_517_000, tank: 230, type: "Shuttle", cargo: 520 },
  Flegel: { mass: 142_121_000, tank: 2_860, type: "Cruiser", cargo: 31_200 },
  Forager: { mass: 7_642_000, tank: 120, type: "Shuttle", cargo: 1_040 },
  Grus: {
    mass: 2_383_202_000,
    tank: 71_340,
    type: "Battleship",
    cargo: 286_000,
  },
  Harpia: { mass: 62_359_000, tank: 1_020, type: "Destroyer", cargo: 3_120 },
  Juav: { mass: 12_928_000, tank: 360, type: "Corvette", cargo: 520 },
  Klinge: { mass: 798_858_000, tank: 21_970, type: "Cruiser", cargo: 31_200 },
  Microptero: { mass: 20_464_000, tank: 240, type: "Frigate", cargo: 1_040 },
  Pici: { mass: 25_921_000, tank: 330, type: "Frigate", cargo: 3_120 },
  Raubtier: { mass: 45_402_000, tank: 690, type: "Frigate", cargo: 2_080 },
  "Rebus-K": {
    mass: 1_474_255_000,
    tank: 41_620,
    type: "Combat Battlecruiser",
    cargo: 312_000,
  },
  "Samoskyd-1": { mass: 24_552_000, tank: 300, type: "Frigate", cargo: 5_720 },
  Strix: { mass: 95_376_000, tank: 1_550, type: "Destroyer", cargo: 4_160 },
  Ungher: { mass: 74_389_000, tank: 1_400, type: "Frigate", cargo: 3_120 },
  Val: { mass: 27_210_000, tank: 550, type: "Frigate", cargo: 6_240 },
} as const;

export function isCompatible(fuel1: FuelName, fuel2: FuelName) {
  const fuel1_is_basic = fuel1 === "uSOF-20";
  const fuel2_is_basic = fuel2 === "uSOF-20";
  return fuel1_is_basic === fuel2_is_basic;
}

export function getEngine(type: ShipType): Engine {
  for (const engineNameS of Object.keys(engines)) {
    const engineName = engineNameS as EngineName;
    if (engines[engineName].types.includes(type)) {
      return engines[engineName];
    }
  }
  throw new Error(`No engine found for type ${type}`);
}

export type ItemName = string;
export type Item = {
  volume: number;
  mass: number;
};
export const items: { [key: ItemName]: Item } = {
  "uSOF-20 Fuel": { volume: 0.28, mass: 42 },
  "SOF-40 Fuel": { volume: 0.28, mass: 42 },
  "EU-40 Fuel": { volume: 0.28, mass: 42 },
  "SOF-80 Fuel": { volume: 0.28, mass: 42 },
  "EU-90 Fuel": { volume: 0.28, mass: 42 },
  "Carbonaceous Ore": { volume: 1, mass: 1671.31 },
  "Carbonaceous Materials": { volume: 0.01, mass: 20 },
  Silicates: { volume: 0.01, mass: 30 },
  "Bulky Cargo Panels": { volume: 100, mass: 294_074 },
  "Mounting Platform": { volume: 5, mass: 17_044 },
  Radar: { volume: 1, mass: 904 },
  "Carbonaceous Fuel": { volume: 1, mass: 1671.31 },
  Thorium: { volume: 0.01, mass: 117 },
  "Steel Beams": { volume: 10, mass: 76_200 },
  "Steel Plates": { volume: 1, mass: 7_620 },
  "Light Metal Alloy": { volume: 0.1, mass: 400 },
  "Light Metal Framing": { volume: 10, mass: 40_200 },
  "Fuel Tank": { volume: 25, mass: 13_442 },
};

export type StructureName = string;
export type StructureBom = { [key: ItemName]: number };
export const posboms: { [key: StructureName]: StructureBom } = {
  "Portable Refinery": {
    "Carbonaceous Ore": 400,
  },
  "Portable Printer": {
    "Carbonaceous Materials": 420,
    Silicates: 60,
  },
  Sepulchre: {
    "Steel Plates": 400,
    "Light Metal Alloy": 6,
  },
  Refuge: {
    Silicates: 430,
    "Steel Plates": 8,
    "Light Metal Alloy": 300,
  },
  "Storage Unit": {
    "Carbonaceous Ore": 400,
  },
  "Printer L": {
    Thorium: 100,
    "Steel Plates": 200,
    "Light Metal Alloy": 500,
    "Bulky Cargo Panels": 50,
  },
  "Smart Storage Unit": {
    "Steel Plates": 60,
    "Light Metal Framing": 6,
    "Bulky Cargo Panels": 5,
  },
  "Smart Turret": {
    "Steel Plates": 500,
    "Light Metal Framing": 50,
    "Mounting Platform": 1,
    Radar: 1,
  },
  "Smart Gate": {
    Thorium: 100,
    "Steel Beams": 100,
    "Light Metal Framing": 100,
    "Fuel Tank": 160,
  },
  Hedgehog: {
    "Steel Plates": 500,
    "Steel Beams": 50,
  },
};

export type BaseBom = { [key: StructureName]: number };
