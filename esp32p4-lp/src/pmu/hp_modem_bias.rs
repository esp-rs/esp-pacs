#[doc = "Register `HP_MODEM_BIAS` writer"]
pub type W = crate::W<HP_MODEM_BIAS_SPEC>;
#[doc = "Field `HP_MODEM_DCM_VSET` writer - PMU_HP_MODEM_DCM_VSET"]
pub type HP_MODEM_DCM_VSET_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HP_MODEM_DCM_MODE` writer - PMU_HP_MODEM_DCM_MODE"]
pub type HP_MODEM_DCM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MODEM_XPD_BIAS` writer - PMU_HP_MODEM_XPD_BIAS"]
pub type HP_MODEM_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DBG_ATTEN` writer - PMU_HP_MODEM_DBG_ATTEN"]
pub type HP_MODEM_DBG_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_MODEM_PD_CUR` writer - PMU_HP_MODEM_PD_CUR"]
pub type HP_MODEM_PD_CUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_BIAS_SLEEP` writer - PMU_HP_MODEM_BIAS_SLEEP"]
pub type HP_MODEM_BIAS_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_BIAS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 18:22 - PMU_HP_MODEM_DCM_VSET"]
    #[inline(always)]
    pub fn hp_modem_dcm_vset(&mut self) -> HP_MODEM_DCM_VSET_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_DCM_VSET_W::new(self, 18)
    }
    #[doc = "Bits 23:24 - PMU_HP_MODEM_DCM_MODE"]
    #[inline(always)]
    pub fn hp_modem_dcm_mode(&mut self) -> HP_MODEM_DCM_MODE_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_DCM_MODE_W::new(self, 23)
    }
    #[doc = "Bit 25 - PMU_HP_MODEM_XPD_BIAS"]
    #[inline(always)]
    pub fn hp_modem_xpd_bias(&mut self) -> HP_MODEM_XPD_BIAS_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_XPD_BIAS_W::new(self, 25)
    }
    #[doc = "Bits 26:29 - PMU_HP_MODEM_DBG_ATTEN"]
    #[inline(always)]
    pub fn hp_modem_dbg_atten(&mut self) -> HP_MODEM_DBG_ATTEN_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_DBG_ATTEN_W::new(self, 26)
    }
    #[doc = "Bit 30 - PMU_HP_MODEM_PD_CUR"]
    #[inline(always)]
    pub fn hp_modem_pd_cur(&mut self) -> HP_MODEM_PD_CUR_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_PD_CUR_W::new(self, 30)
    }
    #[doc = "Bit 31 - PMU_HP_MODEM_BIAS_SLEEP"]
    #[inline(always)]
    pub fn hp_modem_bias_sleep(&mut self) -> HP_MODEM_BIAS_SLEEP_W<'_, HP_MODEM_BIAS_SPEC> {
        HP_MODEM_BIAS_SLEEP_W::new(self, 31)
    }
}
#[doc = "PMU_HP_MODEM_BIAS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_bias::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_BIAS_SPEC;
impl crate::RegisterSpec for HP_MODEM_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_bias::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_BIAS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_MODEM_BIAS to value 0"]
impl crate::Resettable for HP_MODEM_BIAS_SPEC {}
