#[doc = "Register `HP_SLEEP_LP_DIG_POWER` reader"]
pub type R = crate::R<HP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "Register `HP_SLEEP_LP_DIG_POWER` writer"]
pub type W = crate::W<HP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "Field `HP_SLEEP_LP_PAD_SLP_SEL` reader - need_des"]
pub type HP_SLEEP_LP_PAD_SLP_SEL_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_LP_PAD_SLP_SEL` writer - need_des"]
pub type HP_SLEEP_LP_PAD_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_BOD_SOURCE_SEL` reader - need_des"]
pub type HP_SLEEP_BOD_SOURCE_SEL_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_BOD_SOURCE_SEL` writer - need_des"]
pub type HP_SLEEP_BOD_SOURCE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_VDDBAT_MODE` reader - need_des"]
pub type HP_SLEEP_VDDBAT_MODE_R = crate::FieldReader;
#[doc = "Field `HP_SLEEP_VDDBAT_MODE` writer - need_des"]
pub type HP_SLEEP_VDDBAT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HP_SLEEP_LP_MEM_DSLP` reader - need_des"]
pub type HP_SLEEP_LP_MEM_DSLP_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_LP_MEM_DSLP` writer - need_des"]
pub type HP_SLEEP_LP_MEM_DSLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SLEEP_PD_LP_PERI_PD_EN` reader - need_des"]
pub type HP_SLEEP_PD_LP_PERI_PD_EN_R = crate::BitReader;
#[doc = "Field `HP_SLEEP_PD_LP_PERI_PD_EN` writer - need_des"]
pub type HP_SLEEP_PD_LP_PERI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_pad_slp_sel(&self) -> HP_SLEEP_LP_PAD_SLP_SEL_R {
        HP_SLEEP_LP_PAD_SLP_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_bod_source_sel(&self) -> HP_SLEEP_BOD_SOURCE_SEL_R {
        HP_SLEEP_BOD_SOURCE_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_vddbat_mode(&self) -> HP_SLEEP_VDDBAT_MODE_R {
        HP_SLEEP_VDDBAT_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_lp_mem_dslp(&self) -> HP_SLEEP_LP_MEM_DSLP_R {
        HP_SLEEP_LP_MEM_DSLP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sleep_pd_lp_peri_pd_en(&self) -> HP_SLEEP_PD_LP_PERI_PD_EN_R {
        HP_SLEEP_PD_LP_PERI_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_SLEEP_LP_DIG_POWER")
            .field("hp_sleep_lp_pad_slp_sel", &self.hp_sleep_lp_pad_slp_sel())
            .field("hp_sleep_bod_source_sel", &self.hp_sleep_bod_source_sel())
            .field("hp_sleep_vddbat_mode", &self.hp_sleep_vddbat_mode())
            .field("hp_sleep_lp_mem_dslp", &self.hp_sleep_lp_mem_dslp())
            .field(
                "hp_sleep_pd_lp_peri_pd_en",
                &self.hp_sleep_pd_lp_peri_pd_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_lp_pad_slp_sel(
        &mut self,
    ) -> HP_SLEEP_LP_PAD_SLP_SEL_W<HP_SLEEP_LP_DIG_POWER_SPEC> {
        HP_SLEEP_LP_PAD_SLP_SEL_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_bod_source_sel(
        &mut self,
    ) -> HP_SLEEP_BOD_SOURCE_SEL_W<HP_SLEEP_LP_DIG_POWER_SPEC> {
        HP_SLEEP_BOD_SOURCE_SEL_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_vddbat_mode(&mut self) -> HP_SLEEP_VDDBAT_MODE_W<HP_SLEEP_LP_DIG_POWER_SPEC> {
        HP_SLEEP_VDDBAT_MODE_W::new(self, 28)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_lp_mem_dslp(&mut self) -> HP_SLEEP_LP_MEM_DSLP_W<HP_SLEEP_LP_DIG_POWER_SPEC> {
        HP_SLEEP_LP_MEM_DSLP_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_sleep_pd_lp_peri_pd_en(
        &mut self,
    ) -> HP_SLEEP_PD_LP_PERI_PD_EN_W<HP_SLEEP_LP_DIG_POWER_SPEC> {
        HP_SLEEP_PD_LP_PERI_PD_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_lp_dig_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_lp_dig_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_SLEEP_LP_DIG_POWER_SPEC;
impl crate::RegisterSpec for HP_SLEEP_LP_DIG_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_sleep_lp_dig_power::R`](R) reader structure"]
impl crate::Readable for HP_SLEEP_LP_DIG_POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_sleep_lp_dig_power::W`](W) writer structure"]
impl crate::Writable for HP_SLEEP_LP_DIG_POWER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_SLEEP_LP_DIG_POWER to value 0"]
impl crate::Resettable for HP_SLEEP_LP_DIG_POWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
