#[doc = "Register `AF_LUM_C` reader"]
pub type R = crate::R<AF_LUM_C_SPEC>;
#[doc = "Field `AF_LUMC` reader - this field represents the result of accumulation of pix light of focus window c"]
pub type AF_LUMC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - this field represents the result of accumulation of pix light of focus window c"]
    #[inline(always)]
    pub fn af_lumc(&self) -> AF_LUMC_R {
        AF_LUMC_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_LUM_C")
            .field("af_lumc", &self.af_lumc())
            .finish()
    }
}
#[doc = "result of lum of af window c\n\nYou can [`read`](crate::Reg::read) this register and get [`af_lum_c::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_LUM_C_SPEC;
impl crate::RegisterSpec for AF_LUM_C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_lum_c::R`](R) reader structure"]
impl crate::Readable for AF_LUM_C_SPEC {}
#[doc = "`reset()` method sets AF_LUM_C to value 0"]
impl crate::Resettable for AF_LUM_C_SPEC {
    const RESET_VALUE: u32 = 0;
}
