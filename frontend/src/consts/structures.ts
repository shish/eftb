import type { ItemName } from "./items";
import _structures from "./structures_data";

export type StructureName = keyof typeof _structures;
export type Structure = {
  components: { [key in ItemName]?: number };
  group: string;
};
export const posboms: { [key in StructureName]: Structure } = _structures;

export type BaseBom = { [key in StructureName]?: number };
