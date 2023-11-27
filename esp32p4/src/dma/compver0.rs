#[doc = "Register `COMPVER0` reader"]
pub type R = crate::R<COMPVER0_SPEC>;
#[doc = "Field `DMAC_COMPVER` reader - NA"]
pub type DMAC_COMPVER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn dmac_compver(&self) -> DMAC_COMPVER_R {
        DMAC_COMPVER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMPVER0")
            .field(
                "dmac_compver",
                &format_args!("{}", self.dmac_compver().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMPVER0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compver0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMPVER0_SPEC;
impl crate::RegisterSpec for COMPVER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compver0::R`](R) reader structure"]
impl crate::Readable for COMPVER0_SPEC {}
#[doc = "`reset()` method sets COMPVER0 to value 0x3230_302a"]
impl crate::Resettable for COMPVER0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3230_302a;
}
