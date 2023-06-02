#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - System ROM configuration register 0"]
    pub rom_ctrl_0: ROM_CTRL_0,
    #[doc = "0x04 - System ROM configuration register 1"]
    pub rom_ctrl_1: ROM_CTRL_1,
    #[doc = "0x08 - System SRAM configuration register 0"]
    pub sram_ctrl_0: SRAM_CTRL_0,
    #[doc = "0x0c - System SRAM configuration register 1"]
    pub sram_ctrl_1: SRAM_CTRL_1,
    #[doc = "0x10 - CPU peripheral clock enable register"]
    pub cpu_peri_clk_en: CPU_PERI_CLK_EN,
    #[doc = "0x14 - CPU peripheral reset register"]
    pub cpu_peri_rst_en: CPU_PERI_RST_EN,
    #[doc = "0x18 - CPU peripheral clock configuration register"]
    pub cpu_per_conf: CPU_PER_CONF,
    #[doc = "0x1c - JTAG configuration register 0"]
    pub jtag_ctrl_0: JTAG_CTRL_0,
    #[doc = "0x20 - JTAG configuration register 1"]
    pub jtag_ctrl_1: JTAG_CTRL_1,
    #[doc = "0x24 - JTAG configuration register 2"]
    pub jtag_ctrl_2: JTAG_CTRL_2,
    #[doc = "0x28 - JTAG configuration register 3"]
    pub jtag_ctrl_3: JTAG_CTRL_3,
    #[doc = "0x2c - JTAG configuration register 4"]
    pub jtag_ctrl_4: JTAG_CTRL_4,
    #[doc = "0x30 - JTAG configuration register 5"]
    pub jtag_ctrl_5: JTAG_CTRL_5,
    #[doc = "0x34 - JTAG configuration register 6"]
    pub jtag_ctrl_6: JTAG_CTRL_6,
    #[doc = "0x38 - JTAG configuration register 7"]
    pub jtag_ctrl_7: JTAG_CTRL_7,
    #[doc = "0x3c - Memory power-related controlling register (under low-sleep)"]
    pub mem_pd_mask: MEM_PD_MASK,
    #[doc = "0x40 - System peripheral clock (for hardware accelerators) enable register"]
    pub perip_clk_en0: PERIP_CLK_EN0,
    #[doc = "0x44 - System peripheral clock (for hardware accelerators) enable register 1"]
    pub perip_clk_en1: PERIP_CLK_EN1,
    #[doc = "0x48 - System peripheral (hardware accelerators) reset register 0"]
    pub perip_rst_en0: PERIP_RST_EN0,
    #[doc = "0x4c - System peripheral (hardware accelerators) reset register 1"]
    pub perip_rst_en1: PERIP_RST_EN1,
    #[doc = "0x50 - Low power clock divider integer register"]
    pub lpck_div_int: LPCK_DIV_INT,
    #[doc = "0x54 - Divider fraction configuration register for low-power clock"]
    pub bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    #[doc = "0x58 - CPU interrupt controlling register 0"]
    pub cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    #[doc = "0x5c - CPU interrupt controlling register 1"]
    pub cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    #[doc = "0x60 - CPU interrupt controlling register 2"]
    pub cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    #[doc = "0x64 - CPU interrupt controlling register 3"]
    pub cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    #[doc = "0x68 - RSA memory remapping register"]
    pub rsa_pd_ctrl: RSA_PD_CTRL,
    #[doc = "0x6c - EDMA enable register"]
    pub bustoextmem_ena: BUSTOEXTMEM_ENA,
    #[doc = "0x70 - Cache control register"]
    pub cache_control: CACHE_CONTROL,
    #[doc = "0x74 - External memory encrypt and decrypt controlling register"]
    pub external_device_encrypt_decrypt_control: EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    #[doc = "0x78 - RTC fast memory configuration register"]
    pub rtc_fastmem_config: RTC_FASTMEM_CONFIG,
    #[doc = "0x7c - RTC fast memory CRC controlling register"]
    pub rtc_fastmem_crc: RTC_FASTMEM_CRC,
    #[doc = "0x80 - Redundant ECO control register"]
    pub redundant_eco_ctrl: REDUNDANT_ECO_CTRL,
    #[doc = "0x84 - Clock gate control register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x88 - System SRAM configuration register 2"]
    pub sram_ctrl_2: SRAM_CTRL_2,
    #[doc = "0x8c - SoC clock configuration register"]
    pub sysclk_conf: SYSCLK_CONF,
    _reserved36: [u8; 0x0f6c],
    #[doc = "0xffc - Version control register"]
    pub date: DATE,
}
#[doc = "ROM_CTRL_0 (rw) register accessor: an alias for `Reg<ROM_CTRL_0_SPEC>`"]
pub type ROM_CTRL_0 = crate::Reg<rom_ctrl_0::ROM_CTRL_0_SPEC>;
#[doc = "System ROM configuration register 0"]
pub mod rom_ctrl_0;
#[doc = "ROM_CTRL_1 (rw) register accessor: an alias for `Reg<ROM_CTRL_1_SPEC>`"]
pub type ROM_CTRL_1 = crate::Reg<rom_ctrl_1::ROM_CTRL_1_SPEC>;
#[doc = "System ROM configuration register 1"]
pub mod rom_ctrl_1;
#[doc = "SRAM_CTRL_0 (rw) register accessor: an alias for `Reg<SRAM_CTRL_0_SPEC>`"]
pub type SRAM_CTRL_0 = crate::Reg<sram_ctrl_0::SRAM_CTRL_0_SPEC>;
#[doc = "System SRAM configuration register 0"]
pub mod sram_ctrl_0;
#[doc = "SRAM_CTRL_1 (rw) register accessor: an alias for `Reg<SRAM_CTRL_1_SPEC>`"]
pub type SRAM_CTRL_1 = crate::Reg<sram_ctrl_1::SRAM_CTRL_1_SPEC>;
#[doc = "System SRAM configuration register 1"]
pub mod sram_ctrl_1;
#[doc = "CPU_PERI_CLK_EN (rw) register accessor: an alias for `Reg<CPU_PERI_CLK_EN_SPEC>`"]
pub type CPU_PERI_CLK_EN = crate::Reg<cpu_peri_clk_en::CPU_PERI_CLK_EN_SPEC>;
#[doc = "CPU peripheral clock enable register"]
pub mod cpu_peri_clk_en;
#[doc = "CPU_PERI_RST_EN (rw) register accessor: an alias for `Reg<CPU_PERI_RST_EN_SPEC>`"]
pub type CPU_PERI_RST_EN = crate::Reg<cpu_peri_rst_en::CPU_PERI_RST_EN_SPEC>;
#[doc = "CPU peripheral reset register"]
pub mod cpu_peri_rst_en;
#[doc = "CPU_PER_CONF (rw) register accessor: an alias for `Reg<CPU_PER_CONF_SPEC>`"]
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
#[doc = "CPU peripheral clock configuration register"]
pub mod cpu_per_conf;
#[doc = "JTAG_CTRL_0 (w) register accessor: an alias for `Reg<JTAG_CTRL_0_SPEC>`"]
pub type JTAG_CTRL_0 = crate::Reg<jtag_ctrl_0::JTAG_CTRL_0_SPEC>;
#[doc = "JTAG configuration register 0"]
pub mod jtag_ctrl_0;
#[doc = "JTAG_CTRL_1 (w) register accessor: an alias for `Reg<JTAG_CTRL_1_SPEC>`"]
pub type JTAG_CTRL_1 = crate::Reg<jtag_ctrl_1::JTAG_CTRL_1_SPEC>;
#[doc = "JTAG configuration register 1"]
pub mod jtag_ctrl_1;
#[doc = "JTAG_CTRL_2 (w) register accessor: an alias for `Reg<JTAG_CTRL_2_SPEC>`"]
pub type JTAG_CTRL_2 = crate::Reg<jtag_ctrl_2::JTAG_CTRL_2_SPEC>;
#[doc = "JTAG configuration register 2"]
pub mod jtag_ctrl_2;
#[doc = "JTAG_CTRL_3 (w) register accessor: an alias for `Reg<JTAG_CTRL_3_SPEC>`"]
pub type JTAG_CTRL_3 = crate::Reg<jtag_ctrl_3::JTAG_CTRL_3_SPEC>;
#[doc = "JTAG configuration register 3"]
pub mod jtag_ctrl_3;
#[doc = "JTAG_CTRL_4 (w) register accessor: an alias for `Reg<JTAG_CTRL_4_SPEC>`"]
pub type JTAG_CTRL_4 = crate::Reg<jtag_ctrl_4::JTAG_CTRL_4_SPEC>;
#[doc = "JTAG configuration register 4"]
pub mod jtag_ctrl_4;
#[doc = "JTAG_CTRL_5 (w) register accessor: an alias for `Reg<JTAG_CTRL_5_SPEC>`"]
pub type JTAG_CTRL_5 = crate::Reg<jtag_ctrl_5::JTAG_CTRL_5_SPEC>;
#[doc = "JTAG configuration register 5"]
pub mod jtag_ctrl_5;
#[doc = "JTAG_CTRL_6 (w) register accessor: an alias for `Reg<JTAG_CTRL_6_SPEC>`"]
pub type JTAG_CTRL_6 = crate::Reg<jtag_ctrl_6::JTAG_CTRL_6_SPEC>;
#[doc = "JTAG configuration register 6"]
pub mod jtag_ctrl_6;
#[doc = "JTAG_CTRL_7 (w) register accessor: an alias for `Reg<JTAG_CTRL_7_SPEC>`"]
pub type JTAG_CTRL_7 = crate::Reg<jtag_ctrl_7::JTAG_CTRL_7_SPEC>;
#[doc = "JTAG configuration register 7"]
pub mod jtag_ctrl_7;
#[doc = "MEM_PD_MASK (rw) register accessor: an alias for `Reg<MEM_PD_MASK_SPEC>`"]
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
#[doc = "Memory power-related controlling register (under low-sleep)"]
pub mod mem_pd_mask;
#[doc = "PERIP_CLK_EN0 (rw) register accessor: an alias for `Reg<PERIP_CLK_EN0_SPEC>`"]
pub type PERIP_CLK_EN0 = crate::Reg<perip_clk_en0::PERIP_CLK_EN0_SPEC>;
#[doc = "System peripheral clock (for hardware accelerators) enable register"]
pub mod perip_clk_en0;
#[doc = "PERIP_CLK_EN1 (rw) register accessor: an alias for `Reg<PERIP_CLK_EN1_SPEC>`"]
pub type PERIP_CLK_EN1 = crate::Reg<perip_clk_en1::PERIP_CLK_EN1_SPEC>;
#[doc = "System peripheral clock (for hardware accelerators) enable register 1"]
pub mod perip_clk_en1;
#[doc = "PERIP_RST_EN0 (rw) register accessor: an alias for `Reg<PERIP_RST_EN0_SPEC>`"]
pub type PERIP_RST_EN0 = crate::Reg<perip_rst_en0::PERIP_RST_EN0_SPEC>;
#[doc = "System peripheral (hardware accelerators) reset register 0"]
pub mod perip_rst_en0;
#[doc = "PERIP_RST_EN1 (rw) register accessor: an alias for `Reg<PERIP_RST_EN1_SPEC>`"]
pub type PERIP_RST_EN1 = crate::Reg<perip_rst_en1::PERIP_RST_EN1_SPEC>;
#[doc = "System peripheral (hardware accelerators) reset register 1"]
pub mod perip_rst_en1;
#[doc = "LPCK_DIV_INT (rw) register accessor: an alias for `Reg<LPCK_DIV_INT_SPEC>`"]
pub type LPCK_DIV_INT = crate::Reg<lpck_div_int::LPCK_DIV_INT_SPEC>;
#[doc = "Low power clock divider integer register"]
pub mod lpck_div_int;
#[doc = "BT_LPCK_DIV_FRAC (rw) register accessor: an alias for `Reg<BT_LPCK_DIV_FRAC_SPEC>`"]
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
#[doc = "Divider fraction configuration register for low-power clock"]
pub mod bt_lpck_div_frac;
#[doc = "CPU_INTR_FROM_CPU_0 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_0_SPEC>`"]
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
#[doc = "CPU interrupt controlling register 0"]
pub mod cpu_intr_from_cpu_0;
#[doc = "CPU_INTR_FROM_CPU_1 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_1_SPEC>`"]
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
#[doc = "CPU interrupt controlling register 1"]
pub mod cpu_intr_from_cpu_1;
#[doc = "CPU_INTR_FROM_CPU_2 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_2_SPEC>`"]
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
#[doc = "CPU interrupt controlling register 2"]
pub mod cpu_intr_from_cpu_2;
#[doc = "CPU_INTR_FROM_CPU_3 (rw) register accessor: an alias for `Reg<CPU_INTR_FROM_CPU_3_SPEC>`"]
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
#[doc = "CPU interrupt controlling register 3"]
pub mod cpu_intr_from_cpu_3;
#[doc = "RSA_PD_CTRL (rw) register accessor: an alias for `Reg<RSA_PD_CTRL_SPEC>`"]
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
#[doc = "RSA memory remapping register"]
pub mod rsa_pd_ctrl;
#[doc = "BUSTOEXTMEM_ENA (rw) register accessor: an alias for `Reg<BUSTOEXTMEM_ENA_SPEC>`"]
pub type BUSTOEXTMEM_ENA = crate::Reg<bustoextmem_ena::BUSTOEXTMEM_ENA_SPEC>;
#[doc = "EDMA enable register"]
pub mod bustoextmem_ena;
#[doc = "CACHE_CONTROL (rw) register accessor: an alias for `Reg<CACHE_CONTROL_SPEC>`"]
pub type CACHE_CONTROL = crate::Reg<cache_control::CACHE_CONTROL_SPEC>;
#[doc = "Cache control register"]
pub mod cache_control;
#[doc = "EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL (rw) register accessor: an alias for `Reg<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>`"]
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
#[doc = "External memory encrypt and decrypt controlling register"]
pub mod external_device_encrypt_decrypt_control;
#[doc = "RTC_FASTMEM_CONFIG (rw) register accessor: an alias for `Reg<RTC_FASTMEM_CONFIG_SPEC>`"]
pub type RTC_FASTMEM_CONFIG = crate::Reg<rtc_fastmem_config::RTC_FASTMEM_CONFIG_SPEC>;
#[doc = "RTC fast memory configuration register"]
pub mod rtc_fastmem_config;
#[doc = "RTC_FASTMEM_CRC (r) register accessor: an alias for `Reg<RTC_FASTMEM_CRC_SPEC>`"]
pub type RTC_FASTMEM_CRC = crate::Reg<rtc_fastmem_crc::RTC_FASTMEM_CRC_SPEC>;
#[doc = "RTC fast memory CRC controlling register"]
pub mod rtc_fastmem_crc;
#[doc = "Redundant_ECO_Ctrl (rw) register accessor: an alias for `Reg<REDUNDANT_ECO_CTRL_SPEC>`"]
pub type REDUNDANT_ECO_CTRL = crate::Reg<redundant_eco_ctrl::REDUNDANT_ECO_CTRL_SPEC>;
#[doc = "Redundant ECO control register"]
pub mod redundant_eco_ctrl;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "Clock gate control register"]
pub mod clock_gate;
#[doc = "SRAM_CTRL_2 (rw) register accessor: an alias for `Reg<SRAM_CTRL_2_SPEC>`"]
pub type SRAM_CTRL_2 = crate::Reg<sram_ctrl_2::SRAM_CTRL_2_SPEC>;
#[doc = "System SRAM configuration register 2"]
pub mod sram_ctrl_2;
#[doc = "SYSCLK_CONF (rw) register accessor: an alias for `Reg<SYSCLK_CONF_SPEC>`"]
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
#[doc = "SoC clock configuration register"]
pub mod sysclk_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
