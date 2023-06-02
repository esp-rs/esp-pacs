#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLP_WAKEUP_INT_ENA` reader - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLP_WAKEUP_INT_ENA` writer - enable sleep wakeup interrupt"]
pub type SLP_WAKEUP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `SLP_REJECT_INT_ENA` reader - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLP_REJECT_INT_ENA` writer - enable sleep reject interrupt"]
pub type SLP_REJECT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `SDIO_IDLE_INT_ENA` reader - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE_INT_ENA` writer - enable SDIO idle interrupt"]
pub type SDIO_IDLE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `WDT_INT_ENA` reader - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_R = crate::BitReader;
#[doc = "Field `WDT_INT_ENA` writer - enable RTC WDT interrupt"]
pub type WDT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TIME_VALID_INT_ENA` reader - enable RTC time valid interrupt"]
pub type TIME_VALID_INT_ENA_R = crate::BitReader;
#[doc = "Field `TIME_VALID_INT_ENA` writer - enable RTC time valid interrupt"]
pub type TIME_VALID_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ULP_CP_INT_ENA` reader - enable ULP-coprocessor interrupt"]
pub type ULP_CP_INT_ENA_R = crate::BitReader;
#[doc = "Field `ULP_CP_INT_ENA` writer - enable ULP-coprocessor interrupt"]
pub type ULP_CP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TOUCH_INT_ENA` reader - enable touch interrupt"]
pub type TOUCH_INT_ENA_R = crate::BitReader;
#[doc = "Field `TOUCH_INT_ENA` writer - enable touch interrupt"]
pub type TOUCH_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `BROWN_OUT_INT_ENA` reader - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `BROWN_OUT_INT_ENA` writer - enable brown out interrupt"]
pub type BROWN_OUT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `MAIN_TIMER_INT_ENA` reader - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_INT_ENA` writer - enable RTC main timer interrupt"]
pub type MAIN_TIMER_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    pub fn slp_wakeup_int_ena(&self) -> SLP_WAKEUP_INT_ENA_R {
        SLP_WAKEUP_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    pub fn slp_reject_int_ena(&self) -> SLP_REJECT_INT_ENA_R {
        SLP_REJECT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&self) -> SDIO_IDLE_INT_ENA_R {
        SDIO_IDLE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    pub fn wdt_int_ena(&self) -> WDT_INT_ENA_R {
        WDT_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - enable RTC time valid interrupt"]
    #[inline(always)]
    pub fn time_valid_int_ena(&self) -> TIME_VALID_INT_ENA_R {
        TIME_VALID_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    pub fn ulp_cp_int_ena(&self) -> ULP_CP_INT_ENA_R {
        ULP_CP_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable touch interrupt"]
    #[inline(always)]
    pub fn touch_int_ena(&self) -> TOUCH_INT_ENA_R {
        TOUCH_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - enable brown out interrupt"]
    #[inline(always)]
    pub fn brown_out_int_ena(&self) -> BROWN_OUT_INT_ENA_R {
        BROWN_OUT_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable RTC main timer interrupt"]
    #[inline(always)]
    pub fn main_timer_int_ena(&self) -> MAIN_TIMER_INT_ENA_R {
        MAIN_TIMER_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
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
                "time_valid_int_ena",
                &format_args!("{}", self.time_valid_int_ena().bit()),
            )
            .field(
                "ulp_cp_int_ena",
                &format_args!("{}", self.ulp_cp_int_ena().bit()),
            )
            .field(
                "touch_int_ena",
                &format_args!("{}", self.touch_int_ena().bit()),
            )
            .field(
                "brown_out_int_ena",
                &format_args!("{}", self.brown_out_int_ena().bit()),
            )
            .field(
                "main_timer_int_ena",
                &format_args!("{}", self.main_timer_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - enable sleep wakeup interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_wakeup_int_ena(&mut self) -> SLP_WAKEUP_INT_ENA_W<0> {
        SLP_WAKEUP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - enable sleep reject interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn slp_reject_int_ena(&mut self) -> SLP_REJECT_INT_ENA_W<1> {
        SLP_REJECT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - enable SDIO idle interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_idle_int_ena(&mut self) -> SDIO_IDLE_INT_ENA_W<2> {
        SDIO_IDLE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - enable RTC WDT interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_int_ena(&mut self) -> WDT_INT_ENA_W<3> {
        WDT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - enable RTC time valid interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn time_valid_int_ena(&mut self) -> TIME_VALID_INT_ENA_W<4> {
        TIME_VALID_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - enable ULP-coprocessor interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_cp_int_ena(&mut self) -> ULP_CP_INT_ENA_W<5> {
        ULP_CP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - enable touch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn touch_int_ena(&mut self) -> TOUCH_INT_ENA_W<6> {
        TOUCH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - enable brown out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn brown_out_int_ena(&mut self) -> BROWN_OUT_INT_ENA_W<7> {
        BROWN_OUT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - enable RTC main timer interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_int_ena(&mut self) -> MAIN_TIMER_INT_ENA_W<8> {
        MAIN_TIMER_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
