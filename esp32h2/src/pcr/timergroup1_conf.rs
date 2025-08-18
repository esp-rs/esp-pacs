#[doc = "Register `TIMERGROUP1_CONF` reader"]
pub type R = crate::R<TIMERGROUP1_CONF_SPEC>;
#[doc = "Register `TIMERGROUP1_CONF` writer"]
pub type W = crate::W<TIMERGROUP1_CONF_SPEC>;
#[doc = "Field `TG1_CLK_EN` reader - Set 1 to enable timer_group1 apb clock"]
pub type TG1_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG1_CLK_EN` writer - Set 1 to enable timer_group1 apb clock"]
pub type TG1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG1_RST_EN` reader - Set 0 to reset timer_group1 module"]
pub type TG1_RST_EN_R = crate::BitReader;
#[doc = "Field `TG1_RST_EN` writer - Set 0 to reset timer_group1 module"]
pub type TG1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG1_WDT_READY` reader - Query this field after reset timer_group1 wdt module"]
pub type TG1_WDT_READY_R = crate::BitReader;
#[doc = "Field `TG1_TIMER0_READY` reader - Query this field after reset timer_group1 timer0 module"]
pub type TG1_TIMER0_READY_R = crate::BitReader;
#[doc = "Field `TG1_TIMER1_READY` reader - reserved"]
pub type TG1_TIMER1_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable timer_group1 apb clock"]
    #[inline(always)]
    pub fn tg1_clk_en(&self) -> TG1_CLK_EN_R {
        TG1_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group1 module"]
    #[inline(always)]
    pub fn tg1_rst_en(&self) -> TG1_RST_EN_R {
        TG1_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset timer_group1 wdt module"]
    #[inline(always)]
    pub fn tg1_wdt_ready(&self) -> TG1_WDT_READY_R {
        TG1_WDT_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Query this field after reset timer_group1 timer0 module"]
    #[inline(always)]
    pub fn tg1_timer0_ready(&self) -> TG1_TIMER0_READY_R {
        TG1_TIMER0_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn tg1_timer1_ready(&self) -> TG1_TIMER1_READY_R {
        TG1_TIMER1_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP1_CONF")
            .field("tg1_clk_en", &self.tg1_clk_en())
            .field("tg1_rst_en", &self.tg1_rst_en())
            .field("tg1_wdt_ready", &self.tg1_wdt_ready())
            .field("tg1_timer0_ready", &self.tg1_timer0_ready())
            .field("tg1_timer1_ready", &self.tg1_timer1_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable timer_group1 apb clock"]
    #[inline(always)]
    pub fn tg1_clk_en(&mut self) -> TG1_CLK_EN_W<'_, TIMERGROUP1_CONF_SPEC> {
        TG1_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group1 module"]
    #[inline(always)]
    pub fn tg1_rst_en(&mut self) -> TG1_RST_EN_W<'_, TIMERGROUP1_CONF_SPEC> {
        TG1_RST_EN_W::new(self, 1)
    }
}
#[doc = "TIMERGROUP1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup1_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup1_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGROUP1_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergroup1_conf::R`](R) reader structure"]
impl crate::Readable for TIMERGROUP1_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergroup1_conf::W`](W) writer structure"]
impl crate::Writable for TIMERGROUP1_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERGROUP1_CONF to value 0x1d"]
impl crate::Resettable for TIMERGROUP1_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1d;
}
