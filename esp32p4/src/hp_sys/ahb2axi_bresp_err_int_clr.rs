#[doc = "Register `AHB2AXI_BRESP_ERR_INT_CLR` writer"]
pub type W = crate::W<AHB2AXI_BRESP_ERR_INT_CLR_SPEC>;
#[doc = "Field `CPU_ICM_H2X_BRESP_ERR_INT_CLR` writer - Write 1 to clear cpu_icm_h2x_bresp_err int"]
pub type CPU_ICM_H2X_BRESP_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AHB2AXI_BRESP_ERR_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - Write 1 to clear cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_icm_h2x_bresp_err_int_clr(
        &mut self,
    ) -> CPU_ICM_H2X_BRESP_ERR_INT_CLR_W<AHB2AXI_BRESP_ERR_INT_CLR_SPEC> {
        CPU_ICM_H2X_BRESP_ERR_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2axi_bresp_err_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB2AXI_BRESP_ERR_INT_CLR_SPEC;
impl crate::RegisterSpec for AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ahb2axi_bresp_err_int_clr::W`](W) writer structure"]
impl crate::Writable for AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB2AXI_BRESP_ERR_INT_CLR to value 0"]
impl crate::Resettable for AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
