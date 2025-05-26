#ifndef _MTD_LETTERBOX_H_
#define _MTD_LETTERBOX_H_

#include <stdint.h>

#define MTD_LETTERBOX_PATH "/tmp/mtd_letterbox"

/// Application-specific demands that have to be set by the controlled application.
struct LocalDemand {
    /// Recommended number of threads to use for the next parallel iteration.
    int32_t num_threads;
};

struct Request {
    /// A unique identifier of the parallel region we are controlling.
    int32_t region_uid;
    /// The maximum number of threads allowed for this parallel region.
    int32_t max_threads;
};

struct Sample {
    /// A unique identifier of the parallel region we are controlling.
    int32_t region_uid;
    /// Total runtime of the previous iteration.
    float runtime;
    /// Total usertime of the previous iteration.
    float usertime;
    /// Total energy consumption of the previous iteration.
    float energy;
};

#endif  // _MTD_LETTERBOX_H_
