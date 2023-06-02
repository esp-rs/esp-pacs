#[doc = "Register `LOW_POWER_ST` reader"]
pub struct R(crate::R<LOW_POWER_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOW_POWER_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOW_POWER_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOW_POWER_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOW_POWER_ST` writer"]
pub struct W(crate::W<LOW_POWER_ST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOW_POWER_ST_SPEC>;
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
impl From<crate::W<LOW_POWER_ST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOW_POWER_ST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPD_DIG` reader - digital wrap power down"]
pub type XPD_DIG_R = crate::BitReader;
#[doc = "Field `XPD_DIG` writer - digital wrap power down"]
pub type XPD_DIG_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `TOUCH_STATE_START` reader - touch should start to work"]
pub type TOUCH_STATE_START_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_START` writer - touch should start to work"]
pub type TOUCH_STATE_START_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `TOUCH_STATE_SWITCH` reader - touch is about to working. Switch rtc main state"]
pub type TOUCH_STATE_SWITCH_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_SWITCH` writer - touch is about to working. Switch rtc main state"]
pub type TOUCH_STATE_SWITCH_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `TOUCH_STATE_SLP` reader - touch is in sleep state"]
pub type TOUCH_STATE_SLP_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_SLP` writer - touch is in sleep state"]
pub type TOUCH_STATE_SLP_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `TOUCH_STATE_DONE` reader - touch is done"]
pub type TOUCH_STATE_DONE_R = crate::BitReader;
#[doc = "Field `TOUCH_STATE_DONE` writer - touch is done"]
pub type TOUCH_STATE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `COCPU_STATE_START` reader - ulp/cocpu should start to work"]
pub type COCPU_STATE_START_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_START` writer - ulp/cocpu should start to work"]
pub type COCPU_STATE_START_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `COCPU_STATE_SWITCH` reader - ulp/cocpu is about to working. Switch rtc main state"]
pub type COCPU_STATE_SWITCH_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_SWITCH` writer - ulp/cocpu is about to working. Switch rtc main state"]
pub type COCPU_STATE_SWITCH_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `COCPU_STATE_SLP` reader - ulp/cocpu is in sleep state"]
pub type COCPU_STATE_SLP_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_SLP` writer - ulp/cocpu is in sleep state"]
pub type COCPU_STATE_SLP_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `COCPU_STATE_DONE` reader - ulp/cocpu is done"]
pub type COCPU_STATE_DONE_R = crate::BitReader;
#[doc = "Field `COCPU_STATE_DONE` writer - ulp/cocpu is done"]
pub type COCPU_STATE_DONE_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_XTAL_ISO` reader - no use any more"]
pub type MAIN_STATE_XTAL_ISO_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_XTAL_ISO` writer - no use any more"]
pub type MAIN_STATE_XTAL_ISO_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_PLL_ON` reader - rtc main state machine is in states that pll should be running"]
pub type MAIN_STATE_PLL_ON_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_PLL_ON` writer - rtc main state machine is in states that pll should be running"]
pub type MAIN_STATE_PLL_ON_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `RDY_FOR_WAKEUP` reader - rtc is ready to receive wake up trigger from wake up source"]
pub type RDY_FOR_WAKEUP_R = crate::BitReader;
#[doc = "Field `RDY_FOR_WAKEUP` writer - rtc is ready to receive wake up trigger from wake up source"]
pub type RDY_FOR_WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_WAIT_END` reader - rtc main state machine has been waited for some cycles"]
pub type MAIN_STATE_WAIT_END_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_WAIT_END` writer - rtc main state machine has been waited for some cycles"]
pub type MAIN_STATE_WAIT_END_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `IN_WAKEUP_STATE` reader - rtc main state machine is in the states of wakeup process"]
pub type IN_WAKEUP_STATE_R = crate::BitReader;
#[doc = "Field `IN_WAKEUP_STATE` writer - rtc main state machine is in the states of wakeup process"]
pub type IN_WAKEUP_STATE_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `IN_LOW_POWER_STATE` reader - rtc main state machine is in the states of low power"]
pub type IN_LOW_POWER_STATE_R = crate::BitReader;
#[doc = "Field `IN_LOW_POWER_STATE` writer - rtc main state machine is in the states of low power"]
pub type IN_LOW_POWER_STATE_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_IN_WAIT_8M` reader - rtc main state machine is in wait 8m state"]
pub type MAIN_STATE_IN_WAIT_8M_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_WAIT_8M` writer - rtc main state machine is in wait 8m state"]
pub type MAIN_STATE_IN_WAIT_8M_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_IN_WAIT_PLL` reader - rtc main state machine is in wait pll state"]
pub type MAIN_STATE_IN_WAIT_PLL_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_WAIT_PLL` writer - rtc main state machine is in wait pll state"]
pub type MAIN_STATE_IN_WAIT_PLL_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_IN_WAIT_XTL` reader - rtc main state machine is in wait xtal state"]
pub type MAIN_STATE_IN_WAIT_XTL_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_WAIT_XTL` writer - rtc main state machine is in wait xtal state"]
pub type MAIN_STATE_IN_WAIT_XTL_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_IN_SLP` reader - rtc main state machine is in sleep state"]
pub type MAIN_STATE_IN_SLP_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_SLP` writer - rtc main state machine is in sleep state"]
pub type MAIN_STATE_IN_SLP_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE_IN_IDLE` reader - rtc main state machine is in idle state"]
pub type MAIN_STATE_IN_IDLE_R = crate::BitReader;
#[doc = "Field `MAIN_STATE_IN_IDLE` writer - rtc main state machine is in idle state"]
pub type MAIN_STATE_IN_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, LOW_POWER_ST_SPEC, O>;
#[doc = "Field `MAIN_STATE` reader - rtc main state machine status"]
pub type MAIN_STATE_R = crate::FieldReader;
#[doc = "Field `MAIN_STATE` writer - rtc main state machine status"]
pub type MAIN_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, LOW_POWER_ST_SPEC, 4, O>;
impl R {
    #[doc = "Bit 8 - digital wrap power down"]
    #[inline(always)]
    pub fn xpd_dig(&self) -> XPD_DIG_R {
        XPD_DIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - touch should start to work"]
    #[inline(always)]
    pub fn touch_state_start(&self) -> TOUCH_STATE_START_R {
        TOUCH_STATE_START_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - touch is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn touch_state_switch(&self) -> TOUCH_STATE_SWITCH_R {
        TOUCH_STATE_SWITCH_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - touch is in sleep state"]
    #[inline(always)]
    pub fn touch_state_slp(&self) -> TOUCH_STATE_SLP_R {
        TOUCH_STATE_SLP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - touch is done"]
    #[inline(always)]
    pub fn touch_state_done(&self) -> TOUCH_STATE_DONE_R {
        TOUCH_STATE_DONE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ulp/cocpu should start to work"]
    #[inline(always)]
    pub fn cocpu_state_start(&self) -> COCPU_STATE_START_R {
        COCPU_STATE_START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ulp/cocpu is about to working. Switch rtc main state"]
    #[inline(always)]
    pub fn cocpu_state_switch(&self) -> COCPU_STATE_SWITCH_R {
        COCPU_STATE_SWITCH_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ulp/cocpu is in sleep state"]
    #[inline(always)]
    pub fn cocpu_state_slp(&self) -> COCPU_STATE_SLP_R {
        COCPU_STATE_SLP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ulp/cocpu is done"]
    #[inline(always)]
    pub fn cocpu_state_done(&self) -> COCPU_STATE_DONE_R {
        COCPU_STATE_DONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - no use any more"]
    #[inline(always)]
    pub fn main_state_xtal_iso(&self) -> MAIN_STATE_XTAL_ISO_R {
        MAIN_STATE_XTAL_ISO_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - rtc main state machine is in states that pll should be running"]
    #[inline(always)]
    pub fn main_state_pll_on(&self) -> MAIN_STATE_PLL_ON_R {
        MAIN_STATE_PLL_ON_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rtc is ready to receive wake up trigger from wake up source"]
    #[inline(always)]
    pub fn rdy_for_wakeup(&self) -> RDY_FOR_WAKEUP_R {
        RDY_FOR_WAKEUP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - rtc main state machine has been waited for some cycles"]
    #[inline(always)]
    pub fn main_state_wait_end(&self) -> MAIN_STATE_WAIT_END_R {
        MAIN_STATE_WAIT_END_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - rtc main state machine is in the states of wakeup process"]
    #[inline(always)]
    pub fn in_wakeup_state(&self) -> IN_WAKEUP_STATE_R {
        IN_WAKEUP_STATE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - rtc main state machine is in the states of low power"]
    #[inline(always)]
    pub fn in_low_power_state(&self) -> IN_LOW_POWER_STATE_R {
        IN_LOW_POWER_STATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - rtc main state machine is in wait 8m state"]
    #[inline(always)]
    pub fn main_state_in_wait_8m(&self) -> MAIN_STATE_IN_WAIT_8M_R {
        MAIN_STATE_IN_WAIT_8M_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - rtc main state machine is in wait pll state"]
    #[inline(always)]
    pub fn main_state_in_wait_pll(&self) -> MAIN_STATE_IN_WAIT_PLL_R {
        MAIN_STATE_IN_WAIT_PLL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - rtc main state machine is in wait xtal state"]
    #[inline(always)]
    pub fn main_state_in_wait_xtl(&self) -> MAIN_STATE_IN_WAIT_XTL_R {
        MAIN_STATE_IN_WAIT_XTL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - rtc main state machine is in sleep state"]
    #[inline(always)]
    pub fn main_state_in_slp(&self) -> MAIN_STATE_IN_SLP_R {
        MAIN_STATE_IN_SLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - rtc main state machine is in idle state"]
    #[inline(always)]
    pub fn main_state_in_idle(&self) -> MAIN_STATE_IN_IDLE_R {
        MAIN_STATE_IN_IDLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - rtc main state machine status"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOW_POWER_ST")
            .field("xpd_dig", &format_args!("{}", self.xpd_dig().bit()))
            .field(
                "touch_state_start",
                &format_args!("{}", self.touch_state_start().bit()),
            )
            .field(
                "touch_state_switch",
                &format_args!("{}", self.touch_state_switch().bit()),
            )
            .field(
                "touch_state_slp",
                &format_args!("{}", self.touch_state_slp().bit()),
            )
            .field(
                "touch_state_done",
                &format_args!("{}", self.touch_state_done().bit()),
            )
            .field(
                "cocpu_state_start",
                &format_args!("{}", self.cocpu_state_start().bit()),
            )
            .field(
                "cocpu_state_switch",
                &format_args!("{}", self.cocpu_state_switch().bit()),
            )
            .field(
                "cocpu_state_slp",
                &format_args!("{}", self.cocpu_state_slp().bit()),
            )
            .field(
                "cocpu_state_done",
                &format_args!("{}", self.cocpu_state_done().bit()),
            )
            .field(
                "main_state_xtal_iso",
                &format_args!("{}", self.main_state_xtal_iso().bit()),
            )
            .field(
                "main_state_pll_on",
                &format_args!("{}", self.main_state_pll_on().bit()),
            )
            .field(
                "rdy_for_wakeup",
                &format_args!("{}", self.rdy_for_wakeup().bit()),
            )
            .field(
                "main_state_wait_end",
                &format_args!("{}", self.main_state_wait_end().bit()),
            )
            .field(
                "in_wakeup_state",
                &format_args!("{}", self.in_wakeup_state().bit()),
            )
            .field(
                "in_low_power_state",
                &format_args!("{}", self.in_low_power_state().bit()),
            )
            .field(
                "main_state_in_wait_8m",
                &format_args!("{}", self.main_state_in_wait_8m().bit()),
            )
            .field(
                "main_state_in_wait_pll",
                &format_args!("{}", self.main_state_in_wait_pll().bit()),
            )
            .field(
                "main_state_in_wait_xtl",
                &format_args!("{}", self.main_state_in_wait_xtl().bit()),
            )
            .field(
                "main_state_in_slp",
                &format_args!("{}", self.main_state_in_slp().bit()),
            )
            .field(
                "main_state_in_idle",
                &format_args!("{}", self.main_state_in_idle().bit()),
            )
            .field("main_state", &format_args!("{}", self.main_state().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOW_POWER_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8 - digital wrap power down"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_dig(&mut self) -> XPD_DIG_W<8> {
        XPD_DIG_W::new(self)
    }
    #[doc = "Bit 9 - touch should start to work"]
    #[inline(always)]
    #[must_use]
    pub fn touch_state_start(&mut self) -> TOUCH_STATE_START_W<9> {
        TOUCH_STATE_START_W::new(self)
    }
    #[doc = "Bit 10 - touch is about to working. Switch rtc main state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_state_switch(&mut self) -> TOUCH_STATE_SWITCH_W<10> {
        TOUCH_STATE_SWITCH_W::new(self)
    }
    #[doc = "Bit 11 - touch is in sleep state"]
    #[inline(always)]
    #[must_use]
    pub fn touch_state_slp(&mut self) -> TOUCH_STATE_SLP_W<11> {
        TOUCH_STATE_SLP_W::new(self)
    }
    #[doc = "Bit 12 - touch is done"]
    #[inline(always)]
    #[must_use]
    pub fn touch_state_done(&mut self) -> TOUCH_STATE_DONE_W<12> {
        TOUCH_STATE_DONE_W::new(self)
    }
    #[doc = "Bit 13 - ulp/cocpu should start to work"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_state_start(&mut self) -> COCPU_STATE_START_W<13> {
        COCPU_STATE_START_W::new(self)
    }
    #[doc = "Bit 14 - ulp/cocpu is about to working. Switch rtc main state"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_state_switch(&mut self) -> COCPU_STATE_SWITCH_W<14> {
        COCPU_STATE_SWITCH_W::new(self)
    }
    #[doc = "Bit 15 - ulp/cocpu is in sleep state"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_state_slp(&mut self) -> COCPU_STATE_SLP_W<15> {
        COCPU_STATE_SLP_W::new(self)
    }
    #[doc = "Bit 16 - ulp/cocpu is done"]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_state_done(&mut self) -> COCPU_STATE_DONE_W<16> {
        COCPU_STATE_DONE_W::new(self)
    }
    #[doc = "Bit 17 - no use any more"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_xtal_iso(&mut self) -> MAIN_STATE_XTAL_ISO_W<17> {
        MAIN_STATE_XTAL_ISO_W::new(self)
    }
    #[doc = "Bit 18 - rtc main state machine is in states that pll should be running"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_pll_on(&mut self) -> MAIN_STATE_PLL_ON_W<18> {
        MAIN_STATE_PLL_ON_W::new(self)
    }
    #[doc = "Bit 19 - rtc is ready to receive wake up trigger from wake up source"]
    #[inline(always)]
    #[must_use]
    pub fn rdy_for_wakeup(&mut self) -> RDY_FOR_WAKEUP_W<19> {
        RDY_FOR_WAKEUP_W::new(self)
    }
    #[doc = "Bit 20 - rtc main state machine has been waited for some cycles"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_wait_end(&mut self) -> MAIN_STATE_WAIT_END_W<20> {
        MAIN_STATE_WAIT_END_W::new(self)
    }
    #[doc = "Bit 21 - rtc main state machine is in the states of wakeup process"]
    #[inline(always)]
    #[must_use]
    pub fn in_wakeup_state(&mut self) -> IN_WAKEUP_STATE_W<21> {
        IN_WAKEUP_STATE_W::new(self)
    }
    #[doc = "Bit 22 - rtc main state machine is in the states of low power"]
    #[inline(always)]
    #[must_use]
    pub fn in_low_power_state(&mut self) -> IN_LOW_POWER_STATE_W<22> {
        IN_LOW_POWER_STATE_W::new(self)
    }
    #[doc = "Bit 23 - rtc main state machine is in wait 8m state"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_in_wait_8m(&mut self) -> MAIN_STATE_IN_WAIT_8M_W<23> {
        MAIN_STATE_IN_WAIT_8M_W::new(self)
    }
    #[doc = "Bit 24 - rtc main state machine is in wait pll state"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_in_wait_pll(&mut self) -> MAIN_STATE_IN_WAIT_PLL_W<24> {
        MAIN_STATE_IN_WAIT_PLL_W::new(self)
    }
    #[doc = "Bit 25 - rtc main state machine is in wait xtal state"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_in_wait_xtl(&mut self) -> MAIN_STATE_IN_WAIT_XTL_W<25> {
        MAIN_STATE_IN_WAIT_XTL_W::new(self)
    }
    #[doc = "Bit 26 - rtc main state machine is in sleep state"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_in_slp(&mut self) -> MAIN_STATE_IN_SLP_W<26> {
        MAIN_STATE_IN_SLP_W::new(self)
    }
    #[doc = "Bit 27 - rtc main state machine is in idle state"]
    #[inline(always)]
    #[must_use]
    pub fn main_state_in_idle(&mut self) -> MAIN_STATE_IN_IDLE_W<27> {
        MAIN_STATE_IN_IDLE_W::new(self)
    }
    #[doc = "Bits 28:31 - rtc main state machine status"]
    #[inline(always)]
    #[must_use]
    pub fn main_state(&mut self) -> MAIN_STATE_W<28> {
        MAIN_STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low_power_st](index.html) module"]
pub struct LOW_POWER_ST_SPEC;
impl crate::RegisterSpec for LOW_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [low_power_st::R](R) reader structure"]
impl crate::Readable for LOW_POWER_ST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [low_power_st::W](W) writer structure"]
impl crate::Writable for LOW_POWER_ST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOW_POWER_ST to value 0"]
impl crate::Resettable for LOW_POWER_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
