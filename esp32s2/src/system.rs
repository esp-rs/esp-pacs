#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    rom_ctrl_0: ROM_CTRL_0,
    rom_ctrl_1: ROM_CTRL_1,
    sram_ctrl_0: SRAM_CTRL_0,
    sram_ctrl_1: SRAM_CTRL_1,
    cpu_peri_clk_en: CPU_PERI_CLK_EN,
    cpu_peri_rst_en: CPU_PERI_RST_EN,
    cpu_per_conf: CPU_PER_CONF,
    jtag_ctrl_0: JTAG_CTRL_0,
    jtag_ctrl_1: JTAG_CTRL_1,
    jtag_ctrl_2: JTAG_CTRL_2,
    jtag_ctrl_3: JTAG_CTRL_3,
    jtag_ctrl_4: JTAG_CTRL_4,
    jtag_ctrl_5: JTAG_CTRL_5,
    jtag_ctrl_6: JTAG_CTRL_6,
    jtag_ctrl_7: JTAG_CTRL_7,
    mem_pd_mask: MEM_PD_MASK,
    perip_clk_en0: PERIP_CLK_EN0,
    perip_clk_en1: PERIP_CLK_EN1,
    perip_rst_en0: PERIP_RST_EN0,
    perip_rst_en1: PERIP_RST_EN1,
    lpck_div_int: LPCK_DIV_INT,
    bt_lpck_div_frac: BT_LPCK_DIV_FRAC,
    cpu_intr_from_cpu_0: CPU_INTR_FROM_CPU_0,
    cpu_intr_from_cpu_1: CPU_INTR_FROM_CPU_1,
    cpu_intr_from_cpu_2: CPU_INTR_FROM_CPU_2,
    cpu_intr_from_cpu_3: CPU_INTR_FROM_CPU_3,
    rsa_pd_ctrl: RSA_PD_CTRL,
    bustoextmem_ena: BUSTOEXTMEM_ENA,
    cache_control: CACHE_CONTROL,
    external_device_encrypt_decrypt_control: EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    rtc_fastmem_config: RTC_FASTMEM_CONFIG,
    rtc_fastmem_crc: RTC_FASTMEM_CRC,
    redundant_eco_ctrl: REDUNDANT_ECO_CTRL,
    clock_gate: CLOCK_GATE,
    sram_ctrl_2: SRAM_CTRL_2,
    sysclk_conf: SYSCLK_CONF,
    _reserved36: [u8; 0x0f6c],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - System ROM configuration register 0
    #[inline(always)]
    pub const fn rom_ctrl_0(&self) -> &ROM_CTRL_0 {
        &self.rom_ctrl_0
    }
    ///0x04 - System ROM configuration register 1
    #[inline(always)]
    pub const fn rom_ctrl_1(&self) -> &ROM_CTRL_1 {
        &self.rom_ctrl_1
    }
    ///0x08 - System SRAM configuration register 0
    #[inline(always)]
    pub const fn sram_ctrl_0(&self) -> &SRAM_CTRL_0 {
        &self.sram_ctrl_0
    }
    ///0x0c - System SRAM configuration register 1
    #[inline(always)]
    pub const fn sram_ctrl_1(&self) -> &SRAM_CTRL_1 {
        &self.sram_ctrl_1
    }
    ///0x10 - CPU peripheral clock enable register
    #[inline(always)]
    pub const fn cpu_peri_clk_en(&self) -> &CPU_PERI_CLK_EN {
        &self.cpu_peri_clk_en
    }
    ///0x14 - CPU peripheral reset register
    #[inline(always)]
    pub const fn cpu_peri_rst_en(&self) -> &CPU_PERI_RST_EN {
        &self.cpu_peri_rst_en
    }
    ///0x18 - CPU peripheral clock configuration register
    #[inline(always)]
    pub const fn cpu_per_conf(&self) -> &CPU_PER_CONF {
        &self.cpu_per_conf
    }
    ///0x1c - JTAG configuration register 0
    #[inline(always)]
    pub const fn jtag_ctrl_0(&self) -> &JTAG_CTRL_0 {
        &self.jtag_ctrl_0
    }
    ///0x20 - JTAG configuration register 1
    #[inline(always)]
    pub const fn jtag_ctrl_1(&self) -> &JTAG_CTRL_1 {
        &self.jtag_ctrl_1
    }
    ///0x24 - JTAG configuration register 2
    #[inline(always)]
    pub const fn jtag_ctrl_2(&self) -> &JTAG_CTRL_2 {
        &self.jtag_ctrl_2
    }
    ///0x28 - JTAG configuration register 3
    #[inline(always)]
    pub const fn jtag_ctrl_3(&self) -> &JTAG_CTRL_3 {
        &self.jtag_ctrl_3
    }
    ///0x2c - JTAG configuration register 4
    #[inline(always)]
    pub const fn jtag_ctrl_4(&self) -> &JTAG_CTRL_4 {
        &self.jtag_ctrl_4
    }
    ///0x30 - JTAG configuration register 5
    #[inline(always)]
    pub const fn jtag_ctrl_5(&self) -> &JTAG_CTRL_5 {
        &self.jtag_ctrl_5
    }
    ///0x34 - JTAG configuration register 6
    #[inline(always)]
    pub const fn jtag_ctrl_6(&self) -> &JTAG_CTRL_6 {
        &self.jtag_ctrl_6
    }
    ///0x38 - JTAG configuration register 7
    #[inline(always)]
    pub const fn jtag_ctrl_7(&self) -> &JTAG_CTRL_7 {
        &self.jtag_ctrl_7
    }
    ///0x3c - Memory power-related controlling register (under low-sleep)
    #[inline(always)]
    pub const fn mem_pd_mask(&self) -> &MEM_PD_MASK {
        &self.mem_pd_mask
    }
    ///0x40 - System peripheral clock (for hardware accelerators) enable register
    #[inline(always)]
    pub const fn perip_clk_en0(&self) -> &PERIP_CLK_EN0 {
        &self.perip_clk_en0
    }
    ///0x44 - System peripheral clock (for hardware accelerators) enable register 1
    #[inline(always)]
    pub const fn perip_clk_en1(&self) -> &PERIP_CLK_EN1 {
        &self.perip_clk_en1
    }
    ///0x48 - System peripheral (hardware accelerators) reset register 0
    #[inline(always)]
    pub const fn perip_rst_en0(&self) -> &PERIP_RST_EN0 {
        &self.perip_rst_en0
    }
    ///0x4c - System peripheral (hardware accelerators) reset register 1
    #[inline(always)]
    pub const fn perip_rst_en1(&self) -> &PERIP_RST_EN1 {
        &self.perip_rst_en1
    }
    ///0x50 - Low power clock divider integer register
    #[inline(always)]
    pub const fn lpck_div_int(&self) -> &LPCK_DIV_INT {
        &self.lpck_div_int
    }
    ///0x54 - Divider fraction configuration register for low-power clock
    #[inline(always)]
    pub const fn bt_lpck_div_frac(&self) -> &BT_LPCK_DIV_FRAC {
        &self.bt_lpck_div_frac
    }
    ///0x58 - CPU interrupt controlling register 0
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_0(&self) -> &CPU_INTR_FROM_CPU_0 {
        &self.cpu_intr_from_cpu_0
    }
    ///0x5c - CPU interrupt controlling register 1
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_1(&self) -> &CPU_INTR_FROM_CPU_1 {
        &self.cpu_intr_from_cpu_1
    }
    ///0x60 - CPU interrupt controlling register 2
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_2(&self) -> &CPU_INTR_FROM_CPU_2 {
        &self.cpu_intr_from_cpu_2
    }
    ///0x64 - CPU interrupt controlling register 3
    #[inline(always)]
    pub const fn cpu_intr_from_cpu_3(&self) -> &CPU_INTR_FROM_CPU_3 {
        &self.cpu_intr_from_cpu_3
    }
    ///0x68 - RSA memory remapping register
    #[inline(always)]
    pub const fn rsa_pd_ctrl(&self) -> &RSA_PD_CTRL {
        &self.rsa_pd_ctrl
    }
    ///0x6c - EDMA enable register
    #[inline(always)]
    pub const fn bustoextmem_ena(&self) -> &BUSTOEXTMEM_ENA {
        &self.bustoextmem_ena
    }
    ///0x70 - Cache control register
    #[inline(always)]
    pub const fn cache_control(&self) -> &CACHE_CONTROL {
        &self.cache_control
    }
    ///0x74 - External memory encrypt and decrypt controlling register
    #[inline(always)]
    pub const fn external_device_encrypt_decrypt_control(
        &self,
    ) -> &EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL {
        &self.external_device_encrypt_decrypt_control
    }
    ///0x78 - RTC fast memory configuration register
    #[inline(always)]
    pub const fn rtc_fastmem_config(&self) -> &RTC_FASTMEM_CONFIG {
        &self.rtc_fastmem_config
    }
    ///0x7c - RTC fast memory CRC controlling register
    #[inline(always)]
    pub const fn rtc_fastmem_crc(&self) -> &RTC_FASTMEM_CRC {
        &self.rtc_fastmem_crc
    }
    ///0x80 - Redundant ECO control register
    #[inline(always)]
    pub const fn redundant_eco_ctrl(&self) -> &REDUNDANT_ECO_CTRL {
        &self.redundant_eco_ctrl
    }
    ///0x84 - Clock gate control register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x88 - System SRAM configuration register 2
    #[inline(always)]
    pub const fn sram_ctrl_2(&self) -> &SRAM_CTRL_2 {
        &self.sram_ctrl_2
    }
    ///0x8c - SoC clock configuration register
    #[inline(always)]
    pub const fn sysclk_conf(&self) -> &SYSCLK_CONF {
        &self.sysclk_conf
    }
    ///0xffc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**ROM_CTRL_0 (rw) register accessor: System ROM configuration register 0

