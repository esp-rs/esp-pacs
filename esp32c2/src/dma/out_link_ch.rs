#[doc = "Register `OUT_LINK_CH%s` reader"]
pub type R = crate::R<OUT_LINK_CH_SPEC>;
#[doc = "Register `OUT_LINK_CH%s` writer"]
pub type W = crate::W<OUT_LINK_CH_SPEC>;
#[doc = "Field `OUTLINK_ADDR` reader - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `OUTLINK_STOP` reader - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` reader - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` reader - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_CH")
            .field(
                "outlink_addr",
                &format_args!("{}", self.outlink_addr().bits()),
            )
            .field(
                "outlink_stop",
                &format_args!("{}", self.outlink_stop().bit()),
            )
            .field(
                "outlink_start",
                &format_args!("{}", self.outlink_start().bit()),
            )
            .field(
                "outlink_restart",
                &format_args!("{}", self.outlink_restart().bit()),
            )
            .field(
                "outlink_park",
                &format_args!("{}", self.outlink_park().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_LINK_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<OUT_LINK_CH_SPEC> {
        OUTLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<OUT_LINK_CH_SPEC> {
        OUTLINK_STOP_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<OUT_LINK_CH_SPEC> {
        OUTLINK_START_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<OUT_LINK_CH_SPEC> {
        OUTLINK_RESTART_W::new(self, 22)
    }
}
#[doc = "DMA_OUT_LINK_CH%s_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets OUT_LINK_CH%s to value 0x0080_0000"]
impl crate::Resettable for OUT_LINK_CH_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
