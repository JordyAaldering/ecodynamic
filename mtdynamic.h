#ifndef _MTDYNAMIC_H_
#define _MTDYNAMIC_H_

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Incoming {
  int32_t pid;
  int32_t fid;
  float val;
};

struct Outgoing {
  int32_t threads;
};

#endif  // _MTDYNAMIC_H_
