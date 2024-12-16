#[doc = "Register `LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL` reader"]
pub type R = crate::R<LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC>;
#[doc = "Register `LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL` writer"]
pub type W = crate::W<LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC>;
#[doc = "Field `LP_AONCLKRST_RST_EN_SDMMC` reader - hp sdmmc reset en"]
pub type LP_AONCLKRST_RST_EN_SDMMC_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_SDMMC` writer - hp sdmmc reset en"]
pub type LP_AONCLKRST_RST_EN_SDMMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_SDMMC` reader - hp sdmmc force norst"]
pub type LP_AONCLKRST_FORCE_NORST_SDMMC_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_SDMMC` writer - hp sdmmc force norst"]
pub type LP_AONCLKRST_FORCE_NORST_SDMMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_EMAC` reader - hp emac reset en"]
pub type LP_AONCLKRST_RST_EN_EMAC_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_EMAC` writer - hp emac reset en"]
pub type LP_AONCLKRST_RST_EN_EMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_EMAC` reader - hp emac force norst"]
pub type LP_AONCLKRST_FORCE_NORST_EMAC_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_FORCE_NORST_EMAC` writer - hp emac force norst"]
pub type LP_AONCLKRST_FORCE_NORST_EMAC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - hp sdmmc reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_sdmmc(&self) -> LP_AONCLKRST_RST_EN_SDMMC_R {
        LP_AONCLKRST_RST_EN_SDMMC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - hp sdmmc force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_sdmmc(&self) -> LP_AONCLKRST_FORCE_NORST_SDMMC_R {
        LP_AONCLKRST_FORCE_NORST_SDMMC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - hp emac reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_emac(&self) -> LP_AONCLKRST_RST_EN_EMAC_R {
        LP_AONCLKRST_RST_EN_EMAC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - hp emac force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_emac(&self) -> LP_AONCLKRST_FORCE_NORST_EMAC_R {
        LP_AONCLKRST_FORCE_NORST_EMAC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL")
            .field(
                "lp_aonclkrst_rst_en_sdmmc",
                &self.lp_aonclkrst_rst_en_sdmmc(),
            )
            .field(
                "lp_aonclkrst_force_norst_sdmmc",
                &self.lp_aonclkrst_force_norst_sdmmc(),
            )
            .field("lp_aonclkrst_rst_en_emac", &self.lp_aonclkrst_rst_en_emac())
            .field(
                "lp_aonclkrst_force_norst_emac",
                &self.lp_aonclkrst_force_norst_emac(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 28 - hp sdmmc reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_sdmmc(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_SDMMC_W<LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC> {
        LP_AONCLKRST_RST_EN_SDMMC_W::new(self, 28)
    }
    #[doc = "Bit 29 - hp sdmmc force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_sdmmc(
        &mut self,
    ) -> LP_AONCLKRST_FORCE_NORST_SDMMC_W<LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC> {
        LP_AONCLKRST_FORCE_NORST_SDMMC_W::new(self, 29)
    }
    #[doc = "Bit 30 - hp emac reset en"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_emac(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_EMAC_W<LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC> {
        LP_AONCLKRST_RST_EN_EMAC_W::new(self, 30)
    }
    #[doc = "Bit 31 - hp emac force norst"]
    #[inline(always)]
    pub fn lp_aonclkrst_force_norst_emac(
        &mut self,
    ) -> LP_AONCLKRST_FORCE_NORST_EMAC_W<LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC> {
        LP_AONCLKRST_FORCE_NORST_EMAC_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hp_sdmmc_emac_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL to value 0"]
impl crate::Resettable for LP_AONCLKRST_HP_SDMMC_EMAC_RST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
