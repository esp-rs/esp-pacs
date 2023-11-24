#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    core_0_montr_ena: CORE_0_MONTR_ENA,
    core_0_intr_raw: CORE_0_INTR_RAW,
    core_0_intr_ena: CORE_0_INTR_ENA,
    core_0_intr_clr: CORE_0_INTR_CLR,
    core_0_area_dram0_0_min: CORE_0_AREA_DRAM0_0_MIN,
    core_0_area_dram0_0_max: CORE_0_AREA_DRAM0_0_MAX,
    core_0_area_dram0_1_min: CORE_0_AREA_DRAM0_1_MIN,
    core_0_area_dram0_1_max: CORE_0_AREA_DRAM0_1_MAX,
    core_0_area_pif_0_min: CORE_0_AREA_PIF_0_MIN,
    core_0_area_pif_0_max: CORE_0_AREA_PIF_0_MAX,
    core_0_area_pif_1_min: CORE_0_AREA_PIF_1_MIN,
    core_0_area_pif_1_max: CORE_0_AREA_PIF_1_MAX,
    core_0_area_sp: CORE_0_AREA_SP,
    core_0_area_pc: CORE_0_AREA_PC,
    core_0_sp_unstable: CORE_0_SP_UNSTABLE,
    core_0_sp_min: CORE_0_SP_MIN,
    core_0_sp_max: CORE_0_SP_MAX,
    core_0_sp_pc: CORE_0_SP_PC,
    core_0_rcd_pdebugenable: CORE_0_RCD_PDEBUGENABLE,
    core_0_rcd_recording: CORE_0_RCD_RECORDING,
    core_0_rcd_pdebuginst: CORE_0_RCD_PDEBUGINST,
    core_0_rcd_pdebugstatus: CORE_0_RCD_PDEBUGSTATUS,
    core_0_rcd_pdebugdata: CORE_0_RCD_PDEBUGDATA,
    core_0_rcd_pdebugpc: CORE_0_RCD_PDEBUGPC,
    core_0_rcd_pdebugls0stat: CORE_0_RCD_PDEBUGLS0STAT,
    core_0_rcd_pdebugls0addr: CORE_0_RCD_PDEBUGLS0ADDR,
    core_0_rcd_pdebugls0data: CORE_0_RCD_PDEBUGLS0DATA,
    core_0_rcd_sp: CORE_0_RCD_SP,
    core_0_iram0_exception_monitor_0: CORE_0_IRAM0_EXCEPTION_MONITOR_0,
    core_0_iram0_exception_monitor_1: CORE_0_IRAM0_EXCEPTION_MONITOR_1,
    core_0_dram0_exception_monitor_0: CORE_0_DRAM0_EXCEPTION_MONITOR_0,
    core_0_dram0_exception_monitor_1: CORE_0_DRAM0_EXCEPTION_MONITOR_1,
    core_0_dram0_exception_monitor_2: CORE_0_DRAM0_EXCEPTION_MONITOR_2,
    core_0_dram0_exception_monitor_3: CORE_0_DRAM0_EXCEPTION_MONITOR_3,
    core_0_dram0_exception_monitor_4: CORE_0_DRAM0_EXCEPTION_MONITOR_4,
    core_0_dram0_exception_monitor_5: CORE_0_DRAM0_EXCEPTION_MONITOR_5,
    core_1_montr_ena: CORE_1_MONTR_ENA,
    core_1_intr_raw: CORE_1_INTR_RAW,
    core_1_intr_ena: CORE_1_INTR_ENA,
    core_1_intr_clr: CORE_1_INTR_CLR,
    core_1_area_dram0_0_min: CORE_1_AREA_DRAM0_0_MIN,
    core_1_area_dram0_0_max: CORE_1_AREA_DRAM0_0_MAX,
    core_1_area_dram0_1_min: CORE_1_AREA_DRAM0_1_MIN,
    core_1_area_dram0_1_max: CORE_1_AREA_DRAM0_1_MAX,
    core_1_area_pif_0_min: CORE_1_AREA_PIF_0_MIN,
    core_1_area_pif_0_max: CORE_1_AREA_PIF_0_MAX,
    core_1_area_pif_1_min: CORE_1_AREA_PIF_1_MIN,
    core_1_area_pif_1_max: CORE_1_AREA_PIF_1_MAX,
    core_1_area_pc: CORE_1_AREA_PC,
    core_1_area_sp: CORE_1_AREA_SP,
    core_1_sp_unstable: CORE_1_SP_UNSTABLE,
    core_1_sp_min: CORE_1_SP_MIN,
    core_1_sp_max: CORE_1_SP_MAX,
    core_1_sp_pc: CORE_1_SP_PC,
    core_1_rcd_pdebugenable: CORE_1_RCD_PDEBUGENABLE,
    core_1_rcd_recording: CORE_1_RCD_RECORDING,
    core_1_rcd_pdebuginst: CORE_1_RCD_PDEBUGINST,
    core_1_rcd_pdebugstatus: CORE_1_RCD_PDEBUGSTATUS,
    core_1_rcd_pdebugdata: CORE_1_RCD_PDEBUGDATA,
    core_1_rcd_pdebugpc: CORE_1_RCD_PDEBUGPC,
    core_1_rcd_pdebugls0stat: CORE_1_RCD_PDEBUGLS0STAT,
    core_1_rcd_pdebugls0addr: CORE_1_RCD_PDEBUGLS0ADDR,
    core_1_rcd_pdebugls0data: CORE_1_RCD_PDEBUGLS0DATA,
    core_1_rcd_sp: CORE_1_RCD_SP,
    core_1_iram0_exception_monitor_0: CORE_1_IRAM0_EXCEPTION_MONITOR_0,
    core_1_iram0_exception_monitor_1: CORE_1_IRAM0_EXCEPTION_MONITOR_1,
    core_1_dram0_exception_monitor_0: CORE_1_DRAM0_EXCEPTION_MONITOR_0,
    core_1_dram0_exception_monitor_1: CORE_1_DRAM0_EXCEPTION_MONITOR_1,
    core_1_dram0_exception_monitor_2: CORE_1_DRAM0_EXCEPTION_MONITOR_2,
    core_1_dram0_exception_monitor_3: CORE_1_DRAM0_EXCEPTION_MONITOR_3,
    core_1_dram0_exception_monitor_4: CORE_1_DRAM0_EXCEPTION_MONITOR_4,
    core_1_dram0_exception_monitor_5: CORE_1_DRAM0_EXCEPTION_MONITOR_5,
    core_x_iram0_dram0_exception_monitor_0: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0,
    core_x_iram0_dram0_exception_monitor_1: CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1,
    log_setting: LOG_SETTING,
    log_data_0: LOG_DATA_0,
    log_data_1: LOG_DATA_1,
    log_data_2: LOG_DATA_2,
    log_data_3: LOG_DATA_3,
    log_data_mask: LOG_DATA_MASK,
    log_min: LOG_MIN,
    log_max: LOG_MAX,
    log_mem_start: LOG_MEM_START,
    log_mem_end: LOG_MEM_END,
    log_mem_writing_addr: LOG_MEM_WRITING_ADDR,
    log_mem_full_flag: LOG_MEM_FULL_FLAG,
    _reserved86: [u8; 0xa4],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - core0 monitor enable configuration register"]
    #[inline(always)]
    pub const fn core_0_montr_ena(&self) -> &CORE_0_MONTR_ENA {
        &self.core_0_montr_ena
    }
    #[doc = "0x04 - core0 monitor interrupt status register"]
    #[inline(always)]
    pub const fn core_0_intr_raw(&self) -> &CORE_0_INTR_RAW {
        &self.core_0_intr_raw
    }
    #[doc = "0x08 - core0 monitor interrupt enable register"]
    #[inline(always)]
    pub const fn core_0_intr_ena(&self) -> &CORE_0_INTR_ENA {
        &self.core_0_intr_ena
    }
    #[doc = "0x0c - core0 monitor interrupt clr register"]
    #[inline(always)]
    pub const fn core_0_intr_clr(&self) -> &CORE_0_INTR_CLR {
        &self.core_0_intr_clr
    }
    #[doc = "0x10 - core0 dram0 region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_dram0_0_min(&self) -> &CORE_0_AREA_DRAM0_0_MIN {
        &self.core_0_area_dram0_0_min
    }
    #[doc = "0x14 - core0 dram0 region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_dram0_0_max(&self) -> &CORE_0_AREA_DRAM0_0_MAX {
        &self.core_0_area_dram0_0_max
    }
    #[doc = "0x18 - core0 dram0 region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_dram0_1_min(&self) -> &CORE_0_AREA_DRAM0_1_MIN {
        &self.core_0_area_dram0_1_min
    }
    #[doc = "0x1c - core0 dram0 region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_dram0_1_max(&self) -> &CORE_0_AREA_DRAM0_1_MAX {
        &self.core_0_area_dram0_1_max
    }
    #[doc = "0x20 - core0 PIF region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_pif_0_min(&self) -> &CORE_0_AREA_PIF_0_MIN {
        &self.core_0_area_pif_0_min
    }
    #[doc = "0x24 - core0 PIF region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_pif_0_max(&self) -> &CORE_0_AREA_PIF_0_MAX {
        &self.core_0_area_pif_0_max
    }
    #[doc = "0x28 - core0 PIF region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_pif_1_min(&self) -> &CORE_0_AREA_PIF_1_MIN {
        &self.core_0_area_pif_1_min
    }
    #[doc = "0x2c - core0 PIF region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_0_area_pif_1_max(&self) -> &CORE_0_AREA_PIF_1_MAX {
        &self.core_0_area_pif_1_max
    }
    #[doc = "0x30 - core0 area sp status register"]
    #[inline(always)]
    pub const fn core_0_area_sp(&self) -> &CORE_0_AREA_SP {
        &self.core_0_area_sp
    }
    #[doc = "0x34 - core0 area pc status register"]
    #[inline(always)]
    pub const fn core_0_area_pc(&self) -> &CORE_0_AREA_PC {
        &self.core_0_area_pc
    }
    #[doc = "0x38 - core0 sp unstable configuration register"]
    #[inline(always)]
    pub const fn core_0_sp_unstable(&self) -> &CORE_0_SP_UNSTABLE {
        &self.core_0_sp_unstable
    }
    #[doc = "0x3c - core0 sp region configuration regsiter"]
    #[inline(always)]
    pub const fn core_0_sp_min(&self) -> &CORE_0_SP_MIN {
        &self.core_0_sp_min
    }
    #[doc = "0x40 - core0 sp region configuration regsiter"]
    #[inline(always)]
    pub const fn core_0_sp_max(&self) -> &CORE_0_SP_MAX {
        &self.core_0_sp_max
    }
    #[doc = "0x44 - core0 sp pc status register"]
    #[inline(always)]
    pub const fn core_0_sp_pc(&self) -> &CORE_0_SP_PC {
        &self.core_0_sp_pc
    }
    #[doc = "0x48 - core0 pdebug configuration register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugenable(&self) -> &CORE_0_RCD_PDEBUGENABLE {
        &self.core_0_rcd_pdebugenable
    }
    #[doc = "0x4c - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_recording(&self) -> &CORE_0_RCD_RECORDING {
        &self.core_0_rcd_recording
    }
    #[doc = "0x50 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebuginst(&self) -> &CORE_0_RCD_PDEBUGINST {
        &self.core_0_rcd_pdebuginst
    }
    #[doc = "0x54 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugstatus(&self) -> &CORE_0_RCD_PDEBUGSTATUS {
        &self.core_0_rcd_pdebugstatus
    }
    #[doc = "0x58 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugdata(&self) -> &CORE_0_RCD_PDEBUGDATA {
        &self.core_0_rcd_pdebugdata
    }
    #[doc = "0x5c - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugpc(&self) -> &CORE_0_RCD_PDEBUGPC {
        &self.core_0_rcd_pdebugpc
    }
    #[doc = "0x60 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugls0stat(&self) -> &CORE_0_RCD_PDEBUGLS0STAT {
        &self.core_0_rcd_pdebugls0stat
    }
    #[doc = "0x64 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugls0addr(&self) -> &CORE_0_RCD_PDEBUGLS0ADDR {
        &self.core_0_rcd_pdebugls0addr
    }
    #[doc = "0x68 - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_pdebugls0data(&self) -> &CORE_0_RCD_PDEBUGLS0DATA {
        &self.core_0_rcd_pdebugls0data
    }
    #[doc = "0x6c - core0 pdebug status register"]
    #[inline(always)]
    pub const fn core_0_rcd_sp(&self) -> &CORE_0_RCD_SP {
        &self.core_0_rcd_sp
    }
    #[doc = "0x70 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_0(&self) -> &CORE_0_IRAM0_EXCEPTION_MONITOR_0 {
        &self.core_0_iram0_exception_monitor_0
    }
    #[doc = "0x74 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_0_iram0_exception_monitor_1(&self) -> &CORE_0_IRAM0_EXCEPTION_MONITOR_1 {
        &self.core_0_iram0_exception_monitor_1
    }
    #[doc = "0x78 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_0(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_0_dram0_exception_monitor_0
    }
    #[doc = "0x7c - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_1(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_1 {
        &self.core_0_dram0_exception_monitor_1
    }
    #[doc = "0x80 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_2(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_2 {
        &self.core_0_dram0_exception_monitor_2
    }
    #[doc = "0x84 - core0 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_3(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_3 {
        &self.core_0_dram0_exception_monitor_3
    }
    #[doc = "0x88 - core0 bus busy configuration regsiter"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_4(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_4 {
        &self.core_0_dram0_exception_monitor_4
    }
    #[doc = "0x8c - core0 bus busy configuration regsiter"]
    #[inline(always)]
    pub const fn core_0_dram0_exception_monitor_5(&self) -> &CORE_0_DRAM0_EXCEPTION_MONITOR_5 {
        &self.core_0_dram0_exception_monitor_5
    }
    #[doc = "0x90 - Core1 monitor enable configuration register"]
    #[inline(always)]
    pub const fn core_1_montr_ena(&self) -> &CORE_1_MONTR_ENA {
        &self.core_1_montr_ena
    }
    #[doc = "0x94 - Core1 monitor interrupt status register"]
    #[inline(always)]
    pub const fn core_1_intr_raw(&self) -> &CORE_1_INTR_RAW {
        &self.core_1_intr_raw
    }
    #[doc = "0x98 - Core1 monitor interrupt enable register"]
    #[inline(always)]
    pub const fn core_1_intr_ena(&self) -> &CORE_1_INTR_ENA {
        &self.core_1_intr_ena
    }
    #[doc = "0x9c - Core1 monitor interrupt clr register"]
    #[inline(always)]
    pub const fn core_1_intr_clr(&self) -> &CORE_1_INTR_CLR {
        &self.core_1_intr_clr
    }
    #[doc = "0xa0 - Core1 dram0 region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_dram0_0_min(&self) -> &CORE_1_AREA_DRAM0_0_MIN {
        &self.core_1_area_dram0_0_min
    }
    #[doc = "0xa4 - Core1 dram0 region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_dram0_0_max(&self) -> &CORE_1_AREA_DRAM0_0_MAX {
        &self.core_1_area_dram0_0_max
    }
    #[doc = "0xa8 - Core1 dram0 region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_dram0_1_min(&self) -> &CORE_1_AREA_DRAM0_1_MIN {
        &self.core_1_area_dram0_1_min
    }
    #[doc = "0xac - Core1 dram0 region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_dram0_1_max(&self) -> &CORE_1_AREA_DRAM0_1_MAX {
        &self.core_1_area_dram0_1_max
    }
    #[doc = "0xb0 - Core1 PIF region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_pif_0_min(&self) -> &CORE_1_AREA_PIF_0_MIN {
        &self.core_1_area_pif_0_min
    }
    #[doc = "0xb4 - Core1 PIF region0 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_pif_0_max(&self) -> &CORE_1_AREA_PIF_0_MAX {
        &self.core_1_area_pif_0_max
    }
    #[doc = "0xb8 - Core1 PIF region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_pif_1_min(&self) -> &CORE_1_AREA_PIF_1_MIN {
        &self.core_1_area_pif_1_min
    }
    #[doc = "0xbc - Core1 PIF region1 addr configuration register"]
    #[inline(always)]
    pub const fn core_1_area_pif_1_max(&self) -> &CORE_1_AREA_PIF_1_MAX {
        &self.core_1_area_pif_1_max
    }
    #[doc = "0xc0 - Core1 area sp status register"]
    #[inline(always)]
    pub const fn core_1_area_pc(&self) -> &CORE_1_AREA_PC {
        &self.core_1_area_pc
    }
    #[doc = "0xc4 - Core1 area pc status register"]
    #[inline(always)]
    pub const fn core_1_area_sp(&self) -> &CORE_1_AREA_SP {
        &self.core_1_area_sp
    }
    #[doc = "0xc8 - Core1 sp unstable configuration register"]
    #[inline(always)]
    pub const fn core_1_sp_unstable(&self) -> &CORE_1_SP_UNSTABLE {
        &self.core_1_sp_unstable
    }
    #[doc = "0xcc - Core1 sp region configuration regsiter"]
    #[inline(always)]
    pub const fn core_1_sp_min(&self) -> &CORE_1_SP_MIN {
        &self.core_1_sp_min
    }
    #[doc = "0xd0 - Core1 sp region configuration regsiter"]
    #[inline(always)]
    pub const fn core_1_sp_max(&self) -> &CORE_1_SP_MAX {
        &self.core_1_sp_max
    }
    #[doc = "0xd4 - Core1 sp pc status register"]
    #[inline(always)]
    pub const fn core_1_sp_pc(&self) -> &CORE_1_SP_PC {
        &self.core_1_sp_pc
    }
    #[doc = "0xd8 - Core1 pdebug configuration register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebugenable(&self) -> &CORE_1_RCD_PDEBUGENABLE {
        &self.core_1_rcd_pdebugenable
    }
    #[doc = "0xdc - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_recording(&self) -> &CORE_1_RCD_RECORDING {
        &self.core_1_rcd_recording
    }
    #[doc = "0xe0 - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebuginst(&self) -> &CORE_1_RCD_PDEBUGINST {
        &self.core_1_rcd_pdebuginst
    }
    #[doc = "0xe4 - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebugstatus(&self) -> &CORE_1_RCD_PDEBUGSTATUS {
        &self.core_1_rcd_pdebugstatus
    }
    #[doc = "0xe8 - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebugdata(&self) -> &CORE_1_RCD_PDEBUGDATA {
        &self.core_1_rcd_pdebugdata
    }
    #[doc = "0xec - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebugpc(&self) -> &CORE_1_RCD_PDEBUGPC {
        &self.core_1_rcd_pdebugpc
    }
    #[doc = "0xf0 - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebugls0stat(&self) -> &CORE_1_RCD_PDEBUGLS0STAT {
        &self.core_1_rcd_pdebugls0stat
    }
    #[doc = "0xf4 - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebugls0addr(&self) -> &CORE_1_RCD_PDEBUGLS0ADDR {
        &self.core_1_rcd_pdebugls0addr
    }
    #[doc = "0xf8 - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_pdebugls0data(&self) -> &CORE_1_RCD_PDEBUGLS0DATA {
        &self.core_1_rcd_pdebugls0data
    }
    #[doc = "0xfc - Core1 pdebug status register"]
    #[inline(always)]
    pub const fn core_1_rcd_sp(&self) -> &CORE_1_RCD_SP {
        &self.core_1_rcd_sp
    }
    #[doc = "0x100 - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_iram0_exception_monitor_0(&self) -> &CORE_1_IRAM0_EXCEPTION_MONITOR_0 {
        &self.core_1_iram0_exception_monitor_0
    }
    #[doc = "0x104 - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_iram0_exception_monitor_1(&self) -> &CORE_1_IRAM0_EXCEPTION_MONITOR_1 {
        &self.core_1_iram0_exception_monitor_1
    }
    #[doc = "0x108 - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_0(&self) -> &CORE_1_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_1_dram0_exception_monitor_0
    }
    #[doc = "0x10c - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_1(&self) -> &CORE_1_DRAM0_EXCEPTION_MONITOR_1 {
        &self.core_1_dram0_exception_monitor_1
    }
    #[doc = "0x110 - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_2(&self) -> &CORE_1_DRAM0_EXCEPTION_MONITOR_2 {
        &self.core_1_dram0_exception_monitor_2
    }
    #[doc = "0x114 - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_3(&self) -> &CORE_1_DRAM0_EXCEPTION_MONITOR_3 {
        &self.core_1_dram0_exception_monitor_3
    }
    #[doc = "0x118 - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_4(&self) -> &CORE_1_DRAM0_EXCEPTION_MONITOR_4 {
        &self.core_1_dram0_exception_monitor_4
    }
    #[doc = "0x11c - Core1 bus busy status regsiter"]
    #[inline(always)]
    pub const fn core_1_dram0_exception_monitor_5(&self) -> &CORE_1_DRAM0_EXCEPTION_MONITOR_5 {
        &self.core_1_dram0_exception_monitor_5
    }
    #[doc = "0x120 - bus busy configuration register"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor_0(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 {
        &self.core_x_iram0_dram0_exception_monitor_0
    }
    #[doc = "0x124 - bus busy configuration register"]
    #[inline(always)]
    pub const fn core_x_iram0_dram0_exception_monitor_1(
        &self,
    ) -> &CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 {
        &self.core_x_iram0_dram0_exception_monitor_1
    }
    #[doc = "0x128 - log set register"]
    #[inline(always)]
    pub const fn log_setting(&self) -> &LOG_SETTING {
        &self.log_setting
    }
    #[doc = "0x12c - log check data register"]
    #[inline(always)]
    pub const fn log_data_0(&self) -> &LOG_DATA_0 {
        &self.log_data_0
    }
    #[doc = "0x130 - log check data register"]
    #[inline(always)]
    pub const fn log_data_1(&self) -> &LOG_DATA_1 {
        &self.log_data_1
    }
    #[doc = "0x134 - log check data register"]
    #[inline(always)]
    pub const fn log_data_2(&self) -> &LOG_DATA_2 {
        &self.log_data_2
    }
    #[doc = "0x138 - log check data register"]
    #[inline(always)]
    pub const fn log_data_3(&self) -> &LOG_DATA_3 {
        &self.log_data_3
    }
    #[doc = "0x13c - log check data mask register"]
    #[inline(always)]
    pub const fn log_data_mask(&self) -> &LOG_DATA_MASK {
        &self.log_data_mask
    }
    #[doc = "0x140 - log check region configuration register"]
    #[inline(always)]
    pub const fn log_min(&self) -> &LOG_MIN {
        &self.log_min
    }
    #[doc = "0x144 - log check region configuration register"]
    #[inline(always)]
    pub const fn log_max(&self) -> &LOG_MAX {
        &self.log_max
    }
    #[doc = "0x148 - log mem region configuration register"]
    #[inline(always)]
    pub const fn log_mem_start(&self) -> &LOG_MEM_START {
        &self.log_mem_start
    }
    #[doc = "0x14c - log mem region configuration register"]
    #[inline(always)]
    pub const fn log_mem_end(&self) -> &LOG_MEM_END {
        &self.log_mem_end
    }
    #[doc = "0x150 - log mem addr status register"]
    #[inline(always)]
    pub const fn log_mem_writing_addr(&self) -> &LOG_MEM_WRITING_ADDR {
        &self.log_mem_writing_addr
    }
    #[doc = "0x154 - log mem status register"]
    #[inline(always)]
    pub const fn log_mem_full_flag(&self) -> &LOG_MEM_FULL_FLAG {
        &self.log_mem_full_flag
    }
    #[doc = "0x1fc - version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CORE_0_MONTR_ENA (rw) register accessor: core0 monitor enable configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_montr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_montr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_montr_ena`] module"]
