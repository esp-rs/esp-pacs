#[doc = "Register `SYSCLK_FREQ_QUERY_0` reader"]
pub struct R(crate::R<SYSCLK_FREQ_QUERY_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCLK_FREQ_QUERY_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCLK_FREQ_QUERY_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCLK_FREQ_QUERY_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FOSC_FREQ` reader - This field indicates the frequency(MHz) of FOSC."]
pub type FOSC_FREQ_R = crate::FieldReader;
#[doc = "Field `PLL_FREQ` reader - This field indicates the frequency(MHz) of SPLL."]
pub type PLL_FREQ_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - This field indicates the frequency(MHz) of FOSC."]
    #[inline(always)]
    pub fn fosc_freq(&self) -> FOSC_FREQ_R {
        FOSC_FREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:17 - This field indicates the frequency(MHz) of SPLL."]
    #[inline(always)]
    pub fn pll_freq(&self) -> PLL_FREQ_R {
        PLL_FREQ_R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_FREQ_QUERY_0")
            .field("fosc_freq", &format_args!("{}", self.fosc_freq().bits()))
            .field("pll_freq", &format_args!("{}", self.pll_freq().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SYSCLK_FREQ_QUERY_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SYSCLK frequency query 0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysclk_freq_query_0](index.html) module"]
pub struct SYSCLK_FREQ_QUERY_0_SPEC;
impl crate::RegisterSpec for SYSCLK_FREQ_QUERY_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysclk_freq_query_0::R](R) reader structure"]
impl crate::Readable for SYSCLK_FREQ_QUERY_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYSCLK_FREQ_QUERY_0 to value 0x6008"]
impl crate::Resettable for SYSCLK_FREQ_QUERY_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x6008;
}
