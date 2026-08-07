#[doc = "Register `PWM0_CONF` reader"]
pub type R = crate::R<PWM0_CONF_SPEC>;
#[doc = "Register `PWM0_CONF` writer"]
pub type W = crate::W<PWM0_CONF_SPEC>;
#[doc = "Field `PWM0_CLK_EN` reader - Set 1 to enable pwm0 clock"]
pub type PWM0_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM0_CLK_EN` writer - Set 1 to enable pwm0 clock"]
pub type PWM0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_RST_EN` reader - Set 1 to reset pwm0 module"]
pub type PWM0_RST_EN_R = crate::BitReader;
#[doc = "Field `PWM0_RST_EN` writer - Set 1 to reset pwm0 module"]
pub type PWM0_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM0_READY` reader - Query this field after reset pwm0 module"]
pub type PWM0_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable pwm0 clock"]
    #[inline(always)]
    pub fn pwm0_clk_en(&self) -> PWM0_CLK_EN_R {
        PWM0_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset pwm0 module"]
    #[inline(always)]
    pub fn pwm0_rst_en(&self) -> PWM0_RST_EN_R {
        PWM0_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset pwm0 module"]
    #[inline(always)]
    pub fn pwm0_ready(&self) -> PWM0_READY_R {
        PWM0_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM0_CONF")
            .field("pwm0_clk_en", &self.pwm0_clk_en())
            .field("pwm0_rst_en", &self.pwm0_rst_en())
            .field("pwm0_ready", &self.pwm0_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable pwm0 clock"]
    #[inline(always)]
    pub fn pwm0_clk_en(&mut self) -> PWM0_CLK_EN_W<'_, PWM0_CONF_SPEC> {
        PWM0_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset pwm0 module"]
    #[inline(always)]
    pub fn pwm0_rst_en(&mut self) -> PWM0_RST_EN_W<'_, PWM0_CONF_SPEC> {
        PWM0_RST_EN_W::new(self, 1)
    }
}
#[doc = "PWM0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM0_CONF_SPEC;
impl crate::RegisterSpec for PWM0_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm0_conf::R`](R) reader structure"]
impl crate::Readable for PWM0_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm0_conf::W`](W) writer structure"]
impl crate::Writable for PWM0_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM0_CONF to value 0x04"]
impl crate::Resettable for PWM0_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
