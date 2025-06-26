#[doc = "Register `OUT_LINK_CH%s` reader"]
pub type R = crate::R<OUT_LINK_CH_SPEC>;
#[doc = "Register `OUT_LINK_CH%s` writer"]
pub type W = crate::W<OUT_LINK_CH_SPEC>;
#[doc = "Field `OUTLINK_STOP` writer - Configures whether or not to stop AHB_DMA's TX channel %s from transmitting data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` writer - Configures whether or not to enable AHB_DMA's TX channel %s for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` writer - Configures whether or not to restart TX channel %s for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK` reader - Represents the status of the transmit descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Represents the status of the transmit descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_CH")
            .field("outlink_park", &self.outlink_park())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to stop AHB_DMA's TX channel %s from transmitting data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
    #[inline(always)]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<OUT_LINK_CH_SPEC> {
        OUTLINK_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable AHB_DMA's TX channel %s for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<OUT_LINK_CH_SPEC> {
        OUTLINK_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to restart TX channel %s for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
    #[inline(always)]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<OUT_LINK_CH_SPEC> {
        OUTLINK_RESTART_W::new(self, 2)
    }
}
#[doc = "Linked list descriptor configuration and control register of TX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_CH_SPEC;
impl crate::RegisterSpec for OUT_LINK_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link_ch::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link_ch::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_LINK_CH%s to value 0x08"]
impl crate::Resettable for OUT_LINK_CH_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
