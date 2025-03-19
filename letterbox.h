#ifndef _MTD_LETTERBOX_H_
#define _MTD_LETTERBOX_H_

#include <stdint.h>

#define MTD_LETTERBOX_PATH "/tmp/mtd_letterbox"

struct Sample {
    int32_t uid;
    float val;
};

struct Demand {
    uint8_t threads;
};

#endif  // _MTD_LETTERBOX_H_
