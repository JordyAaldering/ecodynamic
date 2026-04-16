## Energy-Aware Dynamic Adaptation of Runtime Systems Through External Control

External resource controller that communicates with applications through a Unix domain socket.

The runtime sends JSON objects containing bookkeeping information and measurements:
- capabilities (e.g. maximum number of threads),
- code region identifiers (e.g. a repeated loop), and
- measurement samples (e.g. runtime, usertime, energy).

The controller replies with JSON messages, containing the desired number of threads for the next iteration of a region.
Internally, the controller also tracks a power limit, which is adjusted system-wide.

A text-based JSON protocol is used intentionally so new fields can be added by the controller without breaking backward compatibility with older runtime versions by ignoring unknown keys.
In addition, this enables more complicated communication patterns in the future, such as pinning strategies, or even broadcasting different implementation of the same algorithm to then dynamically find the best one.
