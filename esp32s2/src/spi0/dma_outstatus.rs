#[doc = "Register `DMA_OUTSTATUS` reader"]
pub struct R(crate::R<DMA_OUTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_OUTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_OUTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_OUTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_OUTDSCR_ADDR` reader - SPI dma out descriptor address."]
pub type DMA_OUTDSCR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMA_OUTDSCR_STATE` reader - SPI dma out descriptor state."]
pub type DMA_OUTDSCR_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_OUT_STATE` reader - SPI dma out data state."]
pub type DMA_OUT_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_OUTFIFO_CNT` reader - The remains of SPI dma outfifo data."]
pub type DMA_OUTFIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_OUTFIFO_FULL` reader - SPI dma outfifo is full."]
pub type DMA_OUTFIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `DMA_OUTFIFO_EMPTY` reader - SPI dma outfifo is empty."]
pub type DMA_OUTFIFO_EMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:17 - SPI dma out descriptor address."]
    #[inline(always)]
    pub fn dma_outdscr_addr(&self) -> DMA_OUTDSCR_ADDR_R {
        DMA_OUTDSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - SPI dma out descriptor state."]
    #[inline(always)]
    pub fn dma_outdscr_state(&self) -> DMA_OUTDSCR_STATE_R {
        DMA_OUTDSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - SPI dma out data state."]
    #[inline(always)]
    pub fn dma_out_state(&self) -> DMA_OUT_STATE_R {
        DMA_OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:29 - The remains of SPI dma outfifo data."]
    #[inline(always)]
    pub fn dma_outfifo_cnt(&self) -> DMA_OUTFIFO_CNT_R {
        DMA_OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - SPI dma outfifo is full."]
    #[inline(always)]
    pub fn dma_outfifo_full(&self) -> DMA_OUTFIFO_FULL_R {
        DMA_OUTFIFO_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI dma outfifo is empty."]
    #[inline(always)]
    pub fn dma_outfifo_empty(&self) -> DMA_OUTFIFO_EMPTY_R {
        DMA_OUTFIFO_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SPI DMA TX status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_outstatus](index.html) module"]
pub struct DMA_OUTSTATUS_SPEC;
impl crate::RegisterSpec for DMA_OUTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_outstatus::R](R) reader structure"]
impl crate::Readable for DMA_OUTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_OUTSTATUS to value 0x8000_0000"]
impl crate::Resettable for DMA_OUTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
