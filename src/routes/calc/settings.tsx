import { createFileRoute } from "@tanstack/react-router";
import { useContext } from "react";
import { SettingsContext } from "../../providers/settings";
import { FuelName } from "../../consts";

export const Route = createFileRoute("/calc/settings")({
  component: Settings,
});

function Settings() {
  const { fuelCosts, setFuelCosts } = useContext(SettingsContext);

  return (
    <section>
      <h2>Settings</h2>
      <h3>Fuel Costs</h3>
      <p>
        Defaults assume that lenses cost 100k lux, catalytic dust costs 5000,
        and fuel is sold for zero profit.
      </p>
      <table className="form">
        <tbody>
          {(Object.entries(fuelCosts) as [FuelName, number][]).map(
            ([name, cost]) => (
              <tr key={name}>
                <th>{name}</th>
                <td>
                  <input
                    type="number"
                    required={true}
                    value={cost}
                    onChange={(e) => {
                      const newCosts = { ...fuelCosts };
                      newCosts[name] = e.target.valueAsNumber;
                      setFuelCosts(newCosts);
                    }}
                  />
                </td>
              </tr>
            ),
          )}
        </tbody>
      </table>
    </section>
  );
}
