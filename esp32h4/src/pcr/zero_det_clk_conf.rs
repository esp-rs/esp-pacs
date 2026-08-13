#[doc = "Register `ZERO_DET_CLK_CONF` reader"]
pub type R = crate::R<ZERO_DET_CLK_CONF_SPEC>;
#[doc = "Register `ZERO_DET_CLK_CONF` writer"]
pub type W = crate::W<ZERO_DET_CLK_CONF_SPEC>;
#[doc = "Field `ZERO_DET_FUNC_CLK_SEL` reader - Configures the clock source of ZERO DETECT.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
pub type ZERO_DET_FUNC_CLK_SEL_R = crate::FieldReader;
#[doc = "Field `ZERO_DET_FUNC_CLK_SEL` writer - Configures the clock source of ZERO DETECT.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
pub type ZERO_DET_FUNC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ZERO_DET_FUNC_CLK_EN` reader - Set 1 to enable zero_det function clock"]
pub type ZERO_DET_FUNC_CLK_EN_R = crate::BitReader;
#[doc = "Field `ZERO_DET_FUNC_CLK_EN` writer - Set 1 to enable zero_det function clock"]
pub type ZERO_DET_FUNC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 20:21 - Configures the clock source of ZERO DETECT.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
    #[inline(always)]
    pub fn zero_det_func_clk_sel(&self) -> ZERO_DET_FUNC_CLK_SEL_R {
        ZERO_DET_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable zero_det function clock"]
    #[inline(always)]
    pub fn zero_det_func_clk_en(&self) -> ZERO_DET_FUNC_CLK_EN_R {
        ZERO_DET_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZERO_DET_CLK_CONF")
            .field("zero_det_func_clk_sel", &self.zero_det_func_clk_sel())
            .field("zero_det_func_clk_en", &self.zero_det_func_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 20:21 - Configures the clock source of ZERO DETECT.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F80M_CLK\\\\"]
    #[inline(always)]
    pub fn zero_det_func_clk_sel(&mut self) -> ZERO_DET_FUNC_CLK_SEL_W<'_, ZERO_DET_CLK_CONF_SPEC> {
        ZERO_DET_FUNC_CLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable zero_det function clock"]
    #[inline(always)]
    pub fn zero_det_func_clk_en(&mut self) -> ZERO_DET_FUNC_CLK_EN_W<'_, ZERO_DET_CLK_CONF_SPEC> {
        ZERO_DET_FUNC_CLK_EN_W::new(self, 22)
    }
}
#[doc = "ZERO_DET_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`zero_det_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zero_det_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZERO_DET_CLK_CONF_SPEC;
impl crate::RegisterSpec for ZERO_DET_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zero_det_clk_conf::R`](R) reader structure"]
impl crate::Readable for ZERO_DET_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`zero_det_clk_conf::W`](W) writer structure"]
impl crate::Writable for ZERO_DET_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ZERO_DET_CLK_CONF to value 0"]
impl crate::Resettable for ZERO_DET_CLK_CONF_SPEC {}
