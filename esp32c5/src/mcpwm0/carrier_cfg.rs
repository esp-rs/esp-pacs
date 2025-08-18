#[doc = "Register `CARRIER%s_CFG` reader"]
pub type R = crate::R<CARRIER_CFG_SPEC>;
#[doc = "Register `CARRIER%s_CFG` writer"]
pub type W = crate::W<CARRIER_CFG_SPEC>;
#[doc = "Field `CHOPPER_EN` reader - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
pub type CHOPPER_EN_R = crate::BitReader;
#[doc = "Field `CHOPPER_EN` writer - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
pub type CHOPPER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPPER_PRESCALE` reader - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
pub type CHOPPER_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CHOPPER_PRESCALE` writer - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
pub type CHOPPER_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHOPPER_DUTY` reader - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
pub type CHOPPER_DUTY_R = crate::FieldReader;
#[doc = "Field `CHOPPER_DUTY` writer - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
pub type CHOPPER_DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHOPPER_OSHTWTH` reader - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
pub type CHOPPER_OSHTWTH_R = crate::FieldReader;
#[doc = "Field `CHOPPER_OSHTWTH` writer - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
pub type CHOPPER_OSHTWTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHOPPER_OUT_INVERT` reader - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type CHOPPER_OUT_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER_OUT_INVERT` writer - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type CHOPPER_OUT_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPPER_IN_INVERT` reader - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type CHOPPER_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER_IN_INVERT` writer - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
pub type CHOPPER_IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
    #[inline(always)]
    pub fn chopper_en(&self) -> CHOPPER_EN_R {
        CHOPPER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper_prescale(&self) -> CHOPPER_PRESCALE_R {
        CHOPPER_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
    #[inline(always)]
    pub fn chopper_duty(&self) -> CHOPPER_DUTY_R {
        CHOPPER_DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
    #[inline(always)]
    pub fn chopper_oshtwth(&self) -> CHOPPER_OSHTWTH_R {
        CHOPPER_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn chopper_out_invert(&self) -> CHOPPER_OUT_INVERT_R {
        CHOPPER_OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn chopper_in_invert(&self) -> CHOPPER_IN_INVERT_R {
        CHOPPER_IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER_CFG")
            .field("chopper_en", &self.chopper_en())
            .field("chopper_prescale", &self.chopper_prescale())
            .field("chopper_duty", &self.chopper_duty())
            .field("chopper_oshtwth", &self.chopper_oshtwth())
            .field("chopper_out_invert", &self.chopper_out_invert())
            .field("chopper_in_invert", &self.chopper_in_invert())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable carrier%s.\\\\0: Bypassed\\\\1: Enabled"]
    #[inline(always)]
    pub fn chopper_en(&mut self) -> CHOPPER_EN_W<'_, CARRIER_CFG_SPEC> {
        CHOPPER_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - Configures the prescale value of PWM carrier%s clock (PC_clk), so that period of PC_clk = period of PWM_clk * (PWM_CARRIER%s_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper_prescale(&mut self) -> CHOPPER_PRESCALE_W<'_, CARRIER_CFG_SPEC> {
        CHOPPER_PRESCALE_W::new(self, 1)
    }
    #[doc = "Bits 5:7 - Configures carrier duty. Duty = PWM_CARRIER%s_DUTY / 8"]
    #[inline(always)]
    pub fn chopper_duty(&mut self) -> CHOPPER_DUTY_W<'_, CARRIER_CFG_SPEC> {
        CHOPPER_DUTY_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - Configures width of the first pulse. Measurement unit: Periods of the carrier."]
    #[inline(always)]
    pub fn chopper_oshtwth(&mut self) -> CHOPPER_OSHTWTH_W<'_, CARRIER_CFG_SPEC> {
        CHOPPER_OSHTWTH_W::new(self, 8)
    }
    #[doc = "Bit 12 - Configures whether or not to invert the output of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn chopper_out_invert(&mut self) -> CHOPPER_OUT_INVERT_W<'_, CARRIER_CFG_SPEC> {
        CHOPPER_OUT_INVERT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to invert the input of PWM%s A and PWM%s B for this submodule.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn chopper_in_invert(&mut self) -> CHOPPER_IN_INVERT_W<'_, CARRIER_CFG_SPEC> {
        CHOPPER_IN_INVERT_W::new(self, 13)
    }
}
#[doc = "Carrier%s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`carrier_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`carrier_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER_CFG_SPEC;
impl crate::RegisterSpec for CARRIER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CARRIER%s_CFG to value 0"]
impl crate::Resettable for CARRIER_CFG_SPEC {}
