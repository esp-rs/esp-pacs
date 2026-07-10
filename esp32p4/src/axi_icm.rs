#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    icm_ver_date: ICM_VER_DATE,
    icm_clk_en: ICM_CLK_EN,
    icm_dlock_status: ICM_DLOCK_STATUS,
    icm_int_raw: ICM_INT_RAW,
    icm_int_st: ICM_INT_ST,
    icm_int_ena: ICM_INT_ENA,
    icm_int_clr: ICM_INT_CLR,
    icm_mst_arb_priority: ICM_MST_ARB_PRIORITY,
    _reserved8: [u8; 0x04],
    icm_slv_arb_priority: ICM_SLV_ARB_PRIORITY,
    icm_mst_arqos: ICM_MST_ARQOS,
    _reserved10: [u8; 0x04],
    icm_mst_awqos: ICM_MST_AWQOS,
    _reserved11: [u8; 0x08],
    icm_sys_addrhole_info: ICM_SYS_ADDRHOLE_INFO,
    icm_cpu_addrhole_addr: ICM_CPU_ADDRHOLE_ADDR,
    icm_cpu_addrhole_info: ICM_CPU_ADDRHOLE_INFO,
    icm_dlock_timeout: ICM_DLOCK_TIMEOUT,
    _reserved15: [u8; 0x04],
    icm_rdn_eco_cs: ICM_RDN_ECO_CS,
    icm_rdn_eco_low: ICM_RDN_ECO_LOW,
    icm_rdn_eco_high: ICM_RDN_ECO_HIGH,
    _reserved18: [u8; 0x0333],
    icm_sys_addrhole_addr: ICM_SYS_ADDRHOLE_ADDR,
}
impl RegisterBlock {
    #[doc = "0x00 - ICM version / date"]
    #[inline(always)]
    pub const fn icm_ver_date(&self) -> &ICM_VER_DATE {
        &self.icm_ver_date
    }
    #[doc = "0x04 - ICM clock enable"]
    #[inline(always)]
    pub const fn icm_clk_en(&self) -> &ICM_CLK_EN {
        &self.icm_clk_en
    }
    #[doc = "0x08 - Deadlock status"]
    #[inline(always)]
    pub const fn icm_dlock_status(&self) -> &ICM_DLOCK_STATUS {
        &self.icm_dlock_status
    }
    #[doc = "0x0c - ICM interrupt raw"]
    #[inline(always)]
    pub const fn icm_int_raw(&self) -> &ICM_INT_RAW {
        &self.icm_int_raw
    }
    #[doc = "0x10 - ICM interrupt status"]
    #[inline(always)]
    pub const fn icm_int_st(&self) -> &ICM_INT_ST {
        &self.icm_int_st
    }
    #[doc = "0x14 - ICM interrupt enable"]
    #[inline(always)]
    pub const fn icm_int_ena(&self) -> &ICM_INT_ENA {
        &self.icm_int_ena
    }
    #[doc = "0x18 - ICM interrupt clear"]
    #[inline(always)]
    pub const fn icm_int_clr(&self) -> &ICM_INT_CLR {
        &self.icm_int_clr
    }
    #[doc = "0x1c - Master arbitration priority"]
    #[inline(always)]
    pub const fn icm_mst_arb_priority(&self) -> &ICM_MST_ARB_PRIORITY {
        &self.icm_mst_arb_priority
    }
    #[doc = "0x24 - Slave arbitration priority"]
    #[inline(always)]
    pub const fn icm_slv_arb_priority(&self) -> &ICM_SLV_ARB_PRIORITY {
        &self.icm_slv_arb_priority
    }
    #[doc = "0x28 - Master read QoS"]
    #[inline(always)]
    pub const fn icm_mst_arqos(&self) -> &ICM_MST_ARQOS {
        &self.icm_mst_arqos
    }
    #[doc = "0x30 - Master write QoS"]
    #[inline(always)]
    pub const fn icm_mst_awqos(&self) -> &ICM_MST_AWQOS {
        &self.icm_mst_awqos
    }
    #[doc = "0x3c - SYS address hole info"]
    #[inline(always)]
    pub const fn icm_sys_addrhole_info(&self) -> &ICM_SYS_ADDRHOLE_INFO {
        &self.icm_sys_addrhole_info
    }
    #[doc = "0x40 - CPU address hole address"]
    #[inline(always)]
    pub const fn icm_cpu_addrhole_addr(&self) -> &ICM_CPU_ADDRHOLE_ADDR {
        &self.icm_cpu_addrhole_addr
    }
    #[doc = "0x44 - CPU address hole info"]
    #[inline(always)]
    pub const fn icm_cpu_addrhole_info(&self) -> &ICM_CPU_ADDRHOLE_INFO {
        &self.icm_cpu_addrhole_info
    }
    #[doc = "0x48 - Deadlock timeout"]
    #[inline(always)]
    pub const fn icm_dlock_timeout(&self) -> &ICM_DLOCK_TIMEOUT {
        &self.icm_dlock_timeout
    }
    #[doc = "0x50 - RDN ECO control/status"]
    #[inline(always)]
    pub const fn icm_rdn_eco_cs(&self) -> &ICM_RDN_ECO_CS {
        &self.icm_rdn_eco_cs
    }
    #[doc = "0x54 - RDN ECO low"]
    #[inline(always)]
    pub const fn icm_rdn_eco_low(&self) -> &ICM_RDN_ECO_LOW {
        &self.icm_rdn_eco_low
    }
    #[doc = "0x58 - RDN ECO high"]
    #[inline(always)]
    pub const fn icm_rdn_eco_high(&self) -> &ICM_RDN_ECO_HIGH {
        &self.icm_rdn_eco_high
    }
    #[doc = "0x38f - SYS address hole address"]
    #[inline(always)]
    pub const fn icm_sys_addrhole_addr(&self) -> &ICM_SYS_ADDRHOLE_ADDR {
        &self.icm_sys_addrhole_addr
    }
}
#[doc = "ICM_VER_DATE (rw) register accessor: ICM version / date\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_ver_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_ver_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_ver_date`] module"]
pub type ICM_VER_DATE = crate::Reg<icm_ver_date::ICM_VER_DATE_SPEC>;
#[doc = "ICM version / date"]
pub mod icm_ver_date;
#[doc = "ICM_CLK_EN (rw) register accessor: ICM clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_clk_en`] module"]
pub type ICM_CLK_EN = crate::Reg<icm_clk_en::ICM_CLK_EN_SPEC>;
#[doc = "ICM clock enable"]
pub mod icm_clk_en;
#[doc = "ICM_DLOCK_STATUS (r) register accessor: Deadlock status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_dlock_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_dlock_status`] module"]
pub type ICM_DLOCK_STATUS = crate::Reg<icm_dlock_status::ICM_DLOCK_STATUS_SPEC>;
#[doc = "Deadlock status"]
pub mod icm_dlock_status;
#[doc = "ICM_INT_RAW (rw) register accessor: ICM interrupt raw\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_int_raw`] module"]
pub type ICM_INT_RAW = crate::Reg<icm_int_raw::ICM_INT_RAW_SPEC>;
#[doc = "ICM interrupt raw"]
pub mod icm_int_raw;
#[doc = "ICM_INT_ST (r) register accessor: ICM interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_int_st`] module"]
pub type ICM_INT_ST = crate::Reg<icm_int_st::ICM_INT_ST_SPEC>;
#[doc = "ICM interrupt status"]
pub mod icm_int_st;
#[doc = "ICM_INT_ENA (rw) register accessor: ICM interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_int_ena`] module"]
pub type ICM_INT_ENA = crate::Reg<icm_int_ena::ICM_INT_ENA_SPEC>;
#[doc = "ICM interrupt enable"]
pub mod icm_int_ena;
#[doc = "ICM_INT_CLR (w) register accessor: ICM interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_int_clr`] module"]
pub type ICM_INT_CLR = crate::Reg<icm_int_clr::ICM_INT_CLR_SPEC>;
#[doc = "ICM interrupt clear"]
pub mod icm_int_clr;
#[doc = "ICM_MST_ARB_PRIORITY (rw) register accessor: Master arbitration priority\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_mst_arb_priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_mst_arb_priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_mst_arb_priority`] module"]
pub type ICM_MST_ARB_PRIORITY = crate::Reg<icm_mst_arb_priority::ICM_MST_ARB_PRIORITY_SPEC>;
#[doc = "Master arbitration priority"]
pub mod icm_mst_arb_priority;
#[doc = "ICM_SLV_ARB_PRIORITY (rw) register accessor: Slave arbitration priority\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_slv_arb_priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_slv_arb_priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_slv_arb_priority`] module"]
pub type ICM_SLV_ARB_PRIORITY = crate::Reg<icm_slv_arb_priority::ICM_SLV_ARB_PRIORITY_SPEC>;
#[doc = "Slave arbitration priority"]
pub mod icm_slv_arb_priority;
#[doc = "ICM_MST_ARQOS (rw) register accessor: Master read QoS\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_mst_arqos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_mst_arqos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_mst_arqos`] module"]
pub type ICM_MST_ARQOS = crate::Reg<icm_mst_arqos::ICM_MST_ARQOS_SPEC>;
#[doc = "Master read QoS"]
pub mod icm_mst_arqos;
#[doc = "ICM_MST_AWQOS (rw) register accessor: Master write QoS\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_mst_awqos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_mst_awqos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_mst_awqos`] module"]
pub type ICM_MST_AWQOS = crate::Reg<icm_mst_awqos::ICM_MST_AWQOS_SPEC>;
#[doc = "Master write QoS"]
pub mod icm_mst_awqos;
#[doc = "ICM_SYS_ADDRHOLE_INFO (r) register accessor: SYS address hole info\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_sys_addrhole_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_sys_addrhole_info`] module"]
pub type ICM_SYS_ADDRHOLE_INFO = crate::Reg<icm_sys_addrhole_info::ICM_SYS_ADDRHOLE_INFO_SPEC>;
#[doc = "SYS address hole info"]
pub mod icm_sys_addrhole_info;
#[doc = "ICM_CPU_ADDRHOLE_ADDR (r) register accessor: CPU address hole address\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_cpu_addrhole_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_cpu_addrhole_addr`] module"]
pub type ICM_CPU_ADDRHOLE_ADDR = crate::Reg<icm_cpu_addrhole_addr::ICM_CPU_ADDRHOLE_ADDR_SPEC>;
#[doc = "CPU address hole address"]
pub mod icm_cpu_addrhole_addr;
#[doc = "ICM_CPU_ADDRHOLE_INFO (r) register accessor: CPU address hole info\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_cpu_addrhole_info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_cpu_addrhole_info`] module"]
pub type ICM_CPU_ADDRHOLE_INFO = crate::Reg<icm_cpu_addrhole_info::ICM_CPU_ADDRHOLE_INFO_SPEC>;
#[doc = "CPU address hole info"]
pub mod icm_cpu_addrhole_info;
#[doc = "ICM_DLOCK_TIMEOUT (rw) register accessor: Deadlock timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_dlock_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_dlock_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_dlock_timeout`] module"]
pub type ICM_DLOCK_TIMEOUT = crate::Reg<icm_dlock_timeout::ICM_DLOCK_TIMEOUT_SPEC>;
#[doc = "Deadlock timeout"]
pub mod icm_dlock_timeout;
#[doc = "ICM_RDN_ECO_CS (rw) register accessor: RDN ECO control/status\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_rdn_eco_cs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_rdn_eco_cs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_rdn_eco_cs`] module"]
pub type ICM_RDN_ECO_CS = crate::Reg<icm_rdn_eco_cs::ICM_RDN_ECO_CS_SPEC>;
#[doc = "RDN ECO control/status"]
pub mod icm_rdn_eco_cs;
#[doc = "ICM_RDN_ECO_LOW (rw) register accessor: RDN ECO low\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_rdn_eco_low::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_rdn_eco_low::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_rdn_eco_low`] module"]
pub type ICM_RDN_ECO_LOW = crate::Reg<icm_rdn_eco_low::ICM_RDN_ECO_LOW_SPEC>;
#[doc = "RDN ECO low"]
pub mod icm_rdn_eco_low;
#[doc = "ICM_RDN_ECO_HIGH (rw) register accessor: RDN ECO high\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_rdn_eco_high::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_rdn_eco_high::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_rdn_eco_high`] module"]
pub type ICM_RDN_ECO_HIGH = crate::Reg<icm_rdn_eco_high::ICM_RDN_ECO_HIGH_SPEC>;
#[doc = "RDN ECO high"]
pub mod icm_rdn_eco_high;
#[doc = "ICM_SYS_ADDRHOLE_ADDR (r) register accessor: SYS address hole address\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_sys_addrhole_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icm_sys_addrhole_addr`] module"]
pub type ICM_SYS_ADDRHOLE_ADDR = crate::Reg<icm_sys_addrhole_addr::ICM_SYS_ADDRHOLE_ADDR_SPEC>;
#[doc = "SYS address hole address"]
pub mod icm_sys_addrhole_addr;
