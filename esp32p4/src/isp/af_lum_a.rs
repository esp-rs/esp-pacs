#[doc = "Register `AF_LUM_A` reader"]
pub type R = crate::R<AF_LUM_A_SPEC>;
#[doc = "Field `AF_LUMA` reader - this field represents the result of accumulation of pix light of focus window a"]
pub type AF_LUMA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:27 - this field represents the result of accumulation of pix light of focus window a"]
    #[inline(always)]
    pub fn af_luma(&self) -> AF_LUMA_R {
        AF_LUMA_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_LUM_A")
            .field("af_luma", &self.af_luma())
            .finish()
    }
}
#[doc = "result of lum of af window a\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_lum_a::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_LUM_A_SPEC;
impl crate::RegisterSpec for AF_LUM_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_lum_a::R`](R) reader structure"]
impl crate::Readable for AF_LUM_A_SPEC {}
#[doc = "`reset()` method sets AF_LUM_A to value 0"]
impl crate::Resettable for AF_LUM_A_SPEC {
    const RESET_VALUE: u32 = 0;
}
