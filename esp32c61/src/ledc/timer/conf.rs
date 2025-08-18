#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `TIMER_DUTY_RES` reader - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
pub type TIMER_DUTY_RES_R = crate::FieldReader;
#[doc = "Field `TIMER_DUTY_RES` writer - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
pub type TIMER_DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLK_DIV_TIMER` reader - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
pub type CLK_DIV_TIMER_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_DIV_TIMER` writer - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
pub type CLK_DIV_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `TIMER_PAUSE` reader - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
pub type TIMER_PAUSE_R = crate::BitReader;
#[doc = "Field `TIMER_PAUSE` writer - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
pub type TIMER_PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_RST` reader - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
pub type TIMER_RST_R = crate::BitReader;
#[doc = "Field `TIMER_RST` writer - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
pub type TIMER_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICK_SEL_TIMER` reader - Configures which clock is timer %s selected. Unused."]
pub type TICK_SEL_TIMER_R = crate::BitReader;
#[doc = "Field `TICK_SEL_TIMER` writer - Configures which clock is timer %s selected. Unused."]
pub type TICK_SEL_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_PARA_UP` writer - Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\\\0: Invalid. No effect\\\\1: Update"]
pub type TIMER_PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
    #[inline(always)]
    pub fn timer_duty_res(&self) -> TIMER_DUTY_RES_R {
        TIMER_DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div_timer(&self) -> CLK_DIV_TIMER_R {
        CLK_DIV_TIMER_R::new((self.bits >> 5) & 0x0003_ffff)
    }
    #[doc = "Bit 23 - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
    #[inline(always)]
    pub fn timer_pause(&self) -> TIMER_PAUSE_R {
        TIMER_PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
    #[inline(always)]
    pub fn timer_rst(&self) -> TIMER_RST_R {
        TIMER_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures which clock is timer %s selected. Unused."]
    #[inline(always)]
    pub fn tick_sel_timer(&self) -> TICK_SEL_TIMER_R {
        TICK_SEL_TIMER_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("timer_duty_res", &self.timer_duty_res())
            .field("clk_div_timer", &self.clk_div_timer())
            .field("timer_pause", &self.timer_pause())
            .field("timer_rst", &self.timer_rst())
            .field("tick_sel_timer", &self.tick_sel_timer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
    #[inline(always)]
    pub fn timer_duty_res(&mut self) -> TIMER_DUTY_RES_W<'_, CONF_SPEC> {
        TIMER_DUTY_RES_W::new(self, 0)
    }
    #[doc = "Bits 5:22 - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div_timer(&mut self) -> CLK_DIV_TIMER_W<'_, CONF_SPEC> {
        CLK_DIV_TIMER_W::new(self, 5)
    }
    #[doc = "Bit 23 - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
    #[inline(always)]
    pub fn timer_pause(&mut self) -> TIMER_PAUSE_W<'_, CONF_SPEC> {
        TIMER_PAUSE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
    #[inline(always)]
    pub fn timer_rst(&mut self) -> TIMER_RST_W<'_, CONF_SPEC> {
        TIMER_RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures which clock is timer %s selected. Unused."]
    #[inline(always)]
    pub fn tick_sel_timer(&mut self) -> TICK_SEL_TIMER_W<'_, CONF_SPEC> {
        TICK_SEL_TIMER_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\\\0: Invalid. No effect\\\\1: Update"]
    #[inline(always)]
    pub fn timer_para_up(&mut self) -> TIMER_PARA_UP_W<'_, CONF_SPEC> {
        TIMER_PARA_UP_W::new(self, 26)
    }
}
#[doc = "Timer 0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CONF to value 0x0100_0000"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
