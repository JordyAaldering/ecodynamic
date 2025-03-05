#ifndef _MTDYNAMIC_H_
#define _MTDYNAMIC_H_

struct Incoming {
  int pid;
  int fid;
  float val;
};

struct Outgoing {
  int threads;
};

#endif  // _MTDYNAMIC_H_
