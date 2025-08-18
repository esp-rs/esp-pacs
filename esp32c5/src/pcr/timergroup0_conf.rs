#[doc = "Register `TIMERGROUP0_CONF` reader"]
pub type R = crate::R<TIMERGROUP0_CONF_SPEC>;
#[doc = "Register `TIMERGROUP0_CONF` writer"]
pub type W = crate::W<TIMERGROUP0_CONF_SPEC>;
#[doc = "Field `TG0_CLK_EN` reader - Set 1 to enable timer_group0 apb clock"]
pub type TG0_CLK_EN_R = crate::BitReader;
#[doc = "Field `TG0_CLK_EN` writer - Set 1 to enable timer_group0 apb clock"]
pub type TG0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_RST_EN` reader - Set 0 to reset timer_group0 module"]
pub type TG0_RST_EN_R = crate::BitReader;
#[doc = "Field `TG0_RST_EN` writer - Set 0 to reset timer_group0 module"]
pub type TG0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG0_WDT_READY` reader - Query this field after reset timer_group0 wdt module"]
pub type TG0_WDT_READY_R = crate::BitReader;
#[doc = "Field `TG0_TIMER0_READY` reader - Query this field after reset timer_group0 timer0 module"]
pub type TG0_TIMER0_READY_R = crate::BitReader;
#[doc = "Field `TG0_TIMER1_READY` reader - Query this field after reset timer_group0 timer1 module"]
pub type TG0_TIMER1_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    pub fn tg0_clk_en(&self) -> TG0_CLK_EN_R {
        TG0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    pub fn tg0_rst_en(&self) -> TG0_RST_EN_R {
        TG0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset timer_group0 wdt module"]
    #[inline(always)]
    pub fn tg0_wdt_ready(&self) -> TG0_WDT_READY_R {
        TG0_WDT_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Query this field after reset timer_group0 timer0 module"]
    #[inline(always)]
    pub fn tg0_timer0_ready(&self) -> TG0_TIMER0_READY_R {
        TG0_TIMER0_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Query this field after reset timer_group0 timer1 module"]
    #[inline(always)]
    pub fn tg0_timer1_ready(&self) -> TG0_TIMER1_READY_R {
        TG0_TIMER1_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMERGROUP0_CONF")
            .field("tg0_clk_en", &self.tg0_clk_en())
            .field("tg0_rst_en", &self.tg0_rst_en())
            .field("tg0_wdt_ready", &self.tg0_wdt_ready())
            .field("tg0_timer0_ready", &self.tg0_timer0_ready())
            .field("tg0_timer1_ready", &self.tg0_timer1_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    pub fn tg0_clk_en(&mut self) -> TG0_CLK_EN_W<'_, TIMERGROUP0_CONF_SPEC> {
        TG0_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    pub fn tg0_rst_en(&mut self) -> TG0_RST_EN_W<'_, TIMERGROUP0_CONF_SPEC> {
        TG0_RST_EN_W::new(self, 1)
    }
}
#[doc = "TIMERGROUP0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`timergroup0_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timergroup0_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMERGROUP0_CONF_SPEC;
impl crate::RegisterSpec for TIMERGROUP0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timergroup0_conf::R`](R) reader structure"]
impl crate::Readable for TIMERGROUP0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timergroup0_conf::W`](W) writer structure"]
impl crate::Writable for TIMERGROUP0_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERGROUP0_CONF to value 0x1d"]
impl crate::Resettable for TIMERGROUP0_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1d;
}
