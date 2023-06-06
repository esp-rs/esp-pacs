#[doc = "Register `RTCCALICFG2` reader"]
pub struct R(crate::R<RTCCALICFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCALICFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCALICFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCALICFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCALICFG2` writer"]
pub struct W(crate::W<RTCCALICFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCALICFG2_SPEC>;
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
impl From<crate::W<RTCCALICFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCALICFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_CALI_TIMEOUT` reader - RTC calibration timeout indicator"]
pub type RTC_CALI_TIMEOUT_R = crate::BitReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` reader - Cycles that release calibration timeout reset"]
pub type RTC_CALI_TIMEOUT_RST_CNT_R = crate::FieldReader;
#[doc = "Field `RTC_CALI_TIMEOUT_RST_CNT` writer - Cycles that release calibration timeout reset"]
pub type RTC_CALI_TIMEOUT_RST_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, RTCCALICFG2_SPEC, 4, O>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` reader - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
pub type RTC_CALI_TIMEOUT_THRES_R = crate::FieldReader<u32>;
#[doc = "Field `RTC_CALI_TIMEOUT_THRES` writer - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
pub type RTC_CALI_TIMEOUT_THRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, RTCCALICFG2_SPEC, 25, O, u32>;
impl R {
    #[doc = "Bit 0 - RTC calibration timeout indicator"]
    #[inline(always)]
    pub fn rtc_cali_timeout(&self) -> RTC_CALI_TIMEOUT_R {
        RTC_CALI_TIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:6 - Cycles that release calibration timeout reset"]
    #[inline(always)]
    pub fn rtc_cali_timeout_rst_cnt(&self) -> RTC_CALI_TIMEOUT_RST_CNT_R {
        RTC_CALI_TIMEOUT_RST_CNT_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:31 - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
    #[inline(always)]
    pub fn rtc_cali_timeout_thres(&self) -> RTC_CALI_TIMEOUT_THRES_R {
        RTC_CALI_TIMEOUT_THRES_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG2")
            .field(
                "rtc_cali_timeout",
                &format_args!("{}", self.rtc_cali_timeout().bit()),
            )
            .field(
                "rtc_cali_timeout_rst_cnt",
                &format_args!("{}", self.rtc_cali_timeout_rst_cnt().bits()),
            )
            .field(
                "rtc_cali_timeout_thres",
                &format_args!("{}", self.rtc_cali_timeout_thres().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTCCALICFG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 3:6 - Cycles that release calibration timeout reset"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_timeout_rst_cnt(&mut self) -> RTC_CALI_TIMEOUT_RST_CNT_W<3> {
        RTC_CALI_TIMEOUT_RST_CNT_W::new(self)
    }
    #[doc = "Bits 7:31 - Threshold value for the RTC calibration timer. If the calibration timer's value exceeds this threshold, a timeout is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn rtc_cali_timeout_thres(&mut self) -> RTC_CALI_TIMEOUT_THRES_W<7> {
        RTC_CALI_TIMEOUT_THRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer group calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg2](index.html) module"]
pub struct RTCCALICFG2_SPEC;
impl crate::RegisterSpec for RTCCALICFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccalicfg2::R](R) reader structure"]
impl crate::Readable for RTCCALICFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtccalicfg2::W](W) writer structure"]
impl crate::Writable for RTCCALICFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCCALICFG2 to value 0xffff_ff98"]
impl crate::Resettable for RTCCALICFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ff98;
}
