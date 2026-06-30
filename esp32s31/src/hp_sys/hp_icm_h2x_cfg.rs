#[doc = "Register `HP_ICM_H2X_CFG` reader"]
pub type R = crate::R<HP_ICM_H2X_CFG_SPEC>;
#[doc = "Register `HP_ICM_H2X_CFG` writer"]
pub type W = crate::W<HP_ICM_H2X_CFG_SPEC>;
#[doc = "Field `HP_CPU_ICM_H2X_POST_WR_EN` reader - need_des"]
pub type HP_CPU_ICM_H2X_POST_WR_EN_R = crate::BitReader;
#[doc = "Field `HP_CPU_ICM_H2X_POST_WR_EN` writer - need_des"]
pub type HP_CPU_ICM_H2X_POST_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPU_ICM_H2X_CUT_THROUGH_EN` reader - need_des"]
pub type HP_CPU_ICM_H2X_CUT_THROUGH_EN_R = crate::BitReader;
#[doc = "Field `HP_CPU_ICM_H2X_CUT_THROUGH_EN` writer - need_des"]
pub type HP_CPU_ICM_H2X_CUT_THROUGH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPU_ICM_H2X_BRIDGE_BUSY` reader - need_des"]
pub type HP_CPU_ICM_H2X_BRIDGE_BUSY_R = crate::BitReader;
#[doc = "Field `HP_SDMMC_ICM_H2X_POST_WR_EN` reader - need_des"]
pub type HP_SDMMC_ICM_H2X_POST_WR_EN_R = crate::BitReader;
#[doc = "Field `HP_SDMMC_ICM_H2X_POST_WR_EN` writer - need_des"]
pub type HP_SDMMC_ICM_H2X_POST_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SDMMC_ICM_H2X_CUT_THROUGH_EN` reader - need_des"]
pub type HP_SDMMC_ICM_H2X_CUT_THROUGH_EN_R = crate::BitReader;
#[doc = "Field `HP_SDMMC_ICM_H2X_CUT_THROUGH_EN` writer - need_des"]
pub type HP_SDMMC_ICM_H2X_CUT_THROUGH_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SDMMC_ICM_H2X_BRIDGE_BUSY` reader - need_des"]
pub type HP_SDMMC_ICM_H2X_BRIDGE_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_post_wr_en(&self) -> HP_CPU_ICM_H2X_POST_WR_EN_R {
        HP_CPU_ICM_H2X_POST_WR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_cut_through_en(&self) -> HP_CPU_ICM_H2X_CUT_THROUGH_EN_R {
        HP_CPU_ICM_H2X_CUT_THROUGH_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_bridge_busy(&self) -> HP_CPU_ICM_H2X_BRIDGE_BUSY_R {
        HP_CPU_ICM_H2X_BRIDGE_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_post_wr_en(&self) -> HP_SDMMC_ICM_H2X_POST_WR_EN_R {
        HP_SDMMC_ICM_H2X_POST_WR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_cut_through_en(&self) -> HP_SDMMC_ICM_H2X_CUT_THROUGH_EN_R {
        HP_SDMMC_ICM_H2X_CUT_THROUGH_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_bridge_busy(&self) -> HP_SDMMC_ICM_H2X_BRIDGE_BUSY_R {
        HP_SDMMC_ICM_H2X_BRIDGE_BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ICM_H2X_CFG")
            .field(
                "hp_cpu_icm_h2x_post_wr_en",
                &self.hp_cpu_icm_h2x_post_wr_en(),
            )
            .field(
                "hp_cpu_icm_h2x_cut_through_en",
                &self.hp_cpu_icm_h2x_cut_through_en(),
            )
            .field(
                "hp_cpu_icm_h2x_bridge_busy",
                &self.hp_cpu_icm_h2x_bridge_busy(),
            )
            .field(
                "hp_sdmmc_icm_h2x_post_wr_en",
                &self.hp_sdmmc_icm_h2x_post_wr_en(),
            )
            .field(
                "hp_sdmmc_icm_h2x_cut_through_en",
                &self.hp_sdmmc_icm_h2x_cut_through_en(),
            )
            .field(
                "hp_sdmmc_icm_h2x_bridge_busy",
                &self.hp_sdmmc_icm_h2x_bridge_busy(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_post_wr_en(
        &mut self,
    ) -> HP_CPU_ICM_H2X_POST_WR_EN_W<'_, HP_ICM_H2X_CFG_SPEC> {
        HP_CPU_ICM_H2X_POST_WR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn hp_cpu_icm_h2x_cut_through_en(
        &mut self,
    ) -> HP_CPU_ICM_H2X_CUT_THROUGH_EN_W<'_, HP_ICM_H2X_CFG_SPEC> {
        HP_CPU_ICM_H2X_CUT_THROUGH_EN_W::new(self, 1)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_post_wr_en(
        &mut self,
    ) -> HP_SDMMC_ICM_H2X_POST_WR_EN_W<'_, HP_ICM_H2X_CFG_SPEC> {
        HP_SDMMC_ICM_H2X_POST_WR_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn hp_sdmmc_icm_h2x_cut_through_en(
        &mut self,
    ) -> HP_SDMMC_ICM_H2X_CUT_THROUGH_EN_W<'_, HP_ICM_H2X_CFG_SPEC> {
        HP_SDMMC_ICM_H2X_CUT_THROUGH_EN_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_icm_h2x_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_icm_h2x_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ICM_H2X_CFG_SPEC;
impl crate::RegisterSpec for HP_ICM_H2X_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_icm_h2x_cfg::R`](R) reader structure"]
impl crate::Readable for HP_ICM_H2X_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_icm_h2x_cfg::W`](W) writer structure"]
impl crate::Writable for HP_ICM_H2X_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ICM_H2X_CFG to value 0x12"]
impl crate::Resettable for HP_ICM_H2X_CFG_SPEC {
    const RESET_VALUE: u32 = 0x12;
}
