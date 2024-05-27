#[doc = "Register `OUT_LINK1` reader"]
pub type R = crate::R<OUT_LINK1_SPEC>;
#[doc = "Register `OUT_LINK1` writer"]
pub type W = crate::W<OUT_LINK1_SPEC>;
#[doc = "Field `OUTLINK_STOP` writer - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` writer - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` writer - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK1")
            .field("outlink_park", &self.outlink_park())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<OUT_LINK1_SPEC> {
        OUTLINK_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<OUT_LINK1_SPEC> {
        OUTLINK_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<OUT_LINK1_SPEC> {
        OUTLINK_RESTART_W::new(self, 2)
    }
}
#[doc = "Link descriptor configure and control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK1_SPEC;
impl crate::RegisterSpec for OUT_LINK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link1::R`](R) reader structure"]
impl crate::Readable for OUT_LINK1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link1::W`](W) writer structure"]
impl crate::Writable for OUT_LINK1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_LINK1 to value 0x08"]
impl crate::Resettable for OUT_LINK1_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
