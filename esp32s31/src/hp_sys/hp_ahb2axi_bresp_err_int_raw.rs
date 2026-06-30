#[doc = "Register `HP_AHB2AXI_BRESP_ERR_INT_RAW` reader"]
pub type R = crate::R<HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC>;
#[doc = "Register `HP_AHB2AXI_BRESP_ERR_INT_RAW` writer"]
pub type W = crate::W<HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC>;
#[doc = "Field `HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW` reader - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
pub type HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW` writer - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
pub type HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW` reader - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
pub type HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW` writer - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
pub type HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_bresp_err_int_raw(&self) -> HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW_R {
        HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_bresp_err_int_raw(&self) -> HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW_R {
        HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_AHB2AXI_BRESP_ERR_INT_RAW")
            .field(
                "hp_sdmmc_icm_h2x_bresp_err_int_raw",
                &self.hp_sdmmc_icm_h2x_bresp_err_int_raw(),
            )
            .field(
                "hp_cpu_icm_h2x_bresp_err_int_raw",
                &self.hp_cpu_icm_h2x_bresp_err_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_bresp_err_int_raw(
        &mut self,
    ) -> HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW_W<'_, HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC> {
        HP_SDMMC_ICM_H2X_BRESP_ERR_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - the raw interrupt status of bresp error, triggered when if bresp err occurs in post write mode in ahb2axi."]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_bresp_err_int_raw(
        &mut self,
    ) -> HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW_W<'_, HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC> {
        HP_CPU_ICM_H2X_BRESP_ERR_INT_RAW_W::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ahb2axi_bresp_err_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ahb2axi_bresp_err_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC;
impl crate::RegisterSpec for HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_ahb2axi_bresp_err_int_raw::R`](R) reader structure"]
impl crate::Readable for HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_ahb2axi_bresp_err_int_raw::W`](W) writer structure"]
impl crate::Writable for HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_AHB2AXI_BRESP_ERR_INT_RAW to value 0"]
impl crate::Resettable for HP_AHB2AXI_BRESP_ERR_INT_RAW_SPEC {}
