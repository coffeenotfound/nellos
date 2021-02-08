#ifndef _ONCE_ACNELLKERNEL
#define _ONCE_ACNELLKERNEL

#include <stdint.h>

#undef ACPI_USE_SYSTEM_CLIBRARY // TODO: Maybe provide kernel clib functions (mainly memset, memcpy in the future). Right now let it use it's own
#define ACPI_USE_SYSTEM_MEMFNS_NELL
#undef ACPI_USE_STANDARD_HEADERS // DEBUG:

// Use kernel's AcpiOsAllocateZeroed
#define USE_NATIVE_ALLOCATE_ZEROED

//#define ACPI_DEBUGGER
//#define ACPI_DISASSEMBLER

#define ACPI_MACHINE_WIDTH 64
#define COMPILER_DEPENDENT_INT64 int64_t
#define COMPILER_DEPENDENT_UINT64 uint64_t

#define ACPI_MUTEX_TYPE ACPI_BINARY_SEMAPHORE // TODO: In future maybe use kernel's own Mutexes
#define ACPI_MUTEX_DEBUG

#define ACPI_SIMPLE_RETURN_MACROS // Safer
#undef ACPI_USE_DO_WHILE_0 // Seems unnecessary and dumb

#define ACPI_USE_LOCAL_CACHE // TODO: In future maybe use object cache functionality of the kernel (we don't have that yet though, and we may never have it (because a generic ass "cache_t" is kinda massively simplistic))
// #define ACPI_DBG_TRACK_ALLOCATIONS

#define ACPI_DEBUG_OUTPUT

#define ACPI_LIBRARY

#endif
