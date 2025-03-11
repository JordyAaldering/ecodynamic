#ifndef _MTD_LETTERBOX_H_
#define _MTD_LETTERBOX_H_

#include <stdint.h>

#define MTD_LETTERBOX_PATH "/tmp/mtd_letterbox"

struct Incoming {
    int32_t uid;
    float val;
};

struct Outgoing {
    int32_t threads;
};

#endif  // _MTD_LETTERBOX_H_
