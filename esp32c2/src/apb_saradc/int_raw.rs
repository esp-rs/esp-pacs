#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `THRES1_LOW` reader - Need add description"]
pub type THRES1_LOW_R = crate::BitReader;
#[doc = "Field `THRES0_LOW` reader - Need add description"]
pub type THRES0_LOW_R = crate::BitReader;
#[doc = "Field `THRES1_HIGH` reader - Need add description"]
pub type THRES1_HIGH_R = crate::BitReader;
#[doc = "Field `THRES0_HIGH` reader - Need add description"]
pub type THRES0_HIGH_R = crate::BitReader;
#[doc = "Field `ADC2_DONE` reader - Need add description"]
pub type ADC2_DONE_R = crate::BitReader;
#[doc = "Field `ADC1_DONE` reader - Need add description"]
pub type ADC1_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 26 - Need add description"]
    #[inline(always)]
    pub fn thres1_low(&self) -> THRES1_LOW_R {
        THRES1_LOW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Need add description"]
    #[inline(always)]
    pub fn thres0_low(&self) -> THRES0_LOW_R {
        THRES0_LOW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add description"]
    #[inline(always)]
    pub fn thres1_high(&self) -> THRES1_HIGH_R {
        THRES1_HIGH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn thres0_high(&self) -> THRES0_HIGH_R {
        THRES0_HIGH_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn adc2_done(&self) -> ADC2_DONE_R {
        ADC2_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn adc1_done(&self) -> ADC1_DONE_R {
        ADC1_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("thres1_low", &format_args!("{}", self.thres1_low().bit()))
            .field("thres0_low", &format_args!("{}", self.thres0_low().bit()))
            .field("thres1_high", &format_args!("{}", self.thres1_high().bit()))
            .field("thres0_high", &format_args!("{}", self.thres0_high().bit()))
            .field("adc2_done", &format_args!("{}", self.adc2_done().bit()))
            .field("adc1_done", &format_args!("{}", self.adc1_done().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
