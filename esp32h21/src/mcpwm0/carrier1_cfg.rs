#[doc = "Register `CARRIER1_CFG` reader"]
pub type R = crate::R<CARRIER1_CFG_SPEC>;
#[doc = "Register `CARRIER1_CFG` writer"]
pub type W = crate::W<CARRIER1_CFG_SPEC>;
#[doc = "Field `CHOPPER1_EN` reader - When set, carrier1 function is enabled. When cleared, carrier1 is bypassed"]
pub type CHOPPER1_EN_R = crate::BitReader;
#[doc = "Field `CHOPPER1_EN` writer - When set, carrier1 function is enabled. When cleared, carrier1 is bypassed"]
pub type CHOPPER1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPPER1_PRESCALE` reader - PWM carrier1 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub type CHOPPER1_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CHOPPER1_PRESCALE` writer - PWM carrier1 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub type CHOPPER1_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHOPPER1_DUTY` reader - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub type CHOPPER1_DUTY_R = crate::FieldReader;
#[doc = "Field `CHOPPER1_DUTY` writer - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub type CHOPPER1_DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHOPPER1_OSHTWTH` reader - width of the first pulse in number of periods of the carrier"]
pub type CHOPPER1_OSHTWTH_R = crate::FieldReader;
#[doc = "Field `CHOPPER1_OSHTWTH` writer - width of the first pulse in number of periods of the carrier"]
pub type CHOPPER1_OSHTWTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHOPPER1_OUT_INVERT` reader - when set, invert the output of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_OUT_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER1_OUT_INVERT` writer - when set, invert the output of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_OUT_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPPER1_IN_INVERT` reader - when set, invert the input of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER1_IN_INVERT` writer - when set, invert the input of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, carrier1 function is enabled. When cleared, carrier1 is bypassed"]
    #[inline(always)]
    pub fn chopper1_en(&self) -> CHOPPER1_EN_R {
        CHOPPER1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - PWM carrier1 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper1_prescale(&self) -> CHOPPER1_PRESCALE_R {
        CHOPPER1_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn chopper1_duty(&self) -> CHOPPER1_DUTY_R {
        CHOPPER1_DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - width of the first pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn chopper1_oshtwth(&self) -> CHOPPER1_OSHTWTH_R {
        CHOPPER1_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - when set, invert the output of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_out_invert(&self) -> CHOPPER1_OUT_INVERT_R {
        CHOPPER1_OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - when set, invert the input of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_in_invert(&self) -> CHOPPER1_IN_INVERT_R {
        CHOPPER1_IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER1_CFG")
            .field("chopper1_en", &self.chopper1_en())
            .field("chopper1_prescale", &self.chopper1_prescale())
            .field("chopper1_duty", &self.chopper1_duty())
            .field("chopper1_oshtwth", &self.chopper1_oshtwth())
            .field("chopper1_out_invert", &self.chopper1_out_invert())
            .field("chopper1_in_invert", &self.chopper1_in_invert())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - When set, carrier1 function is enabled. When cleared, carrier1 is bypassed"]
    #[inline(always)]
    pub fn chopper1_en(&mut self) -> CHOPPER1_EN_W<'_, CARRIER1_CFG_SPEC> {
        CHOPPER1_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - PWM carrier1 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper1_prescale(&mut self) -> CHOPPER1_PRESCALE_W<'_, CARRIER1_CFG_SPEC> {
        CHOPPER1_PRESCALE_W::new(self, 1)
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn chopper1_duty(&mut self) -> CHOPPER1_DUTY_W<'_, CARRIER1_CFG_SPEC> {
        CHOPPER1_DUTY_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - width of the first pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn chopper1_oshtwth(&mut self) -> CHOPPER1_OSHTWTH_W<'_, CARRIER1_CFG_SPEC> {
        CHOPPER1_OSHTWTH_W::new(self, 8)
    }
    #[doc = "Bit 12 - when set, invert the output of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_out_invert(&mut self) -> CHOPPER1_OUT_INVERT_W<'_, CARRIER1_CFG_SPEC> {
        CHOPPER1_OUT_INVERT_W::new(self, 12)
    }
    #[doc = "Bit 13 - when set, invert the input of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    pub fn chopper1_in_invert(&mut self) -> CHOPPER1_IN_INVERT_W<'_, CARRIER1_CFG_SPEC> {
        CHOPPER1_IN_INVERT_W::new(self, 13)
    }
}
#[doc = "Carrier enable and configuratoin\n\nYou can [`read`](crate::Reg::read) this register and get [`carrier1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`carrier1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER1_CFG_SPEC;
impl crate::RegisterSpec for CARRIER1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier1_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER1_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier1_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER1_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CARRIER1_CFG to value 0"]
impl crate::Resettable for CARRIER1_CFG_SPEC {}