pub type CORE_0_MONTR_ENA = crate::Reg<core_0_montr_ena::CORE_0_MONTR_ENA_SPEC>;
#[doc = "core0 monitor enable configuration register"]
pub mod core_0_montr_ena;
#[doc = "CORE_0_INTR_RAW (r) register accessor: core0 monitor interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_raw`] module"]
pub type CORE_0_INTR_RAW = crate::Reg<core_0_intr_raw::CORE_0_INTR_RAW_SPEC>;
#[doc = "core0 monitor interrupt status register"]
pub mod core_0_intr_raw;
#[doc = "CORE_0_INTR_ENA (rw) register accessor: core0 monitor interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_ena`] module"]
pub type CORE_0_INTR_ENA = crate::Reg<core_0_intr_ena::CORE_0_INTR_ENA_SPEC>;
#[doc = "core0 monitor interrupt enable register"]
pub mod core_0_intr_ena;
#[doc = "CORE_0_INTR_CLR (rw) register accessor: core0 monitor interrupt clr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_intr_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_intr_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_intr_clr`] module"]
pub type CORE_0_INTR_CLR = crate::Reg<core_0_intr_clr::CORE_0_INTR_CLR_SPEC>;
#[doc = "core0 monitor interrupt clr register"]
pub mod core_0_intr_clr;
#[doc = "CORE_0_AREA_DRAM0_0_MIN (rw) register accessor: core0 dram0 region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_0_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_0_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_0_min`] module"]
pub type CORE_0_AREA_DRAM0_0_MIN =
    crate::Reg<core_0_area_dram0_0_min::CORE_0_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod core_0_area_dram0_0_min;
