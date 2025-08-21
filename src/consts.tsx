import { FuelName } from "./consts/fuels";

export type ShipType =
  | "Shuttle"
  | "Corvette"
  | "Destroyer"
  | "Frigate"
  | "Cruiser"
  | "Combat Battlecruiser"
  | "Battleship";

export type EngineName =
  | "Celerity CD03"
  | "Tempo CD43"
  | "Celerity CD02"
  | "Tempo CD42"
  | "Velocity CD82"
  | "Celerity CD01"
  | "Embark"
  | "Sojourn"
  | "Tempo CD41"
  | "Velocity CD81";
export type Engine = {
  mass: number;
  fuel: FuelName;
  types: ShipType[];
};
export const engines: { [key in EngineName]: Engine } = {
  // Large
  "Celerity CD03": {
    mass: 834_540,
    fuel: "SOF-40",
    types: ["Combat Battlecruiser", "Battleship"],
  },
  "Tempo CD43": {
    mass: 834_540,
    fuel: "SOF-40",
    types: ["Combat Battlecruiser", "Battleship"],
  },
  // Medium
  "Celerity CD02": {
    mass: 417_280,
    fuel: "SOF-40",
    types: ["Destroyer", "Cruiser", "Combat Battlecruiser"],
  },
  "Tempo CD42": {
    mass: 417_280,
    fuel: "SOF-40",
    types: ["Destroyer", "Cruiser", "Combat Battlecruiser"],
  },
  "Velocity CD82": {
    mass: 417_280,
    fuel: "SOF-40",
    types: ["Destroyer", "Cruiser", "Combat Battlecruiser"],
  },
  // Small
  "Celerity CD01": {
    mass: 300_000,
    fuel: "SOF-40",
    types: ["Frigate", "Destroyer"],
  },
  Embark: { mass: 150_000, fuel: "D1", types: ["Shuttle"] },
  Sojourn: { mass: 150_000, fuel: "D1", types: ["Corvette"] },
  "Tempo CD41": {
    mass: 300_000,
    fuel: "SOF-40",
    types: ["Frigate", "Destroyer"],
  },
  "Velocity CD81": {
    mass: 300_000,
    fuel: "SOF-40",
    types: ["Frigate", "Destroyer"],
  },
};

export type ShipName =
  | "HAF"
  | "LORHA"
  | "MAUL"
  | "MCF"
  | "TADES"
  | "USV"
  | "Recurve"
  | "Reflex"
  | "Reiver"
  | "Wend"
  | "Chumaq";
export type Ship = {
  mass: number;
  tank: number;
  type: ShipType;
  cargo: number;
};
export const ships: { [key in ShipName]: Ship } = {
  // Exclave Ventures
  HAF: { mass: 81_883_000, tank: 4_184, type: "Frigate", cargo: 3_120 },
  LORHA: { mass: 31_369_320, tank: 2_508, type: "Frigate", cargo: 6_240 },
  MAUL: { mass: 548_435_920, tank: 24_160, type: "Cruiser", cargo: 20_800 },
  MCF: { mass: 52_313_760, tank: 6_548, type: "Frigate", cargo: 2_080 },
  TADES: { mass: 74_655_480, tank: 5_972, type: "Destroyer", cargo: 3_120 },
  USV: { mass: 30_266_600, tank: 2_420, type: "Frigate", cargo: 3_120 },
  // Keep
  Recurve: { mass: 10400000, tank: 970, type: "Corvette", cargo: 520 },
  Reflex: { mass: 9750000, tank: 1750, type: "Corvette", cargo: 520 },
  Reiver: { mass: 10200000, tank: 1416, type: "Corvette", cargo: 520 },
  Wend: { mass: 6800000, tank: 200, type: "Shuttle", cargo: 520 },
  // Synod
  Chumaq: {
    mass: 1739489536,
    tank: 270_585,
    type: "Combat Battlecruiser",
    cargo: 312_000,
  },
} as const;

export function getEngine(type: ShipType): Engine {
  for (const engineNameS of Object.keys(engines)) {
    const engineName = engineNameS as EngineName;
    if (engines[engineName].types.includes(type)) {
      return engines[engineName];
    }
  }
  throw new Error(`No engine found for type ${type}`);
}

