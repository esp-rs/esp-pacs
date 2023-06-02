#[doc = "Register `GPIO_RTC_CALIB_SYNC` reader"]
pub struct R(crate::R<GPIO_RTC_CALIB_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_RTC_CALIB_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_RTC_CALIB_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_RTC_CALIB_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_RTC_CALIB_SYNC` writer"]
pub struct W(crate::W<GPIO_RTC_CALIB_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_RTC_CALIB_SYNC_SPEC>;
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
impl From<crate::W<GPIO_RTC_CALIB_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_RTC_CALIB_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_PERIOD_NUM` reader - The cycle number of RTC-clock during RTC-clock-calibration"]
pub type RTC_PERIOD_NUM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RTC_PERIOD_NUM` writer - The cycle number of RTC-clock during RTC-clock-calibration"]
pub type RTC_PERIOD_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, GPIO_RTC_CALIB_SYNC_SPEC, 10, O, u16, u16>;
#[doc = "Field `RTC_CALIB_START` reader - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
pub type RTC_CALIB_START_R = crate::BitReader;
#[doc = "Field `RTC_CALIB_START` writer - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
pub type RTC_CALIB_START_W<'a, const O: u8> = crate::BitWriter<'a, GPIO_RTC_CALIB_SYNC_SPEC, O>;
impl R {
    #[doc = "Bits 0:9 - The cycle number of RTC-clock during RTC-clock-calibration"]
    #[inline(always)]
    pub fn rtc_period_num(&self) -> RTC_PERIOD_NUM_R {
        RTC_PERIOD_NUM_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
    #[inline(always)]
    pub fn rtc_calib_start(&self) -> RTC_CALIB_START_R {
        RTC_CALIB_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_RTC_CALIB_SYNC")
            .field(
                "rtc_calib_start",
                &format_args!("{}", self.rtc_calib_start().bit()),
            )
            .field(
                "rtc_period_num",
                &format_args!("{}", self.rtc_period_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_RTC_CALIB_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:9 - The cycle number of RTC-clock during RTC-clock-calibration"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_period_num(&mut self) -> RTC_PERIOD_NUM_W<0> {
        RTC_PERIOD_NUM_W::new(self)
    }
    #[doc = "Bit 31 - Positvie edge of this bit will trigger the RTC-clock-calibration process."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_calib_start(&mut self) -> RTC_CALIB_START_W<31> {
        RTC_CALIB_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Positvie edge of this bit will trigger the RTC-clock-calibration process.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_rtc_calib_sync](index.html) module"]
pub struct GPIO_RTC_CALIB_SYNC_SPEC;
impl crate::RegisterSpec for GPIO_RTC_CALIB_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_rtc_calib_sync::R](R) reader structure"]
impl crate::Readable for GPIO_RTC_CALIB_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_rtc_calib_sync::W](W) writer structure"]
impl crate::Writable for GPIO_RTC_CALIB_SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_RTC_CALIB_SYNC to value 0"]
impl crate::Resettable for GPIO_RTC_CALIB_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
