#[doc = "Register `CALI1` reader"]
pub type R = crate::R<CALI1_SPEC>;
#[doc = "Field `LP_CALI_DIV_NUMERATOR` reader - "]
pub type LP_CALI_DIV_NUMERATOR_R = crate::FieldReader<u16>;
#[doc = "Field `LP_CALI_DIV_DENOMINATOR` reader - "]
pub type LP_CALI_DIV_DENOMINATOR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lp_cali_div_numerator(&self) -> LP_CALI_DIV_NUMERATOR_R {
        LP_CALI_DIV_NUMERATOR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lp_cali_div_denominator(&self) -> LP_CALI_DIV_DENOMINATOR_R {
        LP_CALI_DIV_DENOMINATOR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI1")
            .field("lp_cali_div_numerator", &self.lp_cali_div_numerator())
            .field("lp_cali_div_denominator", &self.lp_cali_div_denominator())
            .finish()
    }
}
#[doc = "LP clock calibration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cali1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI1_SPEC;
impl crate::RegisterSpec for CALI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali1::R`](R) reader structure"]
impl crate::Readable for CALI1_SPEC {}
#[doc = "`reset()` method sets CALI1 to value 0"]
impl crate::Resettable for CALI1_SPEC {}
