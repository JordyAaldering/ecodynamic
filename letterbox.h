#ifndef _MTD_LETTERBOX_H_
#define _MTD_LETTERBOX_H_

#include <stdint.h>

#define MTD_LETTERBOX_PATH "/tmp/mtd_letterbox"

struct Sample {
    /// A unique identifier of the parallel region we are controlling.
    int32_t region_uid;
    /// The maximum number of threads allowed for this parallel region.
    int32_t max_threads;
    /// The number of threads used by the previous iteration.
    int32_t num_threads;
    /// Total runtime of the previous iteration.
    float runtime;
    /// Total usertime of the previous iteration.
    float usertime;
    /// Total energy consumption of the previous iteration.
    float energy;
};

struct Demand {
    /// Recommended number of threads to use for the next parallel iteration.
    int32_t num_threads;
};

#endif  // _MTD_LETTERBOX_H_
