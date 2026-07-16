#[doc = "Register `AHB_DMA_IN_LINK_ADDR_CH%s` reader"]
pub type R = crate::R<AHB_DMA_IN_LINK_ADDR_CH_SPEC>;
#[doc = "Register `AHB_DMA_IN_LINK_ADDR_CH%s` writer"]
pub type W = crate::W<AHB_DMA_IN_LINK_ADDR_CH_SPEC>;
#[doc = "Field `AHB_DMA_INLINK_ADDR_CH` reader - Configures the 32 bits of the first receive descriptor's address."]
pub type AHB_DMA_INLINK_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `AHB_DMA_INLINK_ADDR_CH` writer - Configures the 32 bits of the first receive descriptor's address."]
pub type AHB_DMA_INLINK_ADDR_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address."]
    #[inline(always)]
    pub fn ahb_dma_inlink_addr_ch(&self) -> AHB_DMA_INLINK_ADDR_CH_R {
        AHB_DMA_INLINK_ADDR_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_IN_LINK_ADDR_CH")
            .field("ahb_dma_inlink_addr_ch", &self.ahb_dma_inlink_addr_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the 32 bits of the first receive descriptor's address."]
    #[inline(always)]
    pub fn ahb_dma_inlink_addr_ch(
        &mut self,
    ) -> AHB_DMA_INLINK_ADDR_CH_W<'_, AHB_DMA_IN_LINK_ADDR_CH_SPEC> {
        AHB_DMA_INLINK_ADDR_CH_W::new(self, 0)
    }
}
#[doc = "Link list descriptor address configuration of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_link_addr_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_link_addr_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_IN_LINK_ADDR_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_IN_LINK_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_in_link_addr_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_IN_LINK_ADDR_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_in_link_addr_ch::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_IN_LINK_ADDR_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_IN_LINK_ADDR_CH%s to value 0"]
impl crate::Resettable for AHB_DMA_IN_LINK_ADDR_CH_SPEC {}
