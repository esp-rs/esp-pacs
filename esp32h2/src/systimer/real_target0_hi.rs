#[doc = "Register `REAL_TARGET0_HI` reader"]
pub type R = crate::R<REAL_TARGET0_HI_SPEC>;
#[doc = "Field `TARGET0_HI_RO` reader - actual target value value high 20bits"]
pub type TARGET0_HI_RO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - actual target value value high 20bits"]
    #[inline(always)]
    pub fn target0_hi_ro(&self) -> TARGET0_HI_RO_R {
        TARGET0_HI_RO_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REAL_TARGET0_HI")
            .field(
                "target0_hi_ro",
                &format_args!("{}", self.target0_hi_ro().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REAL_TARGET0_HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "system timer comp0 actual target value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`real_target0_hi::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REAL_TARGET0_HI_SPEC;
impl crate::RegisterSpec for REAL_TARGET0_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`real_target0_hi::R`](R) reader structure"]
impl crate::Readable for REAL_TARGET0_HI_SPEC {}
#[doc = "`reset()` method sets REAL_TARGET0_HI to value 0"]
impl crate::Resettable for REAL_TARGET0_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
