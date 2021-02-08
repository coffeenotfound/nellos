
/*
#[macro_export]
macro_rules! gen_osl {
	($oslty:path) => {
		#[allow(non_snake_case)]
		extern "C" {
			/*
			 * OSL Initialization and shutdown primitives
			 */
			pub extern "C" fn AcpiOsInitialize() -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsTerminate() -> crate::ACPI_STATUS;
			
			/*
			 * ACPI Table interfaces
			 */
			pub extern "C" fn AcpiOsGetRootPointer() -> crate::ACPI_PHYSICAL_ADDRESS;
			
			pub extern "C" fn AcpiOsPredefinedOverride(init_val: *const crate::ACPI_PREDEFINED_NAMES, new_val: &mut crate::ACPI_STRING) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsTableOverride(existing_table: *const crate::ACPI_TABLE_HEADER, new_table: &mut *mut crate::ACPI_TABLE_HEADER) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsPhysicalTableOverride(existing_table: *const crate::ACPI_TABLE_HEADER, new_address: &mut crate::ACPI_PHYSICAL_ADDRESS, new_table_length: &mut crate::UINT32) -> crate::ACPI_STATUS;
			
			/*
			 * Spinlock primitives
			 */
			pub extern "C" fn AcpiOsCreateLock(out_handle: &mut $oslty::ACPI_SPINLOCK) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsDeleteLock(handle: $oslty::ACPI_SPINLOCK);
			
			pub extern "C" fn AcpiOsAcquireLock(handle: $oslty::ACPI_SPINLOCK) -> $oslty::ACPI_CPU_FLAGS;
			
			pub extern "C" fn AcpiOsReleaseLock(handle: $oslty::ACPI_SPINLOCK, flags: $oslty::ACPI_CPU_FLAGS);
			
			/*
			 * Semaphore primitives
			 */
			pub extern "C" fn AcpiOsCreateSemaphore(max_unitx: crate::UINT32, initial_units: crate::UINT32, out_handle: &mut $oslty::ACPI_SEMAPHORE) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsDeleteSemaphore(handle: $oslty::ACPI_SEMAPHORE) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsWaitSemaphore(handle: $oslty::ACPI_SEMAPHORE, units: crate::UINT32, timeout: crate::UINT16) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsSignalSemaphore(handle: $oslty::ACPI_SEMAPHORE, units: crate::UINT32) -> crate::ACPI_STATUS;
			
			/*
			 * Mutex primitives. May be configured to use semaphores instead via
			 * ACPI_MUTEX_TYPE (see platform/acenv.h)
			 */
			pub extern "C" fn AcpiOsCreateMutex(out_handle: &mut $oslty::ACPI_MUTEX) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsDeleteMutex(handle: $oslty::ACPI_MUTEX);
			
			pub extern "C" fn AcpiOsAcquireMutex(handle: $oslty::ACPI_MUTEX, timeout: crate::UINT16) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsReleaseMutex(handle: $oslty::ACPI_MUTEX);
			
			/*
			 * Memory allocation and mapping
			 */
			pub extern "C" fn AcpiOsAllocate(size: crate::ACPI_SIZE) -> *mut cty::c_void;
			
			pub extern "C" fn AcpiOsAllocateZeroed(size: crate::ACPI_SIZE) -> *mut cty::c_void;
			
			pub extern "C" fn AcpiOsFree(memory: *mut cty::c_void);
			
			pub extern "C" fn AcpiOsMapMemory(phys_addr: crate::ACPI_PHYSICAL_ADDRESS, length: crate::ACPI_SIZE) -> *mut cty::c_void;
			
			pub extern "C" fn AcpiOsUnmapMemory(logical_addr: *mut cty::c_void, size: crate::ACPI_SIZE);
			
			pub extern "C" fn AcpiOsGetPhysicalAddress(logical_addr: *mut cty::c_void, physical_addr: crate::ACPI_PHYSICAL_ADDRESS) -> crate::ACPI_STATUS;
			
			/*
			 * Memory/Object Cache
			 */
			////pub extern "C" fn AcpiOsCreateCache(cache_name: *const c_char, object_size: UINT16, max_depth: UINT16, out_cache: &mut *mut ACPI_CACHE_T) -> ACPI_STATUS {
			//	unimplemented!()
			//}
			//
			////pub extern "C" fn AcpiOsDeleteCache(cache: *mut ACPI_CACHE_T) -> ACPI_STATUS {
			//	unimplemented!()
			//}
			//
			////pub extern "C" fn AcpiOsPurgeCache(cache: *mut ACPI_CACHE_T) -> ACPI_STATUS {
			//	unimplemented!()
			//}
			//
			////pub extern "C" fn AcpiOsAcquireObject(cache: *mut ACPI_CACHE_T) -> *mut c_void {
			//	unimplemented!()
			//}
			//
			////pub extern "C" fn AcpiOsReleaseObject(cache: *mut ACPI_CACHE_T, object: *mut c_void) -> ACPI_STATUS {
			//	unimplemented!()
			//}
			
			/*
			 * Interrupt handlers
			 */
			pub extern "C" fn AcpiOsInstallInterruptHandler(interrupt_nr: crate::UINT32, service_routine: crate::ACPI_OSD_HANDLER, context: *mut cty::c_void) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsRemoveInterruptHandler(interrupt_nr: crate::UINT32, service_routine: crate::ACPI_OSD_HANDLER) -> crate::ACPI_STATUS;
			
			/*
			 * Threads and Scheduling
			 */
			pub extern "C" fn AcpiOsGetThreadId() -> $oslty::ACPI_THREAD_ID;
			
			pub extern "C" fn AcpiOsExecute(exec_type: crate::ACPI_EXECUTE_TYPE, funtion: crate::ACPI_OSD_EXEC_CALLBACK, context: *mut cty::c_void) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsWaitEventsComplete();
			
			pub extern "C" fn AcpiOsSleep(millis: crate::UINT64);
			
			pub extern "C" fn AcpiOsStall(micros: crate::UINT32);
			
			/*
			 * Platform and hardware-independent I/O interfaces
			 */
			pub extern "C" fn AcpiOsReadPort(addr: crate::ACPI_IO_ADDRESS, out_val: &mut crate::UINT32, width: crate::UINT32) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsWritePort(addr: crate::ACPI_IO_ADDRESS, val: crate::UINT32, width: crate::UINT32) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsReadMemory(addr: crate::ACPI_PHYSICAL_ADDRESS, out_val: &mut crate::UINT64, width: crate::UINT32) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsWriteMemory(addr: crate::ACPI_PHYSICAL_ADDRESS, val: crate::UINT64, width: crate::UINT32) -> crate::ACPI_STATUS;
			
			/*
			 * Platform and hardware-independent PCI configuration space access
			 */
			pub extern "C" fn AcpiOsReadPciConfiguration(pci_id: *mut crate::ACPI_PCI_ID, reg: crate::UINT32, out_val: &mut crate::UINT64, width: crate::UINT32) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsWritePciConfiguration(pci_id: *mut crate::ACPI_PCI_ID, reg: crate::UINT32, val: crate::UINT64, width: crate::UINT32) -> crate::ACPI_STATUS;
			
			/*
			 * Miscellaneous
			 */
			pub extern "C" fn AcpiOsReadable(ptr: *mut cty::c_void) -> crate::BOOLEAN;
			
			pub extern "C" fn AcpiOsWritable(ptr: *mut cty::c_void, length: crate::ACPI_SIZE) -> crate::BOOLEAN;
			
			pub extern "C" fn AcpiOsGetTimer() -> crate::UINT64;
			
			pub extern "C" fn AcpiOsSignal(function: crate::UINT32, info: *mut cty::c_void) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsEnterSleep(sleep_state: crate::UINT8, reg_a: crate::UINT32, reg_b: crate::UINT32) -> crate::ACPI_STATUS;
			
			/*
			 * Debug print routines
			 */
			pub unsafe extern "C" fn AcpiOsPrintf(format: *const cty::c_char, #[allow(unused_mut)] mut args: ...);
			
			pub extern "C" fn AcpiOsVprintf(format: *const cty::c_char, args: *const crate::va_list);
			
			pub extern "C" fn AcpiOsRedirectOutput(dest: *const cty::c_void);
			
			/*
			 * Debug IO
			 */
			pub extern "C" fn AcpiOsGetLine(buf: *mut cty::char, buf_len: crate::UINT32, bytes_read: &mut crate::UINT32) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsInitializeDebugger() -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsTerminateDebugger();
			
			pub extern "C" fn AcpiOsWaitCommandReady() -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsNotifyCommandComplete() -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsTracePoint(event_type: crate::ACPI_TRACE_EVENT_TYPE, begin: crate::BOOLEAN, aml: *const crate::UINT8, pathname: *const cty::c_char);
			
			/*
			 * Obtain ACPI table(s)
			 */
			pub extern "C" fn AcpiOsGetTableByName(signature: *const cty::c_char, instance: crate::UINT32, out_table: &mut *mut ACPI_TABLE_HEADER, out_addr: &mut *const ACPI_PHYSICAL_ADDRESS) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsGetTableByIndex(index: crate::UINT32, out_table: &mut *mut crate::ACPI_TABLE_HEADER, instance: crate::UINT32, address: *mut ACPI_PHYSICAL_ADDRESS) -> crate::ACPI_STATUS;
			
			pub extern "C" fn AcpiOsGetTableByAddress(address: crate::ACPI_PHYSICAL_ADDRESS, out_table: &mut *mut crate::ACPI_TABLE_HEADER) -> crate::ACPI_STATUS;
			
			/*
			 * Directory manipulation
			 */
			pub extern "C" fn AcpiOsOpenDirectory(pathname: *const cty::c_char, wildcard_spec: *const cty::c_char, requested_file_type: cty::c_char) -> *mut cty::c_void;
			
			pub extern "C" fn AcpiOsGetNextFilename(dir_handle: *mut cty::c_void) -> *const cty::c_char;
			
			pub extern "C" fn AcpiOsCloseDirectory(dir_handle: *mut cty::c_void);
		}
	};
}
*/
