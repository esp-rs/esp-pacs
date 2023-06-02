#[doc = "Register `RTCCALICFG1` reader"]
pub struct R(crate::R<RTCCALICFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCALICFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCALICFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCALICFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RTC_CALI_CYCLING_DATA_VLD` reader - indicate periodic frequency calculation is done."]
pub type RTC_CALI_CYCLING_DATA_VLD_R = crate::BitReader;
#[doc = "Field `RTC_CALI_VALUE` reader - When one-shot or periodic frequency calculation is done, read this value to calculate RTC slow clock's frequency."]
pub type RTC_CALI_VALUE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0 - indicate periodic frequency calculation is done."]
    #[inline(always)]
    pub fn rtc_cali_cycling_data_vld(&self) -> RTC_CALI_CYCLING_DATA_VLD_R {
        RTC_CALI_CYCLING_DATA_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 7:31 - When one-shot or periodic frequency calculation is done, read this value to calculate RTC slow clock's frequency."]
    #[inline(always)]
    pub fn rtc_cali_value(&self) -> RTC_CALI_VALUE_R {
        RTC_CALI_VALUE_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCALICFG1")
            .field(
                "rtc_cali_cycling_data_vld",
                &format_args!("{}", self.rtc_cali_cycling_data_vld().bit()),
            )
            .field(
                "rtc_cali_value",
                &format_args!("{}", self.rtc_cali_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RTCCALICFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RTC calibration configure1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccalicfg1](index.html) module"]
pub struct RTCCALICFG1_SPEC;
impl crate::RegisterSpec for RTCCALICFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtccalicfg1::R](R) reader structure"]
impl crate::Readable for RTCCALICFG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTCCALICFG1 to value 0"]
impl crate::Resettable for RTCCALICFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
