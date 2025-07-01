#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `CALC_DONE` reader - The raw interrupt status bit for the ecdsa_calc_done_int interrupt"]
pub type CALC_DONE_R = crate::BitReader;
#[doc = "Field `SHA_RELEASE` reader - The raw interrupt status bit for the ecdsa_sha_release_int interrupt"]
pub type SHA_RELEASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the ecdsa_calc_done_int interrupt"]
    #[inline(always)]
    pub fn calc_done(&self) -> CALC_DONE_R {
        CALC_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the ecdsa_sha_release_int interrupt"]
    #[inline(always)]
    pub fn sha_release(&self) -> SHA_RELEASE_R {
        SHA_RELEASE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("calc_done", &self.calc_done())
            .field("sha_release", &self.sha_release())
            .finish()
    }
}
#[doc = "ECDSA interrupt raw register, valid in level.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
