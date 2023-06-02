#[doc = "Register `GEN2_FORCE` reader"]
pub struct R(crate::R<GEN2_FORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN2_FORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN2_FORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN2_FORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN2_FORCE` writer"]
pub struct W(crate::W<GEN2_FORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN2_FORCE_SPEC>;
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
impl From<crate::W<GEN2_FORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN2_FORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GEN2_CNTUFORCE_UPMETHOD` reader - Updating method for continuous software force of PWM generator 2. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ,when bit1 is set to 1: TEP, when bit2 is set to 1: TEA, when bit3 is set to 1: TEB, when bit4 is set to 1: sync, when bit5 is set to 1: disable update. (TEA/B here and below means an event generated when the timer's value equals to that of register A/B.)"]
pub type GEN2_CNTUFORCE_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN2_CNTUFORCE_UPMETHOD` writer - Updating method for continuous software force of PWM generator 2. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ,when bit1 is set to 1: TEP, when bit2 is set to 1: TEA, when bit3 is set to 1: TEB, when bit4 is set to 1: sync, when bit5 is set to 1: disable update. (TEA/B here and below means an event generated when the timer's value equals to that of register A/B.)"]
pub type GEN2_CNTUFORCE_UPMETHOD_W<'a, const O: u8> = crate::FieldWriter<'a, GEN2_FORCE_SPEC, 6, O>;
#[doc = "Field `GEN2_A_CNTUFORCE_MODE` reader - Continuous software force mode for PWM2A. 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_A_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN2_A_CNTUFORCE_MODE` writer - Continuous software force mode for PWM2A. 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_A_CNTUFORCE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, GEN2_FORCE_SPEC, 2, O>;
#[doc = "Field `GEN2_B_CNTUFORCE_MODE` reader - Continuous software force mode for PWM2B. 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_B_CNTUFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN2_B_CNTUFORCE_MODE` writer - Continuous software force mode for PWM2B. 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_B_CNTUFORCE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, GEN2_FORCE_SPEC, 2, O>;
#[doc = "Field `GEN2_A_NCIFORCE` reader - Trigger of non-continuous immediate software-force event for PWM2A, a toggle will trigger a force event."]
pub type GEN2_A_NCIFORCE_R = crate::BitReader;
#[doc = "Field `GEN2_A_NCIFORCE` writer - Trigger of non-continuous immediate software-force event for PWM2A, a toggle will trigger a force event."]
pub type GEN2_A_NCIFORCE_W<'a, const O: u8> = crate::BitWriter<'a, GEN2_FORCE_SPEC, O>;
#[doc = "Field `GEN2_A_NCIFORCE_MODE` reader - non-continuous immediate software force mode for PWM2A, 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_A_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN2_A_NCIFORCE_MODE` writer - non-continuous immediate software force mode for PWM2A, 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_A_NCIFORCE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, GEN2_FORCE_SPEC, 2, O>;
#[doc = "Field `GEN2_B_NCIFORCE` reader - Trigger of non-continuous immediate software-force event for PWM2B, a toggle will trigger a force event."]
pub type GEN2_B_NCIFORCE_R = crate::BitReader;
#[doc = "Field `GEN2_B_NCIFORCE` writer - Trigger of non-continuous immediate software-force event for PWM2B, a toggle will trigger a force event."]
pub type GEN2_B_NCIFORCE_W<'a, const O: u8> = crate::BitWriter<'a, GEN2_FORCE_SPEC, O>;
#[doc = "Field `GEN2_B_NCIFORCE_MODE` reader - non-continuous immediate software force mode for PWM2B, 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_B_NCIFORCE_MODE_R = crate::FieldReader;
#[doc = "Field `GEN2_B_NCIFORCE_MODE` writer - non-continuous immediate software force mode for PWM2B, 0: disabled, 1: low, 2: high, 3: disabled"]
pub type GEN2_B_NCIFORCE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, GEN2_FORCE_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:5 - Updating method for continuous software force of PWM generator 2. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ,when bit1 is set to 1: TEP, when bit2 is set to 1: TEA, when bit3 is set to 1: TEB, when bit4 is set to 1: sync, when bit5 is set to 1: disable update. (TEA/B here and below means an event generated when the timer's value equals to that of register A/B.)"]
    #[inline(always)]
    pub fn gen2_cntuforce_upmethod(&self) -> GEN2_CNTUFORCE_UPMETHOD_R {
        GEN2_CNTUFORCE_UPMETHOD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Continuous software force mode for PWM2A. 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    pub fn gen2_a_cntuforce_mode(&self) -> GEN2_A_CNTUFORCE_MODE_R {
        GEN2_A_CNTUFORCE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Continuous software force mode for PWM2B. 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    pub fn gen2_b_cntuforce_mode(&self) -> GEN2_B_CNTUFORCE_MODE_R {
        GEN2_B_CNTUFORCE_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Trigger of non-continuous immediate software-force event for PWM2A, a toggle will trigger a force event."]
    #[inline(always)]
    pub fn gen2_a_nciforce(&self) -> GEN2_A_NCIFORCE_R {
        GEN2_A_NCIFORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - non-continuous immediate software force mode for PWM2A, 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    pub fn gen2_a_nciforce_mode(&self) -> GEN2_A_NCIFORCE_MODE_R {
        GEN2_A_NCIFORCE_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Trigger of non-continuous immediate software-force event for PWM2B, a toggle will trigger a force event."]
    #[inline(always)]
    pub fn gen2_b_nciforce(&self) -> GEN2_B_NCIFORCE_R {
        GEN2_B_NCIFORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - non-continuous immediate software force mode for PWM2B, 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    pub fn gen2_b_nciforce_mode(&self) -> GEN2_B_NCIFORCE_MODE_R {
        GEN2_B_NCIFORCE_MODE_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN2_FORCE")
            .field(
                "gen2_cntuforce_upmethod",
                &format_args!("{}", self.gen2_cntuforce_upmethod().bits()),
            )
            .field(
                "gen2_a_cntuforce_mode",
                &format_args!("{}", self.gen2_a_cntuforce_mode().bits()),
            )
            .field(
                "gen2_b_cntuforce_mode",
                &format_args!("{}", self.gen2_b_cntuforce_mode().bits()),
            )
            .field(
                "gen2_a_nciforce",
                &format_args!("{}", self.gen2_a_nciforce().bit()),
            )
            .field(
                "gen2_a_nciforce_mode",
                &format_args!("{}", self.gen2_a_nciforce_mode().bits()),
            )
            .field(
                "gen2_b_nciforce",
                &format_args!("{}", self.gen2_b_nciforce().bit()),
            )
            .field(
                "gen2_b_nciforce_mode",
                &format_args!("{}", self.gen2_b_nciforce_mode().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN2_FORCE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Updating method for continuous software force of PWM generator 2. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ,when bit1 is set to 1: TEP, when bit2 is set to 1: TEA, when bit3 is set to 1: TEB, when bit4 is set to 1: sync, when bit5 is set to 1: disable update. (TEA/B here and below means an event generated when the timer's value equals to that of register A/B.)"]
    #[inline(always)]
    #[must_use]
    pub fn gen2_cntuforce_upmethod(&mut self) -> GEN2_CNTUFORCE_UPMETHOD_W<0> {
        GEN2_CNTUFORCE_UPMETHOD_W::new(self)
    }
    #[doc = "Bits 6:7 - Continuous software force mode for PWM2A. 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gen2_a_cntuforce_mode(&mut self) -> GEN2_A_CNTUFORCE_MODE_W<6> {
        GEN2_A_CNTUFORCE_MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Continuous software force mode for PWM2B. 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gen2_b_cntuforce_mode(&mut self) -> GEN2_B_CNTUFORCE_MODE_W<8> {
        GEN2_B_CNTUFORCE_MODE_W::new(self)
    }
    #[doc = "Bit 10 - Trigger of non-continuous immediate software-force event for PWM2A, a toggle will trigger a force event."]
    #[inline(always)]
    #[must_use]
    pub fn gen2_a_nciforce(&mut self) -> GEN2_A_NCIFORCE_W<10> {
        GEN2_A_NCIFORCE_W::new(self)
    }
    #[doc = "Bits 11:12 - non-continuous immediate software force mode for PWM2A, 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gen2_a_nciforce_mode(&mut self) -> GEN2_A_NCIFORCE_MODE_W<11> {
        GEN2_A_NCIFORCE_MODE_W::new(self)
    }
    #[doc = "Bit 13 - Trigger of non-continuous immediate software-force event for PWM2B, a toggle will trigger a force event."]
    #[inline(always)]
    #[must_use]
    pub fn gen2_b_nciforce(&mut self) -> GEN2_B_NCIFORCE_W<13> {
        GEN2_B_NCIFORCE_W::new(self)
    }
    #[doc = "Bits 14:15 - non-continuous immediate software force mode for PWM2B, 0: disabled, 1: low, 2: high, 3: disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gen2_b_nciforce_mode(&mut self) -> GEN2_B_NCIFORCE_MODE_W<14> {
        GEN2_B_NCIFORCE_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Permissives to force PWM2A and PWM2B outputs by software\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen2_force](index.html) module"]
pub struct GEN2_FORCE_SPEC;
impl crate::RegisterSpec for GEN2_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen2_force::R](R) reader structure"]
impl crate::Readable for GEN2_FORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen2_force::W](W) writer structure"]
impl crate::Writable for GEN2_FORCE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN2_FORCE to value 0x20"]
impl crate::Resettable for GEN2_FORCE_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
