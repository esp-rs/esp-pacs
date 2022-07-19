#[doc = "Register `RTC_LOW_POWER_ST` reader"]
pub struct R(crate::R<RTC_LOW_POWER_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_LOW_POWER_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_LOW_POWER_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_LOW_POWER_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_LOW_POWER_ST` writer"]
pub struct W(crate::W<RTC_LOW_POWER_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_LOW_POWER_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_LOW_POWER_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_LOW_POWER_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPD_DIG` reader - digital wrap power down"]
pub type XPD_DIG_R = crate::BitReader<bool>;
#[doc = "Field `XPD_DIG` writer - digital wrap power down"]
pub type XPD_DIG_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 8>;
#[doc = "Field `RTC_TOUCH_STATE_START` reader - touch should start to work"]
pub type RTC_TOUCH_STATE_START_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_START` writer - touch should start to work"]
pub type RTC_TOUCH_STATE_START_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 9>;
#[doc = "Field `RTC_TOUCH_STATE_SWITCH` reader - touch is about to working. Switch rtc main state"]
pub type RTC_TOUCH_STATE_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_SWITCH` writer - touch is about to working. Switch rtc main state"]
pub type RTC_TOUCH_STATE_SWITCH_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 10>;
#[doc = "Field `RTC_TOUCH_STATE_SLP` reader - touch is in sleep state"]
pub type RTC_TOUCH_STATE_SLP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_SLP` writer - touch is in sleep state"]
pub type RTC_TOUCH_STATE_SLP_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 11>;
#[doc = "Field `RTC_TOUCH_STATE_DONE` reader - touch is done"]
pub type RTC_TOUCH_STATE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_DONE` writer - touch is done"]
pub type RTC_TOUCH_STATE_DONE_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 12>;
#[doc = "Field `RTC_COCPU_STATE_START` reader - ulp/cocpu should start to work"]
pub type RTC_COCPU_STATE_START_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_START` writer - ulp/cocpu should start to work"]
pub type RTC_COCPU_STATE_START_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 13>;
#[doc = "Field `RTC_COCPU_STATE_SWITCH` reader - ulp/cocpu is about to working. Switch rtc main state"]
pub type RTC_COCPU_STATE_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_SWITCH` writer - ulp/cocpu is about to working. Switch rtc main state"]
pub type RTC_COCPU_STATE_SWITCH_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 14>;
#[doc = "Field `RTC_COCPU_STATE_SLP` reader - ulp/cocpu is in sleep state"]
pub type RTC_COCPU_STATE_SLP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_SLP` writer - ulp/cocpu is in sleep state"]
pub type RTC_COCPU_STATE_SLP_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 15>;
#[doc = "Field `RTC_COCPU_STATE_DONE` reader - ulp/cocpu is done"]
pub type RTC_COCPU_STATE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_DONE` writer - ulp/cocpu is done"]
pub type RTC_COCPU_STATE_DONE_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 16>;
#[doc = "Field `RTC_MAIN_STATE_XTAL_ISO` reader - no use any more"]
pub type RTC_MAIN_STATE_XTAL_ISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_XTAL_ISO` writer - no use any more"]
pub type RTC_MAIN_STATE_XTAL_ISO_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 17>;
#[doc = "Field `RTC_MAIN_STATE_PLL_ON` reader - rtc main state machine is in states that pll should be running"]
pub type RTC_MAIN_STATE_PLL_ON_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_PLL_ON` writer - rtc main state machine is in states that pll should be running"]
pub type RTC_MAIN_STATE_PLL_ON_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 18>;
#[doc = "Field `RTC_RDY_FOR_WAKEUP` reader - rtc is ready to receive wake up trigger from wake up source"]
pub type RTC_RDY_FOR_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_RDY_FOR_WAKEUP` writer - rtc is ready to receive wake up trigger from wake up source"]
pub type RTC_RDY_FOR_WAKEUP_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 19>;
#[doc = "Field `RTC_MAIN_STATE_WAIT_END` reader - rtc main state machine has been waited for some cycles"]
pub type RTC_MAIN_STATE_WAIT_END_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_WAIT_END` writer - rtc main state machine has been waited for some cycles"]
pub type RTC_MAIN_STATE_WAIT_END_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 20>;
#[doc = "Field `RTC_IN_WAKEUP_STATE` reader - rtc main state machine is in the states of wakeup process"]
pub type RTC_IN_WAKEUP_STATE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_IN_WAKEUP_STATE` writer - rtc main state machine is in the states of wakeup process"]
pub type RTC_IN_WAKEUP_STATE_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 21>;
#[doc = "Field `RTC_IN_LOW_POWER_STATE` reader - rtc main state machine is in the states of low power"]
pub type RTC_IN_LOW_POWER_STATE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_IN_LOW_POWER_STATE` writer - rtc main state machine is in the states of low power"]
pub type RTC_IN_LOW_POWER_STATE_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 22>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_8M` reader - rtc main state machine is in wait 8m state"]
pub type RTC_MAIN_STATE_IN_WAIT_8M_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_8M` writer - rtc main state machine is in wait 8m state"]
pub type RTC_MAIN_STATE_IN_WAIT_8M_W<'a> =
    crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 23>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_PLL` reader - rtc main state machine is in wait pll state"]
pub type RTC_MAIN_STATE_IN_WAIT_PLL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_PLL` writer - rtc main state machine is in wait pll state"]
pub type RTC_MAIN_STATE_IN_WAIT_PLL_W<'a> =
    crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 24>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_XTL` reader - rtc main state machine is in wait xtal state"]
pub type RTC_MAIN_STATE_IN_WAIT_XTL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_XTL` writer - rtc main state machine is in wait xtal state"]
pub type RTC_MAIN_STATE_IN_WAIT_XTL_W<'a> =
    crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 25>;
#[doc = "Field `RTC_MAIN_STATE_IN_SLP` reader - rtc main state machine is in sleep state"]
pub type RTC_MAIN_STATE_IN_SLP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_SLP` writer - rtc main state machine is in sleep state"]
pub type RTC_MAIN_STATE_IN_SLP_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 26>;
#[doc = "Field `RTC_MAIN_STATE_IN_IDLE` reader - rtc main state machine is in idle state"]
pub type RTC_MAIN_STATE_IN_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_IDLE` writer - rtc main state machine is in idle state"]
pub type RTC_MAIN_STATE_IN_IDLE_W<'a> = crate::BitWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, bool, 27>;
#[doc = "Field `RTC_MAIN_STATE` reader - rtc main state machine status"]
pub type RTC_MAIN_STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_MAIN_STATE` writer - rtc main state machine status"]
pub type RTC_MAIN_STATE_W<'a> = crate::FieldWriter<'a, u32, RTC_LOW_POWER_ST_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bit 8 - digital wrap power down"]
    #[inline(always)]
    pub fn xpd_dig(&self) -> XPD_DIG_R {
        XPD_DIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - touch should start to work"]
    #[inline(always)]
    pub fn rtc_touch_state_start(&self) -> RTC_TOUCH_STATE_START_R {
        RTC_TOUCH_STATE_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - touch is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn rtc_touch_state_switch(&self) -> RTC_TOUCH_STATE_SWITCH_R {
        RTC_TOUCH_STATE_SWITCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - touch is in sleep state"]
    #[inline(always)]
    pub fn rtc_touch_state_slp(&self) -> RTC_TOUCH_STATE_SLP_R {
        RTC_TOUCH_STATE_SLP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - touch is done"]
    #[inline(always)]
    pub fn rtc_touch_state_done(&self) -> RTC_TOUCH_STATE_DONE_R {
        RTC_TOUCH_STATE_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ulp/cocpu should start to work"]
    #[inline(always)]
    pub fn rtc_cocpu_state_start(&self) -> RTC_COCPU_STATE_START_R {
        RTC_COCPU_STATE_START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ulp/cocpu is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn rtc_cocpu_state_switch(&self) -> RTC_COCPU_STATE_SWITCH_R {
        RTC_COCPU_STATE_SWITCH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ulp/cocpu is in sleep state"]
    #[inline(always)]
    pub fn rtc_cocpu_state_slp(&self) -> RTC_COCPU_STATE_SLP_R {
        RTC_COCPU_STATE_SLP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ulp/cocpu is done"]
    #[inline(always)]
    pub fn rtc_cocpu_state_done(&self) -> RTC_COCPU_STATE_DONE_R {
        RTC_COCPU_STATE_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no use any more"]
    #[inline(always)]
    pub fn rtc_main_state_xtal_iso(&self) -> RTC_MAIN_STATE_XTAL_ISO_R {
        RTC_MAIN_STATE_XTAL_ISO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - rtc main state machine is in states that pll should be running"]
    #[inline(always)]
    pub fn rtc_main_state_pll_on(&self) -> RTC_MAIN_STATE_PLL_ON_R {
        RTC_MAIN_STATE_PLL_ON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rtc is ready to receive wake up trigger from wake up source"]
    #[inline(always)]
    pub fn rtc_rdy_for_wakeup(&self) -> RTC_RDY_FOR_WAKEUP_R {
        RTC_RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - rtc main state machine has been waited for some cycles"]
    #[inline(always)]
    pub fn rtc_main_state_wait_end(&self) -> RTC_MAIN_STATE_WAIT_END_R {
        RTC_MAIN_STATE_WAIT_END_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - rtc main state machine is in the states of wakeup process"]
    #[inline(always)]
    pub fn rtc_in_wakeup_state(&self) -> RTC_IN_WAKEUP_STATE_R {
        RTC_IN_WAKEUP_STATE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - rtc main state machine is in the states of low power"]
    #[inline(always)]
    pub fn rtc_in_low_power_state(&self) -> RTC_IN_LOW_POWER_STATE_R {
        RTC_IN_LOW_POWER_STATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - rtc main state machine is in wait 8m state"]
    #[inline(always)]
    pub fn rtc_main_state_in_wait_8m(&self) -> RTC_MAIN_STATE_IN_WAIT_8M_R {
        RTC_MAIN_STATE_IN_WAIT_8M_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - rtc main state machine is in wait pll state"]
    #[inline(always)]
    pub fn rtc_main_state_in_wait_pll(&self) -> RTC_MAIN_STATE_IN_WAIT_PLL_R {
        RTC_MAIN_STATE_IN_WAIT_PLL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - rtc main state machine is in wait xtal state"]
    #[inline(always)]
    pub fn rtc_main_state_in_wait_xtl(&self) -> RTC_MAIN_STATE_IN_WAIT_XTL_R {
        RTC_MAIN_STATE_IN_WAIT_XTL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - rtc main state machine is in sleep state"]
    #[inline(always)]
    pub fn rtc_main_state_in_slp(&self) -> RTC_MAIN_STATE_IN_SLP_R {
        RTC_MAIN_STATE_IN_SLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - rtc main state machine is in idle state"]
    #[inline(always)]
    pub fn rtc_main_state_in_idle(&self) -> RTC_MAIN_STATE_IN_IDLE_R {
        RTC_MAIN_STATE_IN_IDLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - rtc main state machine status"]
    #[inline(always)]
    pub fn rtc_main_state(&self) -> RTC_MAIN_STATE_R {
        RTC_MAIN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - digital wrap power down"]
    #[inline(always)]
    pub fn xpd_dig(&mut self) -> XPD_DIG_W {
        XPD_DIG_W::new(self)
    }
    #[doc = "Bit 9 - touch should start to work"]
    #[inline(always)]
    pub fn rtc_touch_state_start(&mut self) -> RTC_TOUCH_STATE_START_W {
        RTC_TOUCH_STATE_START_W::new(self)
    }
    #[doc = "Bit 10 - touch is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn rtc_touch_state_switch(&mut self) -> RTC_TOUCH_STATE_SWITCH_W {
        RTC_TOUCH_STATE_SWITCH_W::new(self)
    }
    #[doc = "Bit 11 - touch is in sleep state"]
    #[inline(always)]
    pub fn rtc_touch_state_slp(&mut self) -> RTC_TOUCH_STATE_SLP_W {
        RTC_TOUCH_STATE_SLP_W::new(self)
    }
    #[doc = "Bit 12 - touch is done"]
    #[inline(always)]
    pub fn rtc_touch_state_done(&mut self) -> RTC_TOUCH_STATE_DONE_W {
        RTC_TOUCH_STATE_DONE_W::new(self)
    }
    #[doc = "Bit 13 - ulp/cocpu should start to work"]
    #[inline(always)]
    pub fn rtc_cocpu_state_start(&mut self) -> RTC_COCPU_STATE_START_W {
        RTC_COCPU_STATE_START_W::new(self)
    }
    #[doc = "Bit 14 - ulp/cocpu is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn rtc_cocpu_state_switch(&mut self) -> RTC_COCPU_STATE_SWITCH_W {
        RTC_COCPU_STATE_SWITCH_W::new(self)
    }
    #[doc = "Bit 15 - ulp/cocpu is in sleep state"]
    #[inline(always)]
    pub fn rtc_cocpu_state_slp(&mut self) -> RTC_COCPU_STATE_SLP_W {
        RTC_COCPU_STATE_SLP_W::new(self)
    }
    #[doc = "Bit 16 - ulp/cocpu is done"]
    #[inline(always)]
    pub fn rtc_cocpu_state_done(&mut self) -> RTC_COCPU_STATE_DONE_W {
        RTC_COCPU_STATE_DONE_W::new(self)
    }
    #[doc = "Bit 17 - no use any more"]
    #[inline(always)]
    pub fn rtc_main_state_xtal_iso(&mut self) -> RTC_MAIN_STATE_XTAL_ISO_W {
        RTC_MAIN_STATE_XTAL_ISO_W::new(self)
    }
    #[doc = "Bit 18 - rtc main state machine is in states that pll should be running"]
    #[inline(always)]
    pub fn rtc_main_state_pll_on(&mut self) -> RTC_MAIN_STATE_PLL_ON_W {
        RTC_MAIN_STATE_PLL_ON_W::new(self)
    }
    #[doc = "Bit 19 - rtc is ready to receive wake up trigger from wake up source"]
    #[inline(always)]
    pub fn rtc_rdy_for_wakeup(&mut self) -> RTC_RDY_FOR_WAKEUP_W {
        RTC_RDY_FOR_WAKEUP_W::new(self)
    }
    #[doc = "Bit 20 - rtc main state machine has been waited for some cycles"]
    #[inline(always)]
    pub fn rtc_main_state_wait_end(&mut self) -> RTC_MAIN_STATE_WAIT_END_W {
        RTC_MAIN_STATE_WAIT_END_W::new(self)
    }
    #[doc = "Bit 21 - rtc main state machine is in the states of wakeup process"]
    #[inline(always)]
    pub fn rtc_in_wakeup_state(&mut self) -> RTC_IN_WAKEUP_STATE_W {
        RTC_IN_WAKEUP_STATE_W::new(self)
    }
    #[doc = "Bit 22 - rtc main state machine is in the states of low power"]
    #[inline(always)]
    pub fn rtc_in_low_power_state(&mut self) -> RTC_IN_LOW_POWER_STATE_W {
        RTC_IN_LOW_POWER_STATE_W::new(self)
    }
    #[doc = "Bit 23 - rtc main state machine is in wait 8m state"]
    #[inline(always)]
    pub fn rtc_main_state_in_wait_8m(&mut self) -> RTC_MAIN_STATE_IN_WAIT_8M_W {
        RTC_MAIN_STATE_IN_WAIT_8M_W::new(self)
    }
    #[doc = "Bit 24 - rtc main state machine is in wait pll state"]
    #[inline(always)]
    pub fn rtc_main_state_in_wait_pll(&mut self) -> RTC_MAIN_STATE_IN_WAIT_PLL_W {
        RTC_MAIN_STATE_IN_WAIT_PLL_W::new(self)
    }
    #[doc = "Bit 25 - rtc main state machine is in wait xtal state"]
    #[inline(always)]
    pub fn rtc_main_state_in_wait_xtl(&mut self) -> RTC_MAIN_STATE_IN_WAIT_XTL_W {
        RTC_MAIN_STATE_IN_WAIT_XTL_W::new(self)
    }
    #[doc = "Bit 26 - rtc main state machine is in sleep state"]
    #[inline(always)]
    pub fn rtc_main_state_in_slp(&mut self) -> RTC_MAIN_STATE_IN_SLP_W {
        RTC_MAIN_STATE_IN_SLP_W::new(self)
    }
    #[doc = "Bit 27 - rtc main state machine is in idle state"]
    #[inline(always)]
    pub fn rtc_main_state_in_idle(&mut self) -> RTC_MAIN_STATE_IN_IDLE_W {
        RTC_MAIN_STATE_IN_IDLE_W::new(self)
    }
    #[doc = "Bits 28:31 - rtc main state machine status"]
    #[inline(always)]
    pub fn rtc_main_state(&mut self) -> RTC_MAIN_STATE_W {
        RTC_MAIN_STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_low_power_st](index.html) module"]
pub struct RTC_LOW_POWER_ST_SPEC;
impl crate::RegisterSpec for RTC_LOW_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_low_power_st::R](R) reader structure"]
impl crate::Readable for RTC_LOW_POWER_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_low_power_st::W](W) writer structure"]
impl crate::Writable for RTC_LOW_POWER_ST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_LOW_POWER_ST to value 0"]
impl crate::Resettable for RTC_LOW_POWER_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
