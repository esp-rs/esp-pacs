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
    clear_irq: CLEAR_IRQ,
    irq_ena: IRQ_ENA,
    date: DATE,
    _reserved12: [u8; 0x10],
    h_mem: [H_MEM; 64],
    m_mem: [M_MEM; 64],
    _reserved14: [u8; 0x0740],
    _3_mode: _3_MODE,
    _3_clean_m: _3_CLEAN_M,
    _reserved16: [u8; 0x04],
    _3_dma_block_num: _3_DMA_BLOCK_NUM,
    _3_start: _3_START,
    _3_continue: _3_CONTINUE,
    _3_busy: _3_BUSY,
    _3_dma_start: _3_DMA_START,
    _3_dma_continue: _3_DMA_CONTINUE,
    _3_clear_int: _3_CLEAR_INT,
    _3_int_ena: _3_INT_ENA,
    _3_shake_length: _3_SHAKE_LENGTH,
    _reserved25: [u8; 0xd0],
    _3_m_out_mem: [_3_M_OUT_MEM; 200],
    _reserved26: [u8; 0x38],
    _3_m_mem: [_3_M_MEM; 200],
}
impl RegisterBlock {
    #[doc = "0x00 - Configures SHA algorithm"]
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    #[doc = "0x04 - SHA 512/t configuration register 0."]
    #[inline(always)]
    pub const fn t_string(&self) -> &T_STRING {
        &self.t_string
    }
    #[doc = "0x08 - SHA 512/t configuration register 1."]
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
    #[doc = "0x18 - Represents if SHA Accelerator is busy or not"]
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
    pub const fn clear_irq(&self) -> &CLEAR_IRQ {
        &self.clear_irq
    }
    #[doc = "0x28 - DMA-SHA interrupt enable register"]
    #[inline(always)]
    pub const fn irq_ena(&self) -> &IRQ_ENA {
        &self.irq_ena
    }
    #[doc = "0x2c - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    #[doc = "0x40..0x80 - Sha H memory which contains intermediate hash or finial hash."]
    #[inline(always)]
    pub const fn h_mem(&self, n: usize) -> &H_MEM {
        &self.h_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - Sha H memory which contains intermediate hash or finial hash."]
    #[inline(always)]
    pub fn h_mem_iter(&self) -> impl Iterator<Item = &H_MEM> {
        self.h_mem.iter()
    }
    #[doc = "0x80..0xc0 - Sha M memory which contains message."]
    #[inline(always)]
    pub const fn m_mem(&self, n: usize) -> &M_MEM {
        &self.m_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Sha M memory which contains message."]
    #[inline(always)]
    pub fn m_mem_iter(&self) -> impl Iterator<Item = &M_MEM> {
        self.m_mem.iter()
    }
    #[doc = "0x800 - Initial configuration register 0."]
    #[inline(always)]
    pub const fn _3_mode(&self) -> &_3_MODE {
        &self._3_mode
    }
    #[doc = "0x804 - Initial configuration register 1."]
    #[inline(always)]
    pub const fn _3_clean_m(&self) -> &_3_CLEAN_M {
        &self._3_clean_m
    }
    #[doc = "0x80c - DMA configuration register 0."]
    #[inline(always)]
    pub const fn _3_dma_block_num(&self) -> &_3_DMA_BLOCK_NUM {
        &self._3_dma_block_num
    }
    #[doc = "0x810 - Typical SHA3 configuration register 0."]
    #[inline(always)]
    pub const fn _3_start(&self) -> &_3_START {
        &self._3_start
    }
    #[doc = "0x814 - Typical SHA3 configuration register 1."]
    #[inline(always)]
    pub const fn _3_continue(&self) -> &_3_CONTINUE {
        &self._3_continue
    }
    #[doc = "0x818 - Busy register."]
    #[inline(always)]
    pub const fn _3_busy(&self) -> &_3_BUSY {
        &self._3_busy
    }
    #[doc = "0x81c - DMA configuration register 1."]
    #[inline(always)]
    pub const fn _3_dma_start(&self) -> &_3_DMA_START {
        &self._3_dma_start
    }
    #[doc = "0x820 - DMA configuration register 2."]
    #[inline(always)]
    pub const fn _3_dma_continue(&self) -> &_3_DMA_CONTINUE {
        &self._3_dma_continue
    }
    #[doc = "0x824 - Interrupt clear register."]
    #[inline(always)]
    pub const fn _3_clear_int(&self) -> &_3_CLEAR_INT {
        &self._3_clear_int
    }
    #[doc = "0x828 - Interrupt enable register."]
    #[inline(always)]
    pub const fn _3_int_ena(&self) -> &_3_INT_ENA {
        &self._3_int_ena
    }
    #[doc = "0x82c - DMA configuration register 3."]
    #[inline(always)]
    pub const fn _3_shake_length(&self) -> &_3_SHAKE_LENGTH {
        &self._3_shake_length
    }
    #[doc = "0x900..0x9c8 - Sha3 hash reg which contains intermediate hash or finial hash."]
    #[inline(always)]
    pub const fn _3_m_out_mem(&self, n: usize) -> &_3_M_OUT_MEM {
        &self._3_m_out_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x900..0x9c8 - Sha3 hash reg which contains intermediate hash or finial hash."]
    #[inline(always)]
    pub fn _3_m_out_mem_iter(&self) -> impl Iterator<Item = &_3_M_OUT_MEM> {
        self._3_m_out_mem.iter()
    }
    #[doc = "0xa00..0xac8 - Sha3 message reg which contains message."]
    #[inline(always)]
    pub const fn _3_m_mem(&self, n: usize) -> &_3_M_MEM {
        &self._3_m_mem[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa00..0xac8 - Sha3 message reg which contains message."]
    #[inline(always)]
    pub fn _3_m_mem_iter(&self) -> impl Iterator<Item = &_3_M_MEM> {
        self._3_m_mem.iter()
    }
}
#[doc = "MODE (rw) register accessor: Configures SHA algorithm\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Configures SHA algorithm"]
pub mod mode;
#[doc = "T_STRING (rw) register accessor: SHA 512/t configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`t_string::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_string::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_string`] module"]
pub type T_STRING = crate::Reg<t_string::T_STRING_SPEC>;
#[doc = "SHA 512/t configuration register 0."]
pub mod t_string;
#[doc = "T_LENGTH (rw) register accessor: SHA 512/t configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`t_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@t_length`] module"]
pub type T_LENGTH = crate::Reg<t_length::T_LENGTH_SPEC>;
#[doc = "SHA 512/t configuration register 1."]
pub mod t_length;
#[doc = "DMA_BLOCK_NUM (rw) register accessor: Block number register (only effective for DMA-SHA)\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_block_num`] module"]
pub type DMA_BLOCK_NUM = crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>;
#[doc = "Block number register (only effective for DMA-SHA)"]
pub mod dma_block_num;
#[doc = "START (r) register accessor: Starts the SHA accelerator for Typical SHA operation\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`] module"]
pub type START = crate::Reg<start::START_SPEC>;
#[doc = "Starts the SHA accelerator for Typical SHA operation"]
pub mod start;
#[doc = "CONTINUE (r) register accessor: Continues SHA operation (only effective in Typical SHA mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`continue_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@continue_`] module"]
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
#[doc = "Continues SHA operation (only effective in Typical SHA mode)"]
pub mod continue_;
#[doc = "BUSY (r) register accessor: Represents if SHA Accelerator is busy or not\n\nYou can [`read`](crate::Reg::read) this register and get [`busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@busy`] module"]
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
#[doc = "Represents if SHA Accelerator is busy or not"]
pub mod busy;
#[doc = "DMA_START (w) register accessor: Starts the SHA accelerator for DMA-SHA operation\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_start`] module"]
pub type DMA_START = crate::Reg<dma_start::DMA_START_SPEC>;
#[doc = "Starts the SHA accelerator for DMA-SHA operation"]
pub mod dma_start;
#[doc = "DMA_CONTINUE (w) register accessor: Continues SHA operation (only effective in DMA-SHA mode)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_continue`] module"]
pub type DMA_CONTINUE = crate::Reg<dma_continue::DMA_CONTINUE_SPEC>;
#[doc = "Continues SHA operation (only effective in DMA-SHA mode)"]
pub mod dma_continue;
#[doc = "CLEAR_IRQ (w) register accessor: DMA-SHA interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear_irq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear_irq`] module"]
pub type CLEAR_IRQ = crate::Reg<clear_irq::CLEAR_IRQ_SPEC>;
#[doc = "DMA-SHA interrupt clear register"]
pub mod clear_irq;
#[doc = "IRQ_ENA (rw) register accessor: DMA-SHA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_ena`] module"]
pub type IRQ_ENA = crate::Reg<irq_ena::IRQ_ENA_SPEC>;
#[doc = "DMA-SHA interrupt enable register"]
pub mod irq_ena;
pub use crate::dma::date;
pub use crate::dma::DATE;
#[doc = "H_MEM (rw) register accessor: Sha H memory which contains intermediate hash or finial hash.\n\nYou can [`read`](crate::Reg::read) this register and get [`h_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`h_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@h_mem`] module"]
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
#[doc = "Sha H memory which contains intermediate hash or finial hash."]
pub mod h_mem;
#[doc = "M_MEM (rw) register accessor: Sha M memory which contains message.\n\nYou can [`read`](crate::Reg::read) this register and get [`m_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_mem`] module"]
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
#[doc = "Sha M memory which contains message."]
pub mod m_mem;
#[doc = "_3_MODE (rw) register accessor: Initial configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_mode`] module"]
pub type _3_MODE = crate::Reg<_3_mode::_3_MODE_SPEC>;
#[doc = "Initial configuration register 0."]
pub mod _3_mode;
#[doc = "_3_CLEAN_M (w) register accessor: Initial configuration register 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_clean_m::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_clean_m`] module"]
pub type _3_CLEAN_M = crate::Reg<_3_clean_m::_3_CLEAN_M_SPEC>;
#[doc = "Initial configuration register 1."]
pub mod _3_clean_m;
#[doc = "_3_DMA_BLOCK_NUM (rw) register accessor: DMA configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_dma_block_num::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_dma_block_num::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_dma_block_num`] module"]
pub type _3_DMA_BLOCK_NUM = crate::Reg<_3_dma_block_num::_3_DMA_BLOCK_NUM_SPEC>;
#[doc = "DMA configuration register 0."]
pub mod _3_dma_block_num;
#[doc = "_3_START (w) register accessor: Typical SHA3 configuration register 0.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_start`] module"]
pub type _3_START = crate::Reg<_3_start::_3_START_SPEC>;
#[doc = "Typical SHA3 configuration register 0."]
pub mod _3_start;
#[doc = "_3_CONTINUE (w) register accessor: Typical SHA3 configuration register 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_continue`] module"]
pub type _3_CONTINUE = crate::Reg<_3_continue::_3_CONTINUE_SPEC>;
#[doc = "Typical SHA3 configuration register 1."]
pub mod _3_continue;
#[doc = "_3_BUSY (r) register accessor: Busy register.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_busy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_busy`] module"]
pub type _3_BUSY = crate::Reg<_3_busy::_3_BUSY_SPEC>;
#[doc = "Busy register."]
pub mod _3_busy;
#[doc = "_3_DMA_START (w) register accessor: DMA configuration register 1.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_dma_start`] module"]
pub type _3_DMA_START = crate::Reg<_3_dma_start::_3_DMA_START_SPEC>;
#[doc = "DMA configuration register 1."]
pub mod _3_dma_start;
#[doc = "_3_DMA_CONTINUE (w) register accessor: DMA configuration register 2.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_dma_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_dma_continue`] module"]
pub type _3_DMA_CONTINUE = crate::Reg<_3_dma_continue::_3_DMA_CONTINUE_SPEC>;
#[doc = "DMA configuration register 2."]
pub mod _3_dma_continue;
#[doc = "_3_CLEAR_INT (w) register accessor: Interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_clear_int::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_clear_int`] module"]
pub type _3_CLEAR_INT = crate::Reg<_3_clear_int::_3_CLEAR_INT_SPEC>;
#[doc = "Interrupt clear register."]
pub mod _3_clear_int;
#[doc = "_3_INT_ENA (rw) register accessor: Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_int_ena`] module"]
pub type _3_INT_ENA = crate::Reg<_3_int_ena::_3_INT_ENA_SPEC>;
#[doc = "Interrupt enable register."]
pub mod _3_int_ena;
#[doc = "_3_SHAKE_LENGTH (w) register accessor: DMA configuration register 3.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_shake_length::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_shake_length`] module"]
pub type _3_SHAKE_LENGTH = crate::Reg<_3_shake_length::_3_SHAKE_LENGTH_SPEC>;
#[doc = "DMA configuration register 3."]
pub mod _3_shake_length;
#[doc = "_3_M_OUT_MEM (rw) register accessor: Sha3 hash reg which contains intermediate hash or finial hash.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_m_out_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_m_out_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_m_out_mem`] module"]
pub type _3_M_OUT_MEM = crate::Reg<_3_m_out_mem::_3_M_OUT_MEM_SPEC>;
#[doc = "Sha3 hash reg which contains intermediate hash or finial hash."]
pub mod _3_m_out_mem;
#[doc = "_3_M_MEM (rw) register accessor: Sha3 message reg which contains message.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_m_mem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_m_mem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_3_m_mem`] module"]
pub type _3_M_MEM = crate::Reg<_3_m_mem::_3_M_MEM_SPEC>;
#[doc = "Sha3 message reg which contains message."]
pub mod _3_m_mem;
