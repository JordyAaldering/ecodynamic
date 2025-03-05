#ifndef _MTD_LETTERBOX_H_
#define _MTD_LETTERBOX_H_

#include <stdint.h>

struct Incoming {
    int32_t pid;
    int32_t fid;
    float val;
};

struct Outgoing {
    int32_t threads;
};

#endif  // _MTD_LETTERBOX_H_
