#[doc = "Register `HP_SLEEP_BIAS` reader"]
pub type R = crate::R<HP_SLEEP_BIAS_SPEC>;
#[doc = "Register `HP_SLEEP_BIAS` writer"]
pub type W = crate::W<HP_SLEEP_BIAS_SPEC>;
#[doc = "Field `HP_SLEEP_DCDC_CCM_ENB` reader - need_des"]
pub type HP_SLEEP_DCDC_CCM_ENB_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_DCDC_CCM_ENB` writer - need_des"]
pub type HP_SLEEP_DCDC_CCM_ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_DCDC_CLEAR_RDY` reader - need_des"]
pub type HP_SLEEP_DCDC_CLEAR_RDY_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_DCDC_CLEAR_RDY` writer - need_des"]
pub type HP_SLEEP_DCDC_CLEAR_RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_DIG_PMU_DPCUR_BIAS` reader - need_des"]
pub type HP_SLEEP_DIG_PMU_DPCUR_BIAS_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DIG_PMU_DPCUR_BIAS` writer - need_des"]
pub type HP_SLEEP_DIG_PMU_DPCUR_BIAS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP_DIG_PMU_DSFMOS` reader - need_des"]
pub type HP_SLEEP_DIG_PMU_DSFMOS_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DIG_PMU_DSFMOS` writer - need_des"]
pub type HP_SLEEP_DIG_PMU_DSFMOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_SLEEP_DCM_VSET` reader - need_des"]
pub type HP_SLEEP_DCM_VSET_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DCM_VSET` writer - need_des"]
pub type HP_SLEEP_DCM_VSET_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HP_SLEEP_DCM_MODE` reader - need_des"]
pub type HP_SLEEP_DCM_MODE_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_DCM_MODE` writer - need_des"]
pub type HP_SLEEP_DCM_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP_XPD_TRX` reader - need_des"]
pub type HP_SLEEP_XPD_TRX_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_TRX` writer - need_des"]
pub type HP_SLEEP_XPD_TRX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_XPD_BIAS` reader - need_des"]
pub type HP_SLEEP_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_XPD_BIAS` writer - need_des"]
pub type HP_SLEEP_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_DISCNNT_DIG_RTC` reader - need_des"]
pub type HP_SLEEP_DISCNNT_DIG_RTC_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_DISCNNT_DIG_RTC` writer - need_des"]
pub type HP_SLEEP_DISCNNT_DIG_RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_PD_CUR` reader - need_des"]
pub type HP_SLEEP_PD_CUR_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_CUR` writer - need_des"]
pub type HP_SLEEP_PD_CUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - need_des"]
pub type SLEEP_R = crate::BitReader;
#[doc = "Field `SLEEP` writer - need_des"]
pub type SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcdc_ccm_enb(&self) -> HP_SLEEP_DCDC_CCM_ENB_R {
        HP_SLEEP_DCDC_CCM_ENB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcdc_clear_rdy(&self) -> HP_SLEEP_DCDC_CLEAR_RDY_R {
        HP_SLEEP_DCDC_CLEAR_RDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dig_pmu_dpcur_bias(&self) -> HP_SLEEP_DIG_PMU_DPCUR_BIAS_R {
        HP_SLEEP_DIG_PMU_DPCUR_BIAS_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:16 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dig_pmu_dsfmos(&self) -> HP_SLEEP_DIG_PMU_DSFMOS_R {
        HP_SLEEP_DIG_PMU_DSFMOS_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:21 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_vset(&self) -> HP_SLEEP_DCM_VSET_R {
        HP_SLEEP_DCM_VSET_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_mode(&self) -> HP_SLEEP_DCM_MODE_R {
        HP_SLEEP_DCM_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_trx(&self) -> HP_SLEEP_XPD_TRX_R {
        HP_SLEEP_XPD_TRX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_bias(&self) -> HP_SLEEP_XPD_BIAS_R {
        HP_SLEEP_XPD_BIAS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_discnnt_dig_rtc(&self) -> HP_SLEEP_DISCNNT_DIG_RTC_R {
        HP_SLEEP_DISCNNT_DIG_RTC_R::new(((self.bits >> 29) & 1) != 0)
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
            .field("hp_sleep_dcdc_ccm_enb", &self.hp_sleep_dcdc_ccm_enb())
            .field("hp_sleep_dcdc_clear_rdy", &self.hp_sleep_dcdc_clear_rdy())
            .field(
                "hp_sleep_dig_pmu_dpcur_bias",
                &self.hp_sleep_dig_pmu_dpcur_bias(),
            )
            .field("hp_sleep_dig_pmu_dsfmos", &self.hp_sleep_dig_pmu_dsfmos())
            .field("hp_sleep_dcm_vset", &self.hp_sleep_dcm_vset())
            .field("hp_sleep_dcm_mode", &self.hp_sleep_dcm_mode())
            .field("hp_sleep_xpd_trx", &self.hp_sleep_xpd_trx())
            .field("hp_sleep_xpd_bias", &self.hp_sleep_xpd_bias())
            .field("hp_sleep_discnnt_dig_rtc", &self.hp_sleep_discnnt_dig_rtc())
            .field("hp_sleep_pd_cur", &self.hp_sleep_pd_cur())
            .field("sleep", &self.sleep())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcdc_ccm_enb(&mut self) -> HP_SLEEP_DCDC_CCM_ENB_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DCDC_CCM_ENB_W::new(self, 9)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcdc_clear_rdy(&mut self) -> HP_SLEEP_DCDC_CLEAR_RDY_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DCDC_CLEAR_RDY_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dig_pmu_dpcur_bias(
        &mut self,
    ) -> HP_SLEEP_DIG_PMU_DPCUR_BIAS_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DIG_PMU_DPCUR_BIAS_W::new(self, 11)
    }
    #[doc = "Bits 13:16 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dig_pmu_dsfmos(&mut self) -> HP_SLEEP_DIG_PMU_DSFMOS_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DIG_PMU_DSFMOS_W::new(self, 13)
    }
    #[doc = "Bits 17:21 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_vset(&mut self) -> HP_SLEEP_DCM_VSET_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DCM_VSET_W::new(self, 17)
    }
    #[doc = "Bits 22:23 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_dcm_mode(&mut self) -> HP_SLEEP_DCM_MODE_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DCM_MODE_W::new(self, 22)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_trx(&mut self) -> HP_SLEEP_XPD_TRX_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_XPD_TRX_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_xpd_bias(&mut self) -> HP_SLEEP_XPD_BIAS_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_XPD_BIAS_W::new(self, 25)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_discnnt_dig_rtc(
        &mut self,
    ) -> HP_SLEEP_DISCNNT_DIG_RTC_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_DISCNNT_DIG_RTC_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_cur(&mut self) -> HP_SLEEP_PD_CUR_W<'_, HP_SLEEP_BIAS_SPEC> {
        HP_SLEEP_PD_CUR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<'_, HP_SLEEP_BIAS_SPEC> {
        SLEEP_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_bias::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_bias::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_SLEEP_BIAS_SPEC;
impl crate::RegisterSpec for HP_SLEEP_BIAS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_bias::R`](R) reader structure"]
impl crate::Readable for HP_SLEEP_BIAS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_bias::W`](W) writer structure"]
impl crate::Writable for HP_SLEEP_BIAS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_SLEEP_BIAS to value 0x012e_8a00"]
impl crate::Resettable for HP_SLEEP_BIAS_SPEC {
    const RESET_VALUE: u32 = 0x012e_8a00;
}
