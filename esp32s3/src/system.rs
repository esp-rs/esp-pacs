#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Core0 control regiter 0"]
    pub core_1_control_0: CORE_1_CONTROL_0,
    #[doc = "0x04 - Core0 control regiter 1"]
    pub core_1_control_1: CORE_1_CONTROL_1,
    #[doc = "0x08 - cpu_peripheral clock configuration register"]
    pub cpu_peri_clk_en: CPU_PERI_CLK_EN,
    #[doc = "0x0c - cpu_peripheral reset configuration regsiter"]
    pub cpu_peri_rst_en: CPU_PERI_RST_EN,
    #[doc = "0x10 - cpu peripheral clock configuration register"]
    pub cpu_per_conf: CPU_PER_CONF,
    #[doc = "0x14 - memory power down mask configuration register"]
    pub mem_pd_mask: MEM_PD_MASK,
    #[doc = "0x18 - peripheral clock configuration regsiter 0"]
    pub perip_clk_en0: PERIP_CLK_EN0,
    #[doc = "0x1c - peripheral clock configuration regsiter 1"]
    pub perip_clk_en1: PERIP_CLK_EN1,
    #[doc = "0x20 - peripheral reset configuration register0"]
    pub perip_rst_en0: PERIP_RST_EN0,
    #[doc = "0x24 - peripheral reset configuration regsiter 1"]
    pub perip_rst_en1: PERIP_RST_EN1,
    #[doc = "0x28 - low power clock frequent division factor configuration regsiter"]
    pub bt_lpck_div_int: BT_LPCK_DIV_INT,
    #[doc = "0x2c - low power clock configuration register"]
    pub bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    #[doc = "0x30 - interrupt source register 0"]
    pub cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    #[doc = "0x34 - interrupt source register 1"]
    pub cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    #[doc = "0x38 - interrupt source register 2"]
    pub cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    #[doc = "0x3c - interrupt source register 3"]
    pub cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    #[doc = "0x40 - rsa memory power control register"]
    pub rsa_pd_ctrl: RSA_PD_CTRL,
    #[doc = "0x44 - EDMA control register"]
    pub edma_ctrl: EDMA_CTRL,
    #[doc = "0x48 - Cache control register"]
    pub cache_control: CACHE_CONTROL,
    #[doc = "0x4c - External memory encrypt and decrypt control register"]
    pub external_device_encrypt_decrypt_control: EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    #[doc = "0x50 - RTC fast memory configuration register"]
    pub rtc_fastmem_config: RTC_FASTMEM_CONFIG,
    #[doc = "0x54 - RTC fast memory CRC control register"]
    pub rtc_fastmem_crc: RTC_FASTMEM_CRC,
    #[doc = "0x58 - ******* Description ***********"]
    pub redundant_eco_ctrl: REDUNDANT_ECO_CTRL,
    #[doc = "0x5c - ******* Description ***********"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x60 - System clock configuration register."]
    pub sysclk_conf: SYSCLK_CONF,
    #[doc = "0x64 - ******* Description ***********"]
    pub mem_pvt: MEM_PVT,
    #[doc = "0x68 - ******* Description ***********"]
    pub comb_pvt_lvt_conf: COMB_PVT_LVT_CONF,
    #[doc = "0x6c - ******* Description ***********"]
    pub comb_pvt_nvt_conf: COMB_PVT_NVT_CONF,
    #[doc = "0x70 - ******* Description ***********"]
    pub comb_pvt_hvt_conf: COMB_PVT_HVT_CONF,
    #[doc = "0x74 - ******* Description ***********"]
    pub comb_pvt_err_lvt_site0: COMB_PVT_ERR_LVT_SITE0,
    #[doc = "0x78 - ******* Description ***********"]
    pub comb_pvt_err_nvt_site0: COMB_PVT_ERR_NVT_SITE0,
    #[doc = "0x7c - ******* Description ***********"]
    pub comb_pvt_err_hvt_site0: COMB_PVT_ERR_HVT_SITE0,
    #[doc = "0x80 - ******* Description ***********"]
    pub comb_pvt_err_lvt_site1: COMB_PVT_ERR_LVT_SITE1,
    #[doc = "0x84 - ******* Description ***********"]
    pub comb_pvt_err_nvt_site1: COMB_PVT_ERR_NVT_SITE1,
    #[doc = "0x88 - ******* Description ***********"]
    pub comb_pvt_err_hvt_site1: COMB_PVT_ERR_HVT_SITE1,
    #[doc = "0x8c - ******* Description ***********"]
    pub comb_pvt_err_lvt_site2: COMB_PVT_ERR_LVT_SITE2,
    #[doc = "0x90 - ******* Description ***********"]
    pub comb_pvt_err_nvt_site2: COMB_PVT_ERR_NVT_SITE2,
    #[doc = "0x94 - ******* Description ***********"]
    pub comb_pvt_err_hvt_site2: COMB_PVT_ERR_HVT_SITE2,
    #[doc = "0x98 - ******* Description ***********"]
    pub comb_pvt_err_lvt_site3: COMB_PVT_ERR_LVT_SITE3,
    #[doc = "0x9c - ******* Description ***********"]
    pub comb_pvt_err_nvt_site3: COMB_PVT_ERR_NVT_SITE3,
    #[doc = "0xa0 - ******* Description ***********"]
    pub comb_pvt_err_hvt_site3: COMB_PVT_ERR_HVT_SITE3,
    _reserved41: [u8; 0x0f58],
    #[doc = "0xffc - version register"]
    pub date: DATE,
}
#[doc = "CORE_1_CONTROL_0 (rw) register accessor: an alias for `Reg<CORE_1_CONTROL_0_SPEC>`"]
pub type CORE_1_CONTROL_0 = crate::Reg<core_1_control_0::CORE_1_CONTROL_0_SPEC>;
#[doc = "Core0 control regiter 0"]
pub mod core_1_control_0;
#[doc = "CORE_1_CONTROL_1 (rw) register accessor: an alias for `Reg<CORE_1_CONTROL_1_SPEC>`"]
pub type CORE_1_CONTROL_1 = crate::Reg<core_1_control_1::CORE_1_CONTROL_1_SPEC>;
#[doc = "Core0 control regiter 1"]
pub mod core_1_control_1;
#[doc = "CPU_PERI_CLK_EN (rw) register accessor: an alias for `Reg<CPU_PERI_CLK_EN_SPEC>`"]
pub type CPU_PERI_CLK_EN = crate::Reg<cpu_peri_clk_en::CPU_PERI_CLK_EN_SPEC>;
#[doc = "cpu_peripheral clock configuration register"]
pub mod cpu_peri_clk_en;
#[doc = "CPU_PERI_RST_EN (rw) register accessor: an alias for `Reg<CPU_PERI_RST_EN_SPEC>`"]
pub type CPU_PERI_RST_EN = crate::Reg<cpu_peri_rst_en::CPU_PERI_RST_EN_SPEC>;
#[doc = "cpu_peripheral reset configuration regsiter"]
pub mod cpu_peri_rst_en;
#[doc = "CPU_PER_CONF (rw) register accessor: an alias for `Reg<CPU_PER_CONF_SPEC>`"]
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
#[doc = "cpu peripheral clock configuration register"]
pub mod cpu_per_conf;
#[doc = "MEM_PD_MASK (rw) register accessor: an alias for `Reg<MEM_PD_MASK_SPEC>`"]
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
#[doc = "memory power down mask configuration register"]
pub mod mem_pd_mask;
#[doc = "PERIP_CLK_EN0 (rw) register accessor: an alias for `Reg<PERIP_CLK_EN0_SPEC>`"]
pub type PERIP_CLK_EN0 = crate::Reg<perip_clk_en0::PERIP_CLK_EN0_SPEC>;
#[doc = "peripheral clock configuration regsiter 0"]
pub mod perip_clk_en0;
#[doc = "PERIP_CLK_EN1 (rw) register accessor: an alias for `Reg<PERIP_CLK_EN1_SPEC>`"]
pub type PERIP_CLK_EN1 = crate::Reg<perip_clk_en1::PERIP_CLK_EN1_SPEC>;
#[doc = "peripheral clock configuration regsiter 1"]
pub mod perip_clk_en1;
#[doc = "PERIP_RST_EN0 (rw) register accessor: an alias for `Reg<PERIP_RST_EN0_SPEC>`"]
pub type PERIP_RST_EN0 = crate::Reg<perip_rst_en0::PERIP_RST_EN0_SPEC>;
#[doc = "peripheral reset configuration register0"]
pub mod perip_rst_en0;
#[doc = "PERIP_RST_EN1 (rw) register accessor: an alias for `Reg<PERIP_RST_EN1_SPEC>`"]
pub type PERIP_RST_EN1 = crate::Reg<perip_rst_en1::PERIP_RST_EN1_SPEC>;
#[doc = "peripheral reset configuration regsiter 1"]
pub mod perip_rst_en1;
#[doc = "BT_LPCK_DIV_INT (rw) register accessor: an alias for `Reg<BT_LPCK_DIV_INT_SPEC>`"]
pub type BT_LPCK_DIV_INT = crate::Reg<bt_lpck_div_int::BT_LPCK_DIV_INT_SPEC>;
#[doc = "low power clock frequent division factor configuration regsiter"]
pub mod bt_lpck_div_int;
#[doc = "BT_LPCK_DIV_FRAC (rw) register accessor: an alias for `Reg<BT_LPCK_DIV_FRAC_SPEC>`"]
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "low power clock configuration register"]
pub mod bt_lpck_div_frac;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "interrupt source register 0"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "interrupt source register 1"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "interrupt source register 2"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "interrupt source register 3"]
pub mod cpu_intr_from_cpu_3;
#[doc = "RSA_PD_CTRL (rw) register accessor: an alias for `Reg<RSA_PD_CTRL_SPEC>`"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = "rsa memory power control register"]
pub mod rsa_pd_ctrl;
#[doc = "EDMA_CTRL (rw) register accessor: an alias for `Reg<EDMA_CTRL_SPEC>`"]
pub type EDMA_CTRL = crate::Reg<edma_ctrl::EDMA_CTRL_SPEC>;
#[doc = "EDMA control register"]
pub mod edma_ctrl;
#[doc = "CACHE_CONTROL (rw) register accessor: an alias for `Reg<CACHE_CONTROL_SPEC>`"]
pub type CACHE_CONTROL = crate::Reg<cache_control::CACHE_CONTROL_SPEC>;
#[doc = "Cache control register"]
pub mod cache_control;
#[doc = "EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL (rw) register accessor: an alias for `Reg<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>`"]
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
#[doc = "External memory encrypt and decrypt control register"]
pub mod external_device_encrypt_decrypt_control;
#[doc = "RTC_FASTMEM_CONFIG (rw) register accessor: an alias for `Reg<RTC_FASTMEM_CONFIG_SPEC>`"]
pub type RTC_FASTMEM_CONFIG = crate::Reg<rtc_fastmem_config::RTC_FASTMEM_CONFIG_SPEC>;
#[doc = "RTC fast memory configuration register"]
pub mod rtc_fastmem_config;
#[doc = "RTC_FASTMEM_CRC (r) register accessor: an alias for `Reg<RTC_FASTMEM_CRC_SPEC>`"]
pub type RTC_FASTMEM_CRC = crate::Reg<rtc_fastmem_crc::RTC_FASTMEM_CRC_SPEC>;
#[doc = "RTC fast memory CRC control register"]
pub mod rtc_fastmem_crc;
#[doc = "REDUNDANT_ECO_CTRL (rw) register accessor: an alias for `Reg<REDUNDANT_ECO_CTRL_SPEC>`"]
pub type REDUNDANT_ECO_CTRL = crate::Reg<redundant_eco_ctrl::REDUNDANT_ECO_CTRL_SPEC>;
#[doc = "******* Description ***********"]
pub mod redundant_eco_ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "******* Description ***********"]
pub mod clock_gate;
#[doc = "SYSCLK_CONF (rw) register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "System clock configuration register."]
pub mod sysclk_conf;
#[doc = "MEM_PVT (rw) register accessor: an alias for `Reg<MEM_PVT_SPEC>`"]
pub type MEM_PVT = crate::Reg<mem_pvt::MEM_PVT_SPEC>;
#[doc = "******* Description ***********"]
pub mod mem_pvt;
#[doc = "COMB_PVT_LVT_CONF (rw) register accessor: an alias for `Reg<COMB_PVT_LVT_CONF_SPEC>`"]
pub type COMB_PVT_LVT_CONF = crate::Reg<comb_pvt_lvt_conf::COMB_PVT_LVT_CONF_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_lvt_conf;
#[doc = "COMB_PVT_NVT_CONF (rw) register accessor: an alias for `Reg<COMB_PVT_NVT_CONF_SPEC>`"]
pub type COMB_PVT_NVT_CONF = crate::Reg<comb_pvt_nvt_conf::COMB_PVT_NVT_CONF_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_nvt_conf;
#[doc = "COMB_PVT_HVT_CONF (rw) register accessor: an alias for `Reg<COMB_PVT_HVT_CONF_SPEC>`"]
pub type COMB_PVT_HVT_CONF = crate::Reg<comb_pvt_hvt_conf::COMB_PVT_HVT_CONF_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_hvt_conf;
#[doc = "COMB_PVT_ERR_LVT_SITE0 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE0 = crate::Reg<comb_pvt_err_lvt_site0::COMB_PVT_ERR_LVT_SITE0_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_lvt_site0;
#[doc = "COMB_PVT_ERR_NVT_SITE0 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE0 = crate::Reg<comb_pvt_err_nvt_site0::COMB_PVT_ERR_NVT_SITE0_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_nvt_site0;
#[doc = "COMB_PVT_ERR_HVT_SITE0 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE0_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE0 = crate::Reg<comb_pvt_err_hvt_site0::COMB_PVT_ERR_HVT_SITE0_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_hvt_site0;
#[doc = "COMB_PVT_ERR_LVT_SITE1 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE1 = crate::Reg<comb_pvt_err_lvt_site1::COMB_PVT_ERR_LVT_SITE1_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_lvt_site1;
#[doc = "COMB_PVT_ERR_NVT_SITE1 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE1 = crate::Reg<comb_pvt_err_nvt_site1::COMB_PVT_ERR_NVT_SITE1_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_nvt_site1;
#[doc = "COMB_PVT_ERR_HVT_SITE1 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE1_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE1 = crate::Reg<comb_pvt_err_hvt_site1::COMB_PVT_ERR_HVT_SITE1_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_hvt_site1;
#[doc = "COMB_PVT_ERR_LVT_SITE2 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE2 = crate::Reg<comb_pvt_err_lvt_site2::COMB_PVT_ERR_LVT_SITE2_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_lvt_site2;
#[doc = "COMB_PVT_ERR_NVT_SITE2 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE2 = crate::Reg<comb_pvt_err_nvt_site2::COMB_PVT_ERR_NVT_SITE2_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_nvt_site2;
#[doc = "COMB_PVT_ERR_HVT_SITE2 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE2_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE2 = crate::Reg<comb_pvt_err_hvt_site2::COMB_PVT_ERR_HVT_SITE2_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_hvt_site2;
#[doc = "COMB_PVT_ERR_LVT_SITE3 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_LVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_LVT_SITE3 = crate::Reg<comb_pvt_err_lvt_site3::COMB_PVT_ERR_LVT_SITE3_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_lvt_site3;
#[doc = "COMB_PVT_ERR_NVT_SITE3 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_NVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_NVT_SITE3 = crate::Reg<comb_pvt_err_nvt_site3::COMB_PVT_ERR_NVT_SITE3_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_nvt_site3;
#[doc = "COMB_PVT_ERR_HVT_SITE3 (r) register accessor: an alias for `Reg<COMB_PVT_ERR_HVT_SITE3_SPEC>`"]
pub type COMB_PVT_ERR_HVT_SITE3 = crate::Reg<comb_pvt_err_hvt_site3::COMB_PVT_ERR_HVT_SITE3_SPEC>;
#[doc = "******* Description ***********"]
pub mod comb_pvt_err_hvt_site3;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "version register"]
pub mod date;
