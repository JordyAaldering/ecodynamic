## Energy-Aware Dynamic Adaptation of Runtime Systems Through External Control

External resource controller that communicates with applications through a Unix domain socket.

The runtime sends JSON objects containing bookkeeping information and measurements:
- capabilities (e.g. maximum number of threads),
- code task identifiers (e.g. a repeated loop), and
- measurement samples (e.g. runtime, usertime, energy).

The controller replies with JSON messages, containing the desired number of threads for the next iteration of a task.
Internally, the controller also tracks a power limit, which is adjusted system-wide.

A text-based JSON protocol is used intentionally so new fields can be added by the controller without breaking backward compatibility with older runtime versions by ignoring unknown keys.
In addition, this enables more complicated communication patterns in the future, such as pinning strategies, or even broadcasting different implementation of the same algorithm to then dynamically find the best one.

---

- `api`: the communication protocol between the controller and applications.
- `core`: internal logic shared between controllers.
- `ecodynamic`: Rust library for some convenience functions for communicating with a controller.
- `delta`: delta-based, energy-optimising thread controller.
- `corridor`: corridor-based, runtime-optimising thread controller.
- `genetic`: genetic algorithm, EDT-optimising, thread and powercap controller.
- `fixed`: Template controller that always returns the same configuration.
