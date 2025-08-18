#[doc = "Register `MODEM_LP_TIMER_CONF` reader"]
pub type R = crate::R<MODEM_LP_TIMER_CONF_SPEC>;
#[doc = "Register `MODEM_LP_TIMER_CONF` writer"]
pub type W = crate::W<MODEM_LP_TIMER_CONF_SPEC>;
#[doc = "Field `LP_TIMER_SEL_RTC_SLOW` reader - ."]
pub type LP_TIMER_SEL_RTC_SLOW_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_RTC_SLOW` writer - ."]
pub type LP_TIMER_SEL_RTC_SLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_SEL_8M` reader - ."]
pub type LP_TIMER_SEL_8M_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_8M` writer - ."]
pub type LP_TIMER_SEL_8M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_SEL_XTAL` reader - ."]
pub type LP_TIMER_SEL_XTAL_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_XTAL` writer - ."]
pub type LP_TIMER_SEL_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_SEL_XTAL32K` reader - ."]
pub type LP_TIMER_SEL_XTAL32K_R = crate::BitReader;
#[doc = "Field `LP_TIMER_SEL_XTAL32K` writer - ."]
pub type LP_TIMER_SEL_XTAL32K_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_TIMER_CLK_DIV_NUM` reader - ."]
pub type LP_TIMER_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_TIMER_CLK_DIV_NUM` writer - ."]
pub type LP_TIMER_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_rtc_slow(&self) -> LP_TIMER_SEL_RTC_SLOW_R {
        LP_TIMER_SEL_RTC_SLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_8m(&self) -> LP_TIMER_SEL_8M_R {
        LP_TIMER_SEL_8M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_xtal(&self) -> LP_TIMER_SEL_XTAL_R {
        LP_TIMER_SEL_XTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_xtal32k(&self) -> LP_TIMER_SEL_XTAL32K_R {
        LP_TIMER_SEL_XTAL32K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - ."]
    #[inline(always)]
    pub fn lp_timer_clk_div_num(&self) -> LP_TIMER_CLK_DIV_NUM_R {
        LP_TIMER_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEM_LP_TIMER_CONF")
            .field("lp_timer_sel_rtc_slow", &self.lp_timer_sel_rtc_slow())
            .field("lp_timer_sel_8m", &self.lp_timer_sel_8m())
            .field("lp_timer_sel_xtal", &self.lp_timer_sel_xtal())
            .field("lp_timer_sel_xtal32k", &self.lp_timer_sel_xtal32k())
            .field("lp_timer_clk_div_num", &self.lp_timer_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_rtc_slow(
        &mut self,
    ) -> LP_TIMER_SEL_RTC_SLOW_W<'_, MODEM_LP_TIMER_CONF_SPEC> {
        LP_TIMER_SEL_RTC_SLOW_W::new(self, 0)
    }
    #[doc = "Bit 1 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_8m(&mut self) -> LP_TIMER_SEL_8M_W<'_, MODEM_LP_TIMER_CONF_SPEC> {
        LP_TIMER_SEL_8M_W::new(self, 1)
    }
    #[doc = "Bit 2 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_xtal(&mut self) -> LP_TIMER_SEL_XTAL_W<'_, MODEM_LP_TIMER_CONF_SPEC> {
        LP_TIMER_SEL_XTAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - ."]
    #[inline(always)]
    pub fn lp_timer_sel_xtal32k(&mut self) -> LP_TIMER_SEL_XTAL32K_W<'_, MODEM_LP_TIMER_CONF_SPEC> {
        LP_TIMER_SEL_XTAL32K_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - ."]
    #[inline(always)]
    pub fn lp_timer_clk_div_num(&mut self) -> LP_TIMER_CLK_DIV_NUM_W<'_, MODEM_LP_TIMER_CONF_SPEC> {
        LP_TIMER_CLK_DIV_NUM_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_lp_timer_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_lp_timer_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEM_LP_TIMER_CONF_SPEC;
impl crate::RegisterSpec for MODEM_LP_TIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_lp_timer_conf::R`](R) reader structure"]
impl crate::Readable for MODEM_LP_TIMER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modem_lp_timer_conf::W`](W) writer structure"]
impl crate::Writable for MODEM_LP_TIMER_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEM_LP_TIMER_CONF to value 0"]
impl crate::Resettable for MODEM_LP_TIMER_CONF_SPEC {}
