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
#[doc = "Field `XPD_ROM0` reader - rom0 power down"]
pub type XPD_ROM0_R = crate::BitReader<bool>;
#[doc = "Field `XPD_DIG_DCDC` reader - External DCDC power down"]
pub type XPD_DIG_DCDC_R = crate::BitReader<bool>;
#[doc = "Field `RTC_PERI_ISO` reader - rtc peripheral iso"]
pub type RTC_PERI_ISO_R = crate::BitReader<bool>;
#[doc = "Field `XPD_RTC_PERI` reader - rtc peripheral power down"]
pub type XPD_RTC_PERI_R = crate::BitReader<bool>;
#[doc = "Field `WIFI_ISO` reader - wifi iso"]
pub type WIFI_ISO_R = crate::BitReader<bool>;
#[doc = "Field `XPD_WIFI` reader - wifi wrap power down"]
pub type XPD_WIFI_R = crate::BitReader<bool>;
#[doc = "Field `DIG_ISO` reader - digital wrap iso"]
pub type DIG_ISO_R = crate::BitReader<bool>;
#[doc = "Field `XPD_DIG` reader - digital wrap power down"]
pub type XPD_DIG_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_START` reader - touch should start to work"]
pub type RTC_TOUCH_STATE_START_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_SWITCH` reader - touch is about to working. Switch rtc main state"]
pub type RTC_TOUCH_STATE_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_SLP` reader - touch is in sleep state"]
pub type RTC_TOUCH_STATE_SLP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_TOUCH_STATE_DONE` reader - touch is done"]
pub type RTC_TOUCH_STATE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_START` reader - ulp/cocpu should start to work"]
pub type RTC_COCPU_STATE_START_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_SWITCH` reader - ulp/cocpu is about to working. Switch rtc main state"]
pub type RTC_COCPU_STATE_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_SLP` reader - ulp/cocpu is in sleep state"]
pub type RTC_COCPU_STATE_SLP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_COCPU_STATE_DONE` reader - ulp/cocpu is done"]
pub type RTC_COCPU_STATE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_XTAL_ISO` reader - no use any more"]
pub type RTC_MAIN_STATE_XTAL_ISO_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_PLL_ON` reader - rtc main state machine is in states that pll should be running"]
pub type RTC_MAIN_STATE_PLL_ON_R = crate::BitReader<bool>;
#[doc = "Field `RTC_RDY_FOR_WAKEUP` reader - rtc is ready to receive wake up trigger from wake up source"]
pub type RTC_RDY_FOR_WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_WAIT_END` reader - rtc main state machine has been waited for some cycles"]
pub type RTC_MAIN_STATE_WAIT_END_R = crate::BitReader<bool>;
#[doc = "Field `RTC_IN_WAKEUP_STATE` reader - rtc main state machine is in the states of wakeup process"]
pub type RTC_IN_WAKEUP_STATE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_IN_LOW_POWER_STATE` reader - rtc main state machine is in the states of low power"]
pub type RTC_IN_LOW_POWER_STATE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_8M` reader - rtc main state machine is in wait 8m state"]
pub type RTC_MAIN_STATE_IN_WAIT_8M_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_PLL` reader - rtc main state machine is in wait pll state"]
pub type RTC_MAIN_STATE_IN_WAIT_PLL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_WAIT_XTL` reader - rtc main state machine is in wait xtal state"]
pub type RTC_MAIN_STATE_IN_WAIT_XTL_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_SLP` reader - rtc main state machine is in sleep state"]
pub type RTC_MAIN_STATE_IN_SLP_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE_IN_IDLE` reader - rtc main state machine is in idle state"]
pub type RTC_MAIN_STATE_IN_IDLE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_MAIN_STATE` reader - rtc main state machine status"]
pub type RTC_MAIN_STATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - rom0 power down"]
    #[inline(always)]
    pub fn xpd_rom0(&self) -> XPD_ROM0_R {
        XPD_ROM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - External DCDC power down"]
    #[inline(always)]
    pub fn xpd_dig_dcdc(&self) -> XPD_DIG_DCDC_R {
        XPD_DIG_DCDC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - rtc peripheral iso"]
    #[inline(always)]
    pub fn rtc_peri_iso(&self) -> RTC_PERI_ISO_R {
        RTC_PERI_ISO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rtc peripheral power down"]
    #[inline(always)]
    pub fn xpd_rtc_peri(&self) -> XPD_RTC_PERI_R {
        XPD_RTC_PERI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - wifi iso"]
    #[inline(always)]
    pub fn wifi_iso(&self) -> WIFI_ISO_R {
        WIFI_ISO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - wifi wrap power down"]
    #[inline(always)]
    pub fn xpd_wifi(&self) -> XPD_WIFI_R {
        XPD_WIFI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - digital wrap iso"]
    #[inline(always)]
    pub fn dig_iso(&self) -> DIG_ISO_R {
        DIG_ISO_R::new(((self.bits >> 7) & 1) != 0)
    }
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
#[doc = "reserved register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [low_power_st](index.html) module"]
pub struct LOW_POWER_ST_SPEC;
impl crate::RegisterSpec for LOW_POWER_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [low_power_st::R](R) reader structure"]
impl crate::Readable for LOW_POWER_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOW_POWER_ST to value 0"]
impl crate::Resettable for LOW_POWER_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