#[doc = "CORE_0_AREA_DRAM0_0_MAX (rw) register accessor: core0 dram0 region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_0_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_0_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_0_max`] module"]
pub type CORE_0_AREA_DRAM0_0_MAX =
    crate::Reg<core_0_area_dram0_0_max::CORE_0_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "core0 dram0 region0 addr configuration register"]
pub mod core_0_area_dram0_0_max;
#[doc = "CORE_0_AREA_DRAM0_1_MIN (rw) register accessor: core0 dram0 region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_1_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_1_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_1_min`] module"]
pub type CORE_0_AREA_DRAM0_1_MIN =
    crate::Reg<core_0_area_dram0_1_min::CORE_0_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod core_0_area_dram0_1_min;
#[doc = "CORE_0_AREA_DRAM0_1_MAX (rw) register accessor: core0 dram0 region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_dram0_1_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_dram0_1_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_dram0_1_max`] module"]
pub type CORE_0_AREA_DRAM0_1_MAX =
    crate::Reg<core_0_area_dram0_1_max::CORE_0_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "core0 dram0 region1 addr configuration register"]
pub mod core_0_area_dram0_1_max;
#[doc = "CORE_0_AREA_PIF_0_MIN (rw) register accessor: core0 PIF region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_0_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_0_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_0_min`] module"]
pub type CORE_0_AREA_PIF_0_MIN = crate::Reg<core_0_area_pif_0_min::CORE_0_AREA_PIF_0_MIN_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod core_0_area_pif_0_min;
#[doc = "CORE_0_AREA_PIF_0_MAX (rw) register accessor: core0 PIF region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_0_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_0_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_0_max`] module"]
pub type CORE_0_AREA_PIF_0_MAX = crate::Reg<core_0_area_pif_0_max::CORE_0_AREA_PIF_0_MAX_SPEC>;
#[doc = "core0 PIF region0 addr configuration register"]
pub mod core_0_area_pif_0_max;
#[doc = "CORE_0_AREA_PIF_1_MIN (rw) register accessor: core0 PIF region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_1_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_1_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_1_min`] module"]
pub type CORE_0_AREA_PIF_1_MIN = crate::Reg<core_0_area_pif_1_min::CORE_0_AREA_PIF_1_MIN_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod core_0_area_pif_1_min;
#[doc = "CORE_0_AREA_PIF_1_MAX (rw) register accessor: core0 PIF region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pif_1_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_area_pif_1_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pif_1_max`] module"]
pub type CORE_0_AREA_PIF_1_MAX = crate::Reg<core_0_area_pif_1_max::CORE_0_AREA_PIF_1_MAX_SPEC>;
#[doc = "core0 PIF region1 addr configuration register"]
pub mod core_0_area_pif_1_max;
#[doc = "CORE_0_AREA_SP (r) register accessor: core0 area sp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_sp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_sp`] module"]
pub type CORE_0_AREA_SP = crate::Reg<core_0_area_sp::CORE_0_AREA_SP_SPEC>;
#[doc = "core0 area sp status register"]
pub mod core_0_area_sp;
#[doc = "CORE_0_AREA_PC (r) register accessor: core0 area pc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_area_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_area_pc`] module"]
pub type CORE_0_AREA_PC = crate::Reg<core_0_area_pc::CORE_0_AREA_PC_SPEC>;
#[doc = "core0 area pc status register"]
pub mod core_0_area_pc;
#[doc = "CORE_0_SP_UNSTABLE (rw) register accessor: core0 sp unstable configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_unstable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_sp_unstable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_unstable`] module"]
pub type CORE_0_SP_UNSTABLE = crate::Reg<core_0_sp_unstable::CORE_0_SP_UNSTABLE_SPEC>;
#[doc = "core0 sp unstable configuration register"]
pub mod core_0_sp_unstable;
#[doc = "CORE_0_SP_MIN (rw) register accessor: core0 sp region configuration regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_sp_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_min`] module"]
pub type CORE_0_SP_MIN = crate::Reg<core_0_sp_min::CORE_0_SP_MIN_SPEC>;
#[doc = "core0 sp region configuration regsiter"]
pub mod core_0_sp_min;
#[doc = "CORE_0_SP_MAX (rw) register accessor: core0 sp region configuration regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_sp_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_max`] module"]
pub type CORE_0_SP_MAX = crate::Reg<core_0_sp_max::CORE_0_SP_MAX_SPEC>;
#[doc = "core0 sp region configuration regsiter"]
pub mod core_0_sp_max;
#[doc = "CORE_0_SP_PC (r) register accessor: core0 sp pc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_sp_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_sp_pc`] module"]
pub type CORE_0_SP_PC = crate::Reg<core_0_sp_pc::CORE_0_SP_PC_SPEC>;
#[doc = "core0 sp pc status register"]
pub mod core_0_sp_pc;
#[doc = "CORE_0_RCD_PDEBUGENABLE (rw) register accessor: core0 pdebug configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_pdebugenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugenable`] module"]
pub type CORE_0_RCD_PDEBUGENABLE =
    crate::Reg<core_0_rcd_pdebugenable::CORE_0_RCD_PDEBUGENABLE_SPEC>;
