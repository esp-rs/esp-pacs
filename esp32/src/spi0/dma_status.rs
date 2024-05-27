///Register `DMA_STATUS` reader
pub type R = crate::R<DMA_STATUS_SPEC>;
///Field `DMA_RX_EN` reader - spi dma read data status bit.
pub type DMA_RX_EN_R = crate::BitReader;
///Field `DMA_TX_EN` reader - spi dma write data status bit.
pub type DMA_TX_EN_R = crate::BitReader;
impl R {
    ///Bit 0 - spi dma read data status bit.
    #[inline(always)]
    pub fn dma_rx_en(&self) -> DMA_RX_EN_R {
        DMA_RX_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - spi dma write data status bit.
    #[inline(always)]
    pub fn dma_tx_en(&self) -> DMA_TX_EN_R {
        DMA_TX_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_STATUS")
            .field("dma_rx_en", &self.dma_rx_en())
            .field("dma_tx_en", &self.dma_tx_en())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dma_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_STATUS_SPEC;
impl crate::RegisterSpec for DMA_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_status::R`](R) reader structure
impl crate::Readable for DMA_STATUS_SPEC {}
///`reset()` method sets DMA_STATUS to value 0
impl crate::Resettable for DMA_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
