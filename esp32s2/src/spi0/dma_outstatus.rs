///Register `DMA_OUTSTATUS` reader
pub type R = crate::R<DMA_OUTSTATUS_SPEC>;
///Field `DMA_OUTDSCR_ADDR` reader - SPI dma out descriptor address.
pub type DMA_OUTDSCR_ADDR_R = crate::FieldReader<u32>;
///Field `DMA_OUTDSCR_STATE` reader - SPI dma out descriptor state.
pub type DMA_OUTDSCR_STATE_R = crate::FieldReader;
///Field `DMA_OUT_STATE` reader - SPI dma out data state.
pub type DMA_OUT_STATE_R = crate::FieldReader;
///Field `DMA_OUTFIFO_CNT` reader - The remains of SPI dma outfifo data.
pub type DMA_OUTFIFO_CNT_R = crate::FieldReader;
///Field `DMA_OUTFIFO_FULL` reader - SPI dma outfifo is full.
pub type DMA_OUTFIFO_FULL_R = crate::BitReader;
///Field `DMA_OUTFIFO_EMPTY` reader - SPI dma outfifo is empty.
pub type DMA_OUTFIFO_EMPTY_R = crate::BitReader;
impl R {
    ///Bits 0:17 - SPI dma out descriptor address.
    #[inline(always)]
    pub fn dma_outdscr_addr(&self) -> DMA_OUTDSCR_ADDR_R {
        DMA_OUTDSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:19 - SPI dma out descriptor state.
    #[inline(always)]
    pub fn dma_outdscr_state(&self) -> DMA_OUTDSCR_STATE_R {
        DMA_OUTDSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:22 - SPI dma out data state.
    #[inline(always)]
    pub fn dma_out_state(&self) -> DMA_OUT_STATE_R {
        DMA_OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    ///Bits 23:29 - The remains of SPI dma outfifo data.
    #[inline(always)]
    pub fn dma_outfifo_cnt(&self) -> DMA_OUTFIFO_CNT_R {
        DMA_OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    ///Bit 30 - SPI dma outfifo is full.
    #[inline(always)]
    pub fn dma_outfifo_full(&self) -> DMA_OUTFIFO_FULL_R {
        DMA_OUTFIFO_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - SPI dma outfifo is empty.
    #[inline(always)]
    pub fn dma_outfifo_empty(&self) -> DMA_OUTFIFO_EMPTY_R {
        DMA_OUTFIFO_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUTSTATUS")
            .field("dma_outdscr_addr", &self.dma_outdscr_addr())
            .field("dma_outdscr_state", &self.dma_outdscr_state())
            .field("dma_out_state", &self.dma_out_state())
            .field("dma_outfifo_cnt", &self.dma_outfifo_cnt())
            .field("dma_outfifo_full", &self.dma_outfifo_full())
            .field("dma_outfifo_empty", &self.dma_outfifo_empty())
            .finish()
    }
}
/**SPI DMA TX status

You can [`read`](crate::generic::Reg::read) this register and get [`dma_outstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_OUTSTATUS_SPEC;
impl crate::RegisterSpec for DMA_OUTSTATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_outstatus::R`](R) reader structure
impl crate::Readable for DMA_OUTSTATUS_SPEC {}
///`reset()` method sets DMA_OUTSTATUS to value 0x8000_0000
impl crate::Resettable for DMA_OUTSTATUS_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
