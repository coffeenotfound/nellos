// /#define\s+(\S+)(\s+)(\S+)\s+\((\S+)\)/
// `pub const $1: ACPI_STATUS$2= $3($4);`

use crate::*;

pub const fn EXCEP_ENV(code: u32) -> ACPI_STATUS {code | AE_CODE_ENVIRONMENTAL}
pub const fn EXCEP_PGM(code: u32) -> ACPI_STATUS {code | AE_CODE_PROGRAMMER}
pub const fn EXCEP_TBL(code: u32) -> ACPI_STATUS {code | AE_CODE_ACPI_TABLES}
pub const fn EXCEP_AML(code: u32) -> ACPI_STATUS {code | AE_CODE_AML}
pub const fn EXCEP_CTL(code: u32) -> ACPI_STATUS {code | AE_CODE_CONTROL}

pub const AE_OK: ACPI_STATUS                           = 0x0000;

/*
 * Environmental exceptions
 */
pub const AE_ERROR: ACPI_STATUS                        = EXCEP_ENV(0x0001);
pub const AE_NO_ACPI_TABLES: ACPI_STATUS               = EXCEP_ENV(0x0002);
pub const AE_NO_NAMESPACE: ACPI_STATUS                 = EXCEP_ENV(0x0003);
pub const AE_NO_MEMORY: ACPI_STATUS                    = EXCEP_ENV(0x0004);
pub const AE_NOT_FOUND: ACPI_STATUS                    = EXCEP_ENV(0x0005);
pub const AE_NOT_EXIST: ACPI_STATUS                    = EXCEP_ENV(0x0006);
pub const AE_ALREADY_EXISTS: ACPI_STATUS               = EXCEP_ENV(0x0007);
pub const AE_TYPE: ACPI_STATUS                         = EXCEP_ENV(0x0008);
pub const AE_NULL_OBJECT: ACPI_STATUS                  = EXCEP_ENV(0x0009);
pub const AE_NULL_ENTRY: ACPI_STATUS                   = EXCEP_ENV(0x000A);
pub const AE_BUFFER_OVERFLOW: ACPI_STATUS              = EXCEP_ENV(0x000B);
pub const AE_STACK_OVERFLOW: ACPI_STATUS               = EXCEP_ENV(0x000C);
pub const AE_STACK_UNDERFLOW: ACPI_STATUS              = EXCEP_ENV(0x000D);
pub const AE_NOT_IMPLEMENTED: ACPI_STATUS              = EXCEP_ENV(0x000E);
pub const AE_SUPPORT: ACPI_STATUS                      = EXCEP_ENV(0x000F);
pub const AE_LIMIT: ACPI_STATUS                        = EXCEP_ENV(0x0010);
pub const AE_TIME: ACPI_STATUS                         = EXCEP_ENV(0x0011);
pub const AE_ACQUIRE_DEADLOCK: ACPI_STATUS             = EXCEP_ENV(0x0012);
pub const AE_RELEASE_DEADLOCK: ACPI_STATUS             = EXCEP_ENV(0x0013);
pub const AE_NOT_ACQUIRED: ACPI_STATUS                 = EXCEP_ENV(0x0014);
pub const AE_ALREADY_ACQUIRED: ACPI_STATUS             = EXCEP_ENV(0x0015);
pub const AE_NO_HARDWARE_RESPONSE: ACPI_STATUS         = EXCEP_ENV(0x0016);
pub const AE_NO_GLOBAL_LOCK: ACPI_STATUS               = EXCEP_ENV(0x0017);
pub const AE_ABORT_METHOD: ACPI_STATUS                 = EXCEP_ENV(0x0018);
pub const AE_SAME_HANDLER: ACPI_STATUS                 = EXCEP_ENV(0x0019);
pub const AE_NO_HANDLER: ACPI_STATUS                   = EXCEP_ENV(0x001A);
pub const AE_OWNER_ID_LIMIT: ACPI_STATUS               = EXCEP_ENV(0x001B);
pub const AE_NOT_CONFIGURED: ACPI_STATUS               = EXCEP_ENV(0x001C);
pub const AE_ACCESS: ACPI_STATUS                       = EXCEP_ENV(0x001D);
pub const AE_IO_ERROR: ACPI_STATUS                     = EXCEP_ENV(0x001E);
pub const AE_NUMERIC_OVERFLOW: ACPI_STATUS             = EXCEP_ENV(0x001F);
pub const AE_HEX_OVERFLOW: ACPI_STATUS                 = EXCEP_ENV(0x0020);
pub const AE_DECIMAL_OVERFLOW: ACPI_STATUS             = EXCEP_ENV(0x0021);
pub const AE_OCTAL_OVERFLOW: ACPI_STATUS               = EXCEP_ENV(0x0022);
pub const AE_END_OF_TABLE: ACPI_STATUS                 = EXCEP_ENV(0x0023);

