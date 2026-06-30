#[doc = "Register `HP_AHB2AXI_BRESP_ERR_INT_ENA` reader"]
pub type R = crate::R<HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
#[doc = "Register `HP_AHB2AXI_BRESP_ERR_INT_ENA` writer"]
pub type W = crate::W<HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC>;
#[doc = "Field `HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA` reader - Write 1 to enable sdmmc_icm_h2x_bresp_err int"]
pub type HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA` writer - Write 1 to enable sdmmc_icm_h2x_bresp_err int"]
pub type HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA` reader - Write 1 to enable cpu_icm_h2x_bresp_err int"]
pub type HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA` writer - Write 1 to enable cpu_icm_h2x_bresp_err int"]
pub type HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Write 1 to enable sdmmc_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_bresp_err_int_ena(&self) -> HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA_R {
        HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_bresp_err_int_ena(&self) -> HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA_R {
        HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_AHB2AXI_BRESP_ERR_INT_ENA")
            .field(
                "hp_sdmmc_icm_h2x_bresp_err_int_ena",
                &self.hp_sdmmc_icm_h2x_bresp_err_int_ena(),
            )
            .field(
                "hp_cpu_icm_h2x_bresp_err_int_ena",
                &self.hp_cpu_icm_h2x_bresp_err_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Write 1 to enable sdmmc_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_bresp_err_int_ena(
        &mut self,
    ) -> HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA_W<'_, HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC> {
        HP_SDMMC_ICM_H2X_BRESP_ERR_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Write 1 to enable cpu_icm_h2x_bresp_err int"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_bresp_err_int_ena(
        &mut self,
    ) -> HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA_W<'_, HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC> {
        HP_CPU_ICM_H2X_BRESP_ERR_INT_ENA_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC;
impl crate::RegisterSpec for HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_ahb2axi_bresp_err_int_ena::R`](R) reader structure"]
impl crate::Readable for HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_ahb2axi_bresp_err_int_ena::W`](W) writer structure"]
impl crate::Writable for HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_AHB2AXI_BRESP_ERR_INT_ENA to value 0"]
impl crate::Resettable for HP_AHB2AXI_BRESP_ERR_INT_ENA_SPEC {}
