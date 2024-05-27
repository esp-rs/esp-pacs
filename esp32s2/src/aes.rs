#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    ///0x00..0x20 - AES key register %s
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &KEY {
        &self.key[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - AES key register %s
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &KEY> {
        self.key.iter()
    }
    ///0x20..0x30 - Source data register %s
    #[inline(always)]
    pub const fn text_in(&self, n: usize) -> &TEXT_IN {
        &self.text_in[n]
    }
    ///Iterator for array of:
    ///0x20..0x30 - Source data register %s
    #[inline(always)]
    pub fn text_in_iter(&self) -> impl Iterator<Item = &TEXT_IN> {
        self.text_in.iter()
    }
    ///0x30..0x40 - Result data register %s
    #[inline(always)]
    pub const fn text_out(&self, n: usize) -> &TEXT_OUT {
        &self.text_out[n]
    }
    ///Iterator for array of:
    ///0x30..0x40 - Result data register %s
    #[inline(always)]
    pub fn text_out_iter(&self) -> impl Iterator<Item = &TEXT_OUT> {
        self.text_out.iter()
    }
    ///0x40 - AES working mode configuration register
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    ///0x44 - Endian configuration register
    #[inline(always)]
    pub const fn endian(&self) -> &ENDIAN {
        &self.endian
    }
    ///0x48 - Operation start controlling register
    #[inline(always)]
    pub const fn trigger(&self) -> &TRIGGER {
        &self.trigger
    }
    ///0x4c - Operation status register
    #[inline(always)]
    pub const fn state(&self) -> &STATE {
        &self.state
    }
    ///0x50..0x60 - initialization vector
    #[inline(always)]
    pub const fn iv_mem(&self, n: usize) -> &IV_MEM {
        &self.iv_mem[n]
    }
    ///Iterator for array of:
    ///0x50..0x60 - initialization vector
    #[inline(always)]
    pub fn iv_mem_iter(&self) -> impl Iterator<Item = &IV_MEM> {
        self.iv_mem.iter()
    }
    ///0x60..0x70 - GCM hash subkey
    #[inline(always)]
    pub const fn h_mem(&self, n: usize) -> &H_MEM {
        &self.h_mem[n]
    }
    ///Iterator for array of:
    ///0x60..0x70 - GCM hash subkey
    #[inline(always)]
    pub fn h_mem_iter(&self) -> impl Iterator<Item = &H_MEM> {
        self.h_mem.iter()
    }
    ///0x70..0x80 - J0
    #[inline(always)]
    pub const fn j0_mem(&self, n: usize) -> &J0_MEM {
        &self.j0_mem[n]
    }
    ///Iterator for array of:
    ///0x70..0x80 - J0
    #[inline(always)]
    pub fn j0_mem_iter(&self) -> impl Iterator<Item = &J0_MEM> {
        self.j0_mem.iter()
    }
    ///0x80..0x90 - T0
    #[inline(always)]
    pub const fn t0_mem(&self, n: usize) -> &T0_MEM {
        &self.t0_mem[n]
    }
    ///Iterator for array of:
    ///0x80..0x90 - T0
    #[inline(always)]
    pub fn t0_mem_iter(&self) -> impl Iterator<Item = &T0_MEM> {
        self.t0_mem.iter()
    }
    ///0x90 - DMA enable register
    #[inline(always)]
    pub const fn dma_enable(&self) -> &DMA_ENABLE {
        &self.dma_enable
    }
    ///0x94 - Block operation type register
    #[inline(always)]
    pub const fn block_mode(&self) -> &BLOCK_MODE {
        &self.block_mode
    }
    ///0x98 - Block number configuration register
    #[inline(always)]
    pub const fn block_num(&self) -> &BLOCK_NUM {
        &self.block_num
    }
    ///0x9c - Standard incrementing function register
    #[inline(always)]
    pub const fn inc_sel(&self) -> &INC_SEL {
        &self.inc_sel
    }
    ///0xa0 - AAD block number configuration register
    #[inline(always)]
    pub const fn aad_block_num(&self) -> &AAD_BLOCK_NUM {
        &self.aad_block_num
    }
    ///0xa4 - Remainder bit number of plaintext/ciphertext
    #[inline(always)]
    pub const fn remainder_bit_num(&self) -> &REMAINDER_BIT_NUM {
        &self.remainder_bit_num
    }
    ///0xa8 - Operation continue controlling register
    #[inline(always)]
    pub const fn continue_(&self) -> &CONTINUE {
        &self.continue_
    }
    ///0xac - DMA-AES interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0xb0 - DMA-AES interrupt enable register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0xb4 - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    ///0xb8 - Operation exit controlling register
    #[inline(always)]
    pub const fn dma_exit(&self) -> &DMA_EXIT {
        &self.dma_exit
    }
}
/**KEY (rw) register accessor: AES key register %s

You can [`read`](crate::generic::Reg::read) this register and get [`key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@key`] module*/
pub type KEY = crate::Reg<key::KEY_SPEC>;
///AES key register %s
pub mod key;
/**TEXT_IN (rw) register accessor: Source data register %s

You can [`read`](crate::generic::Reg::read) this register and get [`text_in::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_in::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@text_in`] module*/
pub type TEXT_IN = crate::Reg<text_in::TEXT_IN_SPEC>;
///Source data register %s
pub mod text_in;
/**TEXT_OUT (rw) register accessor: Result data register %s

You can [`read`](crate::generic::Reg::read) this register and get [`text_out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`text_out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@text_out`] module*/
pub type TEXT_OUT = crate::Reg<text_out::TEXT_OUT_SPEC>;
///Result data register %s
pub mod text_out;
/**MODE (rw) register accessor: AES working mode configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mode`] module*/
pub type MODE = crate::Reg<mode::MODE_SPEC>;
///AES working mode configuration register
pub mod mode;
/**ENDIAN (rw) register accessor: Endian configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`endian::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@endian`] module*/
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
///Endian configuration register
pub mod endian;
/**TRIGGER (w) register accessor: Operation start controlling register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trigger`] module*/
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
///Operation start controlling register
pub mod trigger;
/**STATE (r) register accessor: Operation status register

You can [`read`](crate::generic::Reg::read) this register and get [`state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@state`] module*/
pub type STATE = crate::Reg<state::STATE_SPEC>;
///Operation status register
pub mod state;
/**IV_MEM (rw) register accessor: initialization vector

You can [`read`](crate::generic::Reg::read) this register and get [`iv_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iv_mem`] module*/
pub type IV_MEM = crate::Reg<iv_mem::IV_MEM_SPEC>;
///initialization vector
pub mod iv_mem;
/**H_MEM (r) register accessor: GCM hash subkey

You can [`read`](crate::generic::Reg::read) this register and get [`h_mem::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@h_mem`] module*/
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
///GCM hash subkey
pub mod h_mem;
/**J0_MEM (rw) register accessor: J0

You can [`read`](crate::generic::Reg::read) this register and get [`j0_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`j0_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@j0_mem`] module*/
pub type J0_MEM = crate::Reg<j0_mem::J0_MEM_SPEC>;
///J0
pub mod j0_mem;
/**T0_MEM (r) register accessor: T0

You can [`read`](crate::generic::Reg::read) this register and get [`t0_mem::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@t0_mem`] module*/
pub type T0_MEM = crate::Reg<t0_mem::T0_MEM_SPEC>;
///T0
pub mod t0_mem;
/**DMA_ENABLE (rw) register accessor: DMA enable register

You can [`read`](crate::generic::Reg::read) this register and get [`dma_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_enable`] module*/
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
///DMA enable register
pub mod dma_enable;
/**BLOCK_MODE (rw) register accessor: Block operation type register

You can [`read`](crate::generic::Reg::read) this register and get [`block_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@block_mode`] module*/
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
///Block operation type register
pub mod block_mode;
/**BLOCK_NUM (rw) register accessor: Block number configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@block_num`] module*/
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
///Block number configuration register
pub mod block_num;
/**INC_SEL (rw) register accessor: Standard incrementing function register

You can [`read`](crate::generic::Reg::read) this register and get [`inc_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inc_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inc_sel`] module*/
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
///Standard incrementing function register
pub mod inc_sel;
/**AAD_BLOCK_NUM (rw) register accessor: AAD block number configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`aad_block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aad_block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@aad_block_num`] module*/
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
///AAD block number configuration register
pub mod aad_block_num;
/**REMAINDER_BIT_NUM (rw) register accessor: Remainder bit number of plaintext/ciphertext

You can [`read`](crate::generic::Reg::read) this register and get [`remainder_bit_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remainder_bit_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@remainder_bit_num`] module*/
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
///Remainder bit number of plaintext/ciphertext
pub mod remainder_bit_num;
/**CONTINUE (w) register accessor: Operation continue controlling register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@continue_`] module*/
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
///Operation continue controlling register
pub mod continue_;
/**INT_CLR (w) register accessor: DMA-AES interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///DMA-AES interrupt clear register
pub mod int_clr;
/**INT_ENA (rw) register accessor: DMA-AES interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///DMA-AES interrupt enable register
pub mod int_ena;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
/**DMA_EXIT (w) register accessor: Operation exit controlling register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_exit::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_exit`] module*/
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
///Operation exit controlling register
pub mod dma_exit;
