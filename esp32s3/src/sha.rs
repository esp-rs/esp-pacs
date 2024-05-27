#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    h_mem: [H_MEM; 16],
    m_mem: [M_MEM; 16],
}
impl RegisterBlock {
    ///0x00 - Initial configuration register.
    #[inline(always)]
    pub const fn mode(&self) -> &MODE {
        &self.mode
    }
    ///0x04 - SHA 512/t configuration register 0.
    #[inline(always)]
    pub const fn t_string(&self) -> &T_STRING {
        &self.t_string
    }
    ///0x08 - SHA 512/t configuration register 1.
    #[inline(always)]
    pub const fn t_length(&self) -> &T_LENGTH {
        &self.t_length
    }
    ///0x0c - DMA configuration register 0.
    #[inline(always)]
    pub const fn dma_block_num(&self) -> &DMA_BLOCK_NUM {
        &self.dma_block_num
    }
    ///0x10 - Typical SHA configuration register 0.
    #[inline(always)]
    pub const fn start(&self) -> &START {
        &self.start
    }
    ///0x14 - Typical SHA configuration register 1.
    #[inline(always)]
    pub const fn continue_(&self) -> &CONTINUE {
        &self.continue_
    }
    ///0x18 - Busy register.
    #[inline(always)]
    pub const fn busy(&self) -> &BUSY {
        &self.busy
    }
    ///0x1c - DMA configuration register 1.
    #[inline(always)]
    pub const fn dma_start(&self) -> &DMA_START {
        &self.dma_start
    }
    ///0x20 - DMA configuration register 2.
    #[inline(always)]
    pub const fn dma_continue(&self) -> &DMA_CONTINUE {
        &self.dma_continue
    }
    ///0x24 - Interrupt clear register.
    #[inline(always)]
    pub const fn clear_irq(&self) -> &CLEAR_IRQ {
        &self.clear_irq
    }
    ///0x28 - Interrupt enable register.
    #[inline(always)]
    pub const fn irq_ena(&self) -> &IRQ_ENA {
        &self.irq_ena
    }
    ///0x2c - Date register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
    ///0x40..0x80 - Sha H memory which contains intermediate hash or finial hash.
    #[inline(always)]
    pub const fn h_mem(&self, n: usize) -> &H_MEM {
        &self.h_mem[n]
    }
    ///Iterator for array of:
    ///0x40..0x80 - Sha H memory which contains intermediate hash or finial hash.
    #[inline(always)]
    pub fn h_mem_iter(&self) -> impl Iterator<Item = &H_MEM> {
        self.h_mem.iter()
    }
    ///0x80..0xc0 - Sha M memory which contains message.
    #[inline(always)]
    pub const fn m_mem(&self, n: usize) -> &M_MEM {
        &self.m_mem[n]
    }
    ///Iterator for array of:
    ///0x80..0xc0 - Sha M memory which contains message.
    #[inline(always)]
    pub fn m_mem_iter(&self) -> impl Iterator<Item = &M_MEM> {
        self.m_mem.iter()
    }
}
/**MODE (rw) register accessor: Initial configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mode`] module*/
pub type MODE = crate::Reg<mode::MODE_SPEC>;
///Initial configuration register.
pub mod mode;
/**T_STRING (rw) register accessor: SHA 512/t configuration register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`t_string::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_string::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@t_string`] module*/
pub type T_STRING = crate::Reg<t_string::T_STRING_SPEC>;
///SHA 512/t configuration register 0.
pub mod t_string;
/**T_LENGTH (rw) register accessor: SHA 512/t configuration register 1.

You can [`read`](crate::generic::Reg::read) this register and get [`t_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@t_length`] module*/
pub type T_LENGTH = crate::Reg<t_length::T_LENGTH_SPEC>;
///SHA 512/t configuration register 1.
pub mod t_length;
/**DMA_BLOCK_NUM (rw) register accessor: DMA configuration register 0.

You can [`read`](crate::generic::Reg::read) this register and get [`dma_block_num::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_block_num::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_block_num`] module*/
pub type DMA_BLOCK_NUM = crate::Reg<dma_block_num::DMA_BLOCK_NUM_SPEC>;
///DMA configuration register 0.
pub mod dma_block_num;
/**START (w) register accessor: Typical SHA configuration register 0.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@start`] module*/
pub type START = crate::Reg<start::START_SPEC>;
///Typical SHA configuration register 0.
pub mod start;
/**CONTINUE (w) register accessor: Typical SHA configuration register 1.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@continue_`] module*/
pub type CONTINUE = crate::Reg<continue_::CONTINUE_SPEC>;
///Typical SHA configuration register 1.
pub mod continue_;
/**BUSY (r) register accessor: Busy register.

You can [`read`](crate::generic::Reg::read) this register and get [`busy::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busy`] module*/
pub type BUSY = crate::Reg<busy::BUSY_SPEC>;
///Busy register.
pub mod busy;
/**DMA_START (w) register accessor: DMA configuration register 1.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_start`] module*/
pub type DMA_START = crate::Reg<dma_start::DMA_START_SPEC>;
///DMA configuration register 1.
pub mod dma_start;
/**DMA_CONTINUE (w) register accessor: DMA configuration register 2.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_continue::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_continue`] module*/
pub type DMA_CONTINUE = crate::Reg<dma_continue::DMA_CONTINUE_SPEC>;
///DMA configuration register 2.
pub mod dma_continue;
/**CLEAR_IRQ (w) register accessor: Interrupt clear register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_irq::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clear_irq`] module*/
pub type CLEAR_IRQ = crate::Reg<clear_irq::CLEAR_IRQ_SPEC>;
///Interrupt clear register.
pub mod clear_irq;
/**IRQ_ENA (rw) register accessor: Interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`irq_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irq_ena`] module*/
pub type IRQ_ENA = crate::Reg<irq_ena::IRQ_ENA_SPEC>;
///Interrupt enable register.
pub mod irq_ena;
/**DATE (rw) register accessor: Date register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Date register.
pub mod date;
/**H_MEM (rw) register accessor: Sha H memory which contains intermediate hash or finial hash.

You can [`read`](crate::generic::Reg::read) this register and get [`h_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`h_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@h_mem`] module*/
pub type H_MEM = crate::Reg<h_mem::H_MEM_SPEC>;
///Sha H memory which contains intermediate hash or finial hash.
pub mod h_mem;
/**M_MEM (rw) register accessor: Sha M memory which contains message.

You can [`read`](crate::generic::Reg::read) this register and get [`m_mem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m_mem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@m_mem`] module*/
pub type M_MEM = crate::Reg<m_mem::M_MEM_SPEC>;
///Sha M memory which contains message.
pub mod m_mem;
