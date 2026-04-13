#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    key_: [KEY_; 8],
    text_in_: [TEXT_IN_; 4],
    text_out_: [TEXT_OUT_; 4],
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
    _reserved22: [u8; 0x04],
    rx_reset: RX_RESET,
    tx_reset: TX_RESET,
    _reserved24: [u8; 0x08],
    pseudo: PSEUDO,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - AES key data register %s"]
    #[inline(always)]
    pub const fn key_(&self, n: usize) -> &KEY_ {
        &self.key_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - AES key data register %s"]
    #[inline(always)]
    pub fn key__iter(&self) -> impl Iterator<Item = &KEY_> {
        self.key_.iter()
    }
    #[doc = "0x20..0x30 - Source text data register %s"]
    #[inline(always)]
    pub const fn text_in_(&self, n: usize) -> &TEXT_IN_ {
        &self.text_in_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x30 - Source text data register %s"]
    #[inline(always)]
    pub fn text_in__iter(&self) -> impl Iterator<Item = &TEXT_IN_> {
        self.text_in_.iter()
    }
    #[doc = "0x30..0x40 - Result text data register %s"]
    #[inline(always)]
    pub const fn text_out_(&self, n: usize) -> &TEXT_OUT_ {
        &self.text_out_[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x40 - Result text data register %s"]
    #[inline(always)]
    pub fn text_out__iter(&self) -> impl Iterator<Item = &TEXT_OUT_> {
        self.text_out_.iter()
    }
    #[doc = "0x40 - Defines key length and encryption / decryption"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x44 - AES Endian configure register"]
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
    #[doc = "0x90 - Selects the working mode of the AES accelerator"]
    #[inline(always)]
    pub const fn dma_enable(&self) -> &DMA_ENABLE {
        &self.dma_enable
    }
    #[doc = "0x94 - Defines the block cipher mode"]
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
    #[doc = "0xac - DMA-AES interrupt clear register"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &INT_CLEAR {
        &self.int_clear
    }
    #[doc = "0xb0 - DMA-AES interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0xb4 - AES version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0xb8 - Operation exit controlling register"]
    #[inline(always)]
    pub const fn dma_exit(&self) -> &DMA_EXIT {
        &self.dma_exit
    }
    #[doc = "0xc0 - AES-DMA reset rx-fifo register"]
    #[inline(always)]
    pub const fn rx_reset(&self) -> &RX_RESET {
        &self.rx_reset
    }
    #[doc = "0xc4 - AES-DMA reset tx-fifo register"]
    #[inline(always)]
    pub const fn tx_reset(&self) -> &TX_RESET {
        &self.tx_reset
    }
    #[doc = "0xd0 - AES PSEUDO function configure register"]
    #[inline(always)]
    pub const fn pseudo(&self) -> &PSEUDO {
        &self.pseudo
    }
}
#[doc = "KEY_ (rw) register accessor: AES key data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`key_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key_`] module"]
pub type KEY_ = crate::Reg<key_::KEY__SPEC>;
#[doc = "AES key data register %s"]
pub mod key_;
#[doc = "TEXT_IN_ (rw) register accessor: Source text data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`text_in_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_in_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_in_`] module"]
pub type TEXT_IN_ = crate::Reg<text_in_::TEXT_IN__SPEC>;
#[doc = "Source text data register %s"]
pub mod text_in_;
#[doc = "TEXT_OUT_ (rw) register accessor: Result text data register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`text_out_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`text_out_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@text_out_`] module"]
pub type TEXT_OUT_ = crate::Reg<text_out_::TEXT_OUT__SPEC>;
#[doc = "Result text data register %s"]
pub mod text_out_;
#[doc = "MODE (rw) register accessor: Defines key length and encryption / decryption\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Defines key length and encryption / decryption"]
pub mod mode;
#[doc = "ENDIAN (rw) register accessor: AES Endian configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`endian::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endian::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@endian`] module"]
pub type ENDIAN = crate::Reg<endian::ENDIAN_SPEC>;
#[doc = "AES Endian configure register"]
pub mod endian;
#[doc = "TRIGGER (w) register accessor: Operation start controlling register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
pub type TRIGGER = crate::Reg<trigger::TRIGGER_SPEC>;
#[doc = "Operation start controlling register"]
pub mod trigger;
#[doc = "STATE (r) register accessor: Operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "Operation status register"]
pub mod state;
#[doc = "IV_MEM (rw) register accessor: The memory that stores initialization vector\n\nYou can [`read`](crate::Reg::read) this register and get [`iv_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iv_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv_mem`] module"]
pub type IV_MEM = crate::Reg<iv_mem::IV_MEM_SPEC>;
#[doc = "The memory that stores initialization vector"]
pub mod iv_mem;
#[doc = "H_MEM (rw) register accessor: The memory that stores GCM hash subkey\n\nYou can [`read`](crate::Reg::read) this register and get [`h_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`h_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_mem`] module"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "The memory that stores GCM hash subkey"]
pub mod h_mem;
#[doc = "J0_MEM (rw) register accessor: The memory that stores J0\n\nYou can [`read`](crate::Reg::read) this register and get [`j0_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`j0_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@j0_mem`] module"]
pub type J0_MEM = crate::Reg<j0_mem::J0_MEM_SPEC>;
#[doc = "The memory that stores J0"]
pub mod j0_mem;
#[doc = "T0_MEM (rw) register accessor: The memory that stores T0\n\nYou can [`read`](crate::Reg::read) this register and get [`t0_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t0_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t0_mem`] module"]
pub type T0_MEM = crate::Reg<t0_mem::T0_MEM_SPEC>;
#[doc = "The memory that stores T0"]
pub mod t0_mem;
#[doc = "DMA_ENABLE (rw) register accessor: Selects the working mode of the AES accelerator\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_enable`] module"]
pub type DMA_ENABLE = crate::Reg<dma_enable::DMA_ENABLE_SPEC>;
#[doc = "Selects the working mode of the AES accelerator"]
pub mod dma_enable;
#[doc = "BLOCK_MODE (rw) register accessor: Defines the block cipher mode\n\nYou can [`read`](crate::Reg::read) this register and get [`block_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_mode`] module"]
pub type BLOCK_MODE = crate::Reg<block_mode::BLOCK_MODE_SPEC>;
#[doc = "Defines the block cipher mode"]
pub mod block_mode;
#[doc = "BLOCK_NUM (rw) register accessor: Block number configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_num`] module"]
pub type BLOCK_NUM = crate::Reg<block_num::BLOCK_NUM_SPEC>;
#[doc = "Block number configuration register"]
pub mod block_num;
#[doc = "INC_SEL (rw) register accessor: Standard incrementing function register\n\nYou can [`read`](crate::Reg::read) this register and get [`inc_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inc_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inc_sel`] module"]
pub type INC_SEL = crate::Reg<inc_sel::INC_SEL_SPEC>;
#[doc = "Standard incrementing function register"]
pub mod inc_sel;
#[doc = "AAD_BLOCK_NUM (rw) register accessor: Additional Authential Data block number register\n\nYou can [`read`](crate::Reg::read) this register and get [`aad_block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aad_block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aad_block_num`] module"]
pub type AAD_BLOCK_NUM = crate::Reg<aad_block_num::AAD_BLOCK_NUM_SPEC>;
#[doc = "Additional Authential Data block number register"]
pub mod aad_block_num;
#[doc = "REMAINDER_BIT_NUM (rw) register accessor: AES remainder bit number register\n\nYou can [`read`](crate::Reg::read) this register and get [`remainder_bit_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remainder_bit_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remainder_bit_num`] module"]
pub type REMAINDER_BIT_NUM = crate::Reg<remainder_bit_num::REMAINDER_BIT_NUM_SPEC>;
#[doc = "AES remainder bit number register"]
pub mod remainder_bit_num;
#[doc = "CONTINUE (w) register accessor: AES continue register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_`] module"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "AES continue register"]
pub mod continue_;
#[doc = "INT_CLEAR (w) register accessor: DMA-AES interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`] module"]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = "DMA-AES interrupt clear register"]
pub mod int_clear;
#[doc = "INT_ENA (rw) register accessor: DMA-AES interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-AES interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: AES version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "AES version control register"]
pub mod date;
#[doc = "DMA_EXIT (w) register accessor: Operation exit controlling register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_exit::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_exit`] module"]
pub type DMA_EXIT = crate::Reg<dma_exit::DMA_EXIT_SPEC>;
#[doc = "Operation exit controlling register"]
pub mod dma_exit;
#[doc = "RX_RESET (w) register accessor: AES-DMA reset rx-fifo register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_reset`] module"]
pub type RX_RESET = crate::Reg<rx_reset::RX_RESET_SPEC>;
#[doc = "AES-DMA reset rx-fifo register"]
pub mod rx_reset;
#[doc = "TX_RESET (w) register accessor: AES-DMA reset tx-fifo register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_reset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_reset`] module"]
pub type TX_RESET = crate::Reg<tx_reset::TX_RESET_SPEC>;
#[doc = "AES-DMA reset tx-fifo register"]
pub mod tx_reset;
#[doc = "PSEUDO (rw) register accessor: AES PSEUDO function configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`pseudo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseudo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pseudo`] module"]
pub type PSEUDO = crate::Reg<pseudo::PSEUDO_SPEC>;
#[doc = "AES PSEUDO function configure register"]
pub mod pseudo;
