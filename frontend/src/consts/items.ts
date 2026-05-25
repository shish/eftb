import _items from "./items_data";

export type ItemName = keyof typeof _items;
export type Item = {
  volume: number;
  mass: number;
};
export const items: { [key in ItemName]: Item } = _items;
