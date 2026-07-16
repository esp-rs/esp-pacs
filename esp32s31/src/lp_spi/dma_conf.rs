#[doc = "Register `DMA_CONF` writer"]
pub type W = crate::W<DMA_CONF_SPEC>;
#[doc = "Field `RX_AFIFO_RST` writer - "]
pub type RX_AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_AFIFO_RST` writer - "]
pub type BUF_AFIFO_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn rx_afifo_rst(&mut self) -> RX_AFIFO_RST_W<'_, DMA_CONF_SPEC> {
        RX_AFIFO_RST_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn buf_afifo_rst(&mut self) -> BUF_AFIFO_RST_W<'_, DMA_CONF_SPEC> {
        BUF_AFIFO_RST_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_conf::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
