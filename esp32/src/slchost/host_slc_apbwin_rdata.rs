#[doc = "Register `HOST_SLC_APBWIN_RDATA` reader"]
pub struct R(crate::R<HOST_SLC_APBWIN_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC_APBWIN_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC_APBWIN_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC_APBWIN_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_SLC_APBWIN_RDATA` reader - "]
pub type HOST_SLC_APBWIN_RDATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc_apbwin_rdata(&self) -> HOST_SLC_APBWIN_RDATA_R {
        HOST_SLC_APBWIN_RDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC_APBWIN_RDATA")
            .field(
                "host_slc_apbwin_rdata",
                &format_args!("{}", self.host_slc_apbwin_rdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC_APBWIN_RDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc_apbwin_rdata](index.html) module"]
pub struct HOST_SLC_APBWIN_RDATA_SPEC;
impl crate::RegisterSpec for HOST_SLC_APBWIN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc_apbwin_rdata::R](R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLC_APBWIN_RDATA to value 0"]
impl crate::Resettable for HOST_SLC_APBWIN_RDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
