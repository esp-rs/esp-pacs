#[doc = "Register `BLEND_ST` reader"]
pub type R = crate::R<BLEND_ST_SPEC>;
#[doc = "Field `BLEND_SIZE_DIFF_ST` reader - 1: indicate the size of two image is different."]
pub type BLEND_SIZE_DIFF_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: indicate the size of two image is different."]
    #[inline(always)]
    pub fn blend_size_diff_st(&self) -> BLEND_SIZE_DIFF_ST_R {
        BLEND_SIZE_DIFF_ST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEND_ST")
            .field(
                "blend_size_diff_st",
                &format_args!("{}", self.blend_size_diff_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLEND_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Blending engine status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blend_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEND_ST_SPEC;
impl crate::RegisterSpec for BLEND_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blend_st::R`](R) reader structure"]
impl crate::Readable for BLEND_ST_SPEC {}
#[doc = "`reset()` method sets BLEND_ST to value 0"]
impl crate::Resettable for BLEND_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
