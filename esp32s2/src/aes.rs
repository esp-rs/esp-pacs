#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    key: [KEY; 8],
    text_in: [TEXT_IN; 4],
    text_out: [TEXT_OUT; 4],
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
    pub const fn iv_mem(&self, n: usize) -> &IV_MEM {
        &self.iv_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x60 - initialization vector"]
    #[inline(always)]
    pub fn iv_mem_iter(&self) -> impl Iterator<Item = &IV_MEM> {
        self.iv_mem.iter()
    }
    #[doc = "0x60..0x70 - GCM hash subkey"]
    #[inline(always)]
    pub const fn h_mem(&self, n: usize) -> &H_MEM {
        &self.h_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - GCM hash subkey"]
    #[inline(always)]
    pub fn h_mem_iter(&self) -> impl Iterator<Item = &H_MEM> {
        self.h_mem.iter()
    }
    #[doc = "0x70..0x80 - J0"]
    #[inline(always)]
    pub const fn j0_mem(&self, n: usize) -> &J0_MEM {
        &self.j0_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - J0"]
    #[inline(always)]
    pub fn j0_mem_iter(&self) -> impl Iterator<Item = &J0_MEM> {
        self.j0_mem.iter()
    }
    #[doc = "0x80..0x90 - T0"]
    #[inline(always)]
    pub const fn t0_mem(&self, n: usize) -> &T0_MEM {
        &self.t0_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - T0"]
    #[inline(always)]
    pub fn t0_mem_iter(&self) -> impl Iterator<Item = &T0_MEM> {
        self.t0_mem.iter()
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
    pub const fn continue_(&self) -> &CONTINUE {
        &self.continue_
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
#[doc = "KEY (rw) register accessor: AES key register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`] module"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "AES key register %s"]
pub mod key;
#[doc = "TEXT_IN (rw) register accessor: Source data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`text_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_in`] module"]
pub type TEXT_IN = crate::Reg<text_in::TEXT_IN_SPEC>;
#[doc = "Source data register %s"]
pub mod text_in;
#[doc = "TEXT_OUT (rw) register accessor: Result data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`text_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_out`] module"]
pub type TEXT_OUT = crate::Reg<text_out::TEXT_OUT_SPEC>;
#[doc = "Result data register %s"]
pub mod text_out;
#[doc = "MODE (rw) register accessor: AES working mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "AES working mode configuration register"]
pub mod mode;
#[doc = "ENDIAN (rw) register accessor: Endian configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`] module"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = "Endian configuration register"]
pub mod endian;
#[doc = "TRIGGER (w) register accessor: Operation start controlling register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Operation start controlling register"]
pub mod trigger;
#[doc = "STATE (r) register accessor: Operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Operation status register"]
pub mod state;
#[doc = "IV_MEM (rw) register accessor: initialization vector\n\nYou can [`read`](crate::Reg::read) this register and get [`iv_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_mem`] module"]
pub type IV_MEM = crate::Reg<iv_mem::IV_MEM_SPEC>;
#[doc = "initialization vector"]
pub mod iv_mem;
#[doc = "H_MEM (r) register accessor: GCM hash subkey\n\nYou can [`read`](crate::Reg::read) this register and get [`h_mem::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_mem`] module"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "GCM hash subkey"]
pub mod h_mem;
#[doc = "J0_MEM (rw) register accessor: J0\n\nYou can [`read`](crate::Reg::read) this register and get [`j0_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`j0_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@j0_mem`] module"]
pub type J0_MEM = crate::Reg<j0_mem::J0_MEM_SPEC>;
#[doc = "J0"]
pub mod j0_mem;
#[doc = "T0_MEM (r) register accessor: T0\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_mem::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_mem`] module"]
pub type T0_MEM = crate::Reg<t0_mem::T0_MEM_SPEC>;
#[doc = "T0"]
pub mod t0_mem;
#[doc = "DMA_ENABLE (rw) register accessor: DMA enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enable`] module"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "DMA enable register"]
pub mod dma_enable;
#[doc = "BLOCK_MODE (rw) register accessor: Block operation type register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_mode`] module"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "Block operation type register"]
pub mod block_mode;
#[doc = "BLOCK_NUM (rw) register accessor: Block number configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_num`] module"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "Block number configuration register"]
pub mod block_num;
#[doc = "INC_SEL (rw) register accessor: Standard incrementing function register\n\nYou can [`read`](crate::Reg::read) this register and get [`inc_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inc_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inc_sel`] module"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM (rw) register accessor: AAD block number configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`aad_block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aad_block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aad_block_num`] module"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "AAD block number configuration register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM (rw) register accessor: Remainder bit number of plaintext/ciphertext\n\nYou can [`read`](crate::Reg::read) this register and get [`remainder_bit_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remainder_bit_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remainder_bit_num`] module"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "Remainder bit number of plaintext/ciphertext"]
pub mod remainder_bit_num;
#[doc = "CONTINUE (w) register accessor: Operation continue controlling register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_`] module"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "Operation continue controlling register"]
pub mod continue_;
#[doc = "INT_CLR (w) register accessor: DMA-AES interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "DMA-AES interrupt clear register"]
pub mod int_clr;
#[doc = "INT_ENA (rw) register accessor: DMA-AES interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-AES interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (r) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "DMA_EXIT (w) register accessor: Operation exit controlling register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_exit::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_exit`] module"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "Operation exit controlling register"]
pub mod dma_exit;
