#[doc = "Register `LP_SLEEP_LP_DIG_POWER` reader"]
pub type R = crate::R<LP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "Register `LP_SLEEP_LP_DIG_POWER` writer"]
pub type W = crate::W<LP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "Field `LP_SLEEP_BOD_SOURCE_SEL` reader - need_des"]
pub type LP_SLEEP_BOD_SOURCE_SEL_R = crate::BitReader;
#[doc = "Field `LP_SLEEP_BOD_SOURCE_SEL` writer - need_des"]
pub type LP_SLEEP_BOD_SOURCE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_VDDBAT_MODE` reader - need_des"]
pub type LP_SLEEP_VDDBAT_MODE_R = crate::FieldReader;
#[doc = "Field `LP_SLEEP_VDDBAT_MODE` writer - need_des"]
pub type LP_SLEEP_VDDBAT_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_SLEEP_LP_MEM_DSLP` reader - need_des"]
pub type LP_SLEEP_LP_MEM_DSLP_R = crate::BitReader;
#[doc = "Field `LP_SLEEP_LP_MEM_DSLP` writer - need_des"]
pub type LP_SLEEP_LP_MEM_DSLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_SLEEP_PD_LP_PERI_PD_EN` reader - need_des"]
pub type LP_SLEEP_PD_LP_PERI_PD_EN_R = crate::BitReader;
#[doc = "Field `LP_SLEEP_PD_LP_PERI_PD_EN` writer - need_des"]
pub type LP_SLEEP_PD_LP_PERI_PD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_bod_source_sel(&self) -> LP_SLEEP_BOD_SOURCE_SEL_R {
        LP_SLEEP_BOD_SOURCE_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_vddbat_mode(&self) -> LP_SLEEP_VDDBAT_MODE_R {
        LP_SLEEP_VDDBAT_MODE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_lp_mem_dslp(&self) -> LP_SLEEP_LP_MEM_DSLP_R {
        LP_SLEEP_LP_MEM_DSLP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_sleep_pd_lp_peri_pd_en(&self) -> LP_SLEEP_PD_LP_PERI_PD_EN_R {
        LP_SLEEP_PD_LP_PERI_PD_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_SLEEP_LP_DIG_POWER")
            .field(
                "lp_sleep_bod_source_sel",
                &format_args!("{}", self.lp_sleep_bod_source_sel().bit()),
            )
            .field(
                "lp_sleep_vddbat_mode",
                &format_args!("{}", self.lp_sleep_vddbat_mode().bits()),
            )
            .field(
                "lp_sleep_lp_mem_dslp",
                &format_args!("{}", self.lp_sleep_lp_mem_dslp().bit()),
            )
            .field(
                "lp_sleep_pd_lp_peri_pd_en",
                &format_args!("{}", self.lp_sleep_pd_lp_peri_pd_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_SLEEP_LP_DIG_POWER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_bod_source_sel(
        &mut self,
    ) -> LP_SLEEP_BOD_SOURCE_SEL_W<LP_SLEEP_LP_DIG_POWER_SPEC> {
        LP_SLEEP_BOD_SOURCE_SEL_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_vddbat_mode(&mut self) -> LP_SLEEP_VDDBAT_MODE_W<LP_SLEEP_LP_DIG_POWER_SPEC> {
        LP_SLEEP_VDDBAT_MODE_W::new(self, 28)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_lp_mem_dslp(&mut self) -> LP_SLEEP_LP_MEM_DSLP_W<LP_SLEEP_LP_DIG_POWER_SPEC> {
        LP_SLEEP_LP_MEM_DSLP_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_sleep_pd_lp_peri_pd_en(
        &mut self,
    ) -> LP_SLEEP_PD_LP_PERI_PD_EN_W<LP_SLEEP_LP_DIG_POWER_SPEC> {
        LP_SLEEP_PD_LP_PERI_PD_EN_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_dig_power::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_dig_power::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_SLEEP_LP_DIG_POWER_SPEC;
impl crate::RegisterSpec for LP_SLEEP_LP_DIG_POWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_sleep_lp_dig_power::R`](R) reader structure"]
impl crate::Readable for LP_SLEEP_LP_DIG_POWER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_sleep_lp_dig_power::W`](W) writer structure"]
impl crate::Writable for LP_SLEEP_LP_DIG_POWER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_SLEEP_LP_DIG_POWER to value 0"]
impl crate::Resettable for LP_SLEEP_LP_DIG_POWER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
