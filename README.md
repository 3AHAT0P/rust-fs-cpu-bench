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

#### CPU
1. How many calcs should be done by each cpu thread (default: 10_000_000)
```bash
CPU_NUM_CALCS=<integer>
```
2. How many iterations in threads will be done (default: 20_000)
```bash
CPU_NUM_ITERS=<integer>
```


#### FS
1. Max file size in bytes (default: 20_000_000_000)
```bash
MAX_FILE_SIZE=<integer>
```
2. Skip bytes per seek iteration (default: 1024)
```bash
SEEK_SKIP_BYTES=<integer>
```
3. Read file by chunks in size of bytes (default: 1024)
```bash
READ_BYTES=<integer>
```
4. Read exact amount of bytes into the RAM (default: 5_368_709_120)
```
MMAP_READ_BYTES=<integer>
```

### Run
1. Dev mode
```bash
[CPU_NUM_CALCS=, CPU_NUM_ITERS=, MAX_FILE_SIZE=, SEEK_SKIP_BYTES=, READ_BYTES=, MMAP_READ_BYTES=] cargo run [-- [--cpu, --fs]]
```

2. Release mode
```bash
[CPU_NUM_CALCS=, CPU_NUM_ITERS=, MAX_FILE_SIZE=, SEEK_SKIP_BYTES=, READ_BYTES=, MMAP_READ_BYTES=] cargo run --release [-- [--cpu, --fs]]
```