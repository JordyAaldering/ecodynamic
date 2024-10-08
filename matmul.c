

/*
 *  Global Switches
 */

#ifndef SAC_DO_CHECK
#define SAC_DO_CHECK                             0
#endif
#ifndef SAC_DO_CHECK_TYPE
#define SAC_DO_CHECK_TYPE                        0
#endif
#ifndef SAC_DO_CHECK_GPU
#define SAC_DO_CHECK_GPU                         0
#endif
#ifndef SAC_DO_CHECK_BOUNDARY
#define SAC_DO_CHECK_BOUNDARY                    0
#endif
#ifndef SAC_DO_CHECK_MALLOC
#define SAC_DO_CHECK_MALLOC                      0
#endif
#ifndef SAC_DO_CHECK_ERRNO
#define SAC_DO_CHECK_ERRNO                       0
#endif
#ifndef SAC_DO_CHECK_HEAP
#define SAC_DO_CHECK_HEAP                        0
#endif
#ifndef SAC_DO_CHECK_DISTMEM
#define SAC_DO_CHECK_DISTMEM                     0
#endif
#ifndef SAC_DO_CHECK_DISTMEMPHM
#define SAC_DO_CHECK_DISTMEMPHM                  0
#endif

#define SAC_DO_PHM                               1
#define SAC_DO_APS                               1
#define SAC_DO_DAO                               1
#define SAC_DO_MSCA                              1
#define SAC_DO_COMPILE_MODULE                    0

#ifndef SAC_DO_PROFILE
#define SAC_DO_PROFILE                           0
#endif
#ifndef SAC_DO_PROFILE_WITH
#define SAC_DO_PROFILE_WITH                      0
#endif
#ifndef SAC_DO_PROFILE_FUN
#define SAC_DO_PROFILE_FUN                       0
#endif
#ifndef SAC_DO_PROFILE_INL
#define SAC_DO_PROFILE_INL                       0
#endif
#ifndef SAC_DO_PROFILE_LIB
#define SAC_DO_PROFILE_LIB                       0
#endif
#ifndef SAC_DO_PROFILE_MEM
#define SAC_DO_PROFILE_MEM                       0
#endif
#ifndef SAC_DO_PROFILE_OPS
#define SAC_DO_PROFILE_OPS                       0
#endif
#ifndef SAC_DO_PROFILE_CUDA
#define SAC_DO_PROFILE_CUDA                      0
#endif
#ifndef SAC_DO_PROFILE_DISTMEM
#define SAC_DO_PROFILE_DISTMEM                   0
#endif

#ifndef SAC_DO_TRACE
#define SAC_DO_TRACE                             0
#endif
#ifndef SAC_DO_TRACE_REF
#define SAC_DO_TRACE_REF                         0
#endif
#ifndef SAC_DO_TRACE_MEM
#define SAC_DO_TRACE_MEM                         0
#endif
#ifndef SAC_DO_TRACE_PRF
#define SAC_DO_TRACE_PRF                         0
#endif
#ifndef SAC_DO_TRACE_FUN
#define SAC_DO_TRACE_FUN                         0
#endif
#ifndef SAC_DO_TRACE_WL
#define SAC_DO_TRACE_WL                          0
#endif
#ifndef SAC_DO_TRACE_AA
#define SAC_DO_TRACE_AA                          0
#endif
#ifndef SAC_DO_TRACE_MT
#define SAC_DO_TRACE_MT                          0
#endif
#ifndef SAC_DO_TRACE_GPU
#define SAC_DO_TRACE_GPU                         0
#endif
#ifndef SAC_DO_TRACE_RTSPEC
#define SAC_DO_TRACE_RTSPEC                      0
#endif
#ifndef SAC_DO_TRACE_DISTMEM
#define SAC_DO_TRACE_DISTMEM                     0
#endif

#ifndef SAC_DO_CACHESIM
#define SAC_DO_CACHESIM                          0
#endif
#ifndef SAC_DO_CACHESIM_ADV
#define SAC_DO_CACHESIM_ADV                      0
#endif
#ifndef SAC_DO_CACHESIM_GLOBAL
#define SAC_DO_CACHESIM_GLOBAL                   1
#endif
#ifndef SAC_DO_CACHESIM_FILE
#define SAC_DO_CACHESIM_FILE                     0
#endif
#ifndef SAC_DO_CACHESIM_PIPE
#define SAC_DO_CACHESIM_PIPE                     0
#endif
#ifndef SAC_DO_CACHESIM_IMDT
#define SAC_DO_CACHESIM_IMDT                     1
#endif

/*
 * Setup for Multi Threaded Data Parallelism
 */
#define SAC_DO_MULTITHREAD                       1
#define SAC_DO_THREADS_STATIC                    0
#define SAC_DO_MT_CREATE_JOIN                    0
#define SAC_DO_MT_PTHREAD                        1
#define SAC_DO_MT_LPEL                           0

/*
 * Setup for OMP Data Parallelism
 */
#define SAC_DO_MT_OMP                            0
#define SAC_DO_OMP_MACROS                        0

/*
 * Setup for MUTC
 */
#define SAC_MUTC_FUNAP_AS_CREATE                 0
#define SAC_MUTC_THREAD_MALLOC                   0
#define SAC_MUTC_DISABLE_THREAD_MEM              0
#define SAC_MUTC_BENCH                           0
#define SAC_MUTC_MACROS                          0
#define SAC_MUTC_RC_INDIRECT                     0
#define SAC_MUTC_SEQ_DATA_PARALLEL               0

/*
 * Setup for GPU Data Parallelism
 */
#define SAC_CUDA_MACROS                          0

/*
 * Setup for Distributed Memory Data Parallelism
 */
#define SAC_DO_DISTMEM                           0
#define SAC_DO_DISTMEM_GASNET                    0
#define SAC_DO_DISTMEM_GPI                       0
#define SAC_DO_DISTMEM_MPI                       0
#define SAC_DO_DISTMEM_ARMCI                     0
#define SAC_DO_DISTMEM_ALLOC_CACHE_OUTSIDE_DSM   0
#define SAC_DO_DISTMEM_PTR_DESC                  0
#define SAC_DO_DISTMEM_PTR_CACHE                 1

/*
 * Setup for Task Parallelism
 */
#define SAC_DO_FP                                0

/*
 * Debugging Support
 */
#ifndef SAC_DEBUG_RC
#define SAC_DEBUG_RC                             0
#endif

#define SAC_DO_CUDA_FORCE_INIT 1


/*
 *  Global Settings
 */

#define SAC_FORCE_DESC_SIZE -1

/*
 *  MUTC Backend Specific Settings
 */
#define SAC_MUTC_RC_PLACES  1
#define SAC_MUTC_FORCE_SPAWN_FLAGS

#define SAC_C_EXTERN           extern



/*
 *  Global Settings
 */

#ifndef NULL
#  ifdef __cplusplus
#    define NULL         0
#  else
#    define NULL         (void*) 0
#  endif
#endif

#define SAC_SET_TMPDIR              "/tmp"
#define SAC_SET_INITIAL_MASTER_HEAPSIZE      1048576
#define SAC_SET_INITIAL_WORKER_HEAPSIZE      65536
#define SAC_SET_INITIAL_UNIFIED_HEAPSIZE     0

#ifndef SAC_SET_RTSPEC_THREADS
#define SAC_SET_RTSPEC_THREADS              1
#endif

#ifndef SAC_SET_MTMODE
#define SAC_SET_MTMODE               2
#endif

#define SAC_SET_CPU_BIND_STRATEGY 1
#define SAC_SET_BARRIER_TYPE               0
#define SAC_SET_SMART_DECISIONS            0
#define SAC_SET_SMART_FILENAME           "default"
#define SAC_SET_SMART_ARCH               "(null)"
#define SAC_SET_SMART_PERIOD               500
#ifndef SAC_SET_THREADS_MAX
#define SAC_SET_THREADS_MAX          128
#endif

#ifndef SAC_SET_THREADS
#define SAC_SET_THREADS              0
#endif

#ifndef SAC_OMP_ACTIVE_LEVEL
#define SAC_OMP_ACTIVE_LEVEL          1
#endif

#ifndef SAC_SET_MASTERCLASS
#define SAC_SET_MASTERCLASS          0
#endif

#define SAC_SET_NUM_SCHEDULERS       1

#define SAC_SET_CACHE_1_SIZE         -1
#define SAC_SET_CACHE_1_LINE         4
#define SAC_SET_CACHE_1_ASSOC        1
#define SAC_SET_CACHE_1_WRITEPOL     SAC_CS_default
#define SAC_SET_CACHE_1_MSCA_FACTOR  0.00

#define SAC_SET_CACHE_2_SIZE         -1
#define SAC_SET_CACHE_2_LINE         4
#define SAC_SET_CACHE_2_ASSOC        1
#define SAC_SET_CACHE_2_WRITEPOL     SAC_CS_default
#define SAC_SET_CACHE_2_MSCA_FACTOR  0.00

#define SAC_SET_CACHE_3_SIZE         -1
#define SAC_SET_CACHE_3_LINE         4
#define SAC_SET_CACHE_3_ASSOC        1
#define SAC_SET_CACHE_3_WRITEPOL     SAC_CS_default
#define SAC_SET_CACHE_3_MSCA_FACTOR  0.00

#define SAC_SET_CACHESIM_HOST        ""
#define SAC_SET_CACHESIM_FILE        "matmul.cs"
#define SAC_SET_CACHESIM_DIR         "/tmp"
#define SAC_SET_MAXFUN               0
#define SAC_SET_MAXFUNAP             1
#define SBLOCKSZ               16
#define LBLOCKSZ               256



/*
 *  Includes
 */


#include "sac.h"


#if SAC_OMP_MACROS

#include "omp.h"

#endif

#if SAC_CUDA_MACROS

#include <stdio.h>


#include <cuda.h>


#include <cuda_runtime.h>


#include <algorithm>

#endif

/*
 *  SAC-Program matmul.sac :
 */


/*
 *  Global Definitions
 */

SAC_PF_DEFINE()
SAC_HM_DEFINE()


/*
 *  prototypes for locals (FUNDEFS)
 */

SAC_C_EXTERN 
/*
 * ND_FUN_DECL( SACwf__MAIN_CL_ST__main, , 1, out, int, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
 */
SAC_ND_DECL_FUN2( SACwf__MAIN_CL_ST__main, void,  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), int));

SAC_C_EXTERN 
/*
 * MT_MTFUN_DECL( SACf__MAIN_CL_MT___PL__d__d, , 3, out, double, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
void SACf__MAIN_CL_MT___PL__d__d( SAC_MT_MYTHREAD_PARAM(),  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double));

SAC_C_EXTERN 
/*
 * ND_FUN_DECL( SACf__MAIN_CL_ST__main, , 1, out, int, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
 */
SAC_ND_DECL_FUN2( SACf__MAIN_CL_ST__main, void,  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), int));

SAC_C_EXTERN 
/*
 * MT_SPMDFUN_DECL( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 2, inout, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMDFUN_REAL_RETTYPE() SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d( SAC_MT_SPMDFUN_REAL_PARAM_LIST())
;

SAC_C_EXTERN 
/*
 * MT_SPMDFUN_DECL( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 2, inout, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMDFUN_REAL_RETTYPE() SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d( SAC_MT_SPMDFUN_REAL_PARAM_LIST())
;

SAC_C_EXTERN 
/*
 * ND_FUN_DECL( SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550, , 3, out, double, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_ND_DECL_FUN2( SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550, void,  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double));

SAC_C_EXTERN 
/*
 * MT_SPMDFUN_DECL( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 8, inout, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMDFUN_REAL_RETTYPE() SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d( SAC_MT_SPMDFUN_REAL_PARAM_LIST())
;



/*
 *  SPMD infrastructure
 */

SAC_MT_SPMD_FRAME_BEGIN()
/*
 * MT_SPMD_FRAME_ELEMENT( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 2, inout, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMD_FRAME_ELEMENT_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d)
SAC_MT_FRAME_ELEMENT_inout( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 0, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 1, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_SPMD_FRAME_ELEMENT_END( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d)

/*
 * MT_SPMD_FRAME_ELEMENT( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 2, inout, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMD_FRAME_ELEMENT_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d)
SAC_MT_FRAME_ELEMENT_inout( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 0, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 1, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_SPMD_FRAME_ELEMENT_END( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d)

/*
 * MT_SPMD_FRAME_ELEMENT( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 8, inout, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMD_FRAME_ELEMENT_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d)
SAC_MT_FRAME_ELEMENT_inout( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 1, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 2, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 3, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 4, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 5, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 6, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_FRAME_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 7, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_SPMD_FRAME_ELEMENT_END( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d)

SAC_MT_SPMD_FRAME_END()
SAC_MT_SPMD_BARRIER_BEGIN()
/*
 * MT_SPMD_BARRIER_ELEMENT( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 2, inout, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMD_BARRIER_ELEMENT_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d)
SAC_MT_BARRIER_ELEMENT_inout( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 0, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 1, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_SPMD_BARRIER_ELEMENT_END( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d)

/*
 * MT_SPMD_BARRIER_ELEMENT( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 2, inout, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMD_BARRIER_ELEMENT_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d)
SAC_MT_BARRIER_ELEMENT_inout( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 0, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 1, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_SPMD_BARRIER_ELEMENT_END( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d)

/*
 * MT_SPMD_BARRIER_ELEMENT( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 8, inout, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMD_BARRIER_ELEMENT_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d)
SAC_MT_BARRIER_ELEMENT_inout( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 1, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 2, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 3, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 4, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 5, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 6, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
SAC_MT_BARRIER_ELEMENT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 7, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
SAC_MT_SPMD_BARRIER_ELEMENT_END( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d)

SAC_MT_SPMD_BARRIER_END()


/*
 *  function definitions (FUNDEFS)
 */



/****************************************************************************
 * Wrapper function:
 * ST function:
 * _MAIN:_ST::SACwf__MAIN_CL_ST__main(...) [ wrapper ]
 ****************************************************************************/
/*
 * ND_FUN_DEF_BEGIN( SACwf__MAIN_CL_ST__main, , 1, out, int, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
 */
SAC_ND_DEF_FUN_BEGIN2( SACwf__MAIN_CL_ST__main, void,  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), int))
{
  SAC_HM_DEFINE_THREAD_STATUS( SAC_HM_single_threaded)
  SAC_MT_DEFINE_ST_SELF()

  { 
    /*
     * ND_DECL( (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()


    SAC_INIT_LOCAL_MEM()
    /*
     * ND_FUN_AP( SACf__MAIN_CL_ST__main, , 1, out, int, SAC_SET_NT_USG( FAG, (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
     */
    SAC_ND_FUNAP2( SACf__MAIN_CL_ST__main,  SAC_ND_ARG_out( SAC_SET_NT_USG( FAG, (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))), int))

    /*
     * ND_REFRESH__MIRROR( (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_NOOP()

    /*
     * ND_FUN_RET( , 1, out, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
     */
    SAC_ND_RET_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), (SACp_cwc_246, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
    return;
    SAC_CLEANUP_LOCAL_MEM()
  }
/*
   * ND_FUN_DEF_END( SACwf__MAIN_CL_ST__main, , 1, out, int, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
   */
}
SAC_ND_FUN_DEF_END2()



/****************************************************************************
 * MT function:
 * _MAIN:_MT::SACf__MAIN_CL_MT___PL__d__d(...) [ body ]
 ****************************************************************************/
/*
 * MT_MTFUN_DEF_BEGIN( SACf__MAIN_CL_MT___PL__d__d, , 3, out, double, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
void SACf__MAIN_CL_MT___PL__d__d( SAC_MT_MYTHREAD_PARAM(),  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double))
{
  SAC_HM_DEFINE_THREAD_STATUS( SAC_HM_multi_threaded)

  { 
    /*
     * ND_DECL( (SACp_emal_1233__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1233__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1233__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()


    /*
     * ND_DECL__MIRROR_PARAM( (SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
     */
    SAC_NOTHING()

    /*
     * ND_DECL__MIRROR_PARAM( (SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
     */
    SAC_NOTHING()

    SAC_INIT_LOCAL_MEM()
    SAC_ND_PRF_SxS__DATA((SACp_emal_1233__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), T_double, SAC_ND_PRF_ADD, SAC_ND_READ((SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), 0), SAC_ND_READ((SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), 0))
    SAC_ND_DEC_RC_FREE((SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), 1, )
    SAC_ND_DEC_RC_FREE((SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), 1, )
    /*
     * MT_MTFUN_RET( , 1, out, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), (SACp_emal_1233__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
     */
    SAC_ND_RET_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), (SACp_emal_1233__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
    return;
    SAC_CLEANUP_LOCAL_MEM()
  }
/*
   * MT_MTFUN_DEF_END( SACf__MAIN_CL_MT___PL__d__d, , 3, out, double, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_b, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
   */
}



/****************************************************************************
 * ST function:
 * _MAIN:_ST::SACf__MAIN_CL_ST__main(...) [ body ]
 ****************************************************************************/
/*
 * ND_FUN_DEF_BEGIN( SACf__MAIN_CL_ST__main, , 1, out, int, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
 */
SAC_ND_DEF_FUN_BEGIN2( SACf__MAIN_CL_ST__main, void,  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), int))
{
  SAC_HM_DEFINE_THREAD_STATUS( SAC_HM_single_threaded)
  SAC_MT_DEFINE_ST_SELF()

  { 
    /*
     * ND_DECL( (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_1287__emal_1245_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1287__emal_1245_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_1287__emal_1245_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_1285__emal_1243__flat_233, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (BOO, )))))))))), bool, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1285__emal_1243__flat_233, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (BOO, )))))))))), bool, )
    SAC_ND_DECL__DESC( (SACp_pinl_1285__emal_1243__flat_233, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (BOO, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 2, 550, 550)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 2, 550, 550)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL( (SACp_emal_1234__flat_235, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1234__flat_235, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_emal_1234__flat_235, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()


    SAC_INIT_LOCAL_MEM()
    SAC_ND_ALLOC_BEGIN((SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 28, (SAC_ND_A_DIM( (SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_ND_CREATE__SCALAR__DATA((SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
    SAC_ND_ALLOC_BEGIN((SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
    /*
     * ND_SET__SHAPE_arr( (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 9, (SAC_ND_A_DIM( (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
    SAC_ND_CREATE__SCALAR__DATA((SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0.0)
    SAC_ND_ALLOC_BEGIN((SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 2, double)
    /*
     * ND_SET__SHAPE_arr( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 5, (SAC_ND_A_DIM( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (2)), "Assignment with incompatible types found");
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 5, (SAC_ND_A_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) == 550), "Assignment with incompatible types found");
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 5, (SAC_ND_A_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) == 550), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 2, double)
    /*
     * MT_SPMDFUN_AP( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 2, inout, double, SAC_SET_NT_USG( FAG, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), in, double, SAC_SET_NT_USG( FAG, (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
     */
    SAC_MT_BEGIN_SPMD_INVOCATION( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d);
    SAC_MT_SEND_PARAM_inout( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 0, SAC_SET_NT_USG( FAG, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 1, SAC_SET_NT_USG( FAG, (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SPMD_EXECUTE( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d);
    SAC_MT_RECEIVE_RESULT_inout( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 0, 0, SAC_SET_NT_USG( FAG, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 0, 1, SAC_SET_NT_USG( FAG, (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_END_SPMD_INVOCATION( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d);

    /*
     * ND_REFRESH__MIRROR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2)
     */
    SAC_NOOP()

    SAC_ND_ALLOC_BEGIN((SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 2, double)
    /*
     * ND_SET__SHAPE_arr( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 5, (SAC_ND_A_DIM( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (2)), "Assignment with incompatible types found");
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 5, (SAC_ND_A_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) == 550), "Assignment with incompatible types found");
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 5, (SAC_ND_A_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) == 550), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 2, double)
    /*
     * MT_SPMDFUN_AP( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 2, inout, double, SAC_SET_NT_USG( FAG, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), in, double, SAC_SET_NT_USG( FAG, (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
     */
    SAC_MT_BEGIN_SPMD_INVOCATION( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d);
    SAC_MT_SEND_PARAM_inout( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 0, SAC_SET_NT_USG( FAG, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 1, SAC_SET_NT_USG( FAG, (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SPMD_EXECUTE( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d);
    SAC_MT_RECEIVE_RESULT_inout( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 0, 0, SAC_SET_NT_USG( FAG, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 0, 1, SAC_SET_NT_USG( FAG, (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_END_SPMD_INVOCATION( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d);

    /*
     * ND_REFRESH__MIRROR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2)
     */
    SAC_NOOP()

    SAC_NOOP()
    SAC_NOOP()
    /*
     * ND_ASSIGN( (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, (SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, )
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 41, 5, (SAC_ND_A_DIM( (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()
    SAC_NOOP()
    SAC_NOOP()
    SAC_ND_ASSIGN__DATA( (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1242__flat_187, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )

    /*
     * ND_ASSIGN( (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, )
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 41, 5, (SAC_ND_A_DIM( (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()
    SAC_NOOP()
    SAC_NOOP()
    SAC_ND_ASSIGN__DATA( (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), (SACp_emal_1241__mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )

    /*
     * ND_ASSIGN( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, )
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 41, 5, (SAC_ND_A_DIM( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()
    SAC_NOOP()
    SAC_NOOP()
    SAC_ND_ASSIGN__DATA( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_1291_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )

    /*
     * ND_ASSIGN( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, )
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 41, 5, (SAC_ND_A_DIM( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()
    SAC_NOOP()
    SAC_NOOP()
    SAC_ND_ASSIGN__DATA( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), (SACp_pinl_1290_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )

    SAC_ND_GOTO(_dup_1292__f2l_1278_label)
    do 
    { 
      SAC_ND_FREE((SACp_pinl_1285__emal_1243__flat_233, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (BOO, )))))))))), )
      /*
       * ND_ASSIGN( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, (SACp_pinl_1287__emal_1245_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, )
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 41, 5, (SAC_ND_A_DIM( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()
      SAC_NOOP()
      SAC_NOOP()
      SAC_ND_ASSIGN__DATA( (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_1287__emal_1245_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )

      /*
       * ND_ASSIGN( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, )
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 41, 5, (SAC_ND_A_DIM( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()
      SAC_NOOP()
      SAC_NOOP()
      SAC_ND_ASSIGN__DATA( (SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )

      SAC_ND_LABEL(_dup_1292__f2l_1278_label)
      SAC_ND_INC_RC((SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1)
      SAC_ND_INC_RC((SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1)
      SAC_ND_INC_RC((SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1)
      /*
       * ND_FUN_AP( SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550, , 3, out, double, SAC_SET_NT_USG( FAG, (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), in, double, SAC_SET_NT_USG( FAG, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), in, double, SAC_SET_NT_USG( FAG, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
       */
      SAC_ND_FUNAP2( SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550,  SAC_ND_ARG_out( SAC_SET_NT_USG( FAG, (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), double), SAC_ND_ARG_in( SAC_SET_NT_USG( FAG, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), double), SAC_ND_ARG_in( SAC_SET_NT_USG( FAG, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), double))

      /*
       * ND_REFRESH__MIRROR( (SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
       */
      SAC_NOOP()

      SAC_ND_PRF_SxS__DATA((SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), T_double, SAC_ND_PRF_ADD, SAC_ND_READ((SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0), SAC_ND_READ((SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0))
      SAC_ND_FREE((SACp_pinl_1289__f2l_1277_res, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
      SAC_ND_PRF_SxS__DATA((SACp_pinl_1287__emal_1245_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), T_int, SAC_ND_PRF_ADD, 1, SAC_ND_READ((SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0))
      /*
       * ND_ASSIGN( (SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, )
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 41, 17, (SAC_ND_A_DIM( (SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()
      SAC_NOOP()
      SAC_NOOP()
      SAC_ND_ASSIGN__DATA( (SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )

      SAC_ND_PRF_SxS__DATA((SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), T_int, SAC_ND_PRF_ADD, -19, SAC_ND_READ((SACp_pinl_1288__f2l_1276_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0))
      SAC_ND_PRF_SxS__DATA((SACp_pinl_1285__emal_1243__flat_233, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (BOO, )))))))))), T_int, SAC_ND_PRF_LE, SAC_ND_READ((SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), 0)
      SAC_ND_FREE((SACp_pinl_1286__emal_1244__al_1022, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    }
    while (SAC_ND_GETVAR((SACp_pinl_1285__emal_1243__flat_233, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (BOO, )))))))))), SACp_pinl_1285__emal_1243__flat_233));
    SAC_ND_FREE((SACp_pinl_1287__emal_1245_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_DEC_RC_FREE((SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, )
    SAC_ND_DEC_RC_FREE((SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, )
    SAC_ND_FREE((SACp_pinl_1285__emal_1243__flat_233, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (BOO, )))))))))), )
    SAC_ND_PRF_S__DATA((SACp_emal_1234__flat_235, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), SAC_ND_PRF_TOI, SAC_ND_READ((SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0))
    SAC_ND_FREE((SACp_pinl_1282__flat_231, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    /*
     * ND_FUN_RET( , 1, out, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1234__flat_235, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
     */
    SAC_ND_RET_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1234__flat_235, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
    return;
    SAC_CLEANUP_LOCAL_MEM()
  }
/*
   * ND_FUN_DEF_END( SACf__MAIN_CL_ST__main, , 1, out, int, (SAC_arg_1, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
   */
}
SAC_ND_FUN_DEF_END2()



/****************************************************************************
 * SPMD function:
 * _MAIN:_ST::SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d(...) [ body ]
 ****************************************************************************/
/*
 * MT_SPMDFUN_DEF_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 2, inout, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMDFUN_REAL_RETTYPE() SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d( SAC_MT_SPMDFUN_REAL_PARAM_LIST())
{
  SAC_HM_DEFINE_THREAD_STATUS( SAC_HM_multi_threaded)
  SAC_MT_RECEIVE_PARAM_inout( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 0, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 1, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))

  { 
    /* MT parallel branch */
    /*
     * ND_DECL( (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 2, 550, 550)
     */
    SAC_ND_DECL__DATA( (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 1, 2)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = 2;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 2;
    const int SAC_ND_A_MIRROR_DIM( (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;


    /*
     * ND_DECL__MIRROR_PARAM( (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
     */
    SAC_NOTHING()

    SAC_ND_DECL_PARAM_inout((SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double)
    /*
     * ND_DECL__MIRROR_PARAM( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    SAC_INIT_LOCAL_MEM()
    /*
     * MT_SCHEDULER_Block_INIT( 0, 2, 0, 0, 550, 550, 1, 1)
     */

    SAC_ND_ALLOC_BEGIN((SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 9, (SAC_ND_A_DIM( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_ND_ALLOC_BEGIN((SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 9, (SAC_ND_A_DIM( (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_ND_ALLOC_BEGIN((SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 9, (SAC_ND_A_DIM( (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_PF_BEGIN_WITH(genarray)
    /*
     * WL_SCHEDULE__BEGIN( 2)
     */
    {
      int SAC_WL_MT_SCHEDULE_START( 0);
      int SAC_WL_MT_SCHEDULE_STOP( 0);
      int SAC_WL_MT_SCHEDULE_START( 1);
      int SAC_WL_MT_SCHEDULE_STOP( 1);

      /*
       * WL_DECLARE_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      int SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0);
      int SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      /*
       * WL_DEFINE_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      {
        int SAC_i;
        SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 1 * SAC_ND_A_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);
        SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 1;
      }

      /*
       * MT_SCHEDULER_Block_BEGIN( 0, 2, 0, 0, 550, 550, 1, 1)
       */
      SAC_MT_SCHEDULER_Block_DIM0( 0, 550, 1);
      SAC_WL_MT_SCHEDULE_START( 1) = 0;
      SAC_WL_MT_SCHEDULE_STOP( 1) = 550;

      /*
       * WL_INIT_OFFSET( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      SAC_ND_WRITE( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
        = SAC_WL_MT_SCHEDULE_START( 0) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
        + SAC_WL_MT_SCHEDULE_START( 1) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      SAC_WL_MT_STRIDE_LOOP0_BEGIN(0, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_BEGIN(0, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      /*
       * WL_SET_OFFSET( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 2, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
       */
      SAC_ND_WRITE( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) 
        = ( SAC_ND_A_SHAPE( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) * SAC_ND_READ( (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
         + SAC_WL_MT_SCHEDULE_START( 1) ) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      SAC_WL_MT_STRIDE_LOOP0_BEGIN(1, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_BEGIN(1, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_ND_ALLOC_BEGIN((SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
      /*
       * ND_SET__SHAPE_arr( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 9, (SAC_ND_A_DIM( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
      SAC_ND_PRF_S__DATA((SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_PRF_TOD, SAC_ND_READ((SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0))
      /*
       * WL_ASSIGN( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 9, (SAC_ND_A_DIM( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (SAC_ND_A_DIM( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) - SAC_ND_A_SIZE( (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))), "WL expression with illegal dimension found!");
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 37, 9, (SAC_ND_A_SIZE( (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == SAC_WL_SHAPE_FACTOR( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1)), "WL expression with illegal size found!");
      SAC_ND_WRITE_READ_COPY( (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, );

      SAC_ND_FREE((SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
      SAC_WL_INC_OFFSET((SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1236__dlirmov_1224__dup_963__mose_9__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
      SAC_WL_MT_GRID_UNROLL_END(1, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_WL_MT_STRIDE_LOOP_END(1, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_END(0, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_WL_MT_STRIDE_LOOP_END(0, (SACp_dlirmov_1220__dup_960__flat_207, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      /*
       * MT_SCHEDULER_Block_END( 0, 2, 0, 0, 550, 550, 1, 1)
       */


      /*
       * WL_SCHEDULE__END( 2)
       */
    }

    SAC_PF_END_WITH(genarray)
    SAC_ND_LABEL(_comp_1299_SAC_label)
    SAC_ND_FREE((SACp_dlirmov_1223__wlidx_1173__dup_959_a, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_dlirmov_1222__dup_962_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_dlirmov_1221__dup_961_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    /*
     * MT_SPMDFUN_RET( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 1, inout, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, ND, NONE)
     */
    SAC_MT_SYNC_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d);
      SAC_MT_SYNC_FOLD_inout( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 0, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), (SACp_mtspmdfanon_1295__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, ND, NONE);
    SAC_MT_SYNC_CONT( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d);
      SAC_MT_SEND_RESULT_inout( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, SAC_MT_SELF_LOCAL_ID(), 0, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))));
    SAC_MT_SYNC_END( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d);
    SAC_MT_SPMDFUN_REAL_RETURN();

    SAC_CLEANUP_LOCAL_MEM()
  }
/*
   * MT_SPMDFUN_DEF_END( SACf__MAIN_CL_ST___mtspmdf_1296_main__d_550_550__d, 2, inout, double, (SACp_emal_1235__dlirmov_1225__dup_959_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
   */
}



/****************************************************************************
 * SPMD function:
 * _MAIN:_ST::SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d(...) [ body ]
 ****************************************************************************/
/*
 * MT_SPMDFUN_DEF_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 2, inout, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMDFUN_REAL_RETTYPE() SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d( SAC_MT_SPMDFUN_REAL_PARAM_LIST())
{
  SAC_HM_DEFINE_THREAD_STATUS( SAC_HM_multi_threaded)
  SAC_MT_RECEIVE_PARAM_inout( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 0, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 1, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))

  { 
    /* MT parallel branch */
    /*
     * ND_DECL( (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 2, 550, 550)
     */
    SAC_ND_DECL__DATA( (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 1, 2)
     */
    SAC_ND_DECL__DATA( (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = 2;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 2;
    const int SAC_ND_A_MIRROR_DIM( (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;


    /*
     * ND_DECL__MIRROR_PARAM( (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
     */
    SAC_NOTHING()

    SAC_ND_DECL_PARAM_inout((SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double)
    /*
     * ND_DECL__MIRROR_PARAM( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    SAC_INIT_LOCAL_MEM()
    /*
     * MT_SCHEDULER_Block_INIT( 0, 2, 0, 0, 550, 550, 1, 1)
     */

    SAC_ND_ALLOC_BEGIN((SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 9, (SAC_ND_A_DIM( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_ND_ALLOC_BEGIN((SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 9, (SAC_ND_A_DIM( (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_ND_ALLOC_BEGIN((SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 9, (SAC_ND_A_DIM( (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_PF_BEGIN_WITH(genarray)
    /*
     * WL_SCHEDULE__BEGIN( 2)
     */
    {
      int SAC_WL_MT_SCHEDULE_START( 0);
      int SAC_WL_MT_SCHEDULE_STOP( 0);
      int SAC_WL_MT_SCHEDULE_START( 1);
      int SAC_WL_MT_SCHEDULE_STOP( 1);

      /*
       * WL_DECLARE_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      int SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0);
      int SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      /*
       * WL_DEFINE_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      {
        int SAC_i;
        SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 1 * SAC_ND_A_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);
        SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 1;
      }

      /*
       * MT_SCHEDULER_Block_BEGIN( 0, 2, 0, 0, 550, 550, 1, 1)
       */
      SAC_MT_SCHEDULER_Block_DIM0( 0, 550, 1);
      SAC_WL_MT_SCHEDULE_START( 1) = 0;
      SAC_WL_MT_SCHEDULE_STOP( 1) = 550;

      /*
       * WL_INIT_OFFSET( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      SAC_ND_WRITE( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
        = SAC_WL_MT_SCHEDULE_START( 0) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
        + SAC_WL_MT_SCHEDULE_START( 1) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      SAC_WL_MT_STRIDE_LOOP0_BEGIN(0, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_BEGIN(0, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      /*
       * WL_SET_OFFSET( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 2, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
       */
      SAC_ND_WRITE( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) 
        = ( SAC_ND_A_SHAPE( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) * SAC_ND_READ( (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
         + SAC_WL_MT_SCHEDULE_START( 1) ) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      SAC_WL_MT_STRIDE_LOOP0_BEGIN(1, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_BEGIN(1, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_ND_ALLOC_BEGIN((SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
      /*
       * ND_SET__SHAPE_arr( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 9, (SAC_ND_A_DIM( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
      SAC_ND_PRF_S__DATA((SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_PRF_TOD, SAC_ND_READ((SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0))
      /*
       * WL_ASSIGN( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 9, (SAC_ND_A_DIM( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (SAC_ND_A_DIM( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) - SAC_ND_A_SIZE( (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))), "WL expression with illegal dimension found!");
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 38, 9, (SAC_ND_A_SIZE( (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == SAC_WL_SHAPE_FACTOR( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1)), "WL expression with illegal size found!");
      SAC_ND_WRITE_READ_COPY( (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, );

      SAC_ND_FREE((SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
      SAC_WL_INC_OFFSET((SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1239__dlirmov_1216__dup_971__mose_10__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
      SAC_WL_MT_GRID_UNROLL_END(1, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_WL_MT_STRIDE_LOOP_END(1, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_END(0, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_WL_MT_STRIDE_LOOP_END(0, (SACp_dlirmov_1212__dup_968__flat_228, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      /*
       * MT_SCHEDULER_Block_END( 0, 2, 0, 0, 550, 550, 1, 1)
       */


      /*
       * WL_SCHEDULE__END( 2)
       */
    }

    SAC_PF_END_WITH(genarray)
    SAC_ND_LABEL(_comp_1300_SAC_label)
    SAC_ND_FREE((SACp_dlirmov_1215__wlidx_1172__dup_967_b, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_dlirmov_1214__dup_970_j__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_dlirmov_1213__dup_969_i__SSA0_1, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    /*
     * MT_SPMDFUN_RET( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 1, inout, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, ND, NONE)
     */
    SAC_MT_SYNC_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d);
      SAC_MT_SYNC_FOLD_inout( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 0, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), (SACp_mtspmdfanon_1293__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, ND, NONE);
    SAC_MT_SYNC_CONT( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d);
      SAC_MT_SEND_RESULT_inout( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, SAC_MT_SELF_LOCAL_ID(), 0, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))));
    SAC_MT_SYNC_END( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d);
    SAC_MT_SPMDFUN_REAL_RETURN();

    SAC_CLEANUP_LOCAL_MEM()
  }
/*
   * MT_SPMDFUN_DEF_END( SACf__MAIN_CL_ST___mtspmdf_1294_main__d_550_550__d, 2, inout, double, (SACp_emal_1238__dlirmov_1217__dup_967_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACp_mose_9, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
   */
}



/****************************************************************************
 * ST function:
 * _MAIN:_ST::SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550(...) [ body ]
 ****************************************************************************/
/*
 * ND_FUN_DEF_BEGIN( SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550, , 3, out, double, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_ND_DEF_FUN_BEGIN2( SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550, void,  SAC_ND_PARAM_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double), SAC_ND_PARAM_in( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), double))
{
  SAC_HM_DEFINE_THREAD_STATUS( SAC_HM_single_threaded)
  SAC_MT_DEFINE_ST_SELF()

  { 
    SAC_ND_DECL_CONST__DATA((SACp_emal_1271__isaa_1100_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
    SAC_ND_DECL_CONST__DATA((SACp_emal_1270__pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0.0)
    SAC_ND_DECL_CONST__DATA((SACp_emal_1269__iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
    SAC_ND_DECL_CONST__DATA((SACp_emal_1268__iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
    SAC_ND_DECL_CONST__DATA((SACp_emal_1267__iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
    SAC_ND_DECL_CONST__DATA((SACp_emal_1266__iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
    /*
     * ND_DECL( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 2, 550, 550)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL( (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()


    /*
     * ND_DECL__MIRROR_PARAM( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL__MIRROR_PARAM( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    SAC_INIT_LOCAL_MEM()
    SAC_ND_ALLOC_BEGIN((SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 2, double)
    /*
     * ND_SET__SHAPE_arr( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_DIM( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (2)), "Assignment with incompatible types found");
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) == 550), "Assignment with incompatible types found");
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) == 550), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 2, double)
    /*
     * MT_SPMDFUN_AP( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 8, inout, double, SAC_SET_NT_USG( FAG, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))), in, double, SAC_SET_NT_USG( FPA, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, ))))))))))), in, int, SAC_SET_NT_USG( FAG, (SACp_emal_1266__iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))), in, int, SAC_SET_NT_USG( FAG, (SACp_emal_1267__iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))), in, double, SAC_SET_NT_USG( FPA, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, ))))))))))), in, int, SAC_SET_NT_USG( FAG, (SACp_emal_1268__iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))), in, int, SAC_SET_NT_USG( FAG, (SACp_emal_1269__iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))), in, double, SAC_SET_NT_USG( FAG, (SACp_emal_1270__pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
     */
    SAC_MT_BEGIN_SPMD_INVOCATION( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d);
    SAC_MT_SEND_PARAM_inout( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, SAC_SET_NT_USG( FAG, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 1, SAC_SET_NT_USG( FPA, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 2, SAC_SET_NT_USG( FAG, (SACp_emal_1266__iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 3, SAC_SET_NT_USG( FAG, (SACp_emal_1267__iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 4, SAC_SET_NT_USG( FPA, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 5, SAC_SET_NT_USG( FAG, (SACp_emal_1268__iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 6, SAC_SET_NT_USG( FAG, (SACp_emal_1269__iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_SEND_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 7, SAC_SET_NT_USG( FAG, (SACp_emal_1270__pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_SPMD_EXECUTE( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d);
    SAC_MT_RECEIVE_RESULT_inout( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 0, SAC_SET_NT_USG( FAG, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 1, SAC_SET_NT_USG( FPA, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 2, SAC_SET_NT_USG( FAG, (SACp_emal_1266__iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 3, SAC_SET_NT_USG( FAG, (SACp_emal_1267__iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 4, SAC_SET_NT_USG( FPA, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 5, SAC_SET_NT_USG( FAG, (SACp_emal_1268__iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 6, SAC_SET_NT_USG( FAG, (SACp_emal_1269__iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))
    SAC_MT_RECEIVE_RESULT_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, 7, SAC_SET_NT_USG( FAG, (SACp_emal_1270__pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))))
    SAC_MT_END_SPMD_INVOCATION( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d);

    /*
     * ND_REFRESH__MIRROR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2)
     */
    SAC_NOOP()

    SAC_ND_FREE((SACp_emal_1270__pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_ND_FREE((SACp_emal_1269__iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_emal_1268__iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_emal_1267__iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_emal_1266__iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_DEC_RC_FREE((SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), 1, )
    SAC_ND_DEC_RC_FREE((SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), 1, )
    /*
     * ND_PRF_IDX_SEL__DATA( (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_emal_1271__isaa_1100_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
     */
    SAC_TR_PRF_PRINT( ("ND_PRF_IDX_SEL__DATA( (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_emal_1271__isaa_1100_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))"))
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1271__isaa_1100_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "1st argument of _idx_sel_ is not a scalar!");
    SAC_ND_WRITE_READ_COPY( (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_emal_1271__isaa_1100_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), )

    SAC_ND_FREE((SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_ND_FREE((SACp_emal_1271__isaa_1100_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    /*
     * ND_FUN_RET( , 1, out, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
     */
    SAC_ND_RET_out( (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), (SACp_emal_1247__pinl_950__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
    return;
    SAC_CLEANUP_LOCAL_MEM()
  }
/*
   * ND_FUN_DEF_END( SACf__MAIN_CL_ST__matmul___i__d_550_550__d_550_550, , 3, out, double, (SAC_arg_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
   */
}
SAC_ND_FUN_DEF_END2()



/****************************************************************************
 * SPMD function:
 * _MAIN:_ST::SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d(...) [ body ]
 ****************************************************************************/
/*
 * MT_SPMDFUN_DEF_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 8, inout, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
 */
SAC_MT_SPMDFUN_REAL_RETTYPE() SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d( SAC_MT_SPMDFUN_REAL_PARAM_LIST())
{
  SAC_HM_DEFINE_THREAD_STATUS( SAC_HM_multi_threaded)
  SAC_MT_RECEIVE_PARAM_inout( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 1, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 2, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 3, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 4, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 5, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 6, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))))
  SAC_MT_RECEIVE_PARAM_in( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 7, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))

  { 
    /* MT parallel branch */
    /*
     * ND_DECL( (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 2, 550, 550)
     */
    SAC_ND_DECL__DATA( (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 1, 1)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = 1;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;
    const int SAC_ND_A_MIRROR_DIM( (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;

    /*
     * ND_DECL( (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 1, 1)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = 1;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;
    const int SAC_ND_A_MIRROR_DIM( (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;

    /*
     * ND_DECL( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 1, 550)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 550;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 1;

    /*
     * ND_DECL( (SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 1, 1)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = 1;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;
    const int SAC_ND_A_MIRROR_DIM( (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;

    /*
     * ND_DECL( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, 1, 550)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, )
    SAC_ND_DECL__DESC( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 550;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 1;

    /*
     * ND_DECL( (SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 0)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_NOTHING()

    /*
     * ND_DECL( (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, 1, 2)
     */
    SAC_ND_DECL__DATA( (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
    SAC_ND_DECL__DESC( (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = 2;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 2;
    const int SAC_ND_A_MIRROR_DIM( (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) = 1;


    /*
     * ND_DECL__MIRROR_PARAM( (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
     */
    SAC_NOTHING()

    /*
     * ND_DECL__MIRROR_PARAM( (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_NOTHING()

    /*
     * ND_DECL__MIRROR_PARAM( (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_NOTHING()

    /*
     * ND_DECL__MIRROR_PARAM( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    /*
     * ND_DECL__MIRROR_PARAM( (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_NOTHING()

    /*
     * ND_DECL__MIRROR_PARAM( (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_NOTHING()

    /*
     * ND_DECL__MIRROR_PARAM( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    SAC_ND_DECL_PARAM_inout((SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double)
    /*
     * ND_DECL__MIRROR_PARAM( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, 550, 550)
     */
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 550;
    const int SAC_ND_A_MIRROR_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 550;
    const int SAC_ND_A_MIRROR_SIZE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 302500;
    const int SAC_ND_A_MIRROR_DIM( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) = 2;

    SAC_INIT_LOCAL_MEM()
    /*
     * MT_SCHEDULER_Block_INIT( 0, 2, 0, 0, 550, 550, 1, 1)
     */

    SAC_ND_ALLOC_BEGIN((SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_DIM( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_ND_ALLOC_BEGIN((SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_DIM( (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_ND_ALLOC_BEGIN((SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    /*
     * ND_SET__SHAPE_arr( (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
     */
    SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_DIM( (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
    SAC_NOOP()

    SAC_ND_ALLOC_END((SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
    SAC_PF_BEGIN_WITH(genarray)
    /*
     * WL_SCHEDULE__BEGIN( 2)
     */
    {
      int SAC_WL_MT_SCHEDULE_START( 0);
      int SAC_WL_MT_SCHEDULE_STOP( 0);
      int SAC_WL_MT_SCHEDULE_START( 1);
      int SAC_WL_MT_SCHEDULE_STOP( 1);

      /*
       * WL_DECLARE_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      int SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0);
      int SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      /*
       * WL_DEFINE_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      {
        int SAC_i;
        SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 1 * SAC_ND_A_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);
        SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) = 1;
      }

      /*
       * MT_SCHEDULER_Block_BEGIN( 0, 2, 0, 0, 550, 550, 1, 1)
       */
      SAC_MT_SCHEDULER_Block_DIM0( 0, 550, 1);
      SAC_WL_MT_SCHEDULE_START( 1) = 0;
      SAC_WL_MT_SCHEDULE_STOP( 1) = 550;

      /*
       * WL_INIT_OFFSET( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2)
       */
      SAC_ND_WRITE( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
        = SAC_WL_MT_SCHEDULE_START( 0) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
        + SAC_WL_MT_SCHEDULE_START( 1) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      SAC_WL_MT_STRIDE_LOOP0_BEGIN(0, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_BEGIN(0, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      /*
       * WL_SET_OFFSET( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 2, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
       */
      SAC_ND_WRITE( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) 
        = ( SAC_ND_A_SHAPE( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1) * SAC_ND_READ( (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
         + SAC_WL_MT_SCHEDULE_START( 1) ) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1);

      SAC_WL_MT_STRIDE_LOOP0_BEGIN(1, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_BEGIN(1, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_ND_INC_RC((SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), 1)
      SAC_ND_ALLOC_BEGIN((SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_SET__SHAPE_arr( (SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_IDXS2OFFSET_arr( (SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, 550, 550)
       */
      SAC_ND_WRITE( (SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = ( 550 * SAC_ND_READ( (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), 0) + SAC_ND_READ( (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) );

      SAC_ND_ALLOC_BEGIN((SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 1, double)
      /*
       * ND_SET__SHAPE_arr( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 550)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 45, (SAC_ND_A_DIM( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (1)), "Assignment with incompatible types found");
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 45, (SAC_ND_A_SHAPE( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) == 550), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 1, double)
      SAC_ND_ALLOC_BEGIN((SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_SET__SHAPE_arr( (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 45, (SAC_ND_A_DIM( (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      SAC_ND_ALLOC_BEGIN((SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_SET__SHAPE_arr( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 45, (SAC_ND_A_DIM( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      SAC_PF_BEGIN_WITH(genarray)
      /*
       * WL_SCHEDULE__BEGIN( 1)
       */
      {
        int SAC_WL_MT_SCHEDULE_START( 0);
        int SAC_WL_MT_SCHEDULE_STOP( 0);

        /*
         * WL_DECLARE_SHAPE_FACTOR( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1)
         */
        int SAC_WL_SHAPE_FACTOR( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0);

        /*
         * WL_DEFINE_SHAPE_FACTOR( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1)
         */
        {
          int SAC_i;
          SAC_WL_SHAPE_FACTOR( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 1;
        }

        /*
         * MT_SCHEDULER_BEGIN( 0, 1, 0, 550)
         */
        SAC_WL_MT_SCHEDULE_START( 0) = 0;
        SAC_WL_MT_SCHEDULE_STOP( 0) = 550;

        /*
         * WL_INIT_OFFSET( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1)
         */
        SAC_ND_WRITE( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
          = SAC_WL_MT_SCHEDULE_START( 0) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0);

        SAC_WL_STRIDE_LOOP0_BEGIN(0, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
        SAC_WL_GRID_UNROLL_BEGIN(0, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
        SAC_ND_ALLOC_BEGIN((SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
        /*
         * ND_SET__SHAPE_arr( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()

        SAC_ND_ALLOC_END((SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
        /*
         * ND_IDXS2OFFSET_arr( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), 2, 550, 550)
         */
        SAC_ND_WRITE( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = ( 550 * SAC_ND_READ( (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) + SAC_ND_READ( (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), 0) );

        SAC_ND_PRF_SxS__DATA((SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), T_int, SAC_ND_PRF_ADD, SAC_ND_READ((SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), SAC_ND_READ((SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0))
        SAC_ND_ALLOC_BEGIN((SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
        /*
         * ND_SET__SHAPE_arr( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_DIM( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()

        SAC_ND_ALLOC_END((SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
        /*
         * ND_PRF_IDX_SEL__DATA( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
         */
        SAC_TR_PRF_PRINT( ("ND_PRF_IDX_SEL__DATA( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))"))
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "1st argument of _idx_sel_ is not a scalar!");
        SAC_ND_WRITE_READ_COPY( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), )

        SAC_ND_FREE((SACp_emal_1262__ivesli_1186, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
        /*
         * WL_ASSIGN( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 45, (SAC_ND_A_DIM( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (SAC_ND_A_DIM( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) - SAC_ND_A_SIZE( (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))), "WL expression with illegal dimension found!");
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 45, (SAC_ND_A_SIZE( (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == SAC_WL_SHAPE_FACTOR( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)), "WL expression with illegal size found!");
        SAC_ND_WRITE_READ_COPY( (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, );

        SAC_ND_FREE((SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
        SAC_WL_INC_OFFSET((SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1260__pinl_817__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
        SAC_WL_GRID_UNROLL_END(0, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
        SAC_WL_STRIDE_LOOP_END(0, (SACp_pinl_774__hsd_18_index, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
        /*
         * MT_SCHEDULER_END( 0, 1, 0, 550)
         */


        /*
         * WL_SCHEDULE__END( 1)
         */
      }

      SAC_PF_END_WITH(genarray)
      SAC_ND_LABEL(_comp_1301_SAC_label)
      SAC_ND_FREE((SACp_pinl_778__eat_243, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
      SAC_ND_FREE((SACp_wlidx_1175__pinl_761__flat_147, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
      SAC_ND_FREE((SACp_emal_1264__ivesli_1184, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
      SAC_ND_ALLOC_BEGIN((SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_SET__SHAPE_arr( (SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_IDXS2OFFSET_arr( (SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), 2, 550, 550)
       */
      SAC_ND_WRITE( (SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = ( 550 * SAC_ND_READ( (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) + SAC_ND_READ( (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), 0) );

      SAC_ND_ALLOC_BEGIN((SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 1, double)
      /*
       * ND_SET__SHAPE_arr( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 550)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_DIM( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (1)), "Assignment with incompatible types found");
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_SHAPE( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) == 550), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 1, double)
      SAC_ND_ALLOC_BEGIN((SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_SET__SHAPE_arr( (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_DIM( (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      SAC_ND_ALLOC_BEGIN((SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_SET__SHAPE_arr( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_DIM( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      SAC_PF_BEGIN_WITH(genarray)
      /*
       * WL_SCHEDULE__BEGIN( 1)
       */
      {
        int SAC_WL_MT_SCHEDULE_START( 0);
        int SAC_WL_MT_SCHEDULE_STOP( 0);

        /*
         * WL_DECLARE_SHAPE_FACTOR( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1)
         */
        int SAC_WL_SHAPE_FACTOR( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0);

        /*
         * WL_DEFINE_SHAPE_FACTOR( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1)
         */
        {
          int SAC_i;
          SAC_WL_SHAPE_FACTOR( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0) = 1;
        }

        /*
         * MT_SCHEDULER_BEGIN( 0, 1, 0, 550)
         */
        SAC_WL_MT_SCHEDULE_START( 0) = 0;
        SAC_WL_MT_SCHEDULE_STOP( 0) = 550;

        /*
         * WL_INIT_OFFSET( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1)
         */
        SAC_ND_WRITE( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
          = SAC_WL_MT_SCHEDULE_START( 0) * SAC_WL_SHAPE_FACTOR( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0);

        SAC_WL_STRIDE_LOOP0_BEGIN(0, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
        SAC_WL_GRID_UNROLL_BEGIN(0, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
        SAC_ND_ALLOC_BEGIN((SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
        /*
         * ND_SET__SHAPE_arr( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()

        SAC_ND_ALLOC_END((SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
        /*
         * ND_IDXS2OFFSET_arr( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, 550, 550)
         */
        SAC_ND_WRITE( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = ( 550 * SAC_ND_READ( (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), 0) + SAC_ND_READ( (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) );

        SAC_ND_PRF_SxS__DATA((SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), T_int, SAC_ND_PRF_ADD, SAC_ND_READ((SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), SAC_ND_READ((SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0))
        SAC_ND_ALLOC_BEGIN((SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
        /*
         * ND_SET__SHAPE_arr( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_DIM( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()

        SAC_ND_ALLOC_END((SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
        /*
         * ND_PRF_IDX_SEL__DATA( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
         */
        SAC_TR_PRF_PRINT( ("ND_PRF_IDX_SEL__DATA( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))"))
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "1st argument of _idx_sel_ is not a scalar!");
        SAC_ND_WRITE_READ_COPY( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), )

        SAC_ND_FREE((SACp_emal_1256__ivesli_1181, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
        /*
         * WL_ASSIGN( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_DIM( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (SAC_ND_A_DIM( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) - SAC_ND_A_SIZE( (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))), "WL expression with illegal dimension found!");
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_SIZE( (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == SAC_WL_SHAPE_FACTOR( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)), "WL expression with illegal size found!");
        SAC_ND_WRITE_READ_COPY( (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, );

        SAC_ND_FREE((SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
        SAC_WL_INC_OFFSET((SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_emal_1254__pinl_875__mose_7__SSA0_1, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
        SAC_WL_GRID_UNROLL_END(0, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
        SAC_WL_STRIDE_LOOP_END(0, (SACp_pinl_874_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
        /*
         * MT_SCHEDULER_END( 0, 1, 0, 550)
         */


        /*
         * WL_SCHEDULE__END( 1)
         */
      }

      SAC_PF_END_WITH(genarray)
      SAC_ND_LABEL(_comp_1302_SAC_label)
      SAC_ND_FREE((SACp_pinl_876__eat_241, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
      SAC_ND_FREE((SACp_wlidx_1176__pinl_759__flat_145, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
      SAC_ND_FREE((SACp_emal_1258__ivesli_1179, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
      SAC_ND_PRF_VxV__DATA((SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_PRF_MUL, (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), (SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
      SAC_ND_FREE((SACp_emal_1259__pinl_761__flat_147, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
      SAC_ND_ALLOC_BEGIN((SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_SET__SHAPE_arr( (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 18, 12, (SAC_ND_A_DIM( (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()

      SAC_ND_ALLOC_END((SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
      /*
       * ND_ASSIGN( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, )
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 20, 9, (SAC_ND_A_DIM( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
      SAC_NOOP()
      SAC_NOOP()
      SAC_NOOP()
      SAC_ND_ASSIGN__DATA( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )

      SAC_PF_BEGIN_WITH(fold)
      /*
       * WL_SCHEDULE__BEGIN( 1)
       */
      {
        int SAC_WL_MT_SCHEDULE_START( 0);
        int SAC_WL_MT_SCHEDULE_STOP( 0);

        /*
         * MT_SCHEDULER_BEGIN( 0, 1, 0, 550)
         */
        SAC_WL_MT_SCHEDULE_START( 0) = 0;
        SAC_WL_MT_SCHEDULE_STOP( 0) = 550;

        SAC_WL_STRIDE_LOOP0_BEGIN(0, (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
        SAC_WL_GRID_UNROLL_BEGIN(0, (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
        SAC_NOOP()
        SAC_ND_ALLOC_BEGIN((SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
        /*
         * ND_SET__SHAPE_arr( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()

        SAC_ND_ALLOC_END((SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 0, int)
        /*
         * ND_IDXS2OFFSET_arr( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, 550)
         */
        SAC_ND_WRITE( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0) = SAC_ND_READ( (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0);

        SAC_ND_ALLOC_BEGIN((SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
        /*
         * ND_SET__SHAPE_arr( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0)
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 12, (SAC_ND_A_DIM( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()

        SAC_ND_ALLOC_END((SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, 0, double)
        /*
         * ND_PRF_IDX_SEL__DATA( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
         */
        SAC_TR_PRF_PRINT( ("ND_PRF_IDX_SEL__DATA( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))"))
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 11, 20, (SAC_ND_A_DIM( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))) == (0)), "1st argument of _idx_sel_ is not a scalar!");
        SAC_ND_WRITE_READ_COPY( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), )

        SAC_ND_FREE((SACp_emal_1251__ivesli_1177, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
        SAC_ND_PRF_SxS__DATA((SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), T_double, SAC_ND_PRF_ADD, SAC_ND_READ((SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0), SAC_ND_READ((SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0))
        SAC_ND_DEC_RC_FREE((SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, )
        /*
         * ND_UNSHARE( (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, int, )
         */
        SAC_NOOP()

        /*
         * ND_ASSIGN( (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, )
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 19, 28, (SAC_ND_A_DIM( (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()
        SAC_NOOP()
        SAC_NOOP()
        SAC_ND_ASSIGN__DATA( (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), (SACp_emal_1250__pinl_851__mose_7, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )

        /*
         * ND_ASSIGN( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, )
         */
        SAC_ASSURE_TYPE_LINE ("./matmul.sac", 19, 28, (SAC_ND_A_DIM( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (0)), "Assignment with incompatible types found");
        SAC_NOOP()
        SAC_NOOP()
        SAC_NOOP()
        SAC_ND_ASSIGN__DATA( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), (SACp_ufiv_1232__pinl_833__flat_45, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )

        /*
         * WL_FOLD( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 1, (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))))
         */
        SAC_TR_WL_PRINT( ("index vector [%d] -- fold", SAC_ND_READ( (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0)));
        /* fold operation */

        SAC_WL_GRID_UNROLL_END(0, (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
        SAC_WL_STRIDE_LOOP_END(0, (SACp_pinl_829_iv, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
        /*
         * MT_SCHEDULER_END( 0, 1, 0, 550)
         */


        /*
         * WL_SCHEDULE__END( 1)
         */
      }

      SAC_PF_END_WITH(fold)
      SAC_ND_LABEL(_comp_1303_SAC_label)
      SAC_ND_FREE((SACp_emal_1253__pinl_759__flat_145, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), )
      SAC_ND_FREE((SACp_pinl_830__eat_240, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
      /*
       * WL_ASSIGN( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 2, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 2, (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
       */
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_DIM( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == (SAC_ND_A_DIM( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) - SAC_ND_A_SIZE( (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, ))))))))))))), "WL expression with illegal dimension found!");
      SAC_ASSURE_TYPE_LINE ("./matmul.sac", 25, 12, (SAC_ND_A_SIZE( (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, ))))))))))) == SAC_WL_SHAPE_FACTOR( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1)), "WL expression with illegal size found!");
      SAC_ND_WRITE_READ_COPY( (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), SAC_ND_READ( (SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0), (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 0, );

      SAC_ND_DEC_RC_FREE((SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), 1, )
      SAC_WL_INC_OFFSET((SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_824__flat_49, (SCL, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))))
      SAC_WL_MT_GRID_UNROLL_END(1, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_WL_MT_STRIDE_LOOP_END(1, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      SAC_WL_MT_GRID_UNROLL_END(0, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 1)
      SAC_WL_MT_STRIDE_LOOP_END(0, (SACp_pinl_771__flat_143, (AKS, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), (SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0, 550, 1)
      /*
       * MT_SCHEDULER_Block_END( 0, 2, 0, 0, 550, 550, 1, 1)
       */


      /*
       * WL_SCHEDULE__END( 2)
       */
    }

    SAC_PF_END_WITH(genarray)
    SAC_ND_LABEL(_comp_1304_SAC_label)
    SAC_ND_FREE((SACp_wlidx_1174__pinl_751__flat_105, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_pinl_773_j, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    SAC_ND_FREE((SACp_pinl_772_i, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
    /*
     * MT_SPMDFUN_RET( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 1, inout, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, ND, NONE)
     */
    SAC_MT_SYNC_BEGIN( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d);
      SAC_MT_SYNC_FOLD_inout( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 0, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), (SACp_mtspmdfanon_1297__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (NON, (NOT, (NDI, (DOU, )))))))))), double, ND, NONE);
    SAC_MT_SYNC_CONT( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d);
      SAC_MT_SEND_RESULT_inout( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, SAC_MT_SELF_LOCAL_ID(), 0, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))));
    SAC_MT_SYNC_END( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d);
    SAC_MT_SPMDFUN_REAL_RETURN();

    SAC_CLEANUP_LOCAL_MEM()
  }
/*
   * MT_SPMDFUN_DEF_END( SACf__MAIN_CL_ST___mtspmdf_1298_matmul___d_550_550__i__i__d_550_550__i__i__d_550_550__d, 8, inout, double, (SACp_emal_1248__pinl_751__flat_105, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPO, (NOT, (NDI, (DOU, )))))))))), in, double, (SACl_a, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1231, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1230, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACl_b, (AKS, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))), in, int, (SACp_iveras_1229, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, int, (SACp_iveras_1228, (SCL, (NHD, (NUQ, (INT, (GLO, (FPM, (NOT, (NDI, (INT, )))))))))), in, double, (SACp_pinl_742__flat_78, (SCL, (NHD, (NUQ, (FLO, (GLO, (FPM, (NOT, (NDI, (DOU, )))))))))))
   */
}

/*
 * stubs for SACARGfreeDataUdt and SACARGcopyDataUdt
 */
extern void SACARGfreeDataUdt( int, void *);
extern void *SACARGcopyDataUdt( int, int, void *);
void SACARGfreeDataUdt( int size, void *data) {}
void *SACARGcopyDataUdt( int type, int size, void *data) { return ((void *) 0x0); } 

int main( int __argc, char *__argv[])
{
  SAC_MT_DECL_MYTHREAD()
  SAC_ND_DECL__DATA( (SAC_res, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int, )
  SAC_ND_DECL__DESC( (SAC_res, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), )
  SAC_NOTHING()
  SAC_HWLOC_SETUP();
  SAC_MT_SETUP_INITIAL();
  SAC_RTSPEC_SETUP_INITIAL(2, " -noprelude -t mt_pth -mt_bind simple matmul.sac -o matmul -DP=550 -DITER=20", "../sac2c/build_r/sac2c_p");
  SAC_PF_SETUP();
  SAC_HM_SETUP();
  SAC_MT_SETUP();
  SAC_CS_SETUP();
  SAC_RTSPEC_SETUP();
  SAC_COMMANDLINE_SET( __argc, __argv);

  SAC_INVOKE_MAIN_FUN( SACf__MAIN_CL_ST__main, SAC_ND_ARG_out( (SAC_res, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), int));

  SAC_DISTMEM_BARRIER();
  SAC_PF_PRINT();
  SAC_CS_FINALIZE();
  SAC_MT_FINALIZE();
  SAC_HWLOC_FINALIZE();
  SAC_HM_PRINT();

  SAC_RTSPEC_FINALIZE();

  return( SAC_ND_READ( (SAC_res, (SCL, (NHD, (NUQ, (INT, (GLO, (NON, (NOT, (NDI, (INT, )))))))))), 0));
}
