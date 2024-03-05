#[doc = "Register `IN_LINK_CH%s` reader"]
pub type R = crate::R<IN_LINK_CH_SPEC>;
#[doc = "Register `IN_LINK_CH%s` writer"]
pub type W = crate::W<IN_LINK_CH_SPEC>;
#[doc = "Field `INLINK_ADDR` reader - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - This register stores the 20 least significant bits of the first inlink descriptor's address."]
pub type INLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `INLINK_AUTO_RET` reader - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET` writer - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_STOP` reader - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_R = crate::BitReader;
#[doc = "Field `INLINK_STOP` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START` reader - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_R = crate::BitReader;
#[doc = "Field `INLINK_START` writer - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART` reader - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_R = crate::BitReader;
#[doc = "Field `INLINK_RESTART` writer - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_PARK` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type INLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret(&self) -> INLINK_AUTO_RET_R {
        INLINK_AUTO_RET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop(&self) -> INLINK_STOP_R {
        INLINK_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start(&self) -> INLINK_START_R {
        INLINK_START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart(&self) -> INLINK_RESTART_R {
        INLINK_RESTART_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park(&self) -> INLINK_PARK_R {
        INLINK_PARK_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK_CH")
            .field(
                "inlink_addr",
                &format_args!("{}", self.inlink_addr().bits()),
            )
            .field(
                "inlink_auto_ret",
                &format_args!("{}", self.inlink_auto_ret().bit()),
            )
            .field("inlink_stop", &format_args!("{}", self.inlink_stop().bit()))
            .field(
                "inlink_start",
                &format_args!("{}", self.inlink_start().bit()),
            )
            .field(
                "inlink_restart",
                &format_args!("{}", self.inlink_restart().bit()),
            )
            .field("inlink_park", &format_args!("{}", self.inlink_park().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_LINK_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first inlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<IN_LINK_CH_SPEC> {
        INLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_auto_ret(&mut self) -> INLINK_AUTO_RET_W<IN_LINK_CH_SPEC> {
        INLINK_AUTO_RET_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W<IN_LINK_CH_SPEC> {
        INLINK_STOP_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_start(&mut self) -> INLINK_START_W<IN_LINK_CH_SPEC> {
        INLINK_START_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W<IN_LINK_CH_SPEC> {
        INLINK_RESTART_W::new(self, 23)
    }
}
#[doc = "DMA_IN_LINK_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK_CH_SPEC;
impl crate::RegisterSpec for IN_LINK_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link_ch::R`](R) reader structure"]
impl crate::Readable for IN_LINK_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link_ch::W`](W) writer structure"]
impl crate::Writable for IN_LINK_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_LINK_CH%s to value 0x0110_0000"]
impl crate::Resettable for IN_LINK_CH_SPEC {
    const RESET_VALUE: u32 = 0x0110_0000;
}