/*
 * Programmer exceptions
 */
pub const AE_BAD_PARAMETER: ACPI_STATUS                = EXCEP_PGM(0x0001);
pub const AE_BAD_CHARACTER: ACPI_STATUS                = EXCEP_PGM(0x0002);
pub const AE_BAD_PATHNAME: ACPI_STATUS                 = EXCEP_PGM(0x0003);
pub const AE_BAD_DATA: ACPI_STATUS                     = EXCEP_PGM(0x0004);
pub const AE_BAD_HEX_CONSTANT: ACPI_STATUS             = EXCEP_PGM(0x0005);
pub const AE_BAD_OCTAL_CONSTANT: ACPI_STATUS           = EXCEP_PGM(0x0006);
pub const AE_BAD_DECIMAL_CONSTANT: ACPI_STATUS         = EXCEP_PGM(0x0007);
pub const AE_MISSING_ARGUMENTS: ACPI_STATUS            = EXCEP_PGM(0x0008);
pub const AE_BAD_ADDRESS: ACPI_STATUS                  = EXCEP_PGM(0x0009);

/*
 * Acpi table exceptions
 */
pub const AE_BAD_SIGNATURE: ACPI_STATUS                = EXCEP_TBL(0x0001);
pub const AE_BAD_HEADER: ACPI_STATUS                   = EXCEP_TBL(0x0002);
pub const AE_BAD_CHECKSUM: ACPI_STATUS                 = EXCEP_TBL(0x0003);
pub const AE_BAD_VALUE: ACPI_STATUS                    = EXCEP_TBL(0x0004);
pub const AE_INVALID_TABLE_LENGTH: ACPI_STATUS         = EXCEP_TBL(0x0005);

/*
 * AML exceptions. These are caused by problems with
 * the actual AML byte stream
 */
