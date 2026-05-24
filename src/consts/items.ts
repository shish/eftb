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
