#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `DUTY_RES` reader - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
pub type DUTY_RES_R = crate::FieldReader;
#[doc = "Field `DUTY_RES` writer - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
pub type DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLK_DIV` reader - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
pub type CLK_DIV_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_DIV` writer - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
pub type CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `PAUSE` reader - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICK_SEL` reader - Configures which clock is timer %s selected. Unused."]
pub type TICK_SEL_R = crate::BitReader;
#[doc = "Field `TICK_SEL` writer - Configures which clock is timer %s selected. Unused."]
pub type TICK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP` writer - Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\\\0: Invalid. No effect\\\\1: Update"]
pub type PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits >> 5) & 0x0003_ffff)
    }
    #[doc = "Bit 23 - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Configures which clock is timer %s selected. Unused."]
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("duty_res", &self.duty_res())
            .field("clk_div", &self.clk_div())
            .field("pause", &self.pause())
            .field("rst", &self.rst())
            .field("tick_sel", &self.tick_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Configures the bit width of the counter in timer %s. Valid values are 1 to 20."]
    #[inline(always)]
    pub fn duty_res(&mut self) -> DUTY_RES_W<'_, CONF_SPEC> {
        DUTY_RES_W::new(self, 0)
    }
    #[doc = "Bits 5:22 - Configures the divisor for the divider in timer %s.The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div(&mut self) -> CLK_DIV_W<'_, CONF_SPEC> {
        CLK_DIV_W::new(self, 5)
    }
    #[doc = "Bit 23 - Configures whether or not to pause the counter in timer %s.\\\\0: Normal\\\\1: Pause"]
    #[inline(always)]
    pub fn pause(&mut self) -> PAUSE_W<'_, CONF_SPEC> {
        PAUSE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to reset timer %s. The counter will show 0 after reset.\\\\0: Not reset\\\\1: Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RST_W<'_, CONF_SPEC> {
        RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures which clock is timer %s selected. Unused."]
    #[inline(always)]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<'_, CONF_SPEC> {
        TICK_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES.\\\\0: Invalid. No effect\\\\1: Update"]
    #[inline(always)]
    pub fn para_up(&mut self) -> PARA_UP_W<'_, CONF_SPEC> {
        PARA_UP_W::new(self, 26)
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
