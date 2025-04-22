### Flags
If you skip all flags, all tests are run.

If you want to run test separately use following flags:
1. Runs only CPU test
```bash
--cpu
```
2. Runs only FS test
```bash
--fs
```

### ENV Variables
If you want to change the predefined configuration values: copy `.env.example` and fill it with the desired values.
```bash
cp .env.example .env
```


### Run
1. Dev mode
```bash
cargo run [-- [--cpu, --fs]]
```

2. Release mode
```bash
cargo run --release [-- [--cpu, --fs]]
```