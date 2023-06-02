#[doc = "Register `SLC_APBWIN_RDATA` reader"]
pub struct R(crate::R<SLC_APBWIN_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_APBWIN_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_APBWIN_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_APBWIN_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC_APBWIN_RDATA` reader - *******Description***********"]
pub type SLC_APBWIN_RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc_apbwin_rdata(&self) -> SLC_APBWIN_RDATA_R {
        SLC_APBWIN_RDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_APBWIN_RDATA")
            .field(
                "slc_apbwin_rdata",
                &format_args!("{}", self.slc_apbwin_rdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_APBWIN_RDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_apbwin_rdata](index.html) module"]
pub struct SLC_APBWIN_RDATA_SPEC;
impl crate::RegisterSpec for SLC_APBWIN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_apbwin_rdata::R](R) reader structure"]
impl crate::Readable for SLC_APBWIN_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLC_APBWIN_RDATA to value 0"]
impl crate::Resettable for SLC_APBWIN_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