#[doc = "core0 pdebug configuration register"]
pub mod core_0_rcd_pdebugenable;
#[doc = "CORE_0_RCD_RECORDING (rw) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_recording::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_rcd_recording::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_recording`] module"]
pub type CORE_0_RCD_RECORDING = crate::Reg<core_0_rcd_recording::CORE_0_RCD_RECORDING_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_recording;
#[doc = "CORE_0_RCD_PDEBUGINST (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebuginst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebuginst`] module"]
pub type CORE_0_RCD_PDEBUGINST = crate::Reg<core_0_rcd_pdebuginst::CORE_0_RCD_PDEBUGINST_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebuginst;
#[doc = "CORE_0_RCD_PDEBUGSTATUS (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugstatus`] module"]
pub type CORE_0_RCD_PDEBUGSTATUS =
    crate::Reg<core_0_rcd_pdebugstatus::CORE_0_RCD_PDEBUGSTATUS_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugstatus;
#[doc = "CORE_0_RCD_PDEBUGDATA (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugdata`] module"]
pub type CORE_0_RCD_PDEBUGDATA = crate::Reg<core_0_rcd_pdebugdata::CORE_0_RCD_PDEBUGDATA_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugdata;
#[doc = "CORE_0_RCD_PDEBUGPC (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugpc`] module"]
pub type CORE_0_RCD_PDEBUGPC = crate::Reg<core_0_rcd_pdebugpc::CORE_0_RCD_PDEBUGPC_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugpc;
#[doc = "CORE_0_RCD_PDEBUGLS0STAT (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugls0stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugls0stat`] module"]
pub type CORE_0_RCD_PDEBUGLS0STAT =
    crate::Reg<core_0_rcd_pdebugls0stat::CORE_0_RCD_PDEBUGLS0STAT_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugls0stat;