const _items = {
  "D1 Fuel": { volume: 0.28, mass: 20 },
  "D2 Fuel": { volume: 0.28, mass: 30 },
  "SOF-40 Fuel": { volume: 0.28, mass: 25 },
  "EU-40 Fuel": { volume: 0.28, mass: 25 },
  "SOF-80 Fuel": { volume: 0.28, mass: 30 },
  "EU-90 Fuel": { volume: 0.28, mass: 30 },

  "Common Ore": { volume: 1, mass: 2_500 },
  "Metal-rich Ore": { volume: 1, mass: 3_000 },
  "Carbonaceous Ore": { volume: 1, mass: 1_500 },

  "Building Foam": { volume: 470, mass: 4_700_000 },
  "Printed Circuits": { volume: 4, mass: 10_500 },
  "Carbon Weave": { volume: 15, mass: 30_000 },
  "Reinforced Alloys": { volume: 10, mass: 56_000 },
  "Thermal Composites": { volume: 10, mass: 24_200 },
  "Exclave Technocore": { volume: 20, mass: 94_299 },
  "Synod Technocore": { volume: 20, mass: 94_299 },
};
export type ItemName = keyof typeof _items;
export type ItemAttrs = {
  volume: number;
  mass: number;
};
export const items: { [key in ItemName]: ItemAttrs } = _items;

const _posboms = {
  // Core
  Refuge: {
    "Metal-rich Ore": 50,
  },
  "Portable Refinery": {
    "Common Ore": 50,
  },
  "Portable Printer": {
    "Carbonaceous Ore": 50,
  },
  "Network Node": {
    "Carbon Weave": 10,
    "Printed Circuits": 10,
    "Thermal Composites": 10,
  },
  "Portable Storage": {
    "Common Ore": 50,
  },
  // Industry
  "Printer S": {
    "Reinforced Alloys": 15,
    "Printed Circuits": 15,
  },
  "Printer M": {
    "Building Foam": 2,
  },
  "Printer L": {
    "Building Foam": 10,
  },
  "Refinery M": {
    "Reinforced Alloys": 15,
    "Thermal Composites": 15,
  },
  "Refinery L": {
    "Building Foam": 10,
  },
  Assembler: {
    "Reinforced Alloys": 20,
    "Carbon Weave": 10,
    "Printed Circuits": 5,
  },
  "Shipyard S": {
    "Reinforced Alloys": 20,
    "Carbon Weave": 10,
    "Printed Circuits": 5,
  },
  "Shipyard M": {
    "Building Foam": 2,
    "Exclave Technocore": 1,
  },
  "Shipyard L": {
    "Building Foam": 13,
    "Synod Technocore": 1,
  },
  // Storage
  "Smart Storage Unit S": {
    "Common Ore": 250,
    "Metal-rich Ore": 250,
  },
  "Smart Storage Unit M": {
    "Building Foam": 2,
  },
  "Smart Storage Unit L": {
    "Building Foam": 12,
  },
  // Gates
  "Smart Gate": {
    "Building Foam": 650,
  },
  "Small Gate": {
    "Building Foam": 43,
  },
  // Defense
  "Smart Turret": {
    "Building Foam": 1,
  },
  // Hangars
  "Hangar M": {
    "Building Foam": 2,
  },
  "Hangar L": {
    "Building Foam": 13,
  },
  // Misc
  "Totem 1": {
    "Building Foam": 2,
  },
  "Totem 2": {
    "Building Foam": 2,
  },
  "Wall 1": {
    "Building Foam": 2,
  },
  "Wall 2": {
    "Building Foam": 2,
  },
  "Seer I": {
    "Building Foam": 2,
  },
  "Seer II": {
    "Building Foam": 2,
  },
  "Harbinger I": {
    "Building Foam": 2,
  },
  "Harbinger II": {
    "Building Foam": 2,
  },
  "Rainmaker I": {
    "Building Foam": 2,
  },
  "Rainmaker II": {
    "Building Foam": 2,
  },
};
export type StructureName = keyof typeof _posboms;
export type StructureBom = { [key in ItemName]?: number };
export const posboms: { [key in StructureName]: StructureBom } = _posboms;

export type BaseBom = { [key in StructureName]?: number };
