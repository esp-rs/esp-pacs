#[doc = "Register `LP_AONCLKRST_CLOCKCALI` reader"]
pub type R = crate::R<LP_AONCLKRST_CLOCKCALI_SPEC>;
#[doc = "Register `LP_AONCLKRST_CLOCKCALI` writer"]
pub type W = crate::W<LP_AONCLKRST_CLOCKCALI_SPEC>;
#[doc = "Field `LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON` reader - need_des"]
pub type LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON` writer - need_des"]
pub type LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN` reader - need_des"]
pub type LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN` writer - need_des"]
pub type LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN` reader - need_des"]
pub type LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN` writer - need_des"]
pub type LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_clk_cali_xtal_force_on(
        &self,
    ) -> LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON_R {
        LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_clk_cali_fosc_rst_en(&self) -> LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN_R {
        LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_clk_cali_sosc_rst_en(&self) -> LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN_R {
        LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_CLOCKCALI")
            .field(
                "lp_aonclkrst_lp_clk_cali_xtal_force_on",
                &self.lp_aonclkrst_lp_clk_cali_xtal_force_on(),
            )
            .field(
                "lp_aonclkrst_lp_clk_cali_fosc_rst_en",
                &self.lp_aonclkrst_lp_clk_cali_fosc_rst_en(),
            )
            .field(
                "lp_aonclkrst_lp_clk_cali_sosc_rst_en",
                &self.lp_aonclkrst_lp_clk_cali_sosc_rst_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_clk_cali_xtal_force_on(
        &mut self,
    ) -> LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON_W<'_, LP_AONCLKRST_CLOCKCALI_SPEC> {
        LP_AONCLKRST_LP_CLK_CALI_XTAL_FORCE_ON_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_clk_cali_fosc_rst_en(
        &mut self,
    ) -> LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN_W<'_, LP_AONCLKRST_CLOCKCALI_SPEC> {
        LP_AONCLKRST_LP_CLK_CALI_FOSC_RST_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_clk_cali_sosc_rst_en(
        &mut self,
    ) -> LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN_W<'_, LP_AONCLKRST_CLOCKCALI_SPEC> {
        LP_AONCLKRST_LP_CLK_CALI_SOSC_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_clockcali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_clockcali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_CLOCKCALI_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_CLOCKCALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_clockcali::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_CLOCKCALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_clockcali::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_CLOCKCALI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_CLOCKCALI to value 0"]
impl crate::Resettable for LP_AONCLKRST_CLOCKCALI_SPEC {}
