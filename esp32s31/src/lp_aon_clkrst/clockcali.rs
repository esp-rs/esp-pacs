#[doc = "Register `CLOCKCALI` reader"]
pub type R = crate::R<CLOCKCALI_SPEC>;
#[doc = "Register `CLOCKCALI` writer"]
pub type W = crate::W<CLOCKCALI_SPEC>;
#[doc = "Field `LP_CLK_CALI_XTAL_FORCE_ON` reader - need_des"]
pub type LP_CLK_CALI_XTAL_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_CLK_CALI_XTAL_FORCE_ON` writer - need_des"]
pub type LP_CLK_CALI_XTAL_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CLK_CALI_FOSC_RST_EN` reader - need_des"]
pub type LP_CLK_CALI_FOSC_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_CLK_CALI_FOSC_RST_EN` writer - need_des"]
pub type LP_CLK_CALI_FOSC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_CLK_CALI_SOSC_RST_EN` reader - need_des"]
pub type LP_CLK_CALI_SOSC_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_CLK_CALI_SOSC_RST_EN` writer - need_des"]
pub type LP_CLK_CALI_SOSC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_clk_cali_xtal_force_on(&self) -> LP_CLK_CALI_XTAL_FORCE_ON_R {
        LP_CLK_CALI_XTAL_FORCE_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_clk_cali_fosc_rst_en(&self) -> LP_CLK_CALI_FOSC_RST_EN_R {
        LP_CLK_CALI_FOSC_RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_clk_cali_sosc_rst_en(&self) -> LP_CLK_CALI_SOSC_RST_EN_R {
        LP_CLK_CALI_SOSC_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLOCKCALI")
            .field(
                "lp_clk_cali_xtal_force_on",
                &self.lp_clk_cali_xtal_force_on(),
            )
            .field("lp_clk_cali_fosc_rst_en", &self.lp_clk_cali_fosc_rst_en())
            .field("lp_clk_cali_sosc_rst_en", &self.lp_clk_cali_sosc_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_clk_cali_xtal_force_on(&mut self) -> LP_CLK_CALI_XTAL_FORCE_ON_W<'_, CLOCKCALI_SPEC> {
        LP_CLK_CALI_XTAL_FORCE_ON_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_clk_cali_fosc_rst_en(&mut self) -> LP_CLK_CALI_FOSC_RST_EN_W<'_, CLOCKCALI_SPEC> {
        LP_CLK_CALI_FOSC_RST_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_clk_cali_sosc_rst_en(&mut self) -> LP_CLK_CALI_SOSC_RST_EN_W<'_, CLOCKCALI_SPEC> {
        LP_CLK_CALI_SOSC_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clockcali::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clockcali::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCKCALI_SPEC;
impl crate::RegisterSpec for CLOCKCALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clockcali::R`](R) reader structure"]
impl crate::Readable for CLOCKCALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clockcali::W`](W) writer structure"]
impl crate::Writable for CLOCKCALI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCKCALI to value 0"]
impl crate::Resettable for CLOCKCALI_SPEC {}
