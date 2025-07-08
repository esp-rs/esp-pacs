#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `T0_USE_XTAL` reader - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type T0_USE_XTAL_R = crate::BitReader;
#[doc = "Field `T0_USE_XTAL` writer - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type T0_USE_XTAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0_ALARM_EN` reader - Configures whether or not to enable the timer 0 alarm function. This bit will be automatically cleared once an alarm occurs.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type T0_ALARM_EN_R = crate::BitReader;
#[doc = "Field `T0_ALARM_EN` writer - Configures whether or not to enable the timer 0 alarm function. This bit will be automatically cleared once an alarm occurs.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type T0_ALARM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0_DIVCNT_RST` writer - Configures whether or not to reset the timer 0 's clock divider counter. \\\\ 0: No effect \\\\ 1: Reset \\\\"]
pub type T0_DIVCNT_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0_DIVIDER` reader - Represents the timer 0 clock (T0_clk) prescaler value."]
pub type T0_DIVIDER_R = crate::FieldReader<u16>;
#[doc = "Field `T0_DIVIDER` writer - Represents the timer 0 clock (T0_clk) prescaler value."]
pub type T0_DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `T0_AUTORELOAD` reader - Configures whether or not to enable the timer 0 auto-reload function at the time of alarm. \\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type T0_AUTORELOAD_R = crate::BitReader;
#[doc = "Field `T0_AUTORELOAD` writer - Configures whether or not to enable the timer 0 auto-reload function at the time of alarm. \\\\ 0: No effect \\\\ 1: Enable \\\\"]
pub type T0_AUTORELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0_INCREASE` reader - Configures the counting direction of the timer 0 time-base counter. \\\\ 0: Decrement \\\\ 1: Increment \\\\"]
pub type T0_INCREASE_R = crate::BitReader;
#[doc = "Field `T0_INCREASE` writer - Configures the counting direction of the timer 0 time-base counter. \\\\ 0: Decrement \\\\ 1: Increment \\\\"]
pub type T0_INCREASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0_EN` reader - Configures whether or not to enable the timer 0 time-base counter. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type T0_EN_R = crate::BitReader;
#[doc = "Field `T0_EN` writer - Configures whether or not to enable the timer 0 time-base counter. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
pub type T0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn t0_use_xtal(&self) -> T0_USE_XTAL_R {
        T0_USE_XTAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to enable the timer 0 alarm function. This bit will be automatically cleared once an alarm occurs.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn t0_alarm_en(&self) -> T0_ALARM_EN_R {
        T0_ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Represents the timer 0 clock (T0_clk) prescaler value."]
    #[inline(always)]
    pub fn t0_divider(&self) -> T0_DIVIDER_R {
        T0_DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - Configures whether or not to enable the timer 0 auto-reload function at the time of alarm. \\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn t0_autoreload(&self) -> T0_AUTORELOAD_R {
        T0_AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configures the counting direction of the timer 0 time-base counter. \\\\ 0: Decrement \\\\ 1: Increment \\\\"]
    #[inline(always)]
    pub fn t0_increase(&self) -> T0_INCREASE_R {
        T0_INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures whether or not to enable the timer 0 time-base counter. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn t0_en(&self) -> T0_EN_R {
        T0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("t0_use_xtal", &self.t0_use_xtal())
            .field("t0_alarm_en", &self.t0_alarm_en())
            .field("t0_divider", &self.t0_divider())
            .field("t0_autoreload", &self.t0_autoreload())
            .field("t0_increase", &self.t0_increase())
            .field("t0_en", &self.t0_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn t0_use_xtal(&mut self) -> T0_USE_XTAL_W<CONFIG_SPEC> {
        T0_USE_XTAL_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable the timer 0 alarm function. This bit will be automatically cleared once an alarm occurs.\\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn t0_alarm_en(&mut self) -> T0_ALARM_EN_W<CONFIG_SPEC> {
        T0_ALARM_EN_W::new(self, 10)
    }
    #[doc = "Bit 12 - Configures whether or not to reset the timer 0 's clock divider counter. \\\\ 0: No effect \\\\ 1: Reset \\\\"]
    #[inline(always)]
    pub fn t0_divcnt_rst(&mut self) -> T0_DIVCNT_RST_W<CONFIG_SPEC> {
        T0_DIVCNT_RST_W::new(self, 12)
    }
    #[doc = "Bits 13:28 - Represents the timer 0 clock (T0_clk) prescaler value."]
    #[inline(always)]
    pub fn t0_divider(&mut self) -> T0_DIVIDER_W<CONFIG_SPEC> {
        T0_DIVIDER_W::new(self, 13)
    }
    #[doc = "Bit 29 - Configures whether or not to enable the timer 0 auto-reload function at the time of alarm. \\\\ 0: No effect \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn t0_autoreload(&mut self) -> T0_AUTORELOAD_W<CONFIG_SPEC> {
        T0_AUTORELOAD_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures the counting direction of the timer 0 time-base counter. \\\\ 0: Decrement \\\\ 1: Increment \\\\"]
    #[inline(always)]
    pub fn t0_increase(&mut self) -> T0_INCREASE_W<CONFIG_SPEC> {
        T0_INCREASE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to enable the timer 0 time-base counter. \\\\ 0: Disable \\\\ 1: Enable \\\\"]
    #[inline(always)]
    pub fn t0_en(&mut self) -> T0_EN_W<CONFIG_SPEC> {
        T0_EN_W::new(self, 31)
    }
}
#[doc = "Timer 0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0x6000_2000"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x6000_2000;
}
