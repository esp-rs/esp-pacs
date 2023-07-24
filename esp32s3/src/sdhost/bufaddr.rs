#[doc = "Register `BUFADDR` reader"]
pub struct R(crate::R<BUFADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUFADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUFADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUFADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUFADDR` reader - Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC."]
pub type BUFADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC."]
    #[inline(always)]
    pub fn bufaddr(&self) -> BUFADDR_R {
        BUFADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFADDR")
            .field("bufaddr", &format_args!("{}", self.bufaddr().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUFADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Host buffer address pointer register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufaddr](index.html) module"]
pub struct BUFADDR_SPEC;
impl crate::RegisterSpec for BUFADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bufaddr::R](R) reader structure"]
impl crate::Readable for BUFADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUFADDR to value 0"]
impl crate::Resettable for BUFADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
