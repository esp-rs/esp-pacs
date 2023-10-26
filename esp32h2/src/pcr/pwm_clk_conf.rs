#[doc = "Register `PWM_CLK_CONF` reader"]
pub type R = crate::R<PWM_CLK_CONF_SPEC>;
#[doc = "Register `PWM_CLK_CONF` writer"]
pub type W = crate::W<PWM_CLK_CONF_SPEC>;
#[doc = "Field `PWM_DIV_NUM` reader - The integral part of the frequency divider factor of the pwm function clock."]
pub type PWM_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PWM_DIV_NUM` writer - The integral part of the frequency divider factor of the pwm function clock."]
pub type PWM_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PWM_CLKM_SEL` reader - set this field to select clock-source. 0(default): do not select anyone clock, 1: 160MHz, 2: XTAL, 3: FOSC."]
pub type PWM_CLKM_SEL_R = crate::FieldReader;
#[doc = "Field `PWM_CLKM_SEL` writer - set this field to select clock-source. 0(default): do not select anyone clock, 1: 160MHz, 2: XTAL, 3: FOSC."]
pub type PWM_CLKM_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PWM_CLKM_EN` reader - set this field as 1 to activate pwm clkm."]
pub type PWM_CLKM_EN_R = crate::BitReader;
#[doc = "Field `PWM_CLKM_EN` writer - set this field as 1 to activate pwm clkm."]
pub type PWM_CLKM_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the pwm function clock."]
    #[inline(always)]
    pub fn pwm_div_num(&self) -> PWM_DIV_NUM_R {
        PWM_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): do not select anyone clock, 1: 160MHz, 2: XTAL, 3: FOSC."]
    #[inline(always)]
    pub fn pwm_clkm_sel(&self) -> PWM_CLKM_SEL_R {
        PWM_CLKM_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - set this field as 1 to activate pwm clkm."]
    #[inline(always)]
    pub fn pwm_clkm_en(&self) -> PWM_CLKM_EN_R {
        PWM_CLKM_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM_CLK_CONF")
            .field(
                "pwm_div_num",
                &format_args!("{}", self.pwm_div_num().bits()),
            )
            .field(
                "pwm_clkm_sel",
                &format_args!("{}", self.pwm_clkm_sel().bits()),
            )
            .field("pwm_clkm_en", &format_args!("{}", self.pwm_clkm_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWM_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the pwm function clock."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_div_num(&mut self) -> PWM_DIV_NUM_W<PWM_CLK_CONF_SPEC, 12> {
        PWM_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0(default): do not select anyone clock, 1: 160MHz, 2: XTAL, 3: FOSC."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_clkm_sel(&mut self) -> PWM_CLKM_SEL_W<PWM_CLK_CONF_SPEC, 20> {
        PWM_CLKM_SEL_W::new(self)
    }
    #[doc = "Bit 22 - set this field as 1 to activate pwm clkm."]
    #[inline(always)]
    #[must_use]
    pub fn pwm_clkm_en(&mut self) -> PWM_CLKM_EN_W<PWM_CLK_CONF_SPEC, 22> {
        PWM_CLKM_EN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PWM_CLK configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWM_CLK_CONF_SPEC;
impl crate::RegisterSpec for PWM_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_clk_conf::R`](R) reader structure"]
impl crate::Readable for PWM_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwm_clk_conf::W`](W) writer structure"]
impl crate::Writable for PWM_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CLK_CONF to value 0x0040_4000"]
impl crate::Resettable for PWM_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_4000;
}
