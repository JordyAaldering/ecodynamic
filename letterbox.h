#ifndef _MTD_LETTERBOX_H_
#define _MTD_LETTERBOX_H_

#include <stdint.h>

#define MTD_LETTERBOX_PATH "/tmp/mtd_letterbox"

struct Request {
    /// A unique identifier of the parallel region we are controlling.
    int32_t region_uid;
    /// The same region might have a varying input size.
    int32_t problem_size;
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
