#[doc = "Register `DMA_STATUS` reader"]
pub struct R(crate::R<DMA_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_RX_EN` reader - spi dma read data status bit."]
pub type DMA_RX_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_TX_EN` reader - spi dma write data status bit."]
pub type DMA_TX_EN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - spi dma read data status bit."]
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - spi dma write data status bit."]
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_status](index.html) module"]
pub struct DMA_STATUS_SPEC;
impl crate::RegisterSpec for DMA_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_status::R](R) reader structure"]
impl crate::Readable for DMA_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_STATUS to value 0"]
impl crate::Resettable for DMA_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
