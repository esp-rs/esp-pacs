#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - cpu_peripheral clock gating register"]
    pub cpu_peri_clk_en: CPU_PERI_CLK_EN,
    #[doc = "0x04 - cpu_peripheral reset register"]
    pub cpu_peri_rst_en: CPU_PERI_RST_EN,
    #[doc = "0x08 - cpu clock config register"]
    pub cpu_per_conf: CPU_PER_CONF,
    #[doc = "0x0c - memory power down mask register"]
    pub mem_pd_mask: MEM_PD_MASK,
    #[doc = "0x10 - peripheral clock gating register"]
    pub perip_clk_en0: PERIP_CLK_EN0,
    #[doc = "0x14 - peripheral clock gating register"]
    pub perip_clk_en1: PERIP_CLK_EN1,
    #[doc = "0x18 - reserved"]
    pub perip_rst_en0: PERIP_RST_EN0,
    #[doc = "0x1c - peripheral reset register"]
    pub perip_rst_en1: PERIP_RST_EN1,
    #[doc = "0x20 - clock config register"]
    pub bt_lpck_div_int: BT_LPCK_DIV_INT,
    #[doc = "0x24 - clock config register"]
    pub bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    #[doc = "0x28 - interrupt generate register"]
    pub cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    #[doc = "0x2c - interrupt generate register"]
    pub cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    #[doc = "0x30 - interrupt generate register"]
    pub cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    #[doc = "0x34 - interrupt generate register"]
    pub cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    #[doc = "0x38 - rsa memory power control register"]
    pub rsa_pd_ctrl: RSA_PD_CTRL,
    #[doc = "0x3c - edma clcok and reset register"]
    pub edma_ctrl: EDMA_CTRL,
    #[doc = "0x40 - cache control register"]
    pub cache_control: CACHE_CONTROL,
    #[doc = "0x44 - SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG"]
    pub external_device_encrypt_decrypt_control: EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    #[doc = "0x48 - fast memory config register"]
    pub rtc_fastmem_config: RTC_FASTMEM_CONFIG,
    #[doc = "0x4c - reserved"]
    pub rtc_fastmem_crc: RTC_FASTMEM_CRC,
    #[doc = "0x50 - eco register"]
    pub redundant_eco_ctrl: REDUNDANT_ECO_CTRL,
    #[doc = "0x54 - clock gating register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x58 - system clock config register"]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x5c - mem pvt register"]
    pub mem_pvt: MEM_PVT,
    #[doc = "0x60 - mem pvt register"]
    pub comb_pvt_lvt_conf: COMB_PVT_LVT_CONF,
    #[doc = "0x64 - mem pvt register"]
    pub comb_pvt_nvt_conf: COMB_PVT_NVT_CONF,
    #[doc = "0x68 - mem pvt register"]
    pub comb_pvt_hvt_conf: COMB_PVT_HVT_CONF,
    #[doc = "0x6c - mem pvt register"]
    pub comb_pvt_err_lvt_site0: COMB_PVT_ERR_LVT_SITE0,
    #[doc = "0x70 - mem pvt register"]
    pub comb_pvt_err_nvt_site0: COMB_PVT_ERR_NVT_SITE0,
    #[doc = "0x74 - mem pvt register"]
    pub comb_pvt_err_hvt_site0: COMB_PVT_ERR_HVT_SITE0,
    #[doc = "0x78 - mem pvt register"]
    pub comb_pvt_err_lvt_site1: COMB_PVT_ERR_LVT_SITE1,
    #[doc = "0x7c - mem pvt register"]
    pub comb_pvt_err_nvt_site1: COMB_PVT_ERR_NVT_SITE1,
    #[doc = "0x80 - mem pvt register"]
    pub comb_pvt_err_hvt_site1: COMB_PVT_ERR_HVT_SITE1,
    #[doc = "0x84 - mem pvt register"]
    pub comb_pvt_err_lvt_site2: COMB_PVT_ERR_LVT_SITE2,
    #[doc = "0x88 - mem pvt register"]
    pub comb_pvt_err_nvt_site2: COMB_PVT_ERR_NVT_SITE2,
    #[doc = "0x8c - mem pvt register"]
    pub comb_pvt_err_hvt_site2: COMB_PVT_ERR_HVT_SITE2,
    #[doc = "0x90 - mem pvt register"]
    pub comb_pvt_err_lvt_site3: COMB_PVT_ERR_LVT_SITE3,
    #[doc = "0x94 - mem pvt register"]
    pub comb_pvt_err_nvt_site3: COMB_PVT_ERR_NVT_SITE3,
    #[doc = "0x98 - mem pvt register"]
    pub comb_pvt_err_hvt_site3: COMB_PVT_ERR_HVT_SITE3,
    _reserved39: [u8; 0x0f60],
    #[doc = "0xffc - Version register"]
    pub system_reg_date: SYSTEM_REG_DATE,
}
#[doc = "CPU_PERI_CLK_EN (rw) register accessor: an alias for `Reg<CPU_PERI_CLK_EN_SPEC>`"]
pub type CPU_PERI_CLK_EN = crate::Reg<cpu_peri_clk_en::CPU_PERI_CLK_EN_SPEC>;
#[doc = "cpu_peripheral clock gating register"]
pub mod cpu_peri_clk_en;
#[doc = "CPU_PERI_RST_EN (rw) register accessor: an alias for `Reg<CPU_PERI_RST_EN_SPEC>`"]
pub type CPU_PERI_RST_EN = crate::Reg<cpu_peri_rst_en::CPU_PERI_RST_EN_SPEC>;
#[doc = "cpu_peripheral reset register"]
pub mod cpu_peri_rst_en;
#[doc = "CPU_PER_CONF (rw) register accessor: an alias for `Reg<CPU_PER_CONF_SPEC>`"]
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
#[doc = "cpu clock config register"]
pub mod cpu_per_conf;
#[doc = "MEM_PD_MASK (rw) register accessor: an alias for `Reg<MEM_PD_MASK_SPEC>`"]
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
#[doc = "memory power down mask register"]
pub mod mem_pd_mask;
#[doc = "PERIP_CLK_EN0 (rw) register accessor: an alias for `Reg<PERIP_CLK_EN0_SPEC>`"]
pub type PERIP_CLK_EN0 = crate::Reg<perip_clk_en0::PERIP_CLK_EN0_SPEC>;
#[doc = "peripheral clock gating register"]
pub mod perip_clk_en0;
#[doc = "PERIP_CLK_EN1 (rw) register accessor: an alias for `Reg<PERIP_CLK_EN1_SPEC>`"]
pub type PERIP_CLK_EN1 = crate::Reg<perip_clk_en1::PERIP_CLK_EN1_SPEC>;
#[doc = "peripheral clock gating register"]
pub mod perip_clk_en1;
#[doc = "PERIP_RST_EN0 (rw) register accessor: an alias for `Reg<PERIP_RST_EN0_SPEC>`"]
pub type PERIP_RST_EN0 = crate::Reg<perip_rst_en0::PERIP_RST_EN0_SPEC>;
#[doc = "reserved"]
pub mod perip_rst_en0;
#[doc = "PERIP_RST_EN1 (rw) register accessor: an alias for `Reg<PERIP_RST_EN1_SPEC>`"]
pub type PERIP_RST_EN1 = crate::Reg<perip_rst_en1::PERIP_RST_EN1_SPEC>;
#[doc = "peripheral reset register"]
pub mod perip_rst_en1;
#[doc = "BT_LPCK_DIV_INT (rw) register accessor: an alias for `Reg<BT_LPCK_DIV_INT_SPEC>`"]
pub type BT_LPCK_DIV_INT = crate::Reg<bt_lpck_div_int::BT_LPCK_DIV_INT_SPEC>;
#[doc = "clock config register"]
pub mod bt_lpck_div_int;
#[doc = "BT_LPCK_DIV_FRAC (rw) register accessor: an alias for `Reg<BT_LPCK_DIV_FRAC_SPEC>`"]
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "clock config register"]
pub mod bt_lpck_div_frac;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "interrupt generate register"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "interrupt generate register"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "interrupt generate register"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "interrupt generate register"]
pub mod cpu_intr_from_cpu_3;
#[doc = "RSA_PD_CTRL (rw) register accessor: an alias for `Reg<RSA_PD_CTRL_SPEC>`"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = "rsa memory power control register"]
pub mod rsa_pd_ctrl;
#[doc = "EDMA_CTRL (rw) register accessor: an alias for `Reg<EDMA_CTRL_SPEC>`"]
pub type EDMA_CTRL = crate::Reg<edma_ctrl::EDMA_CTRL_SPEC>;
#[doc = "edma clcok and reset register"]
pub mod edma_ctrl;
#[doc = "CACHE_CONTROL (rw) register accessor: an alias for `Reg<CACHE_CONTROL_SPEC>`"]
pub type CACHE_CONTROL = crate::Reg<cache_control::CACHE_CONTROL_SPEC>;
#[doc = "cache control register"]
pub mod cache_control;
#[doc = "EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL (rw) register accessor: an alias for `Reg<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>`"]
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
#[doc = "SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG"]
pub mod external_device_encrypt_decrypt_control;
#[doc = "RTC_FASTMEM_CONFIG (rw) register accessor: an alias for `Reg<RTC_FASTMEM_CONFIG_SPEC>`"]
pub type RTC_FASTMEM_CONFIG = crate::Reg<rtc_fastmem_config::RTC_FASTMEM_CONFIG_SPEC>;
#[doc = "fast memory config register"]
pub mod rtc_fastmem_config;
#[doc = "RTC_FASTMEM_CRC (r) register accessor: an alias for `Reg<RTC_FASTMEM_CRC_SPEC>`"]
pub type RTC_FASTMEM_CRC = crate::Reg<rtc_fastmem_crc::RTC_FASTMEM_CRC_SPEC>;
#[doc = "reserved"]
pub mod rtc_fastmem_crc;
#[doc = "REDUNDANT_ECO_CTRL (rw) register accessor: an alias for `Reg<REDUNDANT_ECO_CTRL_SPEC>`"]
pub type REDUNDANT_ECO_CTRL = crate::Reg<redundant_eco_ctrl::REDUNDANT_ECO_CTRL_SPEC>;
#[doc = "eco register"]
pub mod redundant_eco_ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "clock gating register"]
pub mod clock_gate;
#[doc = "SYSCLK_CONF (rw) register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "system clock config register"]
pub mod sysclk_conf;
#[doc = "MEM_PVT (rw) register accessor: an alias for `Reg<MEM_PVT_SPEC>`"]
pub type MEM_PVT = crate::Reg<mem_pvt::MEM_PVT_SPEC>;
#[doc = "mem pvt register"]
pub mod mem_pvt;
#[doc = "COMB_PVT_LVT_CONF (rw) register accessor: an alias for `Reg<COMB_PVT_LVT_CONF_SPEC>`"]
pub type COMB_PVT_LVT_CONF = crate::Reg<comb_pvt_lvt_conf::COMB_PVT_LVT_CONF_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_lvt_conf;
#[doc = "COMB_PVT_NVT_CONF (rw) register accessor: an alias for `Reg<COMB_PVT_NVT_CONF_SPEC>`"]
pub type COMB_PVT_NVT_CONF = crate::Reg<comb_pvt_nvt_conf::COMB_PVT_NVT_CONF_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_nvt_conf;
#[doc = "COMB_PVT_HVT_CONF (rw) register accessor: an alias for `Reg<COMB_PVT_HVT_CONF_SPEC>`"]
pub type COMB_PVT_HVT_CONF = crate::Reg<comb_pvt_hvt_conf::COMB_PVT_HVT_CONF_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_hvt_conf;
#[doc = "COMB_PVT_ERR_LVT_SITE0 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE0 = crate::Reg<comb_pvt_err_lvt_site0::COMB_PVT_ERR_LVT_SITE0_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_lvt_site0;
#[doc = "COMB_PVT_ERR_NVT_SITE0 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE0 = crate::Reg<comb_pvt_err_nvt_site0::COMB_PVT_ERR_NVT_SITE0_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_nvt_site0;
#[doc = "COMB_PVT_ERR_HVT_SITE0 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE0 = crate::Reg<comb_pvt_err_hvt_site0::COMB_PVT_ERR_HVT_SITE0_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_hvt_site0;
#[doc = "COMB_PVT_ERR_LVT_SITE1 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE1 = crate::Reg<comb_pvt_err_lvt_site1::COMB_PVT_ERR_LVT_SITE1_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_lvt_site1;
#[doc = "COMB_PVT_ERR_NVT_SITE1 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE1 = crate::Reg<comb_pvt_err_nvt_site1::COMB_PVT_ERR_NVT_SITE1_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_nvt_site1;
#[doc = "COMB_PVT_ERR_HVT_SITE1 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE1 = crate::Reg<comb_pvt_err_hvt_site1::COMB_PVT_ERR_HVT_SITE1_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_hvt_site1;
#[doc = "COMB_PVT_ERR_LVT_SITE2 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE2 = crate::Reg<comb_pvt_err_lvt_site2::COMB_PVT_ERR_LVT_SITE2_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_lvt_site2;
#[doc = "COMB_PVT_ERR_NVT_SITE2 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE2 = crate::Reg<comb_pvt_err_nvt_site2::COMB_PVT_ERR_NVT_SITE2_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_nvt_site2;
#[doc = "COMB_PVT_ERR_HVT_SITE2 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE2 = crate::Reg<comb_pvt_err_hvt_site2::COMB_PVT_ERR_HVT_SITE2_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_hvt_site2;
#[doc = "COMB_PVT_ERR_LVT_SITE3 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE3 = crate::Reg<comb_pvt_err_lvt_site3::COMB_PVT_ERR_LVT_SITE3_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_lvt_site3;
#[doc = "COMB_PVT_ERR_NVT_SITE3 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE3 = crate::Reg<comb_pvt_err_nvt_site3::COMB_PVT_ERR_NVT_SITE3_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_nvt_site3;
#[doc = "COMB_PVT_ERR_HVT_SITE3 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE3 = crate::Reg<comb_pvt_err_hvt_site3::COMB_PVT_ERR_HVT_SITE3_SPEC>;
#[doc = "mem pvt register"]
pub mod comb_pvt_err_hvt_site3;
#[doc = "SYSTEM_REG_DATE (rw) register accessor: an alias for `Reg<SYSTEM_REG_DATE_SPEC>`"]
pub type SYSTEM_REG_DATE = crate::Reg<system_reg_date::SYSTEM_REG_DATE_SPEC>;
#[doc = "Version register"]
pub mod system_reg_date;
