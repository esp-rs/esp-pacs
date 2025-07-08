#[doc = "Register `OUT_LINK_CH1` reader"]
pub type R = crate::R<OUT_LINK_CH1_SPEC>;
#[doc = "Register `OUT_LINK_CH1` writer"]
pub type W = crate::W<OUT_LINK_CH1_SPEC>;
#[doc = "Field `OUTLINK_STOP_CH1` writer - Configures whether to stop AHB_DMA's TX channel 1 from transmitting data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
pub type OUTLINK_STOP_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START_CH1` writer - Configures whether to enable AHB_DMA's TX channel 1 for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type OUTLINK_START_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART_CH1` writer - Configures whether to restart TX channel 1 for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
pub type OUTLINK_RESTART_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK_CH1` reader - Represents the status of the transmit descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
pub type OUTLINK_PARK_CH1_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Represents the status of the transmit descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
    #[inline(always)]
    pub fn outlink_park_ch1(&self) -> OUTLINK_PARK_CH1_R {
        OUTLINK_PARK_CH1_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_CH1")
            .field("outlink_park_ch1", &self.outlink_park_ch1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to stop AHB_DMA's TX channel 1 from transmitting data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
    #[inline(always)]
    pub fn outlink_stop_ch1(&mut self) -> OUTLINK_STOP_CH1_W<OUT_LINK_CH1_SPEC> {
        OUTLINK_STOP_CH1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to enable AHB_DMA's TX channel 1 for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn outlink_start_ch1(&mut self) -> OUTLINK_START_CH1_W<OUT_LINK_CH1_SPEC> {
        OUTLINK_START_CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to restart TX channel 1 for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
    #[inline(always)]
    pub fn outlink_restart_ch1(&mut self) -> OUTLINK_RESTART_CH1_W<OUT_LINK_CH1_SPEC> {
        OUTLINK_RESTART_CH1_W::new(self, 2)
    }
}
#[doc = "Push control register of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_CH1_SPEC;
impl crate::RegisterSpec for OUT_LINK_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link_ch1::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_LINK_CH1 to value 0x08"]
impl crate::Resettable for OUT_LINK_CH1_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
