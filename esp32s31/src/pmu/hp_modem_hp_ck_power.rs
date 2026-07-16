#[doc = "Register `HP_MODEM_HP_CK_POWER` reader"]
pub type R = crate::R<HP_MODEM_HP_CK_POWER_SPEC>;
#[doc = "Register `HP_MODEM_HP_CK_POWER` writer"]
pub type W = crate::W<HP_MODEM_HP_CK_POWER_SPEC>;
#[doc = "Field `HP_MODEM_XPD_XTALX2` reader - need_des"]
pub type HP_MODEM_XPD_XTALX2_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_XTALX2` writer - need_des"]
pub type HP_MODEM_XPD_XTALX2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_I2C_ISO_EN` reader - need_des"]
pub type HP_MODEM_I2C_ISO_EN_R = crate::BitReader;
#[doc = "Field `HP_MODEM_I2C_ISO_EN` writer - need_des"]
pub type HP_MODEM_I2C_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_I2C_RETENTION` reader - need_des"]
pub type HP_MODEM_I2C_RETENTION_R = crate::BitReader;
#[doc = "Field `HP_MODEM_I2C_RETENTION` writer - need_des"]
pub type HP_MODEM_I2C_RETENTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_BB_I2C` reader - need_des"]
pub type HP_MODEM_XPD_BB_I2C_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_BB_I2C` writer - need_des"]
pub type HP_MODEM_XPD_BB_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_PLL_I2C` reader - need_des"]
pub type HP_MODEM_XPD_PLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_PLL_I2C` writer - need_des"]
pub type HP_MODEM_XPD_PLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_BBPLL_I2C` reader - "]
pub type HP_MODEM_XPD_BBPLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_BBPLL_I2C` writer - "]
pub type HP_MODEM_XPD_BBPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_APLL_I2C` reader - "]
pub type HP_MODEM_XPD_APLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_APLL_I2C` writer - "]
pub type HP_MODEM_XPD_APLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_MPLL_I2C` reader - "]
pub type HP_MODEM_XPD_MPLL_I2C_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_MPLL_I2C` writer - "]
pub type HP_MODEM_XPD_MPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_PLL` reader - need_des"]
pub type HP_MODEM_XPD_PLL_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_PLL` writer - need_des"]
pub type HP_MODEM_XPD_PLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_BBPLL` reader - "]
pub type HP_MODEM_XPD_BBPLL_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_BBPLL` writer - "]
pub type HP_MODEM_XPD_BBPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_APLL` reader - "]
pub type HP_MODEM_XPD_APLL_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_APLL` writer - "]
pub type HP_MODEM_XPD_APLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_XPD_MPLL` reader - "]
pub type HP_MODEM_XPD_MPLL_R = crate::BitReader;
#[doc = "Field `HP_MODEM_XPD_MPLL` writer - "]
pub type HP_MODEM_XPD_MPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_xtalx2(&self) -> HP_MODEM_XPD_XTALX2_R {
        HP_MODEM_XPD_XTALX2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn hp_modem_i2c_iso_en(&self) -> HP_MODEM_I2C_ISO_EN_R {
        HP_MODEM_I2C_ISO_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_modem_i2c_retention(&self) -> HP_MODEM_I2C_RETENTION_R {
        HP_MODEM_I2C_RETENTION_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_bb_i2c(&self) -> HP_MODEM_XPD_BB_I2C_R {
        HP_MODEM_XPD_BB_I2C_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_pll_i2c(&self) -> HP_MODEM_XPD_PLL_I2C_R {
        HP_MODEM_XPD_PLL_I2C_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn hp_modem_xpd_bbpll_i2c(&self) -> HP_MODEM_XPD_BBPLL_I2C_R {
        HP_MODEM_XPD_BBPLL_I2C_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hp_modem_xpd_apll_i2c(&self) -> HP_MODEM_XPD_APLL_I2C_R {
        HP_MODEM_XPD_APLL_I2C_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn hp_modem_xpd_mpll_i2c(&self) -> HP_MODEM_XPD_MPLL_I2C_R {
        HP_MODEM_XPD_MPLL_I2C_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_pll(&self) -> HP_MODEM_XPD_PLL_R {
        HP_MODEM_XPD_PLL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn hp_modem_xpd_bbpll(&self) -> HP_MODEM_XPD_BBPLL_R {
        HP_MODEM_XPD_BBPLL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn hp_modem_xpd_apll(&self) -> HP_MODEM_XPD_APLL_R {
        HP_MODEM_XPD_APLL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn hp_modem_xpd_mpll(&self) -> HP_MODEM_XPD_MPLL_R {
        HP_MODEM_XPD_MPLL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_HP_CK_POWER")
            .field("hp_modem_xpd_xtalx2", &self.hp_modem_xpd_xtalx2())
            .field("hp_modem_i2c_iso_en", &self.hp_modem_i2c_iso_en())
            .field("hp_modem_i2c_retention", &self.hp_modem_i2c_retention())
            .field("hp_modem_xpd_bb_i2c", &self.hp_modem_xpd_bb_i2c())
            .field("hp_modem_xpd_pll_i2c", &self.hp_modem_xpd_pll_i2c())
            .field("hp_modem_xpd_pll", &self.hp_modem_xpd_pll())
            .field("hp_modem_xpd_bbpll_i2c", &self.hp_modem_xpd_bbpll_i2c())
            .field("hp_modem_xpd_apll_i2c", &self.hp_modem_xpd_apll_i2c())
            .field("hp_modem_xpd_mpll_i2c", &self.hp_modem_xpd_mpll_i2c())
            .field("hp_modem_xpd_bbpll", &self.hp_modem_xpd_bbpll())
            .field("hp_modem_xpd_apll", &self.hp_modem_xpd_apll())
            .field("hp_modem_xpd_mpll", &self.hp_modem_xpd_mpll())
            .finish()
    }
}
impl W {
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_xtalx2(&mut self) -> HP_MODEM_XPD_XTALX2_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_XTALX2_W::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn hp_modem_i2c_iso_en(&mut self) -> HP_MODEM_I2C_ISO_EN_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_I2C_ISO_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_modem_i2c_retention(
        &mut self,
    ) -> HP_MODEM_I2C_RETENTION_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_I2C_RETENTION_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_bb_i2c(&mut self) -> HP_MODEM_XPD_BB_I2C_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_BB_I2C_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_pll_i2c(
        &mut self,
    ) -> HP_MODEM_XPD_PLL_I2C_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_PLL_I2C_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn hp_modem_xpd_bbpll_i2c(
        &mut self,
    ) -> HP_MODEM_XPD_BBPLL_I2C_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_BBPLL_I2C_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hp_modem_xpd_apll_i2c(
        &mut self,
    ) -> HP_MODEM_XPD_APLL_I2C_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_APLL_I2C_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn hp_modem_xpd_mpll_i2c(
        &mut self,
    ) -> HP_MODEM_XPD_MPLL_I2C_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_MPLL_I2C_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_pll(&mut self) -> HP_MODEM_XPD_PLL_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_PLL_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn hp_modem_xpd_bbpll(&mut self) -> HP_MODEM_XPD_BBPLL_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_BBPLL_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn hp_modem_xpd_apll(&mut self) -> HP_MODEM_XPD_APLL_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_APLL_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn hp_modem_xpd_mpll(&mut self) -> HP_MODEM_XPD_MPLL_W<'_, HP_MODEM_HP_CK_POWER_SPEC> {
        HP_MODEM_XPD_MPLL_W::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_modem_hp_ck_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_ck_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_HP_CK_POWER_SPEC;
impl crate::RegisterSpec for HP_MODEM_HP_CK_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_hp_ck_power::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_HP_CK_POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_hp_ck_power::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_HP_CK_POWER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_HP_CK_POWER to value 0"]
impl crate::Resettable for HP_MODEM_HP_CK_POWER_SPEC {}
