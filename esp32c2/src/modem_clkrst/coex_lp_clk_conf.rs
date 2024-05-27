#[doc = "Register `COEX_LP_CLK_CONF` reader"]
pub type R = crate::R<COEX_LP_CLK_CONF_SPEC>;
#[doc = "Register `COEX_LP_CLK_CONF` writer"]
pub type W = crate::W<COEX_LP_CLK_CONF_SPEC>;
#[doc = "Field `COEX_LPCLK_SEL_RTC_SLOW` reader - ."]
pub type COEX_LPCLK_SEL_RTC_SLOW_R = crate::BitReader;
#[doc = "Field `COEX_LPCLK_SEL_RTC_SLOW` writer - ."]
pub type COEX_LPCLK_SEL_RTC_SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COEX_LPCLK_SEL_8M` reader - ."]
pub type COEX_LPCLK_SEL_8M_R = crate::BitReader;
#[doc = "Field `COEX_LPCLK_SEL_8M` writer - ."]
pub type COEX_LPCLK_SEL_8M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COEX_LPCLK_SEL_XTAL` reader - ."]
pub type COEX_LPCLK_SEL_XTAL_R = crate::BitReader;
#[doc = "Field `COEX_LPCLK_SEL_XTAL` writer - ."]
pub type COEX_LPCLK_SEL_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COEX_LPCLK_SEL_XTAL32K` reader - ."]
pub type COEX_LPCLK_SEL_XTAL32K_R = crate::BitReader;
#[doc = "Field `COEX_LPCLK_SEL_XTAL32K` writer - ."]
pub type COEX_LPCLK_SEL_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COEX_LPCLK_DIV_NUM` reader - ."]
pub type COEX_LPCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `COEX_LPCLK_DIV_NUM` writer - ."]
pub type COEX_LPCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn coex_lpclk_sel_rtc_slow(&self) -> COEX_LPCLK_SEL_RTC_SLOW_R {
        COEX_LPCLK_SEL_RTC_SLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn coex_lpclk_sel_8m(&self) -> COEX_LPCLK_SEL_8M_R {
        COEX_LPCLK_SEL_8M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn coex_lpclk_sel_xtal(&self) -> COEX_LPCLK_SEL_XTAL_R {
        COEX_LPCLK_SEL_XTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn coex_lpclk_sel_xtal32k(&self) -> COEX_LPCLK_SEL_XTAL32K_R {
        COEX_LPCLK_SEL_XTAL32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - ."]
    #[inline(always)]
    pub fn coex_lpclk_div_num(&self) -> COEX_LPCLK_DIV_NUM_R {
        COEX_LPCLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COEX_LP_CLK_CONF")
            .field("coex_lpclk_sel_rtc_slow", &self.coex_lpclk_sel_rtc_slow())
            .field("coex_lpclk_sel_8m", &self.coex_lpclk_sel_8m())
            .field("coex_lpclk_sel_xtal", &self.coex_lpclk_sel_xtal())
            .field("coex_lpclk_sel_xtal32k", &self.coex_lpclk_sel_xtal32k())
            .field("coex_lpclk_div_num", &self.coex_lpclk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    #[must_use]
    pub fn coex_lpclk_sel_rtc_slow(&mut self) -> COEX_LPCLK_SEL_RTC_SLOW_W<COEX_LP_CLK_CONF_SPEC> {
        COEX_LPCLK_SEL_RTC_SLOW_W::new(self, 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    #[must_use]
    pub fn coex_lpclk_sel_8m(&mut self) -> COEX_LPCLK_SEL_8M_W<COEX_LP_CLK_CONF_SPEC> {
        COEX_LPCLK_SEL_8M_W::new(self, 1)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    #[must_use]
    pub fn coex_lpclk_sel_xtal(&mut self) -> COEX_LPCLK_SEL_XTAL_W<COEX_LP_CLK_CONF_SPEC> {
        COEX_LPCLK_SEL_XTAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    #[must_use]
    pub fn coex_lpclk_sel_xtal32k(&mut self) -> COEX_LPCLK_SEL_XTAL32K_W<COEX_LP_CLK_CONF_SPEC> {
        COEX_LPCLK_SEL_XTAL32K_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - ."]
    #[inline(always)]
    #[must_use]
    pub fn coex_lpclk_div_num(&mut self) -> COEX_LPCLK_DIV_NUM_W<COEX_LP_CLK_CONF_SPEC> {
        COEX_LPCLK_DIV_NUM_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coex_lp_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_lp_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COEX_LP_CLK_CONF_SPEC;
impl crate::RegisterSpec for COEX_LP_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_lp_clk_conf::R`](R) reader structure"]
impl crate::Readable for COEX_LP_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`coex_lp_clk_conf::W`](W) writer structure"]
impl crate::Writable for COEX_LP_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COEX_LP_CLK_CONF to value 0"]
impl crate::Resettable for COEX_LP_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
