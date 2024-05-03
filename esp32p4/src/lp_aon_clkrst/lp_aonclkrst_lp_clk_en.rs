#[doc = "Register `LP_AONCLKRST_LP_CLK_EN` reader"]
pub type R = crate::R<LP_AONCLKRST_LP_CLK_EN_SPEC>;
#[doc = "Register `LP_AONCLKRST_LP_CLK_EN` writer"]
pub type W = crate::W<LP_AONCLKRST_LP_CLK_EN_SPEC>;
#[doc = "Field `LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_CK_EN_LP_RAM` reader - need_des"]
pub type LP_AONCLKRST_CK_EN_LP_RAM_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_CK_EN_LP_RAM` writer - need_des"]
pub type LP_AONCLKRST_CK_EN_LP_RAM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_ETM_EVENT_TICK_EN` reader - need_des"]
pub type LP_AONCLKRST_ETM_EVENT_TICK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_ETM_EVENT_TICK_EN` writer - need_des"]
pub type LP_AONCLKRST_ETM_EVENT_TICK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_PLL8M_CLK_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_PLL8M_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_PLL8M_CLK_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_PLL8M_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_XTAL_CLK_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_XTAL_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_XTAL_CLK_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_XTAL_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_FOSC_CLK_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_FOSC_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_FOSC_CLK_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_FOSC_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_rtc_xtal_force_on(&self) -> LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_R {
        LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_ck_en_lp_ram(&self) -> LP_AONCLKRST_CK_EN_LP_RAM_R {
        LP_AONCLKRST_CK_EN_LP_RAM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_etm_event_tick_en(&self) -> LP_AONCLKRST_ETM_EVENT_TICK_EN_R {
        LP_AONCLKRST_ETM_EVENT_TICK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_pll8m_clk_force_on(&self) -> LP_AONCLKRST_PLL8M_CLK_FORCE_ON_R {
        LP_AONCLKRST_PLL8M_CLK_FORCE_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_xtal_clk_force_on(&self) -> LP_AONCLKRST_XTAL_CLK_FORCE_ON_R {
        LP_AONCLKRST_XTAL_CLK_FORCE_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_fosc_clk_force_on(&self) -> LP_AONCLKRST_FOSC_CLK_FORCE_ON_R {
        LP_AONCLKRST_FOSC_CLK_FORCE_ON_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_LP_CLK_EN")
            .field(
                "lp_aonclkrst_lp_rtc_xtal_force_on",
                &self.lp_aonclkrst_lp_rtc_xtal_force_on().bit(),
            )
            .field(
                "lp_aonclkrst_ck_en_lp_ram",
                &self.lp_aonclkrst_ck_en_lp_ram().bit(),
            )
            .field(
                "lp_aonclkrst_etm_event_tick_en",
                &self.lp_aonclkrst_etm_event_tick_en().bit(),
            )
            .field(
                "lp_aonclkrst_pll8m_clk_force_on",
                &self.lp_aonclkrst_pll8m_clk_force_on().bit(),
            )
            .field(
                "lp_aonclkrst_xtal_clk_force_on",
                &self.lp_aonclkrst_xtal_clk_force_on().bit(),
            )
            .field(
                "lp_aonclkrst_fosc_clk_force_on",
                &self.lp_aonclkrst_fosc_clk_force_on().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_AONCLKRST_LP_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_lp_rtc_xtal_force_on(
        &mut self,
    ) -> LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_W<LP_AONCLKRST_LP_CLK_EN_SPEC> {
        LP_AONCLKRST_LP_RTC_XTAL_FORCE_ON_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_ck_en_lp_ram(
        &mut self,
    ) -> LP_AONCLKRST_CK_EN_LP_RAM_W<LP_AONCLKRST_LP_CLK_EN_SPEC> {
        LP_AONCLKRST_CK_EN_LP_RAM_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_etm_event_tick_en(
        &mut self,
    ) -> LP_AONCLKRST_ETM_EVENT_TICK_EN_W<LP_AONCLKRST_LP_CLK_EN_SPEC> {
        LP_AONCLKRST_ETM_EVENT_TICK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_pll8m_clk_force_on(
        &mut self,
    ) -> LP_AONCLKRST_PLL8M_CLK_FORCE_ON_W<LP_AONCLKRST_LP_CLK_EN_SPEC> {
        LP_AONCLKRST_PLL8M_CLK_FORCE_ON_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_xtal_clk_force_on(
        &mut self,
    ) -> LP_AONCLKRST_XTAL_CLK_FORCE_ON_W<LP_AONCLKRST_LP_CLK_EN_SPEC> {
        LP_AONCLKRST_XTAL_CLK_FORCE_ON_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_fosc_clk_force_on(
        &mut self,
    ) -> LP_AONCLKRST_FOSC_CLK_FORCE_ON_W<LP_AONCLKRST_LP_CLK_EN_SPEC> {
        LP_AONCLKRST_FOSC_CLK_FORCE_ON_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_lp_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_LP_CLK_EN_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_LP_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_lp_clk_en::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_LP_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_lp_clk_en::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_LP_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_LP_CLK_EN to value 0x0800_0000"]
impl crate::Resettable for LP_AONCLKRST_LP_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0x0800_0000;
}
