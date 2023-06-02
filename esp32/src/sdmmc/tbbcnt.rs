#[doc = "Register `TBBCNT` reader"]
pub struct R(crate::R<TBBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TBBCNT` reader - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
pub type TBBCNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub fn tbbcnt(&self) -> TBBCNT_R {
        TBBCNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBBCNT")
            .field("tbbcnt", &format_args!("{}", self.tbbcnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TBBCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Transferred byte count register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbbcnt](index.html) module"]
pub struct TBBCNT_SPEC;
impl crate::RegisterSpec for TBBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbbcnt::R](R) reader structure"]
impl crate::Readable for TBBCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBBCNT to value 0"]
impl crate::Resettable for TBBCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
