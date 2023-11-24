#[doc = "Register `DSCADDR` reader"]
pub type R = crate::R<DSCADDR_SPEC>;
#[doc = "Field `DSCADDR` reader - Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC."]
pub type DSCADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Descriptor Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the start address of the current descriptor read by the IDMAC."]
    #[inline(always)]
    pub fn dscaddr(&self) -> DSCADDR_R {
        DSCADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSCADDR")
            .field("dscaddr", &format_args!("{}", self.dscaddr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DSCADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Host descriptor address pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscaddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSCADDR_SPEC;
impl crate::RegisterSpec for DSCADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dscaddr::R`](R) reader structure"]
impl crate::Readable for DSCADDR_SPEC {}
#[doc = "`reset()` method sets DSCADDR to value 0"]
impl crate::Resettable for DSCADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
