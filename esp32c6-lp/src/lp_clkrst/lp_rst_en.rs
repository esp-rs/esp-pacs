#[doc = "Register `LP_RST_EN` reader"]
pub type R = crate::R<LP_RST_EN_SPEC>;
#[doc = "Register `LP_RST_EN` writer"]
pub type W = crate::W<LP_RST_EN_SPEC>;
#[doc = "Field `AON_EFUSE_CORE_RESET_EN` reader - need_des"]
pub type AON_EFUSE_CORE_RESET_EN_R = crate::BitReader;
#[doc = "Field `AON_EFUSE_CORE_RESET_EN` writer - need_des"]
pub type AON_EFUSE_CORE_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_RESET_EN` reader - need_des"]
pub type LP_TIMER_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_TIMER_RESET_EN` writer - need_des"]
pub type LP_TIMER_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_RESET_EN` reader - need_des"]
pub type WDT_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_RESET_EN` writer - need_des"]
pub type WDT_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_PERI_RESET_EN` reader - need_des"]
pub type ANA_PERI_RESET_EN_R = crate::BitReader;
#[doc = "Field `ANA_PERI_RESET_EN` writer - need_des"]
pub type ANA_PERI_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn aon_efuse_core_reset_en(&self) -> AON_EFUSE_CORE_RESET_EN_R {
        AON_EFUSE_CORE_RESET_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_timer_reset_en(&self) -> LP_TIMER_RESET_EN_R {
        LP_TIMER_RESET_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn wdt_reset_en(&self) -> WDT_RESET_EN_R {
        WDT_RESET_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn ana_peri_reset_en(&self) -> ANA_PERI_RESET_EN_R {
        ANA_PERI_RESET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RST_EN")
            .field(
                "aon_efuse_core_reset_en",
                &format_args!("{}", self.aon_efuse_core_reset_en().bit()),
            )
            .field(
                "lp_timer_reset_en",
                &format_args!("{}", self.lp_timer_reset_en().bit()),
            )
            .field(
                "wdt_reset_en",
                &format_args!("{}", self.wdt_reset_en().bit()),
            )
            .field(
                "ana_peri_reset_en",
                &format_args!("{}", self.ana_peri_reset_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_RST_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn aon_efuse_core_reset_en(&mut self) -> AON_EFUSE_CORE_RESET_EN_W<LP_RST_EN_SPEC> {
        AON_EFUSE_CORE_RESET_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_timer_reset_en(&mut self) -> LP_TIMER_RESET_EN_W<LP_RST_EN_SPEC> {
        LP_TIMER_RESET_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset_en(&mut self) -> WDT_RESET_EN_W<LP_RST_EN_SPEC> {
        WDT_RESET_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_peri_reset_en(&mut self) -> ANA_PERI_RESET_EN_W<LP_RST_EN_SPEC> {
        ANA_PERI_RESET_EN_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_rst_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rst_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_RST_EN_SPEC;
impl crate::RegisterSpec for LP_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rst_en::R`](R) reader structure"]
impl crate::Readable for LP_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_rst_en::W`](W) writer structure"]
impl crate::Writable for LP_RST_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_RST_EN to value 0"]
impl crate::Resettable for LP_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
