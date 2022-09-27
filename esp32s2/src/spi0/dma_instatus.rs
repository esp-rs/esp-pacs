#[doc = "Register `DMA_INSTATUS` reader"]
pub struct R(crate::R<DMA_INSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_INDSCR_ADDR` reader - SPI dma in descriptor address."]
pub type DMA_INDSCR_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMA_INDSCR_STATE` reader - SPI dma in descriptor state."]
pub type DMA_INDSCR_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_IN_STATE` reader - SPI dma in data state."]
pub type DMA_IN_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_INFIFO_CNT` reader - The remains of SPI dma infifo data."]
pub type DMA_INFIFO_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA_INFIFO_FULL` reader - SPI dma infifo is full."]
pub type DMA_INFIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `DMA_INFIFO_EMPTY` reader - SPI dma infifo is empty."]
pub type DMA_INFIFO_EMPTY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:17 - SPI dma in descriptor address."]
    #[inline(always)]
    pub fn dma_indscr_addr(&self) -> DMA_INDSCR_ADDR_R {
        DMA_INDSCR_ADDR_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:19 - SPI dma in descriptor state."]
    #[inline(always)]
    pub fn dma_indscr_state(&self) -> DMA_INDSCR_STATE_R {
        DMA_INDSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - SPI dma in data state."]
    #[inline(always)]
    pub fn dma_in_state(&self) -> DMA_IN_STATE_R {
        DMA_IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:29 - The remains of SPI dma infifo data."]
    #[inline(always)]
    pub fn dma_infifo_cnt(&self) -> DMA_INFIFO_CNT_R {
        DMA_INFIFO_CNT_R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bit 30 - SPI dma infifo is full."]
    #[inline(always)]
    pub fn dma_infifo_full(&self) -> DMA_INFIFO_FULL_R {
        DMA_INFIFO_FULL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI dma infifo is empty."]
    #[inline(always)]
    pub fn dma_infifo_empty(&self) -> DMA_INFIFO_EMPTY_R {
        DMA_INFIFO_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SPI DMA RX status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_instatus](index.html) module"]
pub struct DMA_INSTATUS_SPEC;
impl crate::RegisterSpec for DMA_INSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_instatus::R](R) reader structure"]
impl crate::Readable for DMA_INSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INSTATUS to value 0x8000_0000"]
impl crate::Resettable for DMA_INSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