You can [`read`](crate::generic::Reg::read) this register and get [`rom_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rom_ctrl_0`] module*/
pub type ROM_CTRL_0 = crate::Reg<rom_ctrl_0::ROM_CTRL_0_SPEC>;
///System ROM configuration register 0
pub mod rom_ctrl_0;
/**ROM_CTRL_1 (rw) register accessor: System ROM configuration register 1

You can [`read`](crate::generic::Reg::read) this register and get [`rom_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rom_ctrl_1`] module*/
pub type ROM_CTRL_1 = crate::Reg<rom_ctrl_1::ROM_CTRL_1_SPEC>;
///System ROM configuration register 1
pub mod rom_ctrl_1;
/**SRAM_CTRL_0 (rw) register accessor: System SRAM configuration register 0

You can [`read`](crate::generic::Reg::read) this register and get [`sram_ctrl_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ctrl_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_ctrl_0`] module*/
pub type SRAM_CTRL_0 = crate::Reg<sram_ctrl_0::SRAM_CTRL_0_SPEC>;
///System SRAM configuration register 0
pub mod sram_ctrl_0;
/**SRAM_CTRL_1 (rw) register accessor: System SRAM configuration register 1

You can [`read`](crate::generic::Reg::read) this register and get [`sram_ctrl_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ctrl_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_ctrl_1`] module*/
pub type SRAM_CTRL_1 = crate::Reg<sram_ctrl_1::SRAM_CTRL_1_SPEC>;
///System SRAM configuration register 1
pub mod sram_ctrl_1;
/**CPU_PERI_CLK_EN (rw) register accessor: CPU peripheral clock enable register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_peri_clk_en`] module*/
pub type CPU_PERI_CLK_EN = crate::Reg<cpu_peri_clk_en::CPU_PERI_CLK_EN_SPEC>;
///CPU peripheral clock enable register
pub mod cpu_peri_clk_en;
/**CPU_PERI_RST_EN (rw) register accessor: CPU peripheral reset register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_peri_rst_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_peri_rst_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_peri_rst_en`] module*/
pub type CPU_PERI_RST_EN = crate::Reg<cpu_peri_rst_en::CPU_PERI_RST_EN_SPEC>;
///CPU peripheral reset register
pub mod cpu_peri_rst_en;
/**CPU_PER_CONF (rw) register accessor: CPU peripheral clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_per_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_per_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_per_conf`] module*/
pub type CPU_PER_CONF = crate::Reg<cpu_per_conf::CPU_PER_CONF_SPEC>;
///CPU peripheral clock configuration register
pub mod cpu_per_conf;
/**JTAG_CTRL_0 (w) register accessor: JTAG configuration register 0

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_0`] module*/
pub type JTAG_CTRL_0 = crate::Reg<jtag_ctrl_0::JTAG_CTRL_0_SPEC>;
///JTAG configuration register 0
pub mod jtag_ctrl_0;
/**JTAG_CTRL_1 (w) register accessor: JTAG configuration register 1

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_1`] module*/
pub type JTAG_CTRL_1 = crate::Reg<jtag_ctrl_1::JTAG_CTRL_1_SPEC>;
///JTAG configuration register 1
pub mod jtag_ctrl_1;
/**JTAG_CTRL_2 (w) register accessor: JTAG configuration register 2

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_2`] module*/
pub type JTAG_CTRL_2 = crate::Reg<jtag_ctrl_2::JTAG_CTRL_2_SPEC>;
///JTAG configuration register 2
pub mod jtag_ctrl_2;
/**JTAG_CTRL_3 (w) register accessor: JTAG configuration register 3

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_3`] module*/
pub type JTAG_CTRL_3 = crate::Reg<jtag_ctrl_3::JTAG_CTRL_3_SPEC>;
///JTAG configuration register 3
pub mod jtag_ctrl_3;
/**JTAG_CTRL_4 (w) register accessor: JTAG configuration register 4

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_4`] module*/
pub type JTAG_CTRL_4 = crate::Reg<jtag_ctrl_4::JTAG_CTRL_4_SPEC>;
///JTAG configuration register 4
pub mod jtag_ctrl_4;
/**JTAG_CTRL_5 (w) register accessor: JTAG configuration register 5

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_5`] module*/
pub type JTAG_CTRL_5 = crate::Reg<jtag_ctrl_5::JTAG_CTRL_5_SPEC>;
///JTAG configuration register 5
pub mod jtag_ctrl_5;
/**JTAG_CTRL_6 (w) register accessor: JTAG configuration register 6

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_6`] module*/
pub type JTAG_CTRL_6 = crate::Reg<jtag_ctrl_6::JTAG_CTRL_6_SPEC>;
///JTAG configuration register 6
pub mod jtag_ctrl_6;
/**JTAG_CTRL_7 (w) register accessor: JTAG configuration register 7

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtag_ctrl_7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@jtag_ctrl_7`] module*/
pub type JTAG_CTRL_7 = crate::Reg<jtag_ctrl_7::JTAG_CTRL_7_SPEC>;
///JTAG configuration register 7
pub mod jtag_ctrl_7;
/**MEM_PD_MASK (rw) register accessor: Memory power-related controlling register (under low-sleep)

You can [`read`](crate::generic::Reg::read) this register and get [`mem_pd_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pd_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_pd_mask`] module*/
pub type MEM_PD_MASK = crate::Reg<mem_pd_mask::MEM_PD_MASK_SPEC>;
///Memory power-related controlling register (under low-sleep)
pub mod mem_pd_mask;
/**PERIP_CLK_EN0 (rw) register accessor: System peripheral clock (for hardware accelerators) enable register

You can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_clk_en0`] module*/
pub type PERIP_CLK_EN0 = crate::Reg<perip_clk_en0::PERIP_CLK_EN0_SPEC>;
///System peripheral clock (for hardware accelerators) enable register
pub mod perip_clk_en0;
/**PERIP_CLK_EN1 (rw) register accessor: System peripheral clock (for hardware accelerators) enable register 1

You can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_clk_en1`] module*/
pub type PERIP_CLK_EN1 = crate::Reg<perip_clk_en1::PERIP_CLK_EN1_SPEC>;
///System peripheral clock (for hardware accelerators) enable register 1
pub mod perip_clk_en1;
/**PERIP_RST_EN0 (rw) register accessor: System peripheral (hardware accelerators) reset register 0

You can [`read`](crate::generic::Reg::read) this register and get [`perip_rst_en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_rst_en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_rst_en0`] module*/
pub type PERIP_RST_EN0 = crate::Reg<perip_rst_en0::PERIP_RST_EN0_SPEC>;
///System peripheral (hardware accelerators) reset register 0
pub mod perip_rst_en0;
/**PERIP_RST_EN1 (rw) register accessor: System peripheral (hardware accelerators) reset register 1

You can [`read`](crate::generic::Reg::read) this register and get [`perip_rst_en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_rst_en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@perip_rst_en1`] module*/
pub type PERIP_RST_EN1 = crate::Reg<perip_rst_en1::PERIP_RST_EN1_SPEC>;
///System peripheral (hardware accelerators) reset register 1
pub mod perip_rst_en1;
/**LPCK_DIV_INT (rw) register accessor: Low power clock divider integer register

You can [`read`](crate::generic::Reg::read) this register and get [`lpck_div_int::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpck_div_int::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpck_div_int`] module*/
pub type LPCK_DIV_INT = crate::Reg<lpck_div_int::LPCK_DIV_INT_SPEC>;
///Low power clock divider integer register
pub mod lpck_div_int;
/**BT_LPCK_DIV_FRAC (rw) register accessor: Divider fraction configuration register for low-power clock

You can [`read`](crate::generic::Reg::read) this register and get [`bt_lpck_div_frac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_lpck_div_frac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_lpck_div_frac`] module*/
pub type BT_LPCK_DIV_FRAC = crate::Reg<bt_lpck_div_frac::BT_LPCK_DIV_FRAC_SPEC>;
///Divider fraction configuration register for low-power clock
pub mod bt_lpck_div_frac;
/**CPU_INTR_FROM_CPU_0 (rw) register accessor: CPU interrupt controlling register 0

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_0`] module*/
pub type CPU_INTR_FROM_CPU_0 = crate::Reg<cpu_intr_from_cpu_0::CPU_INTR_FROM_CPU_0_SPEC>;
///CPU interrupt controlling register 0
pub mod cpu_intr_from_cpu_0;
/**CPU_INTR_FROM_CPU_1 (rw) register accessor: CPU interrupt controlling register 1

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_1`] module*/
pub type CPU_INTR_FROM_CPU_1 = crate::Reg<cpu_intr_from_cpu_1::CPU_INTR_FROM_CPU_1_SPEC>;
///CPU interrupt controlling register 1
pub mod cpu_intr_from_cpu_1;
/**CPU_INTR_FROM_CPU_2 (rw) register accessor: CPU interrupt controlling register 2

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_2`] module*/
pub type CPU_INTR_FROM_CPU_2 = crate::Reg<cpu_intr_from_cpu_2::CPU_INTR_FROM_CPU_2_SPEC>;
///CPU interrupt controlling register 2
pub mod cpu_intr_from_cpu_2;
/**CPU_INTR_FROM_CPU_3 (rw) register accessor: CPU interrupt controlling register 3

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_intr_from_cpu_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_intr_from_cpu_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_intr_from_cpu_3`] module*/
pub type CPU_INTR_FROM_CPU_3 = crate::Reg<cpu_intr_from_cpu_3::CPU_INTR_FROM_CPU_3_SPEC>;
///CPU interrupt controlling register 3
pub mod cpu_intr_from_cpu_3;
/**RSA_PD_CTRL (rw) register accessor: RSA memory remapping register

You can [`read`](crate::generic::Reg::read) this register and get [`rsa_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsa_pd_ctrl`] module*/
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
///RSA memory remapping register
pub mod rsa_pd_ctrl;
/**BUSTOEXTMEM_ENA (rw) register accessor: EDMA enable register

You can [`read`](crate::generic::Reg::read) this register and get [`bustoextmem_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bustoextmem_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bustoextmem_ena`] module*/
pub type BUSTOEXTMEM_ENA = crate::Reg<bustoextmem_ena::BUSTOEXTMEM_ENA_SPEC>;
///EDMA enable register
pub mod bustoextmem_ena;
/**CACHE_CONTROL (rw) register accessor: Cache control register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_control`] module*/
pub type CACHE_CONTROL = crate::Reg<cache_control::CACHE_CONTROL_SPEC>;
///Cache control register
pub mod cache_control;
/**EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL (rw) register accessor: External memory encrypt and decrypt controlling register

You can [`read`](crate::generic::Reg::read) this register and get [`external_device_encrypt_decrypt_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_device_encrypt_decrypt_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@external_device_encrypt_decrypt_control`] module*/
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
///External memory encrypt and decrypt controlling register
pub mod external_device_encrypt_decrypt_control;
/**RTC_FASTMEM_CONFIG (rw) register accessor: RTC fast memory configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_fastmem_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc_fastmem_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_fastmem_config`] module*/
pub type RTC_FASTMEM_CONFIG = crate::Reg<rtc_fastmem_config::RTC_FASTMEM_CONFIG_SPEC>;
///RTC fast memory configuration register
pub mod rtc_fastmem_config;
/**RTC_FASTMEM_CRC (r) register accessor: RTC fast memory CRC controlling register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_fastmem_crc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rtc_fastmem_crc`] module*/
pub type RTC_FASTMEM_CRC = crate::Reg<rtc_fastmem_crc::RTC_FASTMEM_CRC_SPEC>;
///RTC fast memory CRC controlling register
pub mod rtc_fastmem_crc;
/**Redundant_ECO_Ctrl (rw) register accessor: Redundant ECO control register

You can [`read`](crate::generic::Reg::read) this register and get [`redundant_eco_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundant_eco_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@redundant_eco_ctrl`] module*/
#[doc(alias = "Redundant_ECO_Ctrl")]
pub type REDUNDANT_ECO_CTRL = crate::Reg<redundant_eco_ctrl::REDUNDANT_ECO_CTRL_SPEC>;
///Redundant ECO control register
pub mod redundant_eco_ctrl;
/**CLOCK_GATE (rw) register accessor: Clock gate control register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///Clock gate control register
pub mod clock_gate;
/**SRAM_CTRL_2 (rw) register accessor: System SRAM configuration register 2

You can [`read`](crate::generic::Reg::read) this register and get [`sram_ctrl_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ctrl_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_ctrl_2`] module*/
pub type SRAM_CTRL_2 = crate::Reg<sram_ctrl_2::SRAM_CTRL_2_SPEC>;
///System SRAM configuration register 2
pub mod sram_ctrl_2;
/**SYSCLK_CONF (rw) register accessor: SoC clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sysclk_conf`] module*/
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
///SoC clock configuration register
pub mod sysclk_conf;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
