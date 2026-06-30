#[doc = "Register `AHB_DMA_IN_LINK_CH%s` reader"]
pub type R = crate::R<AHB_DMA_IN_LINK_CH_SPEC>;
#[doc = "Register `AHB_DMA_IN_LINK_CH%s` writer"]
pub type W = crate::W<AHB_DMA_IN_LINK_CH_SPEC>;
#[doc = "Field `AHB_DMA_INLINK_AUTO_RET_CH` reader - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\"]
pub type AHB_DMA_INLINK_AUTO_RET_CH_R = crate::BitReader;
#[doc = "Field `AHB_DMA_INLINK_AUTO_RET_CH` writer - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\"]
pub type AHB_DMA_INLINK_AUTO_RET_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_INLINK_STOP_CH` writer - Configures whether to stop AHB_DMA's RX channel %s from receiving data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
pub type AHB_DMA_INLINK_STOP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_INLINK_START_CH` writer - Configures whether to enable AHB_DMA's RX channel %s for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_INLINK_START_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_INLINK_RESTART_CH` writer - Configures whether to restart RX channel %s for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
pub type AHB_DMA_INLINK_RESTART_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_DMA_INLINK_PARK_CH` reader - Represents the status of the receive descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
pub type AHB_DMA_INLINK_PARK_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\"]
    #[inline(always)]
    pub fn ahb_dma_inlink_auto_ret_ch(&self) -> AHB_DMA_INLINK_AUTO_RET_CH_R {
        AHB_DMA_INLINK_AUTO_RET_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the status of the receive descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
    #[inline(always)]
    pub fn ahb_dma_inlink_park_ch(&self) -> AHB_DMA_INLINK_PARK_CH_R {
        AHB_DMA_INLINK_PARK_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_IN_LINK_CH")
            .field(
                "ahb_dma_inlink_auto_ret_ch",
                &self.ahb_dma_inlink_auto_ret_ch(),
            )
            .field("ahb_dma_inlink_park_ch", &self.ahb_dma_inlink_park_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\"]
    #[inline(always)]
    pub fn ahb_dma_inlink_auto_ret_ch(
        &mut self,
    ) -> AHB_DMA_INLINK_AUTO_RET_CH_W<'_, AHB_DMA_IN_LINK_CH_SPEC> {
        AHB_DMA_INLINK_AUTO_RET_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to stop AHB_DMA's RX channel %s from receiving data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
    #[inline(always)]
    pub fn ahb_dma_inlink_stop_ch(
        &mut self,
    ) -> AHB_DMA_INLINK_STOP_CH_W<'_, AHB_DMA_IN_LINK_CH_SPEC> {
        AHB_DMA_INLINK_STOP_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to enable AHB_DMA's RX channel %s for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_inlink_start_ch(
        &mut self,
    ) -> AHB_DMA_INLINK_START_CH_W<'_, AHB_DMA_IN_LINK_CH_SPEC> {
        AHB_DMA_INLINK_START_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether to restart RX channel %s for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
    #[inline(always)]
    pub fn ahb_dma_inlink_restart_ch(
        &mut self,
    ) -> AHB_DMA_INLINK_RESTART_CH_W<'_, AHB_DMA_IN_LINK_CH_SPEC> {
        AHB_DMA_INLINK_RESTART_CH_W::new(self, 3)
    }
}
#[doc = "Linked list descriptor configuration and control register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_link_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_link_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_IN_LINK_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_IN_LINK_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_in_link_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_IN_LINK_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_in_link_ch::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_IN_LINK_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_IN_LINK_CH%s to value 0x11"]
impl crate::Resettable for AHB_DMA_IN_LINK_CH_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
