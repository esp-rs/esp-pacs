#[doc = "Register `PWM1_CONF` reader"]
pub type R = crate::R<PWM1_CONF_SPEC>;
#[doc = "Register `PWM1_CONF` writer"]
pub type W = crate::W<PWM1_CONF_SPEC>;
#[doc = "Field `PWM1_CLK_EN` reader - Set 1 to enable pwm1 clock"]
pub type PWM1_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM1_CLK_EN` writer - Set 1 to enable pwm1 clock"]
pub type PWM1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_RST_EN` reader - Set 1 to reset pwm1 module"]
pub type PWM1_RST_EN_R = crate::BitReader;
#[doc = "Field `PWM1_RST_EN` writer - Set 1 to reset pwm1 module"]
pub type PWM1_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM1_READY` reader - Query this field after reset pwm1 module"]
pub type PWM1_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable pwm1 clock"]
    #[inline(always)]
    pub fn pwm1_clk_en(&self) -> PWM1_CLK_EN_R {
        PWM1_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset pwm1 module"]
    #[inline(always)]
    pub fn pwm1_rst_en(&self) -> PWM1_RST_EN_R {
        PWM1_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset pwm1 module"]
    #[inline(always)]
    pub fn pwm1_ready(&self) -> PWM1_READY_R {
        PWM1_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1_CONF")
            .field("pwm1_clk_en", &self.pwm1_clk_en())
            .field("pwm1_rst_en", &self.pwm1_rst_en())
            .field("pwm1_ready", &self.pwm1_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable pwm1 clock"]
    #[inline(always)]
    pub fn pwm1_clk_en(&mut self) -> PWM1_CLK_EN_W<'_, PWM1_CONF_SPEC> {
        PWM1_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset pwm1 module"]
    #[inline(always)]
    pub fn pwm1_rst_en(&mut self) -> PWM1_RST_EN_W<'_, PWM1_CONF_SPEC> {
        PWM1_RST_EN_W::new(self, 1)
    }
}
#[doc = "PWM1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM1_CONF_SPEC;
impl crate::RegisterSpec for PWM1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1_conf::R`](R) reader structure"]
impl crate::Readable for PWM1_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm1_conf::W`](W) writer structure"]
impl crate::Writable for PWM1_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM1_CONF to value 0x04"]
impl crate::Resettable for PWM1_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