#[doc = "CORE_0_RCD_PDEBUGLS0ADDR (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugls0addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugls0addr`] module"]
pub type CORE_0_RCD_PDEBUGLS0ADDR =
    crate::Reg<core_0_rcd_pdebugls0addr::CORE_0_RCD_PDEBUGLS0ADDR_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugls0addr;
#[doc = "CORE_0_RCD_PDEBUGLS0DATA (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_pdebugls0data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_pdebugls0data`] module"]
pub type CORE_0_RCD_PDEBUGLS0DATA =
    crate::Reg<core_0_rcd_pdebugls0data::CORE_0_RCD_PDEBUGLS0DATA_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_pdebugls0data;
#[doc = "CORE_0_RCD_SP (r) register accessor: core0 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_rcd_sp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_rcd_sp`] module"]
pub type CORE_0_RCD_SP = crate::Reg<core_0_rcd_sp::CORE_0_RCD_SP_SPEC>;
#[doc = "core0 pdebug status register"]
pub mod core_0_rcd_sp;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_0 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_exception_monitor_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_exception_monitor_0`] module"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_iram0_exception_monitor_0::CORE_0_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_iram0_exception_monitor_0;
#[doc = "CORE_0_IRAM0_EXCEPTION_MONITOR_1 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_iram0_exception_monitor_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_iram0_exception_monitor_1`] module"]
pub type CORE_0_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_iram0_exception_monitor_1::CORE_0_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_iram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_0 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_0`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_0_dram0_exception_monitor_0::CORE_0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_0;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_1 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_1`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_0_dram0_exception_monitor_1::CORE_0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_1;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_2 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_2`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_0_dram0_exception_monitor_2::CORE_0_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_2;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_3 (r) register accessor: core0 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_3`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_0_dram0_exception_monitor_3::CORE_0_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "core0 bus busy status regsiter"]
pub mod core_0_dram0_exception_monitor_3;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_4 (r) register accessor: core0 bus busy configuration regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_4`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_4 =
    crate::Reg<core_0_dram0_exception_monitor_4::CORE_0_DRAM0_EXCEPTION_MONITOR_4_SPEC>;
#[doc = "core0 bus busy configuration regsiter"]
pub mod core_0_dram0_exception_monitor_4;
#[doc = "CORE_0_DRAM0_EXCEPTION_MONITOR_5 (r) register accessor: core0 bus busy configuration regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_dram0_exception_monitor_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_0_dram0_exception_monitor_5`] module"]
pub type CORE_0_DRAM0_EXCEPTION_MONITOR_5 =
    crate::Reg<core_0_dram0_exception_monitor_5::CORE_0_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
