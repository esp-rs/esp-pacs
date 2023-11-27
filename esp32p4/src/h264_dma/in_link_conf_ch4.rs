#[doc = "Register `IN_LINK_CONF_CH4` reader"]
pub type R = crate::R<IN_LINK_CONF_CH4_SPEC>;
#[doc = "Register `IN_LINK_CONF_CH4` writer"]
pub type W = crate::W<IN_LINK_CONF_CH4_SPEC>;
#[doc = "Field `INLINK_AUTO_RET_CH4` reader - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH4_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET_CH4` writer - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
pub type INLINK_AUTO_RET_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_STOP_CH4` reader - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_CH4_R = crate::BitReader;
#[doc = "Field `INLINK_STOP_CH4` writer - Set this bit to stop dealing with the inlink descriptors."]
pub type INLINK_STOP_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START_CH4` reader - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_CH4_R = crate::BitReader;
#[doc = "Field `INLINK_START_CH4` writer - Set this bit to start dealing with the inlink descriptors."]
pub type INLINK_START_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART_CH4` reader - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_CH4_R = crate::BitReader;
#[doc = "Field `INLINK_RESTART_CH4` writer - Set this bit to mount a new inlink descriptor."]
pub type INLINK_RESTART_CH4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_PARK_CH4` reader - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
pub type INLINK_PARK_CH4_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    pub fn inlink_auto_ret_ch4(&self) -> INLINK_AUTO_RET_CH4_R {
        INLINK_AUTO_RET_CH4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_stop_ch4(&self) -> INLINK_STOP_CH4_R {
        INLINK_STOP_CH4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    pub fn inlink_start_ch4(&self) -> INLINK_START_CH4_R {
        INLINK_START_CH4_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    pub fn inlink_restart_ch4(&self) -> INLINK_RESTART_CH4_R {
        INLINK_RESTART_CH4_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: the inlink descriptor's FSM is in idle state. 0: the inlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn inlink_park_ch4(&self) -> INLINK_PARK_CH4_R {
        INLINK_PARK_CH4_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK_CONF_CH4")
            .field(
                "inlink_auto_ret_ch4",
                &format_args!("{}", self.inlink_auto_ret_ch4().bit()),
            )
            .field(
                "inlink_stop_ch4",
                &format_args!("{}", self.inlink_stop_ch4().bit()),
            )
            .field(
                "inlink_start_ch4",
                &format_args!("{}", self.inlink_start_ch4().bit()),
            )
            .field(
                "inlink_restart_ch4",
                &format_args!("{}", self.inlink_restart_ch4().bit()),
            )
            .field(
                "inlink_park_ch4",
                &format_args!("{}", self.inlink_park_ch4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_LINK_CONF_CH4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20 - Set this bit to return to current inlink descriptor's address, when there are some errors in current receiving data."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_auto_ret_ch4(&mut self) -> INLINK_AUTO_RET_CH4_W<IN_LINK_CONF_CH4_SPEC> {
        INLINK_AUTO_RET_CH4_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to stop dealing with the inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_stop_ch4(&mut self) -> INLINK_STOP_CH4_W<IN_LINK_CONF_CH4_SPEC> {
        INLINK_STOP_CH4_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to start dealing with the inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_start_ch4(&mut self) -> INLINK_START_CH4_W<IN_LINK_CONF_CH4_SPEC> {
        INLINK_START_CH4_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to mount a new inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_restart_ch4(&mut self) -> INLINK_RESTART_CH4_W<IN_LINK_CONF_CH4_SPEC> {
        INLINK_RESTART_CH4_W::new(self, 23)
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
#[doc = "RX CH4 in_link dscr ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_link_conf_ch4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link_conf_ch4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_LINK_CONF_CH4_SPEC;
impl crate::RegisterSpec for IN_LINK_CONF_CH4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_link_conf_ch4::R`](R) reader structure"]
impl crate::Readable for IN_LINK_CONF_CH4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_link_conf_ch4::W`](W) writer structure"]
impl crate::Writable for IN_LINK_CONF_CH4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_LINK_CONF_CH4 to value 0x0110_0000"]
impl crate::Resettable for IN_LINK_CONF_CH4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0110_0000;
}
