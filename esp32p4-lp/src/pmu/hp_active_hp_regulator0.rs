#[doc = "Register `HP_ACTIVE_HP_REGULATOR0` reader"]
pub type R = crate::R<HP_ACTIVE_HP_REGULATOR0_SPEC>;
#[doc = "Register `HP_ACTIVE_HP_REGULATOR0` writer"]
pub type W = crate::W<HP_ACTIVE_HP_REGULATOR0_SPEC>;
#[doc = "Field `LP_DBIAS_VOL` reader - PMU_LP_DBIAS_VOL"]
pub type LP_DBIAS_VOL_R = crate::FieldReader;
#[doc = "Field `HP_DBIAS_VOL` reader - PMU_HP_DBIAS_VOL"]
pub type HP_DBIAS_VOL_R = crate::FieldReader;
#[doc = "Field `DIG_REGULATOR0_DBIAS_SEL` reader - PMU_DIG_REGULATOR0_DBIAS_SEL"]
pub type DIG_REGULATOR0_DBIAS_SEL_R = crate::BitReader;
#[doc = "Field `DIG_REGULATOR0_DBIAS_SEL` writer - PMU_DIG_REGULATOR0_DBIAS_SEL"]
pub type DIG_REGULATOR0_DBIAS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIG_DBIAS_INIT` writer - PMU_DIG_DBIAS_INIT"]
pub type DIG_DBIAS_INIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD` reader - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD` writer - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD` reader - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD` writer - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_XPD` reader - PMU_HP_ACTIVE_HP_REGULATOR_XPD"]
pub type HP_ACTIVE_HP_REGULATOR_XPD_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_XPD` writer - PMU_HP_ACTIVE_HP_REGULATOR_XPD"]
pub type HP_ACTIVE_HP_REGULATOR_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS` reader - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS` writer - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS` reader - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS` writer - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS"]
pub type HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_DBIAS` reader - PMU_HP_ACTIVE_HP_REGULATOR_DBIAS"]
pub type HP_ACTIVE_HP_REGULATOR_DBIAS_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_HP_REGULATOR_DBIAS` writer - PMU_HP_ACTIVE_HP_REGULATOR_DBIAS"]
pub type HP_ACTIVE_HP_REGULATOR_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 4:8 - PMU_LP_DBIAS_VOL"]
    #[inline(always)]
    pub fn lp_dbias_vol(&self) -> LP_DBIAS_VOL_R {
        LP_DBIAS_VOL_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - PMU_HP_DBIAS_VOL"]
    #[inline(always)]
    pub fn hp_dbias_vol(&self) -> HP_DBIAS_VOL_R {
        HP_DBIAS_VOL_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - PMU_DIG_REGULATOR0_DBIAS_SEL"]
    #[inline(always)]
    pub fn dig_regulator0_dbias_sel(&self) -> DIG_REGULATOR0_DBIAS_SEL_R {
        DIG_REGULATOR0_DBIAS_SEL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_mem_xpd(&self) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_R {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_logic_xpd(&self) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_R {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PMU_HP_ACTIVE_HP_REGULATOR_XPD"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_xpd(&self) -> HP_ACTIVE_HP_REGULATOR_XPD_R {
        HP_ACTIVE_HP_REGULATOR_XPD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_mem_dbias(&self) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_R {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_logic_dbias(
        &self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_R {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - PMU_HP_ACTIVE_HP_REGULATOR_DBIAS"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_dbias(&self) -> HP_ACTIVE_HP_REGULATOR_DBIAS_R {
        HP_ACTIVE_HP_REGULATOR_DBIAS_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_HP_REGULATOR0")
            .field("lp_dbias_vol", &self.lp_dbias_vol())
            .field("hp_dbias_vol", &self.hp_dbias_vol())
            .field("dig_regulator0_dbias_sel", &self.dig_regulator0_dbias_sel())
            .field(
                "hp_active_hp_regulator_slp_mem_xpd",
                &self.hp_active_hp_regulator_slp_mem_xpd(),
            )
            .field(
                "hp_active_hp_regulator_slp_logic_xpd",
                &self.hp_active_hp_regulator_slp_logic_xpd(),
            )
            .field(
                "hp_active_hp_regulator_xpd",
                &self.hp_active_hp_regulator_xpd(),
            )
            .field(
                "hp_active_hp_regulator_slp_mem_dbias",
                &self.hp_active_hp_regulator_slp_mem_dbias(),
            )
            .field(
                "hp_active_hp_regulator_slp_logic_dbias",
                &self.hp_active_hp_regulator_slp_logic_dbias(),
            )
            .field(
                "hp_active_hp_regulator_dbias",
                &self.hp_active_hp_regulator_dbias(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 14 - PMU_DIG_REGULATOR0_DBIAS_SEL"]
    #[inline(always)]
    pub fn dig_regulator0_dbias_sel(
        &mut self,
    ) -> DIG_REGULATOR0_DBIAS_SEL_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        DIG_REGULATOR0_DBIAS_SEL_W::new(self, 14)
    }
    #[doc = "Bit 15 - PMU_DIG_DBIAS_INIT"]
    #[inline(always)]
    pub fn dig_dbias_init(&mut self) -> DIG_DBIAS_INIT_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        DIG_DBIAS_INIT_W::new(self, 15)
    }
    #[doc = "Bit 16 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_mem_xpd(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_XPD_W::new(self, 16)
    }
    #[doc = "Bit 17 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_logic_xpd(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_XPD_W::new(self, 17)
    }
    #[doc = "Bit 18 - PMU_HP_ACTIVE_HP_REGULATOR_XPD"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_xpd(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_XPD_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        HP_ACTIVE_HP_REGULATOR_XPD_W::new(self, 18)
    }
    #[doc = "Bits 19:22 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_mem_dbias(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        HP_ACTIVE_HP_REGULATOR_SLP_MEM_DBIAS_W::new(self, 19)
    }
    #[doc = "Bits 23:26 - PMU_HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_slp_logic_dbias(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        HP_ACTIVE_HP_REGULATOR_SLP_LOGIC_DBIAS_W::new(self, 23)
    }
    #[doc = "Bits 27:31 - PMU_HP_ACTIVE_HP_REGULATOR_DBIAS"]
    #[inline(always)]
    pub fn hp_active_hp_regulator_dbias(
        &mut self,
    ) -> HP_ACTIVE_HP_REGULATOR_DBIAS_W<'_, HP_ACTIVE_HP_REGULATOR0_SPEC> {
        HP_ACTIVE_HP_REGULATOR_DBIAS_W::new(self, 27)
    }
}
#[doc = "PMU_HP_ACTIVE_HP_REGULATOR0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_regulator0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_regulator0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_HP_REGULATOR0_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_HP_REGULATOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_hp_regulator0::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_HP_REGULATOR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_hp_regulator0::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_HP_REGULATOR0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ACTIVE_HP_REGULATOR0 to value 0"]
impl crate::Resettable for HP_ACTIVE_HP_REGULATOR0_SPEC {}