#[doc = "core0 bus busy configuration regsiter"]
pub mod core_0_dram0_exception_monitor_5;
#[doc = "CORE_1_MONTR_ENA (rw) register accessor: Core1 monitor enable configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_montr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_montr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_montr_ena`] module"]
pub type CORE_1_MONTR_ENA = crate::Reg<core_1_montr_ena::CORE_1_MONTR_ENA_SPEC>;
#[doc = "Core1 monitor enable configuration register"]
pub mod core_1_montr_ena;
#[doc = "CORE_1_INTR_RAW (r) register accessor: Core1 monitor interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_intr_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_raw`] module"]
pub type CORE_1_INTR_RAW = crate::Reg<core_1_intr_raw::CORE_1_INTR_RAW_SPEC>;
#[doc = "Core1 monitor interrupt status register"]
pub mod core_1_intr_raw;
#[doc = "CORE_1_INTR_ENA (rw) register accessor: Core1 monitor interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_intr_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_intr_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_ena`] module"]
pub type CORE_1_INTR_ENA = crate::Reg<core_1_intr_ena::CORE_1_INTR_ENA_SPEC>;
#[doc = "Core1 monitor interrupt enable register"]
pub mod core_1_intr_ena;
#[doc = "CORE_1_INTR_CLR (rw) register accessor: Core1 monitor interrupt clr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_intr_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_intr_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_intr_clr`] module"]
pub type CORE_1_INTR_CLR = crate::Reg<core_1_intr_clr::CORE_1_INTR_CLR_SPEC>;
#[doc = "Core1 monitor interrupt clr register"]
pub mod core_1_intr_clr;
#[doc = "CORE_1_AREA_DRAM0_0_MIN (rw) register accessor: Core1 dram0 region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_dram0_0_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_dram0_0_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_dram0_0_min`] module"]
pub type CORE_1_AREA_DRAM0_0_MIN =
    crate::Reg<core_1_area_dram0_0_min::CORE_1_AREA_DRAM0_0_MIN_SPEC>;
#[doc = "Core1 dram0 region0 addr configuration register"]
pub mod core_1_area_dram0_0_min;
#[doc = "CORE_1_AREA_DRAM0_0_MAX (rw) register accessor: Core1 dram0 region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_dram0_0_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_dram0_0_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_dram0_0_max`] module"]
pub type CORE_1_AREA_DRAM0_0_MAX =
    crate::Reg<core_1_area_dram0_0_max::CORE_1_AREA_DRAM0_0_MAX_SPEC>;
#[doc = "Core1 dram0 region0 addr configuration register"]
pub mod core_1_area_dram0_0_max;
#[doc = "CORE_1_AREA_DRAM0_1_MIN (rw) register accessor: Core1 dram0 region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_dram0_1_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_dram0_1_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_dram0_1_min`] module"]
pub type CORE_1_AREA_DRAM0_1_MIN =
    crate::Reg<core_1_area_dram0_1_min::CORE_1_AREA_DRAM0_1_MIN_SPEC>;
#[doc = "Core1 dram0 region1 addr configuration register"]
pub mod core_1_area_dram0_1_min;
#[doc = "CORE_1_AREA_DRAM0_1_MAX (rw) register accessor: Core1 dram0 region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_dram0_1_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_dram0_1_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_dram0_1_max`] module"]
pub type CORE_1_AREA_DRAM0_1_MAX =
    crate::Reg<core_1_area_dram0_1_max::CORE_1_AREA_DRAM0_1_MAX_SPEC>;
