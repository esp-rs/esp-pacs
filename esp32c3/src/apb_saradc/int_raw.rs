#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_RAW` reader - saradc thres1 low interrupt raw"]
pub type APB_SARADC_THRES1_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_RAW` reader - saradc thres0 low interrupt raw"]
pub type APB_SARADC_THRES0_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_RAW` reader - saradc thres1 high interrupt raw"]
pub type APB_SARADC_THRES1_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_RAW` reader - saradc thres0 high interrupt raw"]
pub type APB_SARADC_THRES0_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC2_DONE_INT_RAW` reader - saradc2 done interrupt raw"]
pub type APB_SARADC2_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC1_DONE_INT_RAW` reader - saradc1 done interrupt raw"]
pub type APB_SARADC1_DONE_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 26 - saradc thres1 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_raw(&self) -> APB_SARADC_THRES1_LOW_INT_RAW_R {
        APB_SARADC_THRES1_LOW_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_raw(&self) -> APB_SARADC_THRES0_LOW_INT_RAW_R {
        APB_SARADC_THRES0_LOW_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_raw(&self) -> APB_SARADC_THRES1_HIGH_INT_RAW_R {
        APB_SARADC_THRES1_HIGH_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_raw(&self) -> APB_SARADC_THRES0_HIGH_INT_RAW_R {
        APB_SARADC_THRES0_HIGH_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_raw(&self) -> APB_SARADC2_DONE_INT_RAW_R {
        APB_SARADC2_DONE_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_raw(&self) -> APB_SARADC1_DONE_INT_RAW_R {
        APB_SARADC1_DONE_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "apb_saradc_thres1_low_int_raw",
                &format_args!("{}", self.apb_saradc_thres1_low_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres0_low_int_raw",
                &format_args!("{}", self.apb_saradc_thres0_low_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres1_high_int_raw",
                &format_args!("{}", self.apb_saradc_thres1_high_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres0_high_int_raw",
                &format_args!("{}", self.apb_saradc_thres0_high_int_raw().bit()),
            )
            .field(
                "apb_saradc2_done_int_raw",
                &format_args!("{}", self.apb_saradc2_done_int_raw().bit()),
            )
            .field(
                "apb_saradc1_done_int_raw",
                &format_args!("{}", self.apb_saradc1_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "digital saradc int register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
