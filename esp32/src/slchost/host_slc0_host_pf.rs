#[doc = "Register `HOST_SLC0_HOST_PF` reader"]
pub struct R(crate::R<HOST_SLC0_HOST_PF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC0_HOST_PF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC0_HOST_PF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC0_HOST_PF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOST_SLC0_PF_DATA` reader - "]
pub type HOST_SLC0_PF_DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc0_pf_data(&self) -> HOST_SLC0_PF_DATA_R {
        HOST_SLC0_PF_DATA_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc0_host_pf](index.html) module"]
pub struct HOST_SLC0_HOST_PF_SPEC;
impl crate::RegisterSpec for HOST_SLC0_HOST_PF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc0_host_pf::R](R) reader structure"]
impl crate::Readable for HOST_SLC0_HOST_PF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLC0_HOST_PF to value 0"]
impl crate::Resettable for HOST_SLC0_HOST_PF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
