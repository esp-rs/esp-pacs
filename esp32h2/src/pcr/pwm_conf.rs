#[doc = "Register `PWM_CONF` reader"]
pub type R = crate::R<PWM_CONF_SPEC>;
#[doc = "Register `PWM_CONF` writer"]
pub type W = crate::W<PWM_CONF_SPEC>;
#[doc = "Field `PWM_CLK_EN` reader - Set 1 to enable pwm clock"]
pub type PWM_CLK_EN_R = crate::BitReader;
#[doc = "Field `PWM_CLK_EN` writer - Set 1 to enable pwm clock"]
pub type PWM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_RST_EN` reader - Set 0 to reset pwm module"]
pub type PWM_RST_EN_R = crate::BitReader;
#[doc = "Field `PWM_RST_EN` writer - Set 0 to reset pwm module"]
pub type PWM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWM_READY` reader - Query this field after reset pwm module"]
pub type PWM_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable pwm clock"]
    #[inline(always)]
    pub fn pwm_clk_en(&self) -> PWM_CLK_EN_R {
        PWM_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset pwm module"]
    #[inline(always)]
    pub fn pwm_rst_en(&self) -> PWM_RST_EN_R {
        PWM_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset pwm module"]
    #[inline(always)]
    pub fn pwm_ready(&self) -> PWM_READY_R {
        PWM_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM_CONF")
            .field("pwm_clk_en", &self.pwm_clk_en())
            .field("pwm_rst_en", &self.pwm_rst_en())
            .field("pwm_ready", &self.pwm_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable pwm clock"]
    #[inline(always)]
    pub fn pwm_clk_en(&mut self) -> PWM_CLK_EN_W<PWM_CONF_SPEC> {
        PWM_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset pwm module"]
    #[inline(always)]
    pub fn pwm_rst_en(&mut self) -> PWM_RST_EN_W<PWM_CONF_SPEC> {
        PWM_RST_EN_W::new(self, 1)
    }
}
#[doc = "PWM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_CONF_SPEC;
impl crate::RegisterSpec for PWM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_conf::R`](R) reader structure"]
impl crate::Readable for PWM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm_conf::W`](W) writer structure"]
impl crate::Writable for PWM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWM_CONF to value 0x05"]
impl crate::Resettable for PWM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
