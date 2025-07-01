#[doc = "Register `AF_SUM_C` reader"]
pub type R = crate::R<AF_SUM_C_SPEC>;
#[doc = "Field `AF_SUMC` reader - this field represents the result of accumulation of pix grad of focus window c"]
pub type AF_SUMC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - this field represents the result of accumulation of pix grad of focus window c"]
    #[inline(always)]
    pub fn af_sumc(&self) -> AF_SUMC_R {
        AF_SUMC_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_SUM_C")
            .field("af_sumc", &self.af_sumc())
            .finish()
    }
}
#[doc = "result of sum of af window c\n\nYou can [`read`](crate::Reg::read) this register and get [`af_sum_c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_SUM_C_SPEC;
impl crate::RegisterSpec for AF_SUM_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_sum_c::R`](R) reader structure"]
impl crate::Readable for AF_SUM_C_SPEC {}
#[doc = "`reset()` method sets AF_SUM_C to value 0"]
impl crate::Resettable for AF_SUM_C_SPEC {}
