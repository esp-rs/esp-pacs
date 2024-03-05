#[doc = "Register `CARRIER2_CFG` reader"]
pub type R = crate::R<CARRIER2_CFG_SPEC>;
#[doc = "Register `CARRIER2_CFG` writer"]
pub type W = crate::W<CARRIER2_CFG_SPEC>;
#[doc = "Field `CHOPPER2_EN` reader - When set, carrier2 function is enabled. When cleared, carrier2 is bypassed"]
pub type CHOPPER2_EN_R = crate::BitReader;
#[doc = "Field `CHOPPER2_EN` writer - When set, carrier2 function is enabled. When cleared, carrier2 is bypassed"]
pub type CHOPPER2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPPER2_PRESCALE` reader - PWM carrier2 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub type CHOPPER2_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CHOPPER2_PRESCALE` writer - PWM carrier2 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub type CHOPPER2_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHOPPER2_DUTY` reader - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub type CHOPPER2_DUTY_R = crate::FieldReader;
#[doc = "Field `CHOPPER2_DUTY` writer - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub type CHOPPER2_DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CHOPPER2_OSHTWTH` reader - width of the first pulse in number of periods of the carrier"]
pub type CHOPPER2_OSHTWTH_R = crate::FieldReader;
#[doc = "Field `CHOPPER2_OSHTWTH` writer - width of the first pulse in number of periods of the carrier"]
pub type CHOPPER2_OSHTWTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHOPPER2_OUT_INVERT` reader - when set, invert the output of PWM2A and PWM2B for this submodule"]
pub type CHOPPER2_OUT_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER2_OUT_INVERT` writer - when set, invert the output of PWM2A and PWM2B for this submodule"]
pub type CHOPPER2_OUT_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHOPPER2_IN_INVERT` reader - when set, invert the input of PWM2A and PWM2B for this submodule"]
pub type CHOPPER2_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER2_IN_INVERT` writer - when set, invert the input of PWM2A and PWM2B for this submodule"]
pub type CHOPPER2_IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, carrier2 function is enabled. When cleared, carrier2 is bypassed"]
    #[inline(always)]
    pub fn chopper2_en(&self) -> CHOPPER2_EN_R {
        CHOPPER2_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - PWM carrier2 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    pub fn chopper2_prescale(&self) -> CHOPPER2_PRESCALE_R {
        CHOPPER2_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    pub fn chopper2_duty(&self) -> CHOPPER2_DUTY_R {
        CHOPPER2_DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - width of the first pulse in number of periods of the carrier"]
    #[inline(always)]
    pub fn chopper2_oshtwth(&self) -> CHOPPER2_OSHTWTH_R {
        CHOPPER2_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - when set, invert the output of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn chopper2_out_invert(&self) -> CHOPPER2_OUT_INVERT_R {
        CHOPPER2_OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - when set, invert the input of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    pub fn chopper2_in_invert(&self) -> CHOPPER2_IN_INVERT_R {
        CHOPPER2_IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER2_CFG")
            .field("chopper2_en", &format_args!("{}", self.chopper2_en().bit()))
            .field(
                "chopper2_prescale",
                &format_args!("{}", self.chopper2_prescale().bits()),
            )
            .field(
                "chopper2_duty",
                &format_args!("{}", self.chopper2_duty().bits()),
            )
            .field(
                "chopper2_oshtwth",
                &format_args!("{}", self.chopper2_oshtwth().bits()),
            )
            .field(
                "chopper2_out_invert",
                &format_args!("{}", self.chopper2_out_invert().bit()),
            )
            .field(
                "chopper2_in_invert",
                &format_args!("{}", self.chopper2_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CARRIER2_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - When set, carrier2 function is enabled. When cleared, carrier2 is bypassed"]
    #[inline(always)]
    #[must_use]
    pub fn chopper2_en(&mut self) -> CHOPPER2_EN_W<CARRIER2_CFG_SPEC> {
        CHOPPER2_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - PWM carrier2 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn chopper2_prescale(&mut self) -> CHOPPER2_PRESCALE_W<CARRIER2_CFG_SPEC> {
        CHOPPER2_PRESCALE_W::new(self, 1)
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    #[must_use]
    pub fn chopper2_duty(&mut self) -> CHOPPER2_DUTY_W<CARRIER2_CFG_SPEC> {
        CHOPPER2_DUTY_W::new(self, 5)
    }
    #[doc = "Bits 8:11 - width of the first pulse in number of periods of the carrier"]
    #[inline(always)]
    #[must_use]
    pub fn chopper2_oshtwth(&mut self) -> CHOPPER2_OSHTWTH_W<CARRIER2_CFG_SPEC> {
        CHOPPER2_OSHTWTH_W::new(self, 8)
    }
    #[doc = "Bit 12 - when set, invert the output of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    #[must_use]
    pub fn chopper2_out_invert(&mut self) -> CHOPPER2_OUT_INVERT_W<CARRIER2_CFG_SPEC> {
        CHOPPER2_OUT_INVERT_W::new(self, 12)
    }
    #[doc = "Bit 13 - when set, invert the input of PWM2A and PWM2B for this submodule"]
    #[inline(always)]
    #[must_use]
    pub fn chopper2_in_invert(&mut self) -> CHOPPER2_IN_INVERT_W<CARRIER2_CFG_SPEC> {
        CHOPPER2_IN_INVERT_W::new(self, 13)
    }
}
#[doc = "Carrier enable and configuratoin\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier2_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier2_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER2_CFG_SPEC;
impl crate::RegisterSpec for CARRIER2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier2_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER2_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier2_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER2_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CARRIER2_CFG to value 0"]
impl crate::Resettable for CARRIER2_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
