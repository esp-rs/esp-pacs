#[doc = "Register `AHBINF_RESP_ERR_STATUS0` reader"]
pub type R = crate::R<AHBINF_RESP_ERR_STATUS0_SPEC>;
#[doc = "Field `AHBINF_RESP_ERR_ADDR` reader - Represents the address of the AHB response error."]
pub type AHBINF_RESP_ERR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the address of the AHB response error."]
    #[inline(always)]
    pub fn ahbinf_resp_err_addr(&self) -> AHBINF_RESP_ERR_ADDR_R {
        AHBINF_RESP_ERR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBINF_RESP_ERR_STATUS0")
            .field("ahbinf_resp_err_addr", &self.ahbinf_resp_err_addr())
            .finish()
    }
}
#[doc = "AHB response error status 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbinf_resp_err_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBINF_RESP_ERR_STATUS0_SPEC;
impl crate::RegisterSpec for AHBINF_RESP_ERR_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbinf_resp_err_status0::R`](R) reader structure"]
impl crate::Readable for AHBINF_RESP_ERR_STATUS0_SPEC {}
#[doc = "`reset()` method sets AHBINF_RESP_ERR_STATUS0 to value 0"]
impl crate::Resettable for AHBINF_RESP_ERR_STATUS0_SPEC {}
