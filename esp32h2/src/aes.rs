#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    key_0: KEY_0,
    key_1: KEY_1,
    key_2: KEY_2,
    key_3: KEY_3,
    key_4: KEY_4,
    key_5: KEY_5,
    key_6: KEY_6,
    key_7: KEY_7,
    text_in_0: TEXT_IN_0,
    text_in_1: TEXT_IN_1,
    text_in_2: TEXT_IN_2,
    text_in_3: TEXT_IN_3,
    text_out_0: TEXT_OUT_0,
    text_out_1: TEXT_OUT_1,
    text_out_2: TEXT_OUT_2,
    text_out_3: TEXT_OUT_3,
    mode: MODE,
    endian: ENDIAN,
    trigger: TRIGGER,
    state: STATE,
    iv_mem: [IV_MEM; 4],
    h_mem: [H_MEM; 4],
    j0_mem: [J0_MEM; 4],
    t0_mem: [T0_MEM; 4],
    dma_enable: DMA_ENABLE,
    block_mode: BLOCK_MODE,
    block_num: BLOCK_NUM,
    inc_sel: INC_SEL,
    aad_block_num: AAD_BLOCK_NUM,
    remainder_bit_num: REMAINDER_BIT_NUM,
    continue_: CONTINUE,
    int_clear: INT_CLEAR,
    int_ena: INT_ENA,
    date: DATE,
    dma_exit: DMA_EXIT,
}
impl RegisterBlock {
    #[doc = "0x00 - Key material key_0 configure register"]
    #[inline(always)]
    pub const fn key_0(&self) -> &KEY_0 {
        &self.key_0
    }
    #[doc = "0x04 - Key material key_1 configure register"]
    #[inline(always)]
    pub const fn key_1(&self) -> &KEY_1 {
        &self.key_1
    }
    #[doc = "0x08 - Key material key_2 configure register"]
    #[inline(always)]
    pub const fn key_2(&self) -> &KEY_2 {
        &self.key_2
    }
    #[doc = "0x0c - Key material key_3 configure register"]
    #[inline(always)]
    pub const fn key_3(&self) -> &KEY_3 {
        &self.key_3
    }
    #[doc = "0x10 - Key material key_4 configure register"]
    #[inline(always)]
    pub const fn key_4(&self) -> &KEY_4 {
        &self.key_4
    }
    #[doc = "0x14 - Key material key_5 configure register"]
    #[inline(always)]
    pub const fn key_5(&self) -> &KEY_5 {
        &self.key_5
    }
    #[doc = "0x18 - Key material key_6 configure register"]
    #[inline(always)]
    pub const fn key_6(&self) -> &KEY_6 {
        &self.key_6
    }
    #[doc = "0x1c - Key material key_7 configure register"]
    #[inline(always)]
    pub const fn key_7(&self) -> &KEY_7 {
        &self.key_7
    }
    #[doc = "0x20 - source text material text_in_0 configure register"]
    #[inline(always)]
    pub const fn text_in_0(&self) -> &TEXT_IN_0 {
        &self.text_in_0
    }
    #[doc = "0x24 - source text material text_in_1 configure register"]
    #[inline(always)]
    pub const fn text_in_1(&self) -> &TEXT_IN_1 {
        &self.text_in_1
    }
    #[doc = "0x28 - source text material text_in_2 configure register"]
    #[inline(always)]
    pub const fn text_in_2(&self) -> &TEXT_IN_2 {
        &self.text_in_2
    }
    #[doc = "0x2c - source text material text_in_3 configure register"]
    #[inline(always)]
    pub const fn text_in_3(&self) -> &TEXT_IN_3 {
        &self.text_in_3
    }
    #[doc = "0x30 - result text material text_out_0 configure register"]
    #[inline(always)]
    pub const fn text_out_0(&self) -> &TEXT_OUT_0 {
        &self.text_out_0
    }
    #[doc = "0x34 - result text material text_out_1 configure register"]
    #[inline(always)]
    pub const fn text_out_1(&self) -> &TEXT_OUT_1 {
        &self.text_out_1
    }
    #[doc = "0x38 - result text material text_out_2 configure register"]
    #[inline(always)]
    pub const fn text_out_2(&self) -> &TEXT_OUT_2 {
        &self.text_out_2
    }
    #[doc = "0x3c - result text material text_out_3 configure register"]
    #[inline(always)]
    pub const fn text_out_3(&self) -> &TEXT_OUT_3 {
        &self.text_out_3
    }
    #[doc = "0x40 - AES Mode register"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x44 - AES Endian configure register"]
    #[inline(always)]
    pub const fn endian(&self) -> &ENDIAN {
        &self.endian
    }
    #[doc = "0x48 - AES trigger register"]
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    #[doc = "0x4c - AES state register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x50..0x60 - The memory that stores initialization vector"]
    #[inline(always)]
    pub const fn iv_mem(&self, n: usize) -> &IV_MEM {
        &self.iv_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - The memory that stores initialization vector"]
    #[inline(always)]
    pub fn iv_mem_iter(&self) -> impl Iterator<Item = &IV_MEM> {
        self.iv_mem.iter()
    }
    #[doc = "0x60..0x70 - The memory that stores GCM hash subkey"]
    #[inline(always)]
    pub const fn h_mem(&self, n: usize) -> &H_MEM {
        &self.h_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - The memory that stores GCM hash subkey"]
    #[inline(always)]
    pub fn h_mem_iter(&self) -> impl Iterator<Item = &H_MEM> {
        self.h_mem.iter()
    }
    #[doc = "0x70..0x80 - The memory that stores J0"]
    #[inline(always)]
    pub const fn j0_mem(&self, n: usize) -> &J0_MEM {
        &self.j0_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - The memory that stores J0"]
    #[inline(always)]
    pub fn j0_mem_iter(&self) -> impl Iterator<Item = &J0_MEM> {
        self.j0_mem.iter()
    }
    #[doc = "0x80..0x90 - The memory that stores T0"]
    #[inline(always)]
    pub const fn t0_mem(&self, n: usize) -> &T0_MEM {
        &self.t0_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - The memory that stores T0"]
    #[inline(always)]
    pub fn t0_mem_iter(&self) -> impl Iterator<Item = &T0_MEM> {
        self.t0_mem.iter()
    }
    #[doc = "0x90 - DMA-AES working mode register"]
    #[inline(always)]
    pub const fn dma_enable(&self) -> &DMA_ENABLE {
        &self.dma_enable
    }
    #[doc = "0x94 - AES cipher block mode register"]
    #[inline(always)]
    pub const fn block_mode(&self) -> &BLOCK_MODE {
        &self.block_mode
    }
    #[doc = "0x98 - AES block number register"]
    #[inline(always)]
    pub const fn block_num(&self) -> &BLOCK_NUM {
        &self.block_num
    }
    #[doc = "0x9c - Standard incrementing function configure register"]
    #[inline(always)]
    pub const fn inc_sel(&self) -> &INC_SEL {
        &self.inc_sel
    }
    #[doc = "0xa0 - Additional Authential Data block number register"]
    #[inline(always)]
    pub const fn aad_block_num(&self) -> &AAD_BLOCK_NUM {
        &self.aad_block_num
    }
    #[doc = "0xa4 - AES remainder bit number register"]
    #[inline(always)]
    pub const fn remainder_bit_num(&self) -> &REMAINDER_BIT_NUM {
        &self.remainder_bit_num
    }
    #[doc = "0xa8 - AES continue register"]
    #[inline(always)]
    pub const fn continue_(&self) -> &CONTINUE {
        &self.continue_
    }
    #[doc = "0xac - AES Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &INT_CLEAR {
        &self.int_clear
    }
    #[doc = "0xb0 - AES Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xb4 - AES version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0xb8 - AES-DMA exit config"]
    #[inline(always)]
    pub const fn dma_exit(&self) -> &DMA_EXIT {
        &self.dma_exit
    }
}
#[doc = "KEY_0 (rw) register accessor: Key material key_0 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_0`] module"]
pub type KEY_0 = crate::Reg<key_0::KEY_0_SPEC>;
#[doc = "Key material key_0 configure register"]
pub mod key_0;
#[doc = "KEY_1 (rw) register accessor: Key material key_1 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_1`] module"]
pub type KEY_1 = crate::Reg<key_1::KEY_1_SPEC>;
#[doc = "Key material key_1 configure register"]
pub mod key_1;
#[doc = "KEY_2 (rw) register accessor: Key material key_2 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_2`] module"]
pub type KEY_2 = crate::Reg<key_2::KEY_2_SPEC>;
#[doc = "Key material key_2 configure register"]
pub mod key_2;
#[doc = "KEY_3 (rw) register accessor: Key material key_3 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_3`] module"]
pub type KEY_3 = crate::Reg<key_3::KEY_3_SPEC>;
#[doc = "Key material key_3 configure register"]
pub mod key_3;
#[doc = "KEY_4 (rw) register accessor: Key material key_4 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_4`] module"]
pub type KEY_4 = crate::Reg<key_4::KEY_4_SPEC>;
#[doc = "Key material key_4 configure register"]
pub mod key_4;
#[doc = "KEY_5 (rw) register accessor: Key material key_5 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_5`] module"]
pub type KEY_5 = crate::Reg<key_5::KEY_5_SPEC>;
#[doc = "Key material key_5 configure register"]
pub mod key_5;
#[doc = "KEY_6 (rw) register accessor: Key material key_6 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_6`] module"]
pub type KEY_6 = crate::Reg<key_6::KEY_6_SPEC>;
#[doc = "Key material key_6 configure register"]
pub mod key_6;
#[doc = "KEY_7 (rw) register accessor: Key material key_7 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_7`] module"]
pub type KEY_7 = crate::Reg<key_7::KEY_7_SPEC>;
#[doc = "Key material key_7 configure register"]
pub mod key_7;
#[doc = "TEXT_IN_0 (rw) register accessor: source text material text_in_0 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_in_0`] module"]
pub type TEXT_IN_0 = crate::Reg<text_in_0::TEXT_IN_0_SPEC>;
#[doc = "source text material text_in_0 configure register"]
pub mod text_in_0;
#[doc = "TEXT_IN_1 (rw) register accessor: source text material text_in_1 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_in_1`] module"]
pub type TEXT_IN_1 = crate::Reg<text_in_1::TEXT_IN_1_SPEC>;
#[doc = "source text material text_in_1 configure register"]
pub mod text_in_1;
#[doc = "TEXT_IN_2 (rw) register accessor: source text material text_in_2 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_in_2`] module"]
pub type TEXT_IN_2 = crate::Reg<text_in_2::TEXT_IN_2_SPEC>;
#[doc = "source text material text_in_2 configure register"]
pub mod text_in_2;
#[doc = "TEXT_IN_3 (rw) register accessor: source text material text_in_3 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_in_3`] module"]
pub type TEXT_IN_3 = crate::Reg<text_in_3::TEXT_IN_3_SPEC>;
#[doc = "source text material text_in_3 configure register"]
pub mod text_in_3;
#[doc = "TEXT_OUT_0 (rw) register accessor: result text material text_out_0 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_out_0`] module"]
pub type TEXT_OUT_0 = crate::Reg<text_out_0::TEXT_OUT_0_SPEC>;
#[doc = "result text material text_out_0 configure register"]
pub mod text_out_0;
#[doc = "TEXT_OUT_1 (rw) register accessor: result text material text_out_1 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_out_1`] module"]
pub type TEXT_OUT_1 = crate::Reg<text_out_1::TEXT_OUT_1_SPEC>;
#[doc = "result text material text_out_1 configure register"]
pub mod text_out_1;
#[doc = "TEXT_OUT_2 (rw) register accessor: result text material text_out_2 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_out_2`] module"]
pub type TEXT_OUT_2 = crate::Reg<text_out_2::TEXT_OUT_2_SPEC>;
#[doc = "result text material text_out_2 configure register"]
pub mod text_out_2;
#[doc = "TEXT_OUT_3 (rw) register accessor: result text material text_out_3 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_out_3`] module"]
pub type TEXT_OUT_3 = crate::Reg<text_out_3::TEXT_OUT_3_SPEC>;
#[doc = "result text material text_out_3 configure register"]
pub mod text_out_3;
#[doc = "MODE (rw) register accessor: AES Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES Mode register"]
pub mod mode;
#[doc = "ENDIAN (rw) register accessor: AES Endian configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endian::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`] module"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = "AES Endian configure register"]
pub mod endian;
#[doc = "TRIGGER (w) register accessor: AES trigger register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "AES trigger register"]
pub mod trigger;
#[doc = "STATE (r) register accessor: AES state register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "AES state register"]
pub mod state;
#[doc = "IV_MEM (rw) register accessor: The memory that stores initialization vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_mem`] module"]
pub type IV_MEM = crate::Reg<iv_mem::IV_MEM_SPEC>;
#[doc = "The memory that stores initialization vector"]
pub mod iv_mem;
#[doc = "H_MEM (rw) register accessor: The memory that stores GCM hash subkey\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_mem`] module"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "The memory that stores GCM hash subkey"]
pub mod h_mem;
#[doc = "J0_MEM (rw) register accessor: The memory that stores J0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`j0_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`j0_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@j0_mem`] module"]
pub type J0_MEM = crate::Reg<j0_mem::J0_MEM_SPEC>;
#[doc = "The memory that stores J0"]
pub mod j0_mem;
#[doc = "T0_MEM (rw) register accessor: The memory that stores T0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_mem`] module"]
pub type T0_MEM = crate::Reg<t0_mem::T0_MEM_SPEC>;
#[doc = "The memory that stores T0"]
pub mod t0_mem;
#[doc = "DMA_ENABLE (rw) register accessor: DMA-AES working mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enable`] module"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "DMA-AES working mode register"]
pub mod dma_enable;
#[doc = "BLOCK_MODE (rw) register accessor: AES cipher block mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_mode`] module"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "AES cipher block mode register"]
pub mod block_mode;
#[doc = "BLOCK_NUM (rw) register accessor: AES block number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_num`] module"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "AES block number register"]
pub mod block_num;
#[doc = "INC_SEL (rw) register accessor: Standard incrementing function configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inc_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inc_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inc_sel`] module"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function configure register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM (rw) register accessor: Additional Authential Data block number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aad_block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aad_block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aad_block_num`] module"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "Additional Authential Data block number register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM (rw) register accessor: AES remainder bit number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remainder_bit_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remainder_bit_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remainder_bit_num`] module"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "AES remainder bit number register"]
pub mod remainder_bit_num;
#[doc = "CONTINUE (w) register accessor: AES continue register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_`] module"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "AES continue register"]
pub mod continue_;
#[doc = "INT_CLEAR (w) register accessor: AES Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`] module"]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = "AES Interrupt clear register"]
pub mod int_clear;
#[doc = "INT_ENA (rw) register accessor: AES Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "AES Interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: AES version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "AES version control register"]
pub mod date;
#[doc = "DMA_EXIT (w) register accessor: AES-DMA exit config\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_exit::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_exit`] module"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "AES-DMA exit config"]
pub mod dma_exit;
