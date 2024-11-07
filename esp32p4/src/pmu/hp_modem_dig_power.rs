#[doc = "Register `HP_MODEM_DIG_POWER` reader"]
pub type R = crate::R<HP_MODEM_DIG_POWER_SPEC>;
#[doc = "Register `HP_MODEM_DIG_POWER` writer"]
pub type W = crate::W<HP_MODEM_DIG_POWER_SPEC>;
#[doc = "Field `HP_MODEM_DCDC_SWITCH_PD_EN` reader - need_des"]
pub type HP_MODEM_DCDC_SWITCH_PD_EN_R = crate::BitReader;
#[doc = "Field `HP_MODEM_DCDC_SWITCH_PD_EN` writer - need_des"]
pub type HP_MODEM_DCDC_SWITCH_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_HP_MEM_DSLP` writer - need_des"]
pub type HP_MODEM_HP_MEM_DSLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_PD_HP_MEM_PD_EN` writer - need_des"]
pub type HP_MODEM_PD_HP_MEM_PD_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HP_MODEM_PD_HP_WIFI_PD_EN` writer - need_des"]
pub type HP_MODEM_PD_HP_WIFI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_PD_HP_CPU_PD_EN` writer - need_des"]
pub type HP_MODEM_PD_HP_CPU_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_PD_CNNT_PD_EN` writer - need_des"]
pub type HP_MODEM_PD_CNNT_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_MODEM_PD_TOP_PD_EN` writer - need_des"]
pub type HP_MODEM_PD_TOP_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dcdc_switch_pd_en(&self) -> HP_MODEM_DCDC_SWITCH_PD_EN_R {
        HP_MODEM_DCDC_SWITCH_PD_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_DIG_POWER")
            .field(
                "hp_modem_dcdc_switch_pd_en",
                &self.hp_modem_dcdc_switch_pd_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn hp_modem_dcdc_switch_pd_en(
        &mut self,
    ) -> HP_MODEM_DCDC_SWITCH_PD_EN_W<HP_MODEM_DIG_POWER_SPEC> {
        HP_MODEM_DCDC_SWITCH_PD_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn hp_modem_hp_mem_dslp(&mut self) -> HP_MODEM_HP_MEM_DSLP_W<HP_MODEM_DIG_POWER_SPEC> {
        HP_MODEM_HP_MEM_DSLP_W::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_hp_mem_pd_en(
        &mut self,
    ) -> HP_MODEM_PD_HP_MEM_PD_EN_W<HP_MODEM_DIG_POWER_SPEC> {
        HP_MODEM_PD_HP_MEM_PD_EN_W::new(self, 23)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_hp_wifi_pd_en(
        &mut self,
    ) -> HP_MODEM_PD_HP_WIFI_PD_EN_W<HP_MODEM_DIG_POWER_SPEC> {
        HP_MODEM_PD_HP_WIFI_PD_EN_W::new(self, 27)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_hp_cpu_pd_en(
        &mut self,
    ) -> HP_MODEM_PD_HP_CPU_PD_EN_W<HP_MODEM_DIG_POWER_SPEC> {
        HP_MODEM_PD_HP_CPU_PD_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_cnnt_pd_en(&mut self) -> HP_MODEM_PD_CNNT_PD_EN_W<HP_MODEM_DIG_POWER_SPEC> {
        HP_MODEM_PD_CNNT_PD_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_pd_top_pd_en(&mut self) -> HP_MODEM_PD_TOP_PD_EN_W<HP_MODEM_DIG_POWER_SPEC> {
        HP_MODEM_PD_TOP_PD_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_modem_dig_power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_dig_power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_MODEM_DIG_POWER_SPEC;
impl crate::RegisterSpec for HP_MODEM_DIG_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_modem_dig_power::R`](R) reader structure"]
impl crate::Readable for HP_MODEM_DIG_POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_modem_dig_power::W`](W) writer structure"]
impl crate::Writable for HP_MODEM_DIG_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_MODEM_DIG_POWER to value 0"]
impl crate::Resettable for HP_MODEM_DIG_POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
