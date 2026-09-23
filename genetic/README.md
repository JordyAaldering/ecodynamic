## Thread count and power limit adaptation using a genetic algorithm

Requires certain environment variables to be defined.
Either from the console, or in a `.env` file in the current working directory.

- `CORE_COUNT` The number of physical (performance) cores available on the system. (_required_)
- `THREAD_COUNT` The number of logical threads available on the system. (_required_)
- `MAX_POWER_UW` The maximum power draw of the processor in microwatts. (_required_)
- `MIN_POWER_FRAC` The minimum allowed fraction of the maximum power limit. (_default=0.1_)
- `MAX_POWER_FRAC` The maximum allowed fraction of the maximum power limit. (_default=1.0_)
- `DO_THREAD_CONTROL` Enable thread count control. (_default=false_)
- `DO_PINNING_CONTROL` Enable thread placement control. (_default=false_)
- `DO_POWER_CONTROL` Enable power limiting control. (_default=false_)

For example:

```bash
export MAX_POWER_UW=125000000
CORE_COUNT=4 THREAD_COUNT=8 ./genetic --energy-preference 0.9 [...]
```

The recommended approach is to define an environment file.
Given a project structure:

```
/working/directory
├──.env
└──genetic
```

Where `.env` contains:

```
RUST_LOG=error
CORE_COUNT=4
THREAD_COUNT=8
MAX_POWER_UW=125000000
DO_THREAD_CONTROL=true
DO_POWER_CONTROL=true
```

The environment file may also contain other relevant variables, such as the log level (`RUST_LOG`).
