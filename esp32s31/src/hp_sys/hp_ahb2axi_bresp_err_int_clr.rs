#[doc = "Register `HP_AHB2AXI_BRESP_ERR_INT_CLR` writer"]
pub type W = crate::W<HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC>;
#[doc = "Field `HP_SDMMC_ICM_H2X_BRESP_ERR_INT_CLR` writer - Write 1 to clear sdmmc_icm_h2x_bresp_err int"]
pub type HP_SDMMC_ICM_H2X_BRESP_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR` writer - Write 1 to clear cpu_icm_h2x_bresp_err int"]
pub type HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 30 - Write 1 to clear sdmmc_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_bresp_err_int_clr(
        &mut self,
    ) -> HP_SDMMC_ICM_H2X_BRESP_ERR_INT_CLR_W<'_, HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC> {
        HP_SDMMC_ICM_H2X_BRESP_ERR_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Write 1 to clear cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_bresp_err_int_clr(
        &mut self,
    ) -> HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR_W<'_, HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC> {
        HP_CPU_ICM_H2X_BRESP_ERR_INT_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC;
impl crate::RegisterSpec for HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_ahb2axi_bresp_err_int_clr::W`](W) writer structure"]
impl crate::Writable for HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_AHB2AXI_BRESP_ERR_INT_CLR to value 0"]
impl crate::Resettable for HP_AHB2AXI_BRESP_ERR_INT_CLR_SPEC {}
