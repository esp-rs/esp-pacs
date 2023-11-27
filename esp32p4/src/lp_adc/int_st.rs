#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `COCPU_SARADC1_INT_ST` reader - ADC1 Conversion is done, int status."]
pub type COCPU_SARADC1_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_INT_ST` reader - ADC2 Conversion is done, int status."]
pub type COCPU_SARADC2_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_ERROR_INT_ST` reader - An errro occurs from ADC1, int status."]
pub type COCPU_SARADC1_ERROR_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_ERROR_INT_ST` reader - An errro occurs from ADC2, int status."]
pub type COCPU_SARADC2_ERROR_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC1_WAKE_INT_ST` reader - A wakeup event is triggered from ADC1, int status."]
pub type COCPU_SARADC1_WAKE_INT_ST_R = crate::BitReader;
#[doc = "Field `COCPU_SARADC2_WAKE_INT_ST` reader - A wakeup event is triggered from ADC2, int status."]
pub type COCPU_SARADC2_WAKE_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ADC1 Conversion is done, int status."]
    #[inline(always)]
    pub fn cocpu_saradc1_int_st(&self) -> COCPU_SARADC1_INT_ST_R {
        COCPU_SARADC1_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC2 Conversion is done, int status."]
    #[inline(always)]
    pub fn cocpu_saradc2_int_st(&self) -> COCPU_SARADC2_INT_ST_R {
        COCPU_SARADC2_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - An errro occurs from ADC1, int status."]
    #[inline(always)]
    pub fn cocpu_saradc1_error_int_st(&self) -> COCPU_SARADC1_ERROR_INT_ST_R {
        COCPU_SARADC1_ERROR_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - An errro occurs from ADC2, int status."]
    #[inline(always)]
    pub fn cocpu_saradc2_error_int_st(&self) -> COCPU_SARADC2_ERROR_INT_ST_R {
        COCPU_SARADC2_ERROR_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A wakeup event is triggered from ADC1, int status."]
    #[inline(always)]
    pub fn cocpu_saradc1_wake_int_st(&self) -> COCPU_SARADC1_WAKE_INT_ST_R {
        COCPU_SARADC1_WAKE_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A wakeup event is triggered from ADC2, int status."]
    #[inline(always)]
    pub fn cocpu_saradc2_wake_int_st(&self) -> COCPU_SARADC2_WAKE_INT_ST_R {
        COCPU_SARADC2_WAKE_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "cocpu_saradc1_int_st",
                &format_args!("{}", self.cocpu_saradc1_int_st().bit()),
            )
            .field(
                "cocpu_saradc2_int_st",
                &format_args!("{}", self.cocpu_saradc2_int_st().bit()),
            )
            .field(
                "cocpu_saradc1_error_int_st",
                &format_args!("{}", self.cocpu_saradc1_error_int_st().bit()),
            )
            .field(
                "cocpu_saradc2_error_int_st",
                &format_args!("{}", self.cocpu_saradc2_error_int_st().bit()),
            )
            .field(
                "cocpu_saradc1_wake_int_st",
                &format_args!("{}", self.cocpu_saradc1_wake_int_st().bit()),
            )
            .field(
                "cocpu_saradc2_wake_int_st",
                &format_args!("{}", self.cocpu_saradc2_wake_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt status registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
