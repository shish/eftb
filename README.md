create data/extracted-starmap.json using https://github.com/frontier-reapers/frontier-static-data

create data/star-names.json from https://gist.github.com/blurpesec/36a86540781e0d00f539067497e972db

```
cargo run --bin cli -- build
cargo run --bin cli -- --help
```

```
cargo run --bin web
npm run dev
```
