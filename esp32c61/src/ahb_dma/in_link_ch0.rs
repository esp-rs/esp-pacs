#[doc = "Register `IN_LINK_CH0` reader"]
pub type R = crate::R<IN_LINK_CH0_SPEC>;
#[doc = "Register `IN_LINK_CH0` writer"]
pub type W = crate::W<IN_LINK_CH0_SPEC>;
#[doc = "Field `INLINK_AUTO_RET_CH0` reader - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\."]
pub type INLINK_AUTO_RET_CH0_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET_CH0` writer - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\."]
pub type INLINK_AUTO_RET_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_STOP_CH0` writer - Configures whether to stop AHB_DMA's RX channel 0 from receiving data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
pub type INLINK_STOP_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START_CH0` writer - Configures whether to enable AHB_DMA's RX channel 0 for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
pub type INLINK_START_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART_CH0` writer - Configures whether to restart RX channel 0 for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
pub type INLINK_RESTART_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_PARK_CH0` reader - Represents the status of the receive descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
pub type INLINK_PARK_CH0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch0(&self) -> INLINK_AUTO_RET_CH0_R {
        INLINK_AUTO_RET_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the status of the receive descriptor's FSM.\\\\0: Running\\\\1: Idle\\\\"]
    #[inline(always)]
    pub fn inlink_park_ch0(&self) -> INLINK_PARK_CH0_R {
        INLINK_PARK_CH0_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK_CH0")
            .field("inlink_auto_ret_ch0", &self.inlink_auto_ret_ch0())
            .field("inlink_park_ch0", &self.inlink_park_ch0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to return to current receive descriptor's address when there are some errors in current receiving data.\\\\0: Not return\\\\1: Return\\\\."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch0(&mut self) -> INLINK_AUTO_RET_CH0_W<IN_LINK_CH0_SPEC> {
        INLINK_AUTO_RET_CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether to stop AHB_DMA's RX channel 0 from receiving data.\\\\0: Invalid. No effect\\\\1: Stop\\\\"]
    #[inline(always)]
    pub fn inlink_stop_ch0(&mut self) -> INLINK_STOP_CH0_W<IN_LINK_CH0_SPEC> {
        INLINK_STOP_CH0_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether to enable AHB_DMA's RX channel 0 for data transfer.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn inlink_start_ch0(&mut self) -> INLINK_START_CH0_W<IN_LINK_CH0_SPEC> {
        INLINK_START_CH0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether to restart RX channel 0 for AHB_DMA transfer.\\\\0: Invalid. No effect\\\\1: Restart\\\\"]
    #[inline(always)]
    pub fn inlink_restart_ch0(&mut self) -> INLINK_RESTART_CH0_W<IN_LINK_CH0_SPEC> {
        INLINK_RESTART_CH0_W::new(self, 3)
    }
}
#[doc = "Receive FIFO status of RX channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link_ch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link_ch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK_CH0_SPEC;
impl crate::RegisterSpec for IN_LINK_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link_ch0::R`](R) reader structure"]
impl crate::Readable for IN_LINK_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link_ch0::W`](W) writer structure"]
impl crate::Writable for IN_LINK_CH0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_LINK_CH0 to value 0x11"]
impl crate::Resettable for IN_LINK_CH0_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
