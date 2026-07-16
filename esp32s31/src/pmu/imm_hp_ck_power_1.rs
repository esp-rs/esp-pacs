#[doc = "Register `IMM_HP_CK_POWER_1` reader"]
pub type R = crate::R<IMM_HP_CK_POWER_1_SPEC>;
#[doc = "Register `IMM_HP_CK_POWER_1` writer"]
pub type W = crate::W<IMM_HP_CK_POWER_1_SPEC>;
#[doc = "Field `TIE_LOW_XPD_PLL_I2C` writer - need_des"]
pub type TIE_LOW_XPD_PLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_BBPLL_I2C` reader - "]
pub type TIE_LOW_XPD_BBPLL_I2C_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_BBPLL_I2C` writer - "]
pub type TIE_LOW_XPD_BBPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_APLL_I2C` reader - "]
pub type TIE_LOW_XPD_APLL_I2C_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_APLL_I2C` writer - "]
pub type TIE_LOW_XPD_APLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_MPLL_I2C` reader - "]
pub type TIE_LOW_XPD_MPLL_I2C_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_MPLL_I2C` writer - "]
pub type TIE_LOW_XPD_MPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_PLL` writer - need_des"]
pub type TIE_LOW_XPD_PLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_BBPLL` reader - "]
pub type TIE_LOW_XPD_BBPLL_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_BBPLL` writer - "]
pub type TIE_LOW_XPD_BBPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_APLL` reader - "]
pub type TIE_LOW_XPD_APLL_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_APLL` writer - "]
pub type TIE_LOW_XPD_APLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_XPD_MPLL` reader - "]
pub type TIE_LOW_XPD_MPLL_R = crate::BitReader;
#[doc = "Field `TIE_LOW_XPD_MPLL` writer - "]
pub type TIE_LOW_XPD_MPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_GLOBAL_PLL_ICG` writer - need_des"]
pub type TIE_LOW_GLOBAL_PLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_GLOBAL_BBPLL_ICG` reader - "]
pub type TIE_LOW_GLOBAL_BBPLL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_LOW_GLOBAL_BBPLL_ICG` writer - "]
pub type TIE_LOW_GLOBAL_BBPLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_GLOBAL_APLL_ICG` reader - "]
pub type TIE_LOW_GLOBAL_APLL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_LOW_GLOBAL_APLL_ICG` writer - "]
pub type TIE_LOW_GLOBAL_APLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_LOW_GLOBAL_MPLL_ICG` reader - "]
pub type TIE_LOW_GLOBAL_MPLL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_LOW_GLOBAL_MPLL_ICG` writer - "]
pub type TIE_LOW_GLOBAL_MPLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_GLOBAL_PLL_ICG` writer - need_des"]
pub type TIE_HIGH_GLOBAL_PLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_GLOBAL_BBPLL_ICG` reader - "]
pub type TIE_HIGH_GLOBAL_BBPLL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_GLOBAL_BBPLL_ICG` writer - "]
pub type TIE_HIGH_GLOBAL_BBPLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_GLOBAL_APLL_ICG` reader - "]
pub type TIE_HIGH_GLOBAL_APLL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_GLOBAL_APLL_ICG` writer - "]
pub type TIE_HIGH_GLOBAL_APLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_GLOBAL_MPLL_ICG` reader - "]
pub type TIE_HIGH_GLOBAL_MPLL_ICG_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_GLOBAL_MPLL_ICG` writer - "]
pub type TIE_HIGH_GLOBAL_MPLL_ICG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_PLL_I2C` writer - need_des"]
pub type TIE_HIGH_XPD_PLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_BBPLL_I2C` reader - "]
pub type TIE_HIGH_XPD_BBPLL_I2C_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_BBPLL_I2C` writer - "]
pub type TIE_HIGH_XPD_BBPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_APLL_I2C` reader - "]
pub type TIE_HIGH_XPD_APLL_I2C_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_APLL_I2C` writer - "]
pub type TIE_HIGH_XPD_APLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_MPLL_I2C` reader - "]
pub type TIE_HIGH_XPD_MPLL_I2C_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_MPLL_I2C` writer - "]
pub type TIE_HIGH_XPD_MPLL_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_PLL` writer - need_des"]
pub type TIE_HIGH_XPD_PLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_BBPLL` reader - "]
pub type TIE_HIGH_XPD_BBPLL_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_BBPLL` writer - "]
pub type TIE_HIGH_XPD_BBPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_APLL` reader - "]
pub type TIE_HIGH_XPD_APLL_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_APLL` writer - "]
pub type TIE_HIGH_XPD_APLL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_HIGH_XPD_MPLL` reader - "]
pub type TIE_HIGH_XPD_MPLL_R = crate::BitReader;
#[doc = "Field `TIE_HIGH_XPD_MPLL` writer - "]
pub type TIE_HIGH_XPD_MPLL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tie_low_xpd_bbpll_i2c(&self) -> TIE_LOW_XPD_BBPLL_I2C_R {
        TIE_LOW_XPD_BBPLL_I2C_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tie_low_xpd_apll_i2c(&self) -> TIE_LOW_XPD_APLL_I2C_R {
        TIE_LOW_XPD_APLL_I2C_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tie_low_xpd_mpll_i2c(&self) -> TIE_LOW_XPD_MPLL_I2C_R {
        TIE_LOW_XPD_MPLL_I2C_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tie_low_xpd_bbpll(&self) -> TIE_LOW_XPD_BBPLL_R {
        TIE_LOW_XPD_BBPLL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tie_low_xpd_apll(&self) -> TIE_LOW_XPD_APLL_R {
        TIE_LOW_XPD_APLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tie_low_xpd_mpll(&self) -> TIE_LOW_XPD_MPLL_R {
        TIE_LOW_XPD_MPLL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tie_low_global_bbpll_icg(&self) -> TIE_LOW_GLOBAL_BBPLL_ICG_R {
        TIE_LOW_GLOBAL_BBPLL_ICG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tie_low_global_apll_icg(&self) -> TIE_LOW_GLOBAL_APLL_ICG_R {
        TIE_LOW_GLOBAL_APLL_ICG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tie_low_global_mpll_icg(&self) -> TIE_LOW_GLOBAL_MPLL_ICG_R {
        TIE_LOW_GLOBAL_MPLL_ICG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tie_high_global_bbpll_icg(&self) -> TIE_HIGH_GLOBAL_BBPLL_ICG_R {
        TIE_HIGH_GLOBAL_BBPLL_ICG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tie_high_global_apll_icg(&self) -> TIE_HIGH_GLOBAL_APLL_ICG_R {
        TIE_HIGH_GLOBAL_APLL_ICG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tie_high_global_mpll_icg(&self) -> TIE_HIGH_GLOBAL_MPLL_ICG_R {
        TIE_HIGH_GLOBAL_MPLL_ICG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tie_high_xpd_bbpll_i2c(&self) -> TIE_HIGH_XPD_BBPLL_I2C_R {
        TIE_HIGH_XPD_BBPLL_I2C_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tie_high_xpd_apll_i2c(&self) -> TIE_HIGH_XPD_APLL_I2C_R {
        TIE_HIGH_XPD_APLL_I2C_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tie_high_xpd_mpll_i2c(&self) -> TIE_HIGH_XPD_MPLL_I2C_R {
        TIE_HIGH_XPD_MPLL_I2C_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tie_high_xpd_bbpll(&self) -> TIE_HIGH_XPD_BBPLL_R {
        TIE_HIGH_XPD_BBPLL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tie_high_xpd_apll(&self) -> TIE_HIGH_XPD_APLL_R {
        TIE_HIGH_XPD_APLL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tie_high_xpd_mpll(&self) -> TIE_HIGH_XPD_MPLL_R {
        TIE_HIGH_XPD_MPLL_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMM_HP_CK_POWER_1")
            .field("tie_low_xpd_bbpll_i2c", &self.tie_low_xpd_bbpll_i2c())
            .field("tie_low_xpd_apll_i2c", &self.tie_low_xpd_apll_i2c())
            .field("tie_low_xpd_mpll_i2c", &self.tie_low_xpd_mpll_i2c())
            .field("tie_low_xpd_bbpll", &self.tie_low_xpd_bbpll())
            .field("tie_low_xpd_apll", &self.tie_low_xpd_apll())
            .field("tie_low_xpd_mpll", &self.tie_low_xpd_mpll())
            .field("tie_low_global_bbpll_icg", &self.tie_low_global_bbpll_icg())
            .field("tie_low_global_apll_icg", &self.tie_low_global_apll_icg())
            .field("tie_low_global_mpll_icg", &self.tie_low_global_mpll_icg())
            .field(
                "tie_high_global_bbpll_icg",
                &self.tie_high_global_bbpll_icg(),
            )
            .field("tie_high_global_apll_icg", &self.tie_high_global_apll_icg())
            .field("tie_high_global_mpll_icg", &self.tie_high_global_mpll_icg())
            .field("tie_high_xpd_bbpll_i2c", &self.tie_high_xpd_bbpll_i2c())
            .field("tie_high_xpd_apll_i2c", &self.tie_high_xpd_apll_i2c())
            .field("tie_high_xpd_mpll_i2c", &self.tie_high_xpd_mpll_i2c())
            .field("tie_high_xpd_bbpll", &self.tie_high_xpd_bbpll())
            .field("tie_high_xpd_apll", &self.tie_high_xpd_apll())
            .field("tie_high_xpd_mpll", &self.tie_high_xpd_mpll())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_pll_i2c(&mut self) -> TIE_LOW_XPD_PLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_PLL_I2C_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tie_low_xpd_bbpll_i2c(&mut self) -> TIE_LOW_XPD_BBPLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_BBPLL_I2C_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tie_low_xpd_apll_i2c(&mut self) -> TIE_LOW_XPD_APLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_APLL_I2C_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tie_low_xpd_mpll_i2c(&mut self) -> TIE_LOW_XPD_MPLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_MPLL_I2C_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn tie_low_xpd_pll(&mut self) -> TIE_LOW_XPD_PLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_PLL_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tie_low_xpd_bbpll(&mut self) -> TIE_LOW_XPD_BBPLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_BBPLL_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tie_low_xpd_apll(&mut self) -> TIE_LOW_XPD_APLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_APLL_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tie_low_xpd_mpll(&mut self) -> TIE_LOW_XPD_MPLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_XPD_MPLL_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn tie_low_global_pll_icg(
        &mut self,
    ) -> TIE_LOW_GLOBAL_PLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_GLOBAL_PLL_ICG_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tie_low_global_bbpll_icg(
        &mut self,
    ) -> TIE_LOW_GLOBAL_BBPLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_GLOBAL_BBPLL_ICG_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tie_low_global_apll_icg(
        &mut self,
    ) -> TIE_LOW_GLOBAL_APLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_GLOBAL_APLL_ICG_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tie_low_global_mpll_icg(
        &mut self,
    ) -> TIE_LOW_GLOBAL_MPLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_LOW_GLOBAL_MPLL_ICG_W::new(self, 11)
    }
    #[doc = "Bit 19 - need_des"]
    #[inline(always)]
    pub fn tie_high_global_pll_icg(
        &mut self,
    ) -> TIE_HIGH_GLOBAL_PLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_GLOBAL_PLL_ICG_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tie_high_global_bbpll_icg(
        &mut self,
    ) -> TIE_HIGH_GLOBAL_BBPLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_GLOBAL_BBPLL_ICG_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tie_high_global_apll_icg(
        &mut self,
    ) -> TIE_HIGH_GLOBAL_APLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_GLOBAL_APLL_ICG_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tie_high_global_mpll_icg(
        &mut self,
    ) -> TIE_HIGH_GLOBAL_MPLL_ICG_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_GLOBAL_MPLL_ICG_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_pll_i2c(&mut self) -> TIE_HIGH_XPD_PLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_PLL_I2C_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tie_high_xpd_bbpll_i2c(
        &mut self,
    ) -> TIE_HIGH_XPD_BBPLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_BBPLL_I2C_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tie_high_xpd_apll_i2c(&mut self) -> TIE_HIGH_XPD_APLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_APLL_I2C_W::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tie_high_xpd_mpll_i2c(&mut self) -> TIE_HIGH_XPD_MPLL_I2C_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_MPLL_I2C_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn tie_high_xpd_pll(&mut self) -> TIE_HIGH_XPD_PLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_PLL_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tie_high_xpd_bbpll(&mut self) -> TIE_HIGH_XPD_BBPLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_BBPLL_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tie_high_xpd_apll(&mut self) -> TIE_HIGH_XPD_APLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_APLL_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tie_high_xpd_mpll(&mut self) -> TIE_HIGH_XPD_MPLL_W<'_, IMM_HP_CK_POWER_1_SPEC> {
        TIE_HIGH_XPD_MPLL_W::new(self, 30)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_hp_ck_power_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_ck_power_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMM_HP_CK_POWER_1_SPEC;
impl crate::RegisterSpec for IMM_HP_CK_POWER_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imm_hp_ck_power_1::R`](R) reader structure"]
impl crate::Readable for IMM_HP_CK_POWER_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imm_hp_ck_power_1::W`](W) writer structure"]
impl crate::Writable for IMM_HP_CK_POWER_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMM_HP_CK_POWER_1 to value 0"]
impl crate::Resettable for IMM_HP_CK_POWER_1_SPEC {}
