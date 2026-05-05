#[doc = "Register `HP_ACTIVE_BIAS` reader"]
pub type R = crate::R<HP_ACTIVE_BIAS_SPEC>;
#[doc = "Register `HP_ACTIVE_BIAS` writer"]
pub type W = crate::W<HP_ACTIVE_BIAS_SPEC>;
#[doc = "Field `HP_ACTIVE_DCM_VSET` reader - PMU_HP_ACTIVE_DCM_VSET"]
pub type HP_ACTIVE_DCM_VSET_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_DCM_VSET` writer - PMU_HP_ACTIVE_DCM_VSET"]
pub type HP_ACTIVE_DCM_VSET_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HP_ACTIVE_DCM_MODE` reader - PMU_HP_ACTIVE_DCM_MODE"]
pub type HP_ACTIVE_DCM_MODE_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_DCM_MODE` writer - PMU_HP_ACTIVE_DCM_MODE"]
pub type HP_ACTIVE_DCM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_ACTIVE_XPD_BIAS` reader - PMU_HP_ACTIVE_XPD_BIAS"]
pub type HP_ACTIVE_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_XPD_BIAS` writer - PMU_HP_ACTIVE_XPD_BIAS"]
pub type HP_ACTIVE_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_DBG_ATTEN` reader - PMU_HP_ACTIVE_DBG_ATTEN"]
pub type HP_ACTIVE_DBG_ATTEN_R = crate::FieldReader;
#[doc = "Field `HP_ACTIVE_DBG_ATTEN` writer - PMU_HP_ACTIVE_DBG_ATTEN"]
pub type HP_ACTIVE_DBG_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_ACTIVE_PD_CUR` reader - PMU_HP_ACTIVE_PD_CUR"]
pub type HP_ACTIVE_PD_CUR_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_PD_CUR` writer - PMU_HP_ACTIVE_PD_CUR"]
pub type HP_ACTIVE_PD_CUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_ACTIVE_BIAS_SLEEP` reader - PMU_HP_ACTIVE_BIAS_SLEEP"]
pub type HP_ACTIVE_BIAS_SLEEP_R = crate::BitReader;
#[doc = "Field `HP_ACTIVE_BIAS_SLEEP` writer - PMU_HP_ACTIVE_BIAS_SLEEP"]
pub type HP_ACTIVE_BIAS_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 18:22 - PMU_HP_ACTIVE_DCM_VSET"]
    #[inline(always)]
    pub fn hp_active_dcm_vset(&self) -> HP_ACTIVE_DCM_VSET_R {
        HP_ACTIVE_DCM_VSET_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - PMU_HP_ACTIVE_DCM_MODE"]
    #[inline(always)]
    pub fn hp_active_dcm_mode(&self) -> HP_ACTIVE_DCM_MODE_R {
        HP_ACTIVE_DCM_MODE_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - PMU_HP_ACTIVE_XPD_BIAS"]
    #[inline(always)]
    pub fn hp_active_xpd_bias(&self) -> HP_ACTIVE_XPD_BIAS_R {
        HP_ACTIVE_XPD_BIAS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - PMU_HP_ACTIVE_DBG_ATTEN"]
    #[inline(always)]
    pub fn hp_active_dbg_atten(&self) -> HP_ACTIVE_DBG_ATTEN_R {
        HP_ACTIVE_DBG_ATTEN_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - PMU_HP_ACTIVE_PD_CUR"]
    #[inline(always)]
    pub fn hp_active_pd_cur(&self) -> HP_ACTIVE_PD_CUR_R {
        HP_ACTIVE_PD_CUR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PMU_HP_ACTIVE_BIAS_SLEEP"]
    #[inline(always)]
    pub fn hp_active_bias_sleep(&self) -> HP_ACTIVE_BIAS_SLEEP_R {
        HP_ACTIVE_BIAS_SLEEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ACTIVE_BIAS")
            .field("hp_active_dcm_vset", &self.hp_active_dcm_vset())
            .field("hp_active_dcm_mode", &self.hp_active_dcm_mode())
            .field("hp_active_xpd_bias", &self.hp_active_xpd_bias())
            .field("hp_active_dbg_atten", &self.hp_active_dbg_atten())
            .field("hp_active_pd_cur", &self.hp_active_pd_cur())
            .field("hp_active_bias_sleep", &self.hp_active_bias_sleep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 18:22 - PMU_HP_ACTIVE_DCM_VSET"]
    #[inline(always)]
    pub fn hp_active_dcm_vset(&mut self) -> HP_ACTIVE_DCM_VSET_W<'_, HP_ACTIVE_BIAS_SPEC> {
        HP_ACTIVE_DCM_VSET_W::new(self, 18)
    }
    #[doc = "Bits 23:24 - PMU_HP_ACTIVE_DCM_MODE"]
    #[inline(always)]
    pub fn hp_active_dcm_mode(&mut self) -> HP_ACTIVE_DCM_MODE_W<'_, HP_ACTIVE_BIAS_SPEC> {
        HP_ACTIVE_DCM_MODE_W::new(self, 23)
    }
    #[doc = "Bit 25 - PMU_HP_ACTIVE_XPD_BIAS"]
    #[inline(always)]
    pub fn hp_active_xpd_bias(&mut self) -> HP_ACTIVE_XPD_BIAS_W<'_, HP_ACTIVE_BIAS_SPEC> {
        HP_ACTIVE_XPD_BIAS_W::new(self, 25)
    }
    #[doc = "Bits 26:29 - PMU_HP_ACTIVE_DBG_ATTEN"]
    #[inline(always)]
    pub fn hp_active_dbg_atten(&mut self) -> HP_ACTIVE_DBG_ATTEN_W<'_, HP_ACTIVE_BIAS_SPEC> {
        HP_ACTIVE_DBG_ATTEN_W::new(self, 26)
    }
    #[doc = "Bit 30 - PMU_HP_ACTIVE_PD_CUR"]
    #[inline(always)]
    pub fn hp_active_pd_cur(&mut self) -> HP_ACTIVE_PD_CUR_W<'_, HP_ACTIVE_BIAS_SPEC> {
        HP_ACTIVE_PD_CUR_W::new(self, 30)
    }
    #[doc = "Bit 31 - PMU_HP_ACTIVE_BIAS_SLEEP"]
    #[inline(always)]
    pub fn hp_active_bias_sleep(&mut self) -> HP_ACTIVE_BIAS_SLEEP_W<'_, HP_ACTIVE_BIAS_SPEC> {
        HP_ACTIVE_BIAS_SLEEP_W::new(self, 31)
    }
}
#[doc = "PMU_HP_ACTIVE_BIAS\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ACTIVE_BIAS_SPEC;
impl crate::RegisterSpec for HP_ACTIVE_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_active_bias::R`](R) reader structure"]
impl crate::Readable for HP_ACTIVE_BIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_active_bias::W`](W) writer structure"]
impl crate::Writable for HP_ACTIVE_BIAS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ACTIVE_BIAS to value 0"]
impl crate::Resettable for HP_ACTIVE_BIAS_SPEC {}
