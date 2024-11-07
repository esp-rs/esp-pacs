#[doc = "Register `IN_LINK1` reader"]
pub type R = crate::R<IN_LINK1_SPEC>;
#[doc = "Register `IN_LINK1` writer"]
pub type W = crate::W<IN_LINK1_SPEC>;
#[doc = "Field `INLINK_AUTO_RET` reader - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET` writer - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_STOP` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START` writer - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART` writer - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_PARK` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type INLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret(&self) -> INLINK_AUTO_RET_R {
        INLINK_AUTO_RET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park(&self) -> INLINK_PARK_R {
        INLINK_PARK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK1")
            .field("inlink_auto_ret", &self.inlink_auto_ret())
            .field("inlink_park", &self.inlink_park())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to return to current inlink descriptor's address when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret(&mut self) -> INLINK_AUTO_RET_W<IN_LINK1_SPEC> {
        INLINK_AUTO_RET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W<IN_LINK1_SPEC> {
        INLINK_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start(&mut self) -> INLINK_START_W<IN_LINK1_SPEC> {
        INLINK_START_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W<IN_LINK1_SPEC> {
        INLINK_RESTART_W::new(self, 3)
    }
}
#[doc = "Link descriptor configure and control register of Rx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`in_link1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_link1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK1_SPEC;
impl crate::RegisterSpec for IN_LINK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link1::R`](R) reader structure"]
impl crate::Readable for IN_LINK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link1::W`](W) writer structure"]
impl crate::Writable for IN_LINK1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_LINK1 to value 0x11"]
impl crate::Resettable for IN_LINK1_SPEC {
    const RESET_VALUE: u32 = 0x11;
}
