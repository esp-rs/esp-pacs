#[doc = "Register `HP_MODEM_BIAS` writer"]
pub type W = crate::W<HP_MODEM_BIAS_SPEC>;
#[doc = "Field `HP_MODEM_DCM_VSET` writer - need_des"]
pub type HP_MODEM_DCM_VSET_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HP_MODEM_DCM_MODE` writer - need_des"]
pub type HP_MODEM_DCM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_MODEM_XPD_BIAS` writer - need_des"]
pub type HP_MODEM_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_DBG_ATTEN` writer - need_des"]
pub type HP_MODEM_DBG_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_MODEM_PD_CUR` writer - need_des"]
pub type HP_MODEM_PD_CUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_BIAS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dcm_vset(&mut self) -> HP_MODEM_DCM_VSET_W<HP_MODEM_BIAS_SPEC> {
        HP_MODEM_DCM_VSET_W::new(self, 18)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dcm_mode(&mut self) -> HP_MODEM_DCM_MODE_W<HP_MODEM_BIAS_SPEC> {
        HP_MODEM_DCM_MODE_W::new(self, 23)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_modem_xpd_bias(&mut self) -> HP_MODEM_XPD_BIAS_W<HP_MODEM_BIAS_SPEC> {
        HP_MODEM_XPD_BIAS_W::new(self, 25)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dbg_atten(&mut self) -> HP_MODEM_DBG_ATTEN_W<HP_MODEM_BIAS_SPEC> {
        HP_MODEM_DBG_ATTEN_W::new(self, 26)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_cur(&mut self) -> HP_MODEM_PD_CUR_W<HP_MODEM_BIAS_SPEC> {
        HP_MODEM_PD_CUR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<HP_MODEM_BIAS_SPEC> {
        SLEEP_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_bias::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_BIAS_SPEC;
impl crate::RegisterSpec for HP_MODEM_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hp_modem_bias::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_BIAS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_MODEM_BIAS to value 0x0050_0000"]
impl crate::Resettable for HP_MODEM_BIAS_SPEC {
    const RESET_VALUE: u32 = 0x0050_0000;
}
