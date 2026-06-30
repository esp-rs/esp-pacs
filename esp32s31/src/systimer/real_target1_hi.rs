#[doc = "Register `REAL_TARGET1_HI` reader"]
pub type R = crate::R<REAL_TARGET1_HI_SPEC>;
#[doc = "Field `TARGET1_HI_RO` reader - Represents the actual target value of COMP1, high 20 bits."]
pub type TARGET1_HI_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - Represents the actual target value of COMP1, high 20 bits."]
    #[inline(always)]
    pub fn target1_hi_ro(&self) -> TARGET1_HI_RO_R {
        TARGET1_HI_RO_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REAL_TARGET1_HI")
            .field("target1_hi_ro", &self.target1_hi_ro())
            .finish()
    }
}
#[doc = "Actual target value of COMP1, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target1_hi::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REAL_TARGET1_HI_SPEC;
impl crate::RegisterSpec for REAL_TARGET1_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`real_target1_hi::R`](R) reader structure"]
impl crate::Readable for REAL_TARGET1_HI_SPEC {}
#[doc = "`reset()` method sets REAL_TARGET1_HI to value 0"]
impl crate::Resettable for REAL_TARGET1_HI_SPEC {}
