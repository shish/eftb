# EVE Frontiers Toolbox

## How to prepare the data
1. Create `data/extracted-starmap.json` using https://github.com/frontier-reapers/frontier-static-data (This data is under NDA, don't post it anywhere)
2. Create `data/star-names.json` from https://gist.github.com/blurpesec/36a86540781e0d00f539067497e972db (I don't know where this data came from, and it appears to be a little out of date, but I don't see any better options)
3. Create `data/starmap.bin` using `cargo run -- build` (This compiles the starmap data into a form that's more optimized for route planning)

## Testing with the CLI
* `cargo run -- --help`
* `cargo run -- path E.G1G.6GD Nod`

## Running the web interface
* run the backend: `cargo run --bin web`
* run the frontend: `npm run dev`
* open `http://localhost:5173` in your browser

## Building for production
* `docker build -t eftb .`

## Running in production
* `docker run -p 8000 eftb`
* open `http://localhost:8000` in your browser
