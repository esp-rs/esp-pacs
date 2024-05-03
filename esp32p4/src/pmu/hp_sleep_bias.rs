#[doc = "Register `HP_SLEEP_BIAS` reader"]
pub type R = crate::R<HP_SLEEP_BIAS_SPEC>;
#[doc = "Register `HP_SLEEP_BIAS` writer"]
pub type W = crate::W<HP_SLEEP_BIAS_SPEC>;
#[doc = "Field `HP_SLEEP_DCM_VSET` reader - need_des"]
pub type HP_SLEEP_DCM_VSET_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DCM_VSET` writer - need_des"]
pub type HP_SLEEP_DCM_VSET_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HP_SLEEP_DCM_MODE` reader - need_des"]
pub type HP_SLEEP_DCM_MODE_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DCM_MODE` writer - need_des"]
pub type HP_SLEEP_DCM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP_XPD_BIAS` reader - need_des"]
pub type HP_SLEEP_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_BIAS` writer - need_des"]
pub type HP_SLEEP_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_DBG_ATTEN` reader - need_des"]
pub type HP_SLEEP_DBG_ATTEN_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DBG_ATTEN` writer - need_des"]
pub type HP_SLEEP_DBG_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_SLEEP_PD_CUR` reader - need_des"]
pub type HP_SLEEP_PD_CUR_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_CUR` writer - need_des"]
pub type HP_SLEEP_PD_CUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - need_des"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_vset(&self) -> HP_SLEEP_DCM_VSET_R {
        HP_SLEEP_DCM_VSET_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_mode(&self) -> HP_SLEEP_DCM_MODE_R {
        HP_SLEEP_DCM_MODE_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_bias(&self) -> HP_SLEEP_XPD_BIAS_R {
        HP_SLEEP_XPD_BIAS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dbg_atten(&self) -> HP_SLEEP_DBG_ATTEN_R {
        HP_SLEEP_DBG_ATTEN_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_cur(&self) -> HP_SLEEP_PD_CUR_R {
        HP_SLEEP_PD_CUR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_BIAS")
            .field("hp_sleep_dcm_vset", &self.hp_sleep_dcm_vset().bits())
            .field("hp_sleep_dcm_mode", &self.hp_sleep_dcm_mode().bits())
            .field("hp_sleep_xpd_bias", &self.hp_sleep_xpd_bias().bit())
            .field("hp_sleep_dbg_atten", &self.hp_sleep_dbg_atten().bits())
            .field("hp_sleep_pd_cur", &self.hp_sleep_pd_cur().bit())
            .field("sleep", &self.sleep().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_SLEEP_BIAS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 18:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_dcm_vset(&mut self) -> HP_SLEEP_DCM_VSET_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DCM_VSET_W::new(self, 18)
    }
    #[doc = "Bits 23:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_dcm_mode(&mut self) -> HP_SLEEP_DCM_MODE_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DCM_MODE_W::new(self, 23)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_xpd_bias(&mut self) -> HP_SLEEP_XPD_BIAS_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_XPD_BIAS_W::new(self, 25)
    }
    #[doc = "Bits 26:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_dbg_atten(&mut self) -> HP_SLEEP_DBG_ATTEN_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DBG_ATTEN_W::new(self, 26)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_cur(&mut self) -> HP_SLEEP_PD_CUR_W<HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_PD_CUR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<HP_SLEEP_BIAS_SPEC> {
        SLEEP_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_bias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_bias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_SLEEP_BIAS_SPEC;
impl crate::RegisterSpec for HP_SLEEP_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_bias::R`](R) reader structure"]
impl crate::Readable for HP_SLEEP_BIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_bias::W`](W) writer structure"]
impl crate::Writable for HP_SLEEP_BIAS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_BIAS to value 0x0050_0000"]
impl crate::Resettable for HP_SLEEP_BIAS_SPEC {
    const RESET_VALUE: u32 = 0x0050_0000;
}
