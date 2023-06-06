#[doc = "Register `SLC0_HOST_PF` reader"]
pub struct R(crate::R<SLC0_HOST_PF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC0_HOST_PF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC0_HOST_PF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC0_HOST_PF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SLC0_PF_DATA` reader - *******Description***********"]
pub type SLC0_PF_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc0_pf_data(&self) -> SLC0_PF_DATA_R {
        SLC0_PF_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0_HOST_PF")
            .field(
                "slc0_pf_data",
                &format_args!("{}", self.slc0_pf_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0_HOST_PF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc0_host_pf](index.html) module"]
pub struct SLC0_HOST_PF_SPEC;
impl crate::RegisterSpec for SLC0_HOST_PF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc0_host_pf::R](R) reader structure"]
impl crate::Readable for SLC0_HOST_PF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLC0_HOST_PF to value 0"]
impl crate::Resettable for SLC0_HOST_PF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
