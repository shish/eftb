# EVE Frontier Toolbox

## How to prepare the data

1. Fetch blockchain data with `make sync-blockchain` (Leave it running if you want to stay in-sync)
2. Fetch and compile other data with `make`

## Testing with the CLI

- `cargo run -- --help`
- `cargo run -- path E.G1G.6GD Nod`

## Testing the web interface

- run the backend: `cargo run --bin web`
- run the frontend: `npm run dev`
- open `http://localhost:5173` in your browser

## Running in production

- `docker build -t eftb .`
- `docker run -p 8000 -v ./data:/app/data eftb`
- open `http://localhost:8000` in your browser

## Code layout

- `backend/`

  - `lib.rs` - the heavy number crunching
  - `raw.rs` - data structures imported from CCP's data files
  - `data.rs` - data structures used by the backend for live pathfinding
  - `web.rs` - the web interface
  - `cli.rs` - the CLI interface

- `src/` - the React frontend

  - `api.tsx` - a simple wrapper around the backend API
  - `routes/` - a .tsx file for each page on the site

- `tools/` - various scripts for fetching and processing data
