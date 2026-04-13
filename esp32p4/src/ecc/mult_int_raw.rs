#[doc = "Register `MULT_INT_RAW` reader"]
pub type R = crate::R<MULT_INT_RAW_SPEC>;
#[doc = "Field `MULT_CALC_DONE` reader - The raw interrupt status of the ECC_CALC_DONE_INT interrupt."]
pub type MULT_CALC_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn mult_calc_done(&self) -> MULT_CALC_DONE_R {
        MULT_CALC_DONE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_INT_RAW")
            .field("mult_calc_done", &self.mult_calc_done())
            .finish()
    }
}
#[doc = "ECC raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mult_int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_INT_RAW_SPEC;
impl crate::RegisterSpec for MULT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mult_int_raw::R`](R) reader structure"]
impl crate::Readable for MULT_INT_RAW_SPEC {}
#[doc = "`reset()` method sets MULT_INT_RAW to value 0"]
impl crate::Resettable for MULT_INT_RAW_SPEC {}
