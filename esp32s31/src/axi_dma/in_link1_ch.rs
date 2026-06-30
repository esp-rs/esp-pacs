#[doc = "Register `IN_LINK1_CH%s` reader"]
pub type R = crate::R<IN_LINK1_CH_SPEC>;
#[doc = "Register `IN_LINK1_CH%s` writer"]
pub type W = crate::W<IN_LINK1_CH_SPEC>;
#[doc = "Field `INLINK_AUTO_RET_CH` reader - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET_CH` writer - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_STOP_CH` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START_CH` writer - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART_CH` writer - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_PARK_CH` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type INLINK_PARK_CH_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch(&self) -> INLINK_AUTO_RET_CH_R {
        INLINK_AUTO_RET_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park_ch(&self) -> INLINK_PARK_CH_R {
        INLINK_PARK_CH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK1_CH")
            .field("inlink_auto_ret_ch", &self.inlink_auto_ret_ch())
            .field("inlink_park_ch", &self.inlink_park_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch(&mut self) -> INLINK_AUTO_RET_CH_W<'_, IN_LINK1_CH_SPEC> {
        INLINK_AUTO_RET_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch(&mut self) -> INLINK_STOP_CH_W<'_, IN_LINK1_CH_SPEC> {
        INLINK_STOP_CH_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch(&mut self) -> INLINK_START_CH_W<'_, IN_LINK1_CH_SPEC> {
        INLINK_START_CH_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch(&mut self) -> INLINK_RESTART_CH_W<'_, IN_LINK1_CH_SPEC> {
        INLINK_RESTART_CH_W::new(self, 3)
    }
}
#[doc = "Link descriptor configure and control register of Rx channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link1_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link1_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK1_CH_SPEC;
impl crate::RegisterSpec for IN_LINK1_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link1_ch::R`](R) reader structure"]
impl crate::Readable for IN_LINK1_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link1_ch::W`](W) writer structure"]
impl crate::Writable for IN_LINK1_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_LINK1_CH%s to value 0x11"]
impl crate::Resettable for IN_LINK1_CH_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
