#[doc = "Register `SLC_APBWIN_RDATA` reader"]
pub type R = crate::R<SLC_APBWIN_RDATA_SPEC>;
#[doc = "Field `SLC_APBWIN_RDATA` reader - *******Description***********"]
pub type SLC_APBWIN_RDATA_R = crate::FieldReader<u32>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_apbwin_rdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_APBWIN_RDATA_SPEC;
impl crate::RegisterSpec for SLC_APBWIN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_apbwin_rdata::R`](R) reader structure"]
impl crate::Readable for SLC_APBWIN_RDATA_SPEC {}
#[doc = "`reset()` method sets SLC_APBWIN_RDATA to value 0"]
impl crate::Resettable for SLC_APBWIN_RDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
