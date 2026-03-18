#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - Set 1 to enable timer_group0 apb clock"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Set 1 to enable timer_group0 apb clock"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_EN` reader - Set 0 to reset timer_group0 module"]
pub type RST_EN_R = crate::BitReader;
#[doc = "Field `RST_EN` writer - Set 0 to reset timer_group0 module"]
pub type RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_READY` reader - Query this field after reset timer_group0 wdt module"]
pub type WDT_READY_R = crate::BitReader;
#[doc = "Field `TIMER0_READY` reader - Query this field after reset timer_group0 timer0 module"]
pub type TIMER0_READY_R = crate::BitReader;
#[doc = "Field `TIMER1_READY` reader - reserved"]
pub type TIMER1_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    pub fn rst_en(&self) -> RST_EN_R {
        RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset timer_group0 wdt module"]
    #[inline(always)]
    pub fn wdt_ready(&self) -> WDT_READY_R {
        WDT_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Query this field after reset timer_group0 timer0 module"]
    #[inline(always)]
    pub fn timer0_ready(&self) -> TIMER0_READY_R {
        TIMER0_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reserved"]
    #[inline(always)]
    pub fn timer1_ready(&self) -> TIMER1_READY_R {
        TIMER1_READY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("clk_en", &self.clk_en())
            .field("rst_en", &self.rst_en())
            .field("wdt_ready", &self.wdt_ready())
            .field("timer0_ready", &self.timer0_ready())
            .field("timer1_ready", &self.timer1_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable timer_group0 apb clock"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, CONF_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset timer_group0 module"]
    #[inline(always)]
    pub fn rst_en(&mut self) -> RST_EN_W<'_, CONF_SPEC> {
        RST_EN_W::new(self, 1)
    }
}
#[doc = "TIMERGROUP0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0x1d"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x1d;
}
