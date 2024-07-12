#[doc = "Register `AHB2AXI_BRESP_ERR_INT_ST` reader"]
pub type R = crate::R<AHB2AXI_BRESP_ERR_INT_ST_SPEC>;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_ST` reader - the masked interrupt status of cpu_icm_h2x_bresp_err"]
pub type CPU_ICM_H2X_BRESP_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 31 - the masked interrupt status of cpu_icm_h2x_bresp_err"]
    #[inline(always)]
    pub fn cpu_icm_h2x_bresp_err_int_st(&self) -> CPU_ICM_H2X_BRESP_ERR_INT_ST_R {
        CPU_ICM_H2X_BRESP_ERR_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB2AXI_BRESP_ERR_INT_ST")
            .field(
                "cpu_icm_h2x_bresp_err_int_st",
                &self.cpu_icm_h2x_bresp_err_int_st(),
            )
            .finish()
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2axi_bresp_err_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2AXI_BRESP_ERR_INT_ST_SPEC;
impl crate::RegisterSpec for AHB2AXI_BRESP_ERR_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2axi_bresp_err_int_st::R`](R) reader structure"]
impl crate::Readable for AHB2AXI_BRESP_ERR_INT_ST_SPEC {}
#[doc = "`reset()` method sets AHB2AXI_BRESP_ERR_INT_ST to value 0"]
impl crate::Resettable for AHB2AXI_BRESP_ERR_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
