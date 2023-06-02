#[doc = "Register `INT_ENA_RTC` reader"]
pub struct R(crate::R<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_RTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_RTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_RTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA_RTC` writer"]
pub struct W(crate::W<INT_ENA_RTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_RTC_SPEC>;
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
impl From<crate::W<INT_ENA_RTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_RTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA` reader - Enables interruption when the chip wakes up from sleep."]
pub type SLP_WAKEUP_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP_INT_ENA` writer - Enables interruption when the chip wakes up from sleep."]
pub type SLP_WAKEUP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `SLP_REJECT_INT_ENA` reader - Enables interruption when the chip rejects to go to sleep."]
pub type SLP_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_ENA` writer - Enables interruption when the chip rejects to go to sleep."]
pub type SLP_REJECT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `SDIO_IDLE_INT_ENA` reader - Enables interruption when the SDIO idles."]
pub type SDIO_IDLE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE_INT_ENA` writer - Enables interruption when the SDIO idles."]
pub type SDIO_IDLE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `WDT_INT_ENA` reader - Enables the RTC watchdog interrupt."]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - Enables the RTC watchdog interrupt."]
pub type WDT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `TOUCH_SCAN_DONE_INT_ENA` reader - Enables interruption upon the completion of a touch scanning."]
pub type TOUCH_SCAN_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOUCH_SCAN_DONE_INT_ENA` writer - Enables interruption upon the completion of a touch scanning."]
pub type TOUCH_SCAN_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `ULP_CP_INT_ENA` reader - Enables the ULP co-processor interrupt."]
pub type ULP_CP_INT_ENA_R = crate::BitReader;
#[doc = "Field `ULP_CP_INT_ENA` writer - Enables the ULP co-processor interrupt."]
pub type ULP_CP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `TOUCH_DONE_INT_ENA` reader - Enables interruption upon the completion of a single touch."]
pub type TOUCH_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE_INT_ENA` writer - Enables interruption upon the completion of a single touch."]
pub type TOUCH_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `TOUCH_ACTIVE_INT_ENA` reader - Enables interruption when a touch is detected."]
pub type TOUCH_ACTIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOUCH_ACTIVE_INT_ENA` writer - Enables interruption when a touch is detected."]
pub type TOUCH_ACTIVE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `TOUCH_INACTIVE_INT_ENA` reader - Enables interruption when a touch is released."]
pub type TOUCH_INACTIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE_INT_ENA` writer - Enables interruption when a touch is released."]
pub type TOUCH_INACTIVE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `BROWN_OUT_INT_ENA` reader - Enables the brown out interrupt."]
pub type BROWN_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ENA` writer - Enables the brown out interrupt."]
pub type BROWN_OUT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `MAIN_TIMER_INT_ENA` reader - Enables the RTC main timer interrupt."]
pub type MAIN_TIMER_INT_ENA_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_ENA` writer - Enables the RTC main timer interrupt."]
pub type MAIN_TIMER_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `SARADC1_INT_ENA` reader - Enables the SAR ADC 1 interrupt."]
pub type SARADC1_INT_ENA_R = crate::BitReader;
#[doc = "Field `SARADC1_INT_ENA` writer - Enables the SAR ADC 1 interrupt."]
pub type SARADC1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `TSENS_INT_ENA` reader - Enables the touch sensor interrupt."]
pub type TSENS_INT_ENA_R = crate::BitReader;
#[doc = "Field `TSENS_INT_ENA` writer - Enables the touch sensor interrupt."]
pub type TSENS_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `COCPU_INT_ENA` reader - Enables the ULP-RISCV interrupt."]
pub type COCPU_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_INT_ENA` writer - Enables the ULP-RISCV interrupt."]
pub type COCPU_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `SARADC2_INT_ENA` reader - Enables the SAR ADC 2 interrupt."]
pub type SARADC2_INT_ENA_R = crate::BitReader;
#[doc = "Field `SARADC2_INT_ENA` writer - Enables the SAR ADC 2 interrupt."]
pub type SARADC2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `SWD_INT_ENA` reader - Enables the super watchdog interrupt."]
pub type SWD_INT_ENA_R = crate::BitReader;
#[doc = "Field `SWD_INT_ENA` writer - Enables the super watchdog interrupt."]
pub type SWD_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `XTAL32K_DEAD_INT_ENA` reader - Enables interruption when the 32 kHz crystal is dead."]
pub type XTAL32K_DEAD_INT_ENA_R = crate::BitReader;
#[doc = "Field `XTAL32K_DEAD_INT_ENA` writer - Enables interruption when the 32 kHz crystal is dead."]
pub type XTAL32K_DEAD_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `COCPU_TRAP_INT_ENA` reader - Enables interruption when the ULP-RISCV is trapped."]
pub type COCPU_TRAP_INT_ENA_R = crate::BitReader;
#[doc = "Field `COCPU_TRAP_INT_ENA` writer - Enables interruption when the ULP-RISCV is trapped."]
pub type COCPU_TRAP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `TOUCH_TIMEOUT_INT_ENA` reader - Enables interruption when touch sensor times out."]
pub type TOUCH_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOUCH_TIMEOUT_INT_ENA` writer - Enables interruption when touch sensor times out."]
pub type TOUCH_TIMEOUT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
#[doc = "Field `GLITCH_DET_INT_ENA` reader - Enables interruption when a glitch is detected."]
pub type GLITCH_DET_INT_ENA_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_ENA` writer - Enables interruption when a glitch is detected."]
pub type GLITCH_DET_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_RTC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Enables interruption when the chip wakes up from sleep."]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&self) -> SLP_WAKEUP_INT_ENA_R {
        SLP_WAKEUP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables interruption when the chip rejects to go to sleep."]
    #[inline(always)]
    pub fn slp_reject_int_ena(&self) -> SLP_REJECT_INT_ENA_R {
        SLP_REJECT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables interruption when the SDIO idles."]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&self) -> SDIO_IDLE_INT_ENA_R {
        SDIO_IDLE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables interruption upon the completion of a touch scanning."]
    #[inline(always)]
    pub fn touch_scan_done_int_ena(&self) -> TOUCH_SCAN_DONE_INT_ENA_R {
        TOUCH_SCAN_DONE_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    pub fn ulp_cp_int_ena(&self) -> ULP_CP_INT_ENA_R {
        ULP_CP_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables interruption upon the completion of a single touch."]
    #[inline(always)]
    pub fn touch_done_int_ena(&self) -> TOUCH_DONE_INT_ENA_R {
        TOUCH_DONE_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables interruption when a touch is detected."]
    #[inline(always)]
    pub fn touch_active_int_ena(&self) -> TOUCH_ACTIVE_INT_ENA_R {
        TOUCH_ACTIVE_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables interruption when a touch is released."]
    #[inline(always)]
    pub fn touch_inactive_int_ena(&self) -> TOUCH_INACTIVE_INT_ENA_R {
        TOUCH_INACTIVE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the brown out interrupt."]
    #[inline(always)]
    pub fn brown_out_int_ena(&self) -> BROWN_OUT_INT_ENA_R {
        BROWN_OUT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables the RTC main timer interrupt."]
    #[inline(always)]
    pub fn main_timer_int_ena(&self) -> MAIN_TIMER_INT_ENA_R {
        MAIN_TIMER_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables the SAR ADC 1 interrupt."]
    #[inline(always)]
    pub fn saradc1_int_ena(&self) -> SARADC1_INT_ENA_R {
        SARADC1_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables the touch sensor interrupt."]
    #[inline(always)]
    pub fn tsens_int_ena(&self) -> TSENS_INT_ENA_R {
        TSENS_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the ULP-RISCV interrupt."]
    #[inline(always)]
    pub fn cocpu_int_ena(&self) -> COCPU_INT_ENA_R {
        COCPU_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables the SAR ADC 2 interrupt."]
    #[inline(always)]
    pub fn saradc2_int_ena(&self) -> SARADC2_INT_ENA_R {
        SARADC2_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables the super watchdog interrupt."]
    #[inline(always)]
    pub fn swd_int_ena(&self) -> SWD_INT_ENA_R {
        SWD_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables interruption when the 32 kHz crystal is dead."]
    #[inline(always)]
    pub fn xtal32k_dead_int_ena(&self) -> XTAL32K_DEAD_INT_ENA_R {
        XTAL32K_DEAD_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables interruption when the ULP-RISCV is trapped."]
    #[inline(always)]
    pub fn cocpu_trap_int_ena(&self) -> COCPU_TRAP_INT_ENA_R {
        COCPU_TRAP_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables interruption when touch sensor times out."]
    #[inline(always)]
    pub fn touch_timeout_int_ena(&self) -> TOUCH_TIMEOUT_INT_ENA_R {
        TOUCH_TIMEOUT_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables interruption when a glitch is detected."]
    #[inline(always)]
    pub fn glitch_det_int_ena(&self) -> GLITCH_DET_INT_ENA_R {
        GLITCH_DET_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA_RTC")
            .field(
                "slp_wakeup_int_ena",
                &format_args!("{}", self.slp_wakeup_int_ena().bit()),
            )
            .field(
                "slp_reject_int_ena",
                &format_args!("{}", self.slp_reject_int_ena().bit()),
            )
            .field(
                "sdio_idle_int_ena",
                &format_args!("{}", self.sdio_idle_int_ena().bit()),
            )
            .field("wdt_int_ena", &format_args!("{}", self.wdt_int_ena().bit()))
            .field(
                "touch_scan_done_int_ena",
                &format_args!("{}", self.touch_scan_done_int_ena().bit()),
            )
            .field(
                "ulp_cp_int_ena",
                &format_args!("{}", self.ulp_cp_int_ena().bit()),
            )
            .field(
                "touch_done_int_ena",
                &format_args!("{}", self.touch_done_int_ena().bit()),
            )
            .field(
                "touch_active_int_ena",
                &format_args!("{}", self.touch_active_int_ena().bit()),
            )
            .field(
                "touch_inactive_int_ena",
                &format_args!("{}", self.touch_inactive_int_ena().bit()),
            )
            .field(
                "brown_out_int_ena",
                &format_args!("{}", self.brown_out_int_ena().bit()),
            )
            .field(
                "main_timer_int_ena",
                &format_args!("{}", self.main_timer_int_ena().bit()),
            )
            .field(
                "saradc1_int_ena",
                &format_args!("{}", self.saradc1_int_ena().bit()),
            )
            .field(
                "tsens_int_ena",
                &format_args!("{}", self.tsens_int_ena().bit()),
            )
            .field(
                "cocpu_int_ena",
                &format_args!("{}", self.cocpu_int_ena().bit()),
            )
            .field(
                "saradc2_int_ena",
                &format_args!("{}", self.saradc2_int_ena().bit()),
            )
            .field("swd_int_ena", &format_args!("{}", self.swd_int_ena().bit()))
            .field(
                "xtal32k_dead_int_ena",
                &format_args!("{}", self.xtal32k_dead_int_ena().bit()),
            )
            .field(
                "cocpu_trap_int_ena",
                &format_args!("{}", self.cocpu_trap_int_ena().bit()),
            )
            .field(
                "touch_timeout_int_ena",
                &format_args!("{}", self.touch_timeout_int_ena().bit()),
            )
            .field(
                "glitch_det_int_ena",
                &format_args!("{}", self.glitch_det_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_RTC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enables interruption when the chip wakes up from sleep."]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena(&mut self) -> SLP_WAKEUP_INT_ENA_W<0> {
        SLP_WAKEUP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Enables interruption when the chip rejects to go to sleep."]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena(&mut self) -> SLP_REJECT_INT_ENA_W<1> {
        SLP_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Enables interruption when the SDIO idles."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_ena(&mut self) -> SDIO_IDLE_INT_ENA_W<2> {
        SDIO_IDLE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - Enables the RTC watchdog interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<3> {
        WDT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - Enables interruption upon the completion of a touch scanning."]
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_done_int_ena(&mut self) -> TOUCH_SCAN_DONE_INT_ENA_W<4> {
        TOUCH_SCAN_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - Enables the ULP co-processor interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_int_ena(&mut self) -> ULP_CP_INT_ENA_W<5> {
        ULP_CP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - Enables interruption upon the completion of a single touch."]
    #[inline(always)]
    #[must_use]
    pub fn touch_done_int_ena(&mut self) -> TOUCH_DONE_INT_ENA_W<6> {
        TOUCH_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - Enables interruption when a touch is detected."]
    #[inline(always)]
    #[must_use]
    pub fn touch_active_int_ena(&mut self) -> TOUCH_ACTIVE_INT_ENA_W<7> {
        TOUCH_ACTIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - Enables interruption when a touch is released."]
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive_int_ena(&mut self) -> TOUCH_INACTIVE_INT_ENA_W<8> {
        TOUCH_INACTIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - Enables the brown out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena(&mut self) -> BROWN_OUT_INT_ENA_W<9> {
        BROWN_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - Enables the RTC main timer interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena(&mut self) -> MAIN_TIMER_INT_ENA_W<10> {
        MAIN_TIMER_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - Enables the SAR ADC 1 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn saradc1_int_ena(&mut self) -> SARADC1_INT_ENA_W<11> {
        SARADC1_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - Enables the touch sensor interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tsens_int_ena(&mut self) -> TSENS_INT_ENA_W<12> {
        TSENS_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - Enables the ULP-RISCV interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_int_ena(&mut self) -> COCPU_INT_ENA_W<13> {
        COCPU_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - Enables the SAR ADC 2 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn saradc2_int_ena(&mut self) -> SARADC2_INT_ENA_W<14> {
        SARADC2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - Enables the super watchdog interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn swd_int_ena(&mut self) -> SWD_INT_ENA_W<15> {
        SWD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - Enables interruption when the 32 kHz crystal is dead."]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_dead_int_ena(&mut self) -> XTAL32K_DEAD_INT_ENA_W<16> {
        XTAL32K_DEAD_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - Enables interruption when the ULP-RISCV is trapped."]
    #[inline(always)]
    #[must_use]
    pub fn cocpu_trap_int_ena(&mut self) -> COCPU_TRAP_INT_ENA_W<17> {
        COCPU_TRAP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - Enables interruption when touch sensor times out."]
    #[inline(always)]
    #[must_use]
    pub fn touch_timeout_int_ena(&mut self) -> TOUCH_TIMEOUT_INT_ENA_W<18> {
        TOUCH_TIMEOUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - Enables interruption when a glitch is detected."]
    #[inline(always)]
    #[must_use]
    pub fn glitch_det_int_ena(&mut self) -> GLITCH_DET_INT_ENA_W<19> {
        GLITCH_DET_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC interrupt enabling register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena_rtc](index.html) module"]
pub struct INT_ENA_RTC_SPEC;
impl crate::RegisterSpec for INT_ENA_RTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena_rtc::R](R) reader structure"]
impl crate::Readable for INT_ENA_RTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena_rtc::W](W) writer structure"]
impl crate::Writable for INT_ENA_RTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA_RTC to value 0"]
impl crate::Resettable for INT_ENA_RTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
