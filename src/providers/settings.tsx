import React, { createContext } from "react";
import { useLocalStorage } from "usehooks-ts";
import { FuelName } from "../consts/fuels";

const dustCost = 50000;
type FuelCosts = { [key in FuelName]: number };
const defaultFuelCosts: FuelCosts = {
  D1: 1,
  D2: 5,
  "SOF-40": 100_000 / 2500,
  "EU-40": 100_000 / 2500,
  "SOF-80": 100_000 / 2500 + (44 * dustCost) / 500,
  "EU-90": 100_000 / 2500 + (60 * dustCost) / 500,
};

export interface SettingsContextType {
  fuelCosts: FuelCosts;
  setFuelCosts: (newFuelCosts: FuelCosts) => void;
}

export const SettingsContext = createContext<SettingsContextType>(
  {} as SettingsContextType,
);

export function SettingsProvider(props: { children: React.ReactNode }) {
  const [fuelCosts, setFuelCosts] = useLocalStorage<{
    [key in FuelName]?: number;
  }>("fuel_costs", {});

  return (
    <SettingsContext.Provider
      value={{
        fuelCosts: {
          ...defaultFuelCosts,
          ...fuelCosts,
        },
        setFuelCosts,
      }}
    >
      {props.children}
    </SettingsContext.Provider>
  );
}
