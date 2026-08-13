#[doc = "Register `PWM0_CLK_CONF` reader"]
pub type R = crate::R<PWM0_CLK_CONF_SPEC>;
#[doc = "Register `PWM0_CLK_CONF` writer"]
pub type W = crate::W<PWM0_CLK_CONF_SPEC>;
#[doc = "Field `PWM0_DIV_NUM` reader - The integral part of the frequency divider factor of the pwm0 function clock."]
pub type PWM0_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PWM0_DIV_NUM` writer - The integral part of the frequency divider factor of the pwm0 function clock."]
pub type PWM0_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWM0_CLKM_SEL` reader - Configures the clock source of MCPWM0.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F160M_CLK\\\\"]
pub type PWM0_CLKM_SEL_R = crate::FieldReader;
#[doc = "Field `PWM0_CLKM_SEL` writer - Configures the clock source of MCPWM0.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F160M_CLK\\\\"]
pub type PWM0_CLKM_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWM0_CLKM_EN` reader - set this field as 1 to activate pwm0 clkm."]
pub type PWM0_CLKM_EN_R = crate::BitReader;
#[doc = "Field `PWM0_CLKM_EN` writer - set this field as 1 to activate pwm0 clkm."]
pub type PWM0_CLKM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the pwm0 function clock."]
    #[inline(always)]
    pub fn pwm0_div_num(&self) -> PWM0_DIV_NUM_R {
        PWM0_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - Configures the clock source of MCPWM0.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F160M_CLK\\\\"]
    #[inline(always)]
    pub fn pwm0_clkm_sel(&self) -> PWM0_CLKM_SEL_R {
        PWM0_CLKM_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - set this field as 1 to activate pwm0 clkm."]
    #[inline(always)]
    pub fn pwm0_clkm_en(&self) -> PWM0_CLKM_EN_R {
        PWM0_CLKM_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM0_CLK_CONF")
            .field("pwm0_div_num", &self.pwm0_div_num())
            .field("pwm0_clkm_sel", &self.pwm0_clkm_sel())
            .field("pwm0_clkm_en", &self.pwm0_clkm_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the pwm0 function clock."]
    #[inline(always)]
    pub fn pwm0_div_num(&mut self) -> PWM0_DIV_NUM_W<'_, PWM0_CLK_CONF_SPEC> {
        PWM0_DIV_NUM_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - Configures the clock source of MCPWM0.\\\\ 0 (default): XTAL_CLK\\\\ 1: RC_FAST_CLK\\\\ 2: PLL_F160M_CLK\\\\"]
    #[inline(always)]
    pub fn pwm0_clkm_sel(&mut self) -> PWM0_CLKM_SEL_W<'_, PWM0_CLK_CONF_SPEC> {
        PWM0_CLKM_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - set this field as 1 to activate pwm0 clkm."]
    #[inline(always)]
    pub fn pwm0_clkm_en(&mut self) -> PWM0_CLKM_EN_W<'_, PWM0_CLK_CONF_SPEC> {
        PWM0_CLKM_EN_W::new(self, 22)
    }
}
#[doc = "PWM0_CLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM0_CLK_CONF_SPEC;
impl crate::RegisterSpec for PWM0_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm0_clk_conf::R`](R) reader structure"]
impl crate::Readable for PWM0_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm0_clk_conf::W`](W) writer structure"]
impl crate::Writable for PWM0_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM0_CLK_CONF to value 0x4000"]
impl crate::Resettable for PWM0_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x4000;
}
