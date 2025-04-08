#ifndef _MTD_LETTERBOX_H_
#define _MTD_LETTERBOX_H_

#include <stdint.h>

#define MTD_LETTERBOX_PATH "/tmp/mtd_letterbox"

struct Sample {
    int32_t region_uid;
    int32_t max_threads;
    int32_t num_threads;
    float runtime;
    float usertime;
    float energy;
};

struct Demand {
    int32_t num_threads;
};

#endif  // _MTD_LETTERBOX_H_
