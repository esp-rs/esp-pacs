#[doc = "Register `SDPRF_CTRL` reader"]
pub type R = crate::R<SDPRF_CTRL_SPEC>;
#[doc = "Field `SDPRF_MEM_AUX_CTRL` reader - reserved"]
pub type SDPRF_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn sdprf_mem_aux_ctrl(&self) -> SDPRF_MEM_AUX_CTRL_R {
        SDPRF_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDPRF_CTRL")
            .field("sdprf_mem_aux_ctrl", &self.sdprf_mem_aux_ctrl())
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sdprf_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDPRF_CTRL_SPEC;
impl crate::RegisterSpec for SDPRF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdprf_ctrl::R`](R) reader structure"]
impl crate::Readable for SDPRF_CTRL_SPEC {}
#[doc = "`reset()` method sets SDPRF_CTRL to value 0"]
impl crate::Resettable for SDPRF_CTRL_SPEC {}
