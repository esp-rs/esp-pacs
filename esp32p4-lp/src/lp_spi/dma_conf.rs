#[doc = "Register `DMA_CONF` writer"]
pub type W = crate::W<DMA_CONF_SPEC>;
#[doc = "Field `LP_REG_RX_AFIFO_RST` writer - Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer."]
pub type LP_REG_RX_AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_BUF_AFIFO_RST` writer - Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer."]
pub type LP_REG_BUF_AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 29 - Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer."]
    #[inline(always)]
    pub fn lp_reg_rx_afifo_rst(&mut self) -> LP_REG_RX_AFIFO_RST_W<'_, DMA_CONF_SPEC> {
        LP_REG_RX_AFIFO_RST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer."]
    #[inline(always)]
    pub fn lp_reg_buf_afifo_rst(&mut self) -> LP_REG_BUF_AFIFO_RST_W<'_, DMA_CONF_SPEC> {
        LP_REG_BUF_AFIFO_RST_W::new(self, 30)
    }
}
#[doc = "SPI DMA control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_conf::W`](W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_CONF to value 0"]
impl crate::Resettable for DMA_CONF_SPEC {}