#[doc = "Core1 dram0 region1 addr configuration register"]
pub mod core_1_area_dram0_1_max;
#[doc = "CORE_1_AREA_PIF_0_MIN (rw) register accessor: Core1 PIF region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_pif_0_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_pif_0_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_pif_0_min`] module"]
pub type CORE_1_AREA_PIF_0_MIN = crate::Reg<core_1_area_pif_0_min::CORE_1_AREA_PIF_0_MIN_SPEC>;
#[doc = "Core1 PIF region0 addr configuration register"]
pub mod core_1_area_pif_0_min;
#[doc = "CORE_1_AREA_PIF_0_MAX (rw) register accessor: Core1 PIF region0 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_pif_0_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_pif_0_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_pif_0_max`] module"]
pub type CORE_1_AREA_PIF_0_MAX = crate::Reg<core_1_area_pif_0_max::CORE_1_AREA_PIF_0_MAX_SPEC>;
#[doc = "Core1 PIF region0 addr configuration register"]
pub mod core_1_area_pif_0_max;
#[doc = "CORE_1_AREA_PIF_1_MIN (rw) register accessor: Core1 PIF region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_pif_1_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_pif_1_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_pif_1_min`] module"]
pub type CORE_1_AREA_PIF_1_MIN = crate::Reg<core_1_area_pif_1_min::CORE_1_AREA_PIF_1_MIN_SPEC>;
#[doc = "Core1 PIF region1 addr configuration register"]
pub mod core_1_area_pif_1_min;
#[doc = "CORE_1_AREA_PIF_1_MAX (rw) register accessor: Core1 PIF region1 addr configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_pif_1_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_area_pif_1_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_pif_1_max`] module"]
pub type CORE_1_AREA_PIF_1_MAX = crate::Reg<core_1_area_pif_1_max::CORE_1_AREA_PIF_1_MAX_SPEC>;
#[doc = "Core1 PIF region1 addr configuration register"]
pub mod core_1_area_pif_1_max;
#[doc = "CORE_1_AREA_PC (r) register accessor: Core1 area sp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_pc`] module"]
pub type CORE_1_AREA_PC = crate::Reg<core_1_area_pc::CORE_1_AREA_PC_SPEC>;
#[doc = "Core1 area sp status register"]
pub mod core_1_area_pc;
#[doc = "CORE_1_AREA_SP (r) register accessor: Core1 area pc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_area_sp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_area_sp`] module"]
pub type CORE_1_AREA_SP = crate::Reg<core_1_area_sp::CORE_1_AREA_SP_SPEC>;
#[doc = "Core1 area pc status register"]
pub mod core_1_area_sp;
#[doc = "CORE_1_SP_UNSTABLE (rw) register accessor: Core1 sp unstable configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_sp_unstable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_sp_unstable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_sp_unstable`] module"]
pub type CORE_1_SP_UNSTABLE = crate::Reg<core_1_sp_unstable::CORE_1_SP_UNSTABLE_SPEC>;
#[doc = "Core1 sp unstable configuration register"]
pub mod core_1_sp_unstable;
#[doc = "CORE_1_SP_MIN (rw) register accessor: Core1 sp region configuration regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_sp_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_sp_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_sp_min`] module"]
pub type CORE_1_SP_MIN = crate::Reg<core_1_sp_min::CORE_1_SP_MIN_SPEC>;
#[doc = "Core1 sp region configuration regsiter"]
pub mod core_1_sp_min;
#[doc = "CORE_1_SP_MAX (rw) register accessor: Core1 sp region configuration regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_sp_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_sp_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_sp_max`] module"]
pub type CORE_1_SP_MAX = crate::Reg<core_1_sp_max::CORE_1_SP_MAX_SPEC>;
#[doc = "Core1 sp region configuration regsiter"]
pub mod core_1_sp_max;
#[doc = "CORE_1_SP_PC (r) register accessor: Core1 sp pc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_sp_pc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_sp_pc`] module"]
pub type CORE_1_SP_PC = crate::Reg<core_1_sp_pc::CORE_1_SP_PC_SPEC>;
#[doc = "Core1 sp pc status register"]
pub mod core_1_sp_pc;
#[doc = "CORE_1_RCD_PDEBUGENABLE (rw) register accessor: Core1 pdebug configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugenable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_rcd_pdebugenable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebugenable`] module"]
pub type CORE_1_RCD_PDEBUGENABLE =
    crate::Reg<core_1_rcd_pdebugenable::CORE_1_RCD_PDEBUGENABLE_SPEC>;
#[doc = "Core1 pdebug configuration register"]
pub mod core_1_rcd_pdebugenable;
#[doc = "CORE_1_RCD_RECORDING (rw) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_recording::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_rcd_recording::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_recording`] module"]
pub type CORE_1_RCD_RECORDING = crate::Reg<core_1_rcd_recording::CORE_1_RCD_RECORDING_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_recording;
#[doc = "CORE_1_RCD_PDEBUGINST (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebuginst::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebuginst`] module"]
pub type CORE_1_RCD_PDEBUGINST = crate::Reg<core_1_rcd_pdebuginst::CORE_1_RCD_PDEBUGINST_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebuginst;
#[doc = "CORE_1_RCD_PDEBUGSTATUS (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebugstatus`] module"]
pub type CORE_1_RCD_PDEBUGSTATUS =
    crate::Reg<core_1_rcd_pdebugstatus::CORE_1_RCD_PDEBUGSTATUS_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugstatus;
#[doc = "CORE_1_RCD_PDEBUGDATA (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebugdata`] module"]
pub type CORE_1_RCD_PDEBUGDATA = crate::Reg<core_1_rcd_pdebugdata::CORE_1_RCD_PDEBUGDATA_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugdata;
#[doc = "CORE_1_RCD_PDEBUGPC (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugpc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebugpc`] module"]
pub type CORE_1_RCD_PDEBUGPC = crate::Reg<core_1_rcd_pdebugpc::CORE_1_RCD_PDEBUGPC_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugpc;
#[doc = "CORE_1_RCD_PDEBUGLS0STAT (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugls0stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebugls0stat`] module"]
pub type CORE_1_RCD_PDEBUGLS0STAT =
    crate::Reg<core_1_rcd_pdebugls0stat::CORE_1_RCD_PDEBUGLS0STAT_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugls0stat;
#[doc = "CORE_1_RCD_PDEBUGLS0ADDR (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugls0addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebugls0addr`] module"]
pub type CORE_1_RCD_PDEBUGLS0ADDR =
    crate::Reg<core_1_rcd_pdebugls0addr::CORE_1_RCD_PDEBUGLS0ADDR_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugls0addr;
#[doc = "CORE_1_RCD_PDEBUGLS0DATA (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_pdebugls0data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_pdebugls0data`] module"]
pub type CORE_1_RCD_PDEBUGLS0DATA =
    crate::Reg<core_1_rcd_pdebugls0data::CORE_1_RCD_PDEBUGLS0DATA_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_pdebugls0data;
