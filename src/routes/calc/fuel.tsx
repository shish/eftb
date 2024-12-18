import { createFileRoute } from '@tanstack/react-router'
import { useState, FormEvent } from 'react'
import { api } from '../../api'
import { ships, fuels } from '../../consts'

export const Route = createFileRoute('/calc/fuel')({
  component: FuelCalculator,
})

function FuelCalculator() {
  const [ship, setShip] = useState('Val')
  const [mass, setMass] = useState(28000000)
  const [dist, setDist] = useState(100)
  const [fuelType, setFuelType] = useState('SOF-40')

  const [fuel, setFuel] = useState<null | number>(null)
  const [error, setError] = useState<null | Error>(null)

  function submit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault()
    api(e.target as HTMLFormElement, setFuel, setError)
  }

  return (
    <section>
      <h2>How much fuel will I need?</h2>
      <p>How much fuel will it take to jump a given distance</p>
      <form action="/api/fuel" method="get" onSubmit={submit}>
        <table>
          <tbody>
            <tr>
              <td>Ship</td>
              <td>
                <select
                  value={ship}
                  onChange={(e) => {
                    const ship = ships[e.target.value]
                    setShip(e.target.value)
                    setMass(ship.mass)
                    setFuel(ship.fuel)
                    setFuelType(ship.fuel_type)
                  }}
                >
                  {Object.keys(ships).map((ship) => (
                    <option key={ship} value={ship}>
                      {ship}
                    </option>
                  ))}
                </select>
              </td>
              <td>(Just a shortcut to set mass &amp; fuel type)</td>
            </tr>
            <tr>
              <td>Mass (kg)</td>
              <td>
                <input
                  name="mass"
                  type="number"
                  min="1"
                  required={true}
                  value={mass}
                  onChange={(e) => setMass(parseInt(e.target.value))}
                />
              </td>
            </tr>
            <tr>
              <td>Dist (ly)</td>
              <td>
                <input
                  name="dist"
                  type="number"
                  min="1"
                  required={true}
                  value={dist}
                  onChange={(e) => setDist(parseInt(e.target.value))}
                />
              </td>
            </tr>
            <tr>
              <td>Fuel Type</td>
              <td>
                <select name="efficiency">
                  {Object.entries(fuels).map(([name, value]) => (
                    <option
                      key={name}
                      value={value}
                      selected={name == fuelType}
                    >
                      {name}
                    </option>
                  ))}
                </select>
              </td>
            </tr>
            <tr>
              <td>
                <input type="submit" value="Calculate" />
              </td>
              <td>
                {fuel && `${fuel.toFixed(2)} units`}
                {error && error.message}
              </td>
            </tr>
          </tbody>
        </table>
      </form>
    </section>
  )
}
