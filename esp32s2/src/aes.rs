#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    key: [KEY; 8],
    text_in: [TEXT_IN; 4],
    text_out: [TEXT_OUT; 4],
    mode: MODE,
    endian: ENDIAN,
    trigger: TRIGGER,
    state: STATE,
    iv_: [IV_; 4],
    h_: [H_; 4],
    j0_: [J0_; 4],
    t0_: [T0_; 4],
    dma_enable: DMA_ENABLE,
    block_mode: BLOCK_MODE,
    block_num: BLOCK_NUM,
    inc_sel: INC_SEL,
    aad_block_num: AAD_BLOCK_NUM,
    remainder_bit_num: REMAINDER_BIT_NUM,
    continue_op: CONTINUE_OP,
    int_clr: INT_CLR,
    int_ena: INT_ENA,
    date: DATE,
    dma_exit: DMA_EXIT,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - AES key register %s"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &KEY {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - AES key register %s"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &KEY> {
        self.key.iter()
    }
    #[doc = "0x20..0x30 - Source data register %s"]
    #[inline(always)]
    pub const fn text_in(&self, n: usize) -> &TEXT_IN {
        &self.text_in[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Source data register %s"]
    #[inline(always)]
    pub fn text_in_iter(&self) -> impl Iterator<Item = &TEXT_IN> {
        self.text_in.iter()
    }
    #[doc = "0x30..0x40 - Result data register %s"]
    #[inline(always)]
    pub const fn text_out(&self, n: usize) -> &TEXT_OUT {
        &self.text_out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Result data register %s"]
    #[inline(always)]
    pub fn text_out_iter(&self) -> impl Iterator<Item = &TEXT_OUT> {
        self.text_out.iter()
    }
    #[doc = "0x40 - AES working mode configuration register"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x44 - Endian configuration register"]
    #[inline(always)]
    pub const fn endian(&self) -> &ENDIAN {
        &self.endian
    }
    #[doc = "0x48 - Operation start controlling register"]
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    #[doc = "0x4c - Operation status register"]
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    #[doc = "0x50..0x60 - initialization vector"]
    #[inline(always)]
    pub const fn iv_(&self, n: usize) -> &IV_ {
        &self.iv_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - initialization vector"]
    #[inline(always)]
    pub fn iv__iter(&self) -> impl Iterator<Item = &IV_> {
        self.iv_.iter()
    }
    #[doc = "0x60..0x70 - GCM hash subkey"]
    #[inline(always)]
    pub const fn h_(&self, n: usize) -> &H_ {
        &self.h_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - GCM hash subkey"]
    #[inline(always)]
    pub fn h__iter(&self) -> impl Iterator<Item = &H_> {
        self.h_.iter()
    }
    #[doc = "0x70..0x80 - J0"]
    #[inline(always)]
    pub const fn j0_(&self, n: usize) -> &J0_ {
        &self.j0_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - J0"]
    #[inline(always)]
    pub fn j0__iter(&self) -> impl Iterator<Item = &J0_> {
        self.j0_.iter()
    }
    #[doc = "0x80..0x90 - T0"]
    #[inline(always)]
    pub const fn t0_(&self, n: usize) -> &T0_ {
        &self.t0_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - T0"]
    #[inline(always)]
    pub fn t0__iter(&self) -> impl Iterator<Item = &T0_> {
        self.t0_.iter()
    }
    #[doc = "0x90 - DMA enable register"]
    #[inline(always)]
    pub const fn dma_enable(&self) -> &DMA_ENABLE {
        &self.dma_enable
    }
    #[doc = "0x94 - Block operation type register"]
    #[inline(always)]
    pub const fn block_mode(&self) -> &BLOCK_MODE {
        &self.block_mode
    }
    #[doc = "0x98 - Block number configuration register"]
    #[inline(always)]
    pub const fn block_num(&self) -> &BLOCK_NUM {
        &self.block_num
    }
    #[doc = "0x9c - Standard incrementing function register"]
    #[inline(always)]
    pub const fn inc_sel(&self) -> &INC_SEL {
        &self.inc_sel
    }
    #[doc = "0xa0 - AAD block number configuration register"]
    #[inline(always)]
    pub const fn aad_block_num(&self) -> &AAD_BLOCK_NUM {
        &self.aad_block_num
    }
    #[doc = "0xa4 - Remainder bit number of plaintext/ciphertext"]
    #[inline(always)]
    pub const fn remainder_bit_num(&self) -> &REMAINDER_BIT_NUM {
        &self.remainder_bit_num
    }
    #[doc = "0xa8 - Operation continue controlling register"]
    #[inline(always)]
    pub const fn continue_op(&self) -> &CONTINUE_OP {
        &self.continue_op
    }
    #[doc = "0xac - DMA-AES interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0xb0 - DMA-AES interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xb4 - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0xb8 - Operation exit controlling register"]
    #[inline(always)]
    pub const fn dma_exit(&self) -> &DMA_EXIT {
        &self.dma_exit
    }
}
#[doc = "KEY (rw) register accessor: AES key register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "AES key register %s"]
pub mod key;
#[doc = "TEXT_IN (rw) register accessor: Source data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_in`] module"]
pub type TEXT_IN = crate::Reg<text_in::TEXT_IN_SPEC>;
#[doc = "Source data register %s"]
pub mod text_in;
#[doc = "TEXT_OUT (rw) register accessor: Result data register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`text_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_out`] module"]
pub type TEXT_OUT = crate::Reg<text_out::TEXT_OUT_SPEC>;
#[doc = "Result data register %s"]
pub mod text_out;
#[doc = "MODE (rw) register accessor: AES working mode configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES working mode configuration register"]
pub mod mode;
#[doc = "ENDIAN (rw) register accessor: Endian configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endian::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`] module"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = "Endian configuration register"]
pub mod endian;
#[doc = "TRIGGER (w) register accessor: Operation start controlling register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Operation start controlling register"]
pub mod trigger;
#[doc = "STATE (r) register accessor: Operation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Operation status register"]
pub mod state;
#[doc = "IV_ (rw) register accessor: initialization vector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_`] module"]
pub type IV_ = crate::Reg<iv_::IV__SPEC>;
#[doc = "initialization vector"]
pub mod iv_;
#[doc = "H_ (r) register accessor: GCM hash subkey\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_`] module"]
pub type H_ = crate::Reg<h_::H__SPEC>;
#[doc = "GCM hash subkey"]
pub mod h_;
#[doc = "J0_ (rw) register accessor: J0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`j0_::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`j0_::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@j0_`] module"]
pub type J0_ = crate::Reg<j0_::J0__SPEC>;
#[doc = "J0"]
pub mod j0_;
#[doc = "T0_ (r) register accessor: T0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_`] module"]
pub type T0_ = crate::Reg<t0_::T0__SPEC>;
#[doc = "T0"]
pub mod t0_;
#[doc = "DMA_ENABLE (rw) register accessor: DMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enable`] module"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "DMA enable register"]
pub mod dma_enable;
#[doc = "BLOCK_MODE (rw) register accessor: Block operation type register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_mode`] module"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "Block operation type register"]
pub mod block_mode;
#[doc = "BLOCK_NUM (rw) register accessor: Block number configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_num`] module"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "Block number configuration register"]
pub mod block_num;
#[doc = "INC_SEL (rw) register accessor: Standard incrementing function register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inc_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inc_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inc_sel`] module"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM (rw) register accessor: AAD block number configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aad_block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aad_block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aad_block_num`] module"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "AAD block number configuration register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM (rw) register accessor: Remainder bit number of plaintext/ciphertext\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remainder_bit_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remainder_bit_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remainder_bit_num`] module"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "Remainder bit number of plaintext/ciphertext"]
pub mod remainder_bit_num;
#[doc = "CONTINUE_OP (w) register accessor: Operation continue controlling register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_op::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_op`] module"]
pub type CONTINUE_OP = crate::Reg<continue_op::CONTINUE_OP_SPEC>;
#[doc = "Operation continue controlling register"]
pub mod continue_op;
#[doc = "INT_CLR (w) register accessor: DMA-AES interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "DMA-AES interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ENA (rw) register accessor: DMA-AES interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-AES interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "DMA_EXIT (w) register accessor: Operation exit controlling register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_exit::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_exit`] module"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "Operation exit controlling register"]
pub mod dma_exit;