#[doc = "CORE_1_RCD_SP (r) register accessor: Core1 pdebug status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_rcd_sp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_rcd_sp`] module"]
pub type CORE_1_RCD_SP = crate::Reg<core_1_rcd_sp::CORE_1_RCD_SP_SPEC>;
#[doc = "Core1 pdebug status register"]
pub mod core_1_rcd_sp;
#[doc = "CORE_1_IRAM0_EXCEPTION_MONITOR_0 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_exception_monitor_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_iram0_exception_monitor_0`] module"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_1_iram0_exception_monitor_0::CORE_1_IRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_iram0_exception_monitor_0;
#[doc = "CORE_1_IRAM0_EXCEPTION_MONITOR_1 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_iram0_exception_monitor_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_iram0_exception_monitor_1`] module"]
pub type CORE_1_IRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_1_iram0_exception_monitor_1::CORE_1_IRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_iram0_exception_monitor_1;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_0 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_exception_monitor_0`] module"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_1_dram0_exception_monitor_0::CORE_1_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_0;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_1 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_exception_monitor_1`] module"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_1_dram0_exception_monitor_1::CORE_1_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_1;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_2 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_exception_monitor_2`] module"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_2 =
    crate::Reg<core_1_dram0_exception_monitor_2::CORE_1_DRAM0_EXCEPTION_MONITOR_2_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_2;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_3 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_exception_monitor_3`] module"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_3 =
    crate::Reg<core_1_dram0_exception_monitor_3::CORE_1_DRAM0_EXCEPTION_MONITOR_3_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_3;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_4 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_exception_monitor_4`] module"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_4 =
    crate::Reg<core_1_dram0_exception_monitor_4::CORE_1_DRAM0_EXCEPTION_MONITOR_4_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_4;
#[doc = "CORE_1_DRAM0_EXCEPTION_MONITOR_5 (r) register accessor: Core1 bus busy status regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_dram0_exception_monitor_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_1_dram0_exception_monitor_5`] module"]
pub type CORE_1_DRAM0_EXCEPTION_MONITOR_5 =
    crate::Reg<core_1_dram0_exception_monitor_5::CORE_1_DRAM0_EXCEPTION_MONITOR_5_SPEC>;
#[doc = "Core1 bus busy status regsiter"]
pub mod core_1_dram0_exception_monitor_5;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 (rw) register accessor: bus busy configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor_0`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_0::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_0_SPEC>;
#[doc = "bus busy configuration register"]
pub mod core_x_iram0_dram0_exception_monitor_0;
#[doc = "CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 (rw) register accessor: bus busy configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_x_iram0_dram0_exception_monitor_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_x_iram0_dram0_exception_monitor_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core_x_iram0_dram0_exception_monitor_1`] module"]
pub type CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1 =
    crate::Reg<core_x_iram0_dram0_exception_monitor_1::CORE_X_IRAM0_DRAM0_EXCEPTION_MONITOR_1_SPEC>;
#[doc = "bus busy configuration register"]
pub mod core_x_iram0_dram0_exception_monitor_1;
#[doc = "LOG_SETTING (rw) register accessor: log set register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_setting::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_setting::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_setting`] module"]
pub type LOG_SETTING = crate::Reg<log_setting::LOG_SETTING_SPEC>;
#[doc = "log set register"]
pub mod log_setting;
#[doc = "LOG_DATA_0 (rw) register accessor: log check data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_0`] module"]
pub type LOG_DATA_0 = crate::Reg<log_data_0::LOG_DATA_0_SPEC>;
#[doc = "log check data register"]
pub mod log_data_0;
#[doc = "LOG_DATA_1 (rw) register accessor: log check data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_1`] module"]
pub type LOG_DATA_1 = crate::Reg<log_data_1::LOG_DATA_1_SPEC>;
#[doc = "log check data register"]
pub mod log_data_1;
#[doc = "LOG_DATA_2 (rw) register accessor: log check data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_2`] module"]
pub type LOG_DATA_2 = crate::Reg<log_data_2::LOG_DATA_2_SPEC>;
#[doc = "log check data register"]
pub mod log_data_2;
#[doc = "LOG_DATA_3 (rw) register accessor: log check data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_3`] module"]
pub type LOG_DATA_3 = crate::Reg<log_data_3::LOG_DATA_3_SPEC>;
#[doc = "log check data register"]
pub mod log_data_3;
#[doc = "LOG_DATA_MASK (rw) register accessor: log check data mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_data_mask`] module"]
pub type LOG_DATA_MASK = crate::Reg<log_data_mask::LOG_DATA_MASK_SPEC>;
#[doc = "log check data mask register"]
pub mod log_data_mask;
#[doc = "LOG_MIN (rw) register accessor: log check region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_min::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_min::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_min`] module"]
pub type LOG_MIN = crate::Reg<log_min::LOG_MIN_SPEC>;
#[doc = "log check region configuration register"]
pub mod log_min;
#[doc = "LOG_MAX (rw) register accessor: log check region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_max::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_max::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_max`] module"]
pub type LOG_MAX = crate::Reg<log_max::LOG_MAX_SPEC>;
#[doc = "log check region configuration register"]
pub mod log_max;
#[doc = "LOG_MEM_START (rw) register accessor: log mem region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_start::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_start::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_start`] module"]
pub type LOG_MEM_START = crate::Reg<log_mem_start::LOG_MEM_START_SPEC>;
#[doc = "log mem region configuration register"]
pub mod log_mem_start;
#[doc = "LOG_MEM_END (rw) register accessor: log mem region configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_end`] module"]
pub type LOG_MEM_END = crate::Reg<log_mem_end::LOG_MEM_END_SPEC>;
#[doc = "log mem region configuration register"]
pub mod log_mem_end;
#[doc = "LOG_MEM_WRITING_ADDR (r) register accessor: log mem addr status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_writing_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_writing_addr`] module"]
pub type LOG_MEM_WRITING_ADDR = crate::Reg<log_mem_writing_addr::LOG_MEM_WRITING_ADDR_SPEC>;
#[doc = "log mem addr status register"]
pub mod log_mem_writing_addr;
#[doc = "LOG_MEM_FULL_FLAG (rw) register accessor: log mem status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_mem_full_flag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_mem_full_flag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@log_mem_full_flag`] module"]
pub type LOG_MEM_FULL_FLAG = crate::Reg<log_mem_full_flag::LOG_MEM_FULL_FLAG_SPEC>;
#[doc = "log mem status register"]
pub mod log_mem_full_flag;
#[doc = "DATE (rw) register accessor: version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
