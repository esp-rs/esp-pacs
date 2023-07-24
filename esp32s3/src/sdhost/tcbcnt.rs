#[doc = "Register `TCBCNT` reader"]
pub struct R(crate::R<TCBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TCBCNT` reader - Number of bytes transferred by CIU unit to card."]
pub type TCBCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn tcbcnt(&self) -> TCBCNT_R {
        TCBCNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCBCNT")
            .field("tcbcnt", &format_args!("{}", self.tcbcnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TCBCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transferred byte count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcbcnt](index.html) module"]
pub struct TCBCNT_SPEC;
impl crate::RegisterSpec for TCBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcbcnt::R](R) reader structure"]
impl crate::Readable for TCBCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCBCNT to value 0"]
impl crate::Resettable for TCBCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
