#[doc = "Register `HOST_SLC1_HOST_PF` reader"]
pub type R = crate::R<HOST_SLC1_HOST_PF_SPEC>;
#[doc = "Field `HOST_SLC1_PF_DATA` reader - "]
pub type HOST_SLC1_PF_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slc1_pf_data(&self) -> HOST_SLC1_PF_DATA_R {
        HOST_SLC1_PF_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC1_HOST_PF")
            .field("host_slc1_pf_data", &self.host_slc1_pf_data())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slc1_host_pf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC1_HOST_PF_SPEC;
impl crate::RegisterSpec for HOST_SLC1_HOST_PF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slc1_host_pf::R`](R) reader structure"]
impl crate::Readable for HOST_SLC1_HOST_PF_SPEC {}
#[doc = "`reset()` method sets HOST_SLC1_HOST_PF to value 0"]
impl crate::Resettable for HOST_SLC1_HOST_PF_SPEC {
    const RESET_VALUE: u32 = 0;
}
