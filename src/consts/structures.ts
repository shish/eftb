import type { ItemName } from "./items";

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
