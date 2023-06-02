#[doc = "Register `CARRIER1_CFG` reader"]
pub struct R(crate::R<CARRIER1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CARRIER1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CARRIER1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CARRIER1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CARRIER1_CFG` writer"]
pub struct W(crate::W<CARRIER1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CARRIER1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CARRIER1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CARRIER1_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHOPPER1_EN` reader - When set, carrier1 function is enabled. When cleared, carrier1 is bypassed"]
pub type CHOPPER1_EN_R = crate::BitReader;
#[doc = "Field `CHOPPER1_EN` writer - When set, carrier1 function is enabled. When cleared, carrier1 is bypassed"]
pub type CHOPPER1_EN_W<'a, const O: u8> = crate::BitWriter<'a, CARRIER1_CFG_SPEC, O>;
#[doc = "Field `CHOPPER1_PRESCALE` reader - PWM carrier1 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub type CHOPPER1_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CHOPPER1_PRESCALE` writer - PWM carrier1 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
pub type CHOPPER1_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, CARRIER1_CFG_SPEC, 4, O>;
#[doc = "Field `CHOPPER1_DUTY` reader - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub type CHOPPER1_DUTY_R = crate::FieldReader;
#[doc = "Field `CHOPPER1_DUTY` writer - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
pub type CHOPPER1_DUTY_W<'a, const O: u8> = crate::FieldWriter<'a, CARRIER1_CFG_SPEC, 3, O>;
#[doc = "Field `CHOPPER1_OSHTWTH` reader - width of the first pulse in number of periods of the carrier"]
pub type CHOPPER1_OSHTWTH_R = crate::FieldReader;
#[doc = "Field `CHOPPER1_OSHTWTH` writer - width of the first pulse in number of periods of the carrier"]
pub type CHOPPER1_OSHTWTH_W<'a, const O: u8> = crate::FieldWriter<'a, CARRIER1_CFG_SPEC, 4, O>;
#[doc = "Field `CHOPPER1_OUT_INVERT` reader - when set, invert the output of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_OUT_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER1_OUT_INVERT` writer - when set, invert the output of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_OUT_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, CARRIER1_CFG_SPEC, O>;
#[doc = "Field `CHOPPER1_IN_INVERT` reader - when set, invert the input of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CHOPPER1_IN_INVERT` writer - when set, invert the input of PWM1A and PWM1B for this submodule"]
pub type CHOPPER1_IN_INVERT_W<'a, const O: u8> = crate::BitWriter<'a, CARRIER1_CFG_SPEC, O>;
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
            .field("chopper1_en", &format_args!("{}", self.chopper1_en().bit()))
            .field(
                "chopper1_prescale",
                &format_args!("{}", self.chopper1_prescale().bits()),
            )
            .field(
                "chopper1_duty",
                &format_args!("{}", self.chopper1_duty().bits()),
            )
            .field(
                "chopper1_oshtwth",
                &format_args!("{}", self.chopper1_oshtwth().bits()),
            )
            .field(
                "chopper1_out_invert",
                &format_args!("{}", self.chopper1_out_invert().bit()),
            )
            .field(
                "chopper1_in_invert",
                &format_args!("{}", self.chopper1_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CARRIER1_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - When set, carrier1 function is enabled. When cleared, carrier1 is bypassed"]
    #[inline(always)]
    #[must_use]
    pub fn chopper1_en(&mut self) -> CHOPPER1_EN_W<0> {
        CHOPPER1_EN_W::new(self)
    }
    #[doc = "Bits 1:4 - PWM carrier1 clock (PC_clk) prescale value. Period of PC_clk = period of PWM_clk * (PWM_CARRIER0_PRESCALE + 1)"]
    #[inline(always)]
    #[must_use]
    pub fn chopper1_prescale(&mut self) -> CHOPPER1_PRESCALE_W<1> {
        CHOPPER1_PRESCALE_W::new(self)
    }
    #[doc = "Bits 5:7 - carrier duty selection. Duty = PWM_CARRIER0_DUTY / 8"]
    #[inline(always)]
    #[must_use]
    pub fn chopper1_duty(&mut self) -> CHOPPER1_DUTY_W<5> {
        CHOPPER1_DUTY_W::new(self)
    }
    #[doc = "Bits 8:11 - width of the first pulse in number of periods of the carrier"]
    #[inline(always)]
    #[must_use]
    pub fn chopper1_oshtwth(&mut self) -> CHOPPER1_OSHTWTH_W<8> {
        CHOPPER1_OSHTWTH_W::new(self)
    }
    #[doc = "Bit 12 - when set, invert the output of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    #[must_use]
    pub fn chopper1_out_invert(&mut self) -> CHOPPER1_OUT_INVERT_W<12> {
        CHOPPER1_OUT_INVERT_W::new(self)
    }
    #[doc = "Bit 13 - when set, invert the input of PWM1A and PWM1B for this submodule"]
    #[inline(always)]
    #[must_use]
    pub fn chopper1_in_invert(&mut self) -> CHOPPER1_IN_INVERT_W<13> {
        CHOPPER1_IN_INVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Carrier enable and configuratoin\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [carrier1_cfg](index.html) module"]
pub struct CARRIER1_CFG_SPEC;
impl crate::RegisterSpec for CARRIER1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [carrier1_cfg::R](R) reader structure"]
impl crate::Readable for CARRIER1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [carrier1_cfg::W](W) writer structure"]
impl crate::Writable for CARRIER1_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CARRIER1_CFG to value 0"]
impl crate::Resettable for CARRIER1_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
