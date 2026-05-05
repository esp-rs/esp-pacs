#[doc = "Register `HP_MODEM_HP_REGULATOR0` writer"]
pub type W = crate::W<HP_MODEM_HP_REGULATOR0_SPEC>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_MEM_XPD` writer - PMU_HP_MODEM_HP_REGULATOR_SLP_MEM_XPD"]
pub type HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD` writer - PMU_HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD"]
pub type HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_XPD` writer - PMU_HP_MODEM_HP_REGULATOR_XPD"]
pub type HP_MODEM_HP_REGULATOR_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS` writer - PMU_HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS"]
pub type HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS` writer - PMU_HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS"]
pub type HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_MODEM_HP_REGULATOR_DBIAS` writer - PMU_HP_MODEM_HP_REGULATOR_DBIAS"]
pub type HP_MODEM_HP_REGULATOR_DBIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_HP_REGULATOR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 16 - PMU_HP_MODEM_HP_REGULATOR_SLP_MEM_XPD"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_mem_xpd(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_W<'_, HP_MODEM_HP_REGULATOR0_SPEC> {
        HP_MODEM_HP_REGULATOR_SLP_MEM_XPD_W::new(self, 16)
    }
    #[doc = "Bit 17 - PMU_HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_logic_xpd(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_W<'_, HP_MODEM_HP_REGULATOR0_SPEC> {
        HP_MODEM_HP_REGULATOR_SLP_LOGIC_XPD_W::new(self, 17)
    }
    #[doc = "Bit 18 - PMU_HP_MODEM_HP_REGULATOR_XPD"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_xpd(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_XPD_W<'_, HP_MODEM_HP_REGULATOR0_SPEC> {
        HP_MODEM_HP_REGULATOR_XPD_W::new(self, 18)
    }
    #[doc = "Bits 19:22 - PMU_HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_mem_dbias(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_W<'_, HP_MODEM_HP_REGULATOR0_SPEC> {
        HP_MODEM_HP_REGULATOR_SLP_MEM_DBIAS_W::new(self, 19)
    }
    #[doc = "Bits 23:26 - PMU_HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_slp_logic_dbias(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_W<'_, HP_MODEM_HP_REGULATOR0_SPEC> {
        HP_MODEM_HP_REGULATOR_SLP_LOGIC_DBIAS_W::new(self, 23)
    }
    #[doc = "Bits 27:31 - PMU_HP_MODEM_HP_REGULATOR_DBIAS"]
    #[inline(always)]
    pub fn hp_modem_hp_regulator_dbias(
        &mut self,
    ) -> HP_MODEM_HP_REGULATOR_DBIAS_W<'_, HP_MODEM_HP_REGULATOR0_SPEC> {
        HP_MODEM_HP_REGULATOR_DBIAS_W::new(self, 27)
    }
}
#[doc = "PMU_HP_MODEM_HP_REGULATOR0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_regulator0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_HP_REGULATOR0_SPEC;
impl crate::RegisterSpec for HP_MODEM_HP_REGULATOR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_hp_regulator0::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_HP_REGULATOR0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_HP_REGULATOR0 to value 0"]
impl crate::Resettable for HP_MODEM_HP_REGULATOR0_SPEC {}
