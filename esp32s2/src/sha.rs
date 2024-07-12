#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mode: MODE,
    t_string: T_STRING,
    t_length: T_LENGTH,
    dma_block_num: DMA_BLOCK_NUM,
    start: START,
    continue_: CONTINUE,
    busy: BUSY,
    dma_start: DMA_START,
    dma_continue: DMA_CONTINUE,
    int_clear: INT_CLEAR,
    int_ena: INT_ENA,
    date: DATE,
    _reserved12: [u8; 0x10],
    h_mem: [H_MEM; 16],
    m_mem: [M_MEM; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Defines the algorithm of SHA accelerator"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x04 - String content register for calculating initial Hash Value (only effective for SHA-512/t)"]
    #[inline(always)]
    pub const fn t_string(&self) -> &T_STRING {
        &self.t_string
    }
    #[doc = "0x08 - String length register for calculating initial Hash Value (only effective for SHA-512/t)"]
    #[inline(always)]
    pub const fn t_length(&self) -> &T_LENGTH {
        &self.t_length
    }
    #[doc = "0x0c - Block number register (only effective for DMA-SHA)"]
    #[inline(always)]
    pub const fn dma_block_num(&self) -> &DMA_BLOCK_NUM {
        &self.dma_block_num
    }
    #[doc = "0x10 - Starts the SHA accelerator for Typical SHA operation"]
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    #[doc = "0x14 - Continues SHA operation (only effective in Typical SHA mode)"]
    #[inline(always)]
    pub const fn continue_(&self) -> &CONTINUE {
        &self.continue_
    }
    #[doc = "0x18 - Indicates if SHA Accelerator is busy or not"]
    #[inline(always)]
    pub const fn busy(&self) -> &BUSY {
        &self.busy
    }
    #[doc = "0x1c - Starts the SHA accelerator for DMA-SHA operation"]
    #[inline(always)]
    pub const fn dma_start(&self) -> &DMA_START {
        &self.dma_start
    }
    #[doc = "0x20 - Continues SHA operation (only effective in DMA-SHA mode)"]
    #[inline(always)]
    pub const fn dma_continue(&self) -> &DMA_CONTINUE {
        &self.dma_continue
    }
    #[doc = "0x24 - DMA-SHA interrupt clear register"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &INT_CLEAR {
        &self.int_clear
    }
    #[doc = "0x28 - DMA-SHA interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x2c - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x40..0x80 - Hash value"]
    #[inline(always)]
    pub const fn h_mem(&self, n: usize) -> &H_MEM {
        &self.h_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - Hash value"]
    #[inline(always)]
    pub fn h_mem_iter(&self) -> impl Iterator<Item = &H_MEM> {
        self.h_mem.iter()
    }
    #[doc = "0x80..0x100 - Message"]
    #[inline(always)]
    pub const fn m_mem(&self, n: usize) -> &M_MEM {
        &self.m_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Message"]
    #[inline(always)]
    pub fn m_mem_iter(&self) -> impl Iterator<Item = &M_MEM> {
        self.m_mem.iter()
    }
}
#[doc = "MODE (rw) register accessor: Defines the algorithm of SHA accelerator\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Defines the algorithm of SHA accelerator"]
pub mod mode;
#[doc = "T_STRING (rw) register accessor: String content register for calculating initial Hash Value (only effective for SHA-512/t)\n\nYou can [`read`](crate::Reg::read) this register and get [`t_string::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_string::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_string`] module"]
pub type T_STRING = crate::Reg<t_string::T_STRING_SPEC>;
#[doc = "String content register for calculating initial Hash Value (only effective for SHA-512/t)"]
pub mod t_string;
#[doc = "T_LENGTH (rw) register accessor: String length register for calculating initial Hash Value (only effective for SHA-512/t)\n\nYou can [`read`](crate::Reg::read) this register and get [`t_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_length`] module"]
pub type T_LENGTH = crate::Reg<t_length::T_LENGTH_SPEC>;
#[doc = "String length register for calculating initial Hash Value (only effective for SHA-512/t)"]
pub mod t_length;
#[doc = "DMA_BLOCK_NUM (rw) register accessor: Block number register (only effective for DMA-SHA)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_block_num`] module"]
pub type DMA_BLOCK_NUM = crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>;
#[doc = "Block number register (only effective for DMA-SHA)"]
pub mod dma_block_num;
#[doc = "START (w) register accessor: Starts the SHA accelerator for Typical SHA operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Starts the SHA accelerator for Typical SHA operation"]
pub mod start;
#[doc = "CONTINUE (w) register accessor: Continues SHA operation (only effective in Typical SHA mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_`] module"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "Continues SHA operation (only effective in Typical SHA mode)"]
pub mod continue_;
#[doc = "BUSY (r) register accessor: Indicates if SHA Accelerator is busy or not\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busy`] module"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Indicates if SHA Accelerator is busy or not"]
pub mod busy;
#[doc = "DMA_START (w) register accessor: Starts the SHA accelerator for DMA-SHA operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_start`] module"]
pub type DMA_START = crate::Reg<dma_start::DMA_START_SPEC>;
#[doc = "Starts the SHA accelerator for DMA-SHA operation"]
pub mod dma_start;
#[doc = "DMA_CONTINUE (w) register accessor: Continues SHA operation (only effective in DMA-SHA mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_continue`] module"]
pub type DMA_CONTINUE = crate::Reg<dma_continue::DMA_CONTINUE_SPEC>;
#[doc = "Continues SHA operation (only effective in DMA-SHA mode)"]
pub mod dma_continue;
#[doc = "INT_CLEAR (w) register accessor: DMA-SHA interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`] module"]
pub type INT_CLEAR = crate::Reg<int_clear::INT_CLEAR_SPEC>;
#[doc = "DMA-SHA interrupt clear register"]
pub mod int_clear;
#[doc = "INT_ENA (rw) register accessor: DMA-SHA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "DMA-SHA interrupt enable register"]
pub mod int_ena;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
#[doc = "H_MEM (rw) register accessor: Hash value\n\nYou can [`read`](crate::Reg::read) this register and get [`h_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`h_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_mem`] module"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "Hash value"]
pub mod h_mem;
#[doc = "M_MEM (rw) register accessor: Message\n\nYou can [`read`](crate::Reg::read) this register and get [`m_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_mem`] module"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Message"]
pub mod m_mem;
