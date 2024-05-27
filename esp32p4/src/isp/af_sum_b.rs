#[doc = "Register `AF_SUM_B` reader"]
pub type R = crate::R<AF_SUM_B_SPEC>;
#[doc = "Field `AF_SUMB` reader - this field represents the result of accumulation of pix grad of focus window b"]
pub type AF_SUMB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - this field represents the result of accumulation of pix grad of focus window b"]
    #[inline(always)]
    pub fn af_sumb(&self) -> AF_SUMB_R {
        AF_SUMB_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_SUM_B")
            .field("af_sumb", &self.af_sumb())
            .finish()
    }
}
#[doc = "result of sum of af window b\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_sum_b::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_SUM_B_SPEC;
impl crate::RegisterSpec for AF_SUM_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_sum_b::R`](R) reader structure"]
impl crate::Readable for AF_SUM_B_SPEC {}
#[doc = "`reset()` method sets AF_SUM_B to value 0"]
impl crate::Resettable for AF_SUM_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