pub const AE_AML_BAD_OPCODE: ACPI_STATUS               = EXCEP_AML(0x0001);
pub const AE_AML_NO_OPERAND: ACPI_STATUS               = EXCEP_AML(0x0002);
pub const AE_AML_OPERAND_TYPE: ACPI_STATUS             = EXCEP_AML(0x0003);
pub const AE_AML_OPERAND_VALUE: ACPI_STATUS            = EXCEP_AML(0x0004);
pub const AE_AML_UNINITIALIZED_LOCAL: ACPI_STATUS      = EXCEP_AML(0x0005);
pub const AE_AML_UNINITIALIZED_ARG: ACPI_STATUS        = EXCEP_AML(0x0006);
pub const AE_AML_UNINITIALIZED_ELEMENT: ACPI_STATUS    = EXCEP_AML(0x0007);
pub const AE_AML_NUMERIC_OVERFLOW: ACPI_STATUS         = EXCEP_AML(0x0008);
pub const AE_AML_REGION_LIMIT: ACPI_STATUS             = EXCEP_AML(0x0009);
pub const AE_AML_BUFFER_LIMIT: ACPI_STATUS             = EXCEP_AML(0x000A);
pub const AE_AML_PACKAGE_LIMIT: ACPI_STATUS            = EXCEP_AML(0x000B);
pub const AE_AML_DIVIDE_BY_ZERO: ACPI_STATUS           = EXCEP_AML(0x000C);
pub const AE_AML_BAD_NAME: ACPI_STATUS                 = EXCEP_AML(0x000D);
pub const AE_AML_NAME_NOT_FOUND: ACPI_STATUS           = EXCEP_AML(0x000E);
pub const AE_AML_INTERNAL: ACPI_STATUS                 = EXCEP_AML(0x000F);
pub const AE_AML_INVALID_SPACE_ID: ACPI_STATUS         = EXCEP_AML(0x0010);
pub const AE_AML_STRING_LIMIT: ACPI_STATUS             = EXCEP_AML(0x0011);
pub const AE_AML_NO_RETURN_VALUE: ACPI_STATUS          = EXCEP_AML(0x0012);
pub const AE_AML_METHOD_LIMIT: ACPI_STATUS             = EXCEP_AML(0x0013);
pub const AE_AML_NOT_OWNER: ACPI_STATUS                = EXCEP_AML(0x0014);
pub const AE_AML_MUTEX_ORDER: ACPI_STATUS              = EXCEP_AML(0x0015);
pub const AE_AML_MUTEX_NOT_ACQUIRED: ACPI_STATUS       = EXCEP_AML(0x0016);
pub const AE_AML_INVALID_RESOURCE_TYPE: ACPI_STATUS    = EXCEP_AML(0x0017);
pub const AE_AML_INVALID_INDEX: ACPI_STATUS            = EXCEP_AML(0x0018);
pub const AE_AML_REGISTER_LIMIT: ACPI_STATUS           = EXCEP_AML(0x0019);
pub const AE_AML_NO_WHILE: ACPI_STATUS                 = EXCEP_AML(0x001A);
pub const AE_AML_ALIGNMENT: ACPI_STATUS                = EXCEP_AML(0x001B);
pub const AE_AML_NO_RESOURCE_END_TAG: ACPI_STATUS      = EXCEP_AML(0x001C);
pub const AE_AML_BAD_RESOURCE_VALUE: ACPI_STATUS       = EXCEP_AML(0x001D);
pub const AE_AML_CIRCULAR_REFERENCE: ACPI_STATUS       = EXCEP_AML(0x001E);
pub const AE_AML_BAD_RESOURCE_LENGTH: ACPI_STATUS      = EXCEP_AML(0x001F);
pub const AE_AML_ILLEGAL_ADDRESS: ACPI_STATUS          = EXCEP_AML(0x0020);
pub const AE_AML_LOOP_TIMEOUT: ACPI_STATUS             = EXCEP_AML(0x0021);
pub const AE_AML_UNINITIALIZED_NODE: ACPI_STATUS       = EXCEP_AML(0x0022);
pub const AE_AML_TARGET_TYPE: ACPI_STATUS              = EXCEP_AML(0x0023);
pub const AE_AML_PROTOCOL: ACPI_STATUS                 = EXCEP_AML(0x0024);
pub const AE_AML_BUFFER_LENGTH: ACPI_STATUS            = EXCEP_AML(0x0025);

/*
 * Internal exceptions used for control
 */
pub const AE_CTRL_RETURN_VALUE: ACPI_STATUS            = EXCEP_CTL(0x0001);
pub const AE_CTRL_PENDING: ACPI_STATUS                 = EXCEP_CTL(0x0002);
pub const AE_CTRL_TERMINATE: ACPI_STATUS               = EXCEP_CTL(0x0003);
pub const AE_CTRL_TRUE: ACPI_STATUS                    = EXCEP_CTL(0x0004);
pub const AE_CTRL_FALSE: ACPI_STATUS                   = EXCEP_CTL(0x0005);
pub const AE_CTRL_DEPTH: ACPI_STATUS                   = EXCEP_CTL(0x0006);
pub const AE_CTRL_END: ACPI_STATUS                     = EXCEP_CTL(0x0007);
pub const AE_CTRL_TRANSFER: ACPI_STATUS                = EXCEP_CTL(0x0008);
pub const AE_CTRL_BREAK: ACPI_STATUS                   = EXCEP_CTL(0x0009);
pub const AE_CTRL_CONTINUE: ACPI_STATUS                = EXCEP_CTL(0x000A);
pub const AE_CTRL_PARSE_CONTINUE: ACPI_STATUS          = EXCEP_CTL(0x000B);
pub const AE_CTRL_PARSE_PENDING: ACPI_STATUS           = EXCEP_CTL(0x000C);

//pub type ACPI_CPU_FLAGS = u32;
//pub type ACPI_THREAD_ID = u64;
//
//pub type ACPI_SPINLOCK = *mut cty::c_void;
//pub type ACPI_SEMAPHORE = *mut cty::c_void;
////pub type ACPI_MUTEX = *mut cty::c_void;
//
//pub type ACPI_CACHE_T = ACPI_MEMORY_LIST;
