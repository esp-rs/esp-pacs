#[doc = "Register `DEC_STATUS` reader"]
pub struct R(crate::R<DEC_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEC_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEC_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEC_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEC_WARNINGS` reader - the decode result of 3/4 coding scheme has warning"]
pub type DEC_WARNINGS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - the decode result of 3/4 coding scheme has warning"]
    #[inline(always)]
    pub fn dec_warnings(&self) -> DEC_WARNINGS_R {
        DEC_WARNINGS_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEC_STATUS")
            .field(
                "dec_warnings",
                &format_args!("{}", self.dec_warnings().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEC_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dec_status](index.html) module"]
pub struct DEC_STATUS_SPEC;
impl crate::RegisterSpec for DEC_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dec_status::R](R) reader structure"]
impl crate::Readable for DEC_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEC_STATUS to value 0"]
impl crate::Resettable for DEC_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
