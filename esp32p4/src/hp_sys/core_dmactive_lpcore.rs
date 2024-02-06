#[doc = "Register `CORE_DMACTIVE_LPCORE` reader"]
pub type R = crate::R<CORE_DMACTIVE_LPCORE_SPEC>;
#[doc = "Field `CORE_DMACTIVE_LPCORE` reader - hp core dmactive_lpcore value"]
pub type CORE_DMACTIVE_LPCORE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - hp core dmactive_lpcore value"]
    #[inline(always)]
    pub fn core_dmactive_lpcore(&self) -> CORE_DMACTIVE_LPCORE_R {
        CORE_DMACTIVE_LPCORE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_DMACTIVE_LPCORE")
            .field(
                "core_dmactive_lpcore",
                &format_args!("{}", self.core_dmactive_lpcore().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_DMACTIVE_LPCORE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_dmactive_lpcore::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_DMACTIVE_LPCORE_SPEC;
impl crate::RegisterSpec for CORE_DMACTIVE_LPCORE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_dmactive_lpcore::R`](R) reader structure"]
impl crate::Readable for CORE_DMACTIVE_LPCORE_SPEC {}
#[doc = "`reset()` method sets CORE_DMACTIVE_LPCORE to value 0"]
impl crate::Resettable for CORE_DMACTIVE_LPCORE_SPEC {
    const RESET_VALUE: u32 = 0;
}
