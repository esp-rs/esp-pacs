#[doc = "Register `OUT_LINK_CONF_CH4` reader"]
pub type R = crate::R<OUT_LINK_CONF_CH4_SPEC>;
#[doc = "Register `OUT_LINK_CONF_CH4` writer"]
pub type W = crate::W<OUT_LINK_CONF_CH4_SPEC>;
#[doc = "Field `OUTLINK_STOP_CH4` reader - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_CH4_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP_CH4` writer - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START_CH4` reader - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_CH4_R = crate::BitReader;
#[doc = "Field `OUTLINK_START_CH4` writer - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART_CH4` reader - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_CH4_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART_CH4` writer - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK_CH4` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub type OUTLINK_PARK_CH4_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop_ch4(&self) -> OUTLINK_STOP_CH4_R {
        OUTLINK_STOP_CH4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start_ch4(&self) -> OUTLINK_START_CH4_R {
        OUTLINK_START_CH4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart_ch4(&self) -> OUTLINK_RESTART_CH4_R {
        OUTLINK_RESTART_CH4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park_ch4(&self) -> OUTLINK_PARK_CH4_R {
        OUTLINK_PARK_CH4_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_CONF_CH4")
            .field(
                "outlink_stop_ch4",
                &format_args!("{}", self.outlink_stop_ch4().bit()),
            )
            .field(
                "outlink_start_ch4",
                &format_args!("{}", self.outlink_start_ch4().bit()),
            )
            .field(
                "outlink_restart_ch4",
                &format_args!("{}", self.outlink_restart_ch4().bit()),
            )
            .field(
                "outlink_park_ch4",
                &format_args!("{}", self.outlink_park_ch4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_LINK_CONF_CH4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_stop_ch4(&mut self) -> OUTLINK_STOP_CH4_W<OUT_LINK_CONF_CH4_SPEC> {
        OUTLINK_STOP_CH4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_start_ch4(&mut self) -> OUTLINK_START_CH4_W<OUT_LINK_CONF_CH4_SPEC> {
        OUTLINK_START_CH4_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    #[must_use]
    pub fn outlink_restart_ch4(&mut self) -> OUTLINK_RESTART_CH4_W<OUT_LINK_CONF_CH4_SPEC> {
        OUTLINK_RESTART_CH4_W::new(self, 22)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "TX CH4 out_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link_conf_ch4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link_conf_ch4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_CONF_CH4_SPEC;
impl crate::RegisterSpec for OUT_LINK_CONF_CH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link_conf_ch4::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_CONF_CH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link_conf_ch4::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_CONF_CH4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_LINK_CONF_CH4 to value 0x0080_0000"]
impl crate::Resettable for OUT_LINK_CONF_CH4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
