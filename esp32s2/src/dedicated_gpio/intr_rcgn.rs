#[doc = "Register `INTR_RCGN` reader"]
pub struct R(crate::R<INTR_RCGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_RCGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_RCGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_RCGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_RCGN` writer"]
pub struct W(crate::W<INTR_RCGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_RCGN_SPEC>;
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
impl From<crate::W<INTR_RCGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_RCGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_MODE_CH0` reader - Configure channel 0 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH0` writer - Configure channel 0 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTR_MODE_CH1` reader - Configure channel 1 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH1` writer - Configure channel 1 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTR_MODE_CH2` reader - Configure channel 2 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH2` writer - Configure channel 2 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTR_MODE_CH3` reader - Configure channel 3 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH3` writer - Configure channel 3 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTR_MODE_CH4` reader - Configure channel 4 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH4` writer - Configure channel 4 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTR_MODE_CH5` reader - Configure channel 5 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH5` writer - Configure channel 5 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTR_MODE_CH6` reader - Configure channel 6 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH6` writer - Configure channel 6 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
#[doc = "Field `INTR_MODE_CH7` reader - Configure channel 7 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTR_MODE_CH7` writer - Configure channel 7 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
pub type INTR_MODE_CH7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_RCGN_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Configure channel 0 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch0(&self) -> INTR_MODE_CH0_R {
        INTR_MODE_CH0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Configure channel 1 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch1(&self) -> INTR_MODE_CH1_R {
        INTR_MODE_CH1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Configure channel 2 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch2(&self) -> INTR_MODE_CH2_R {
        INTR_MODE_CH2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Configure channel 3 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch3(&self) -> INTR_MODE_CH3_R {
        INTR_MODE_CH3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Configure channel 4 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch4(&self) -> INTR_MODE_CH4_R {
        INTR_MODE_CH4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Configure channel 5 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch5(&self) -> INTR_MODE_CH5_R {
        INTR_MODE_CH5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Configure channel 6 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch6(&self) -> INTR_MODE_CH6_R {
        INTR_MODE_CH6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Configure channel 7 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch7(&self) -> INTR_MODE_CH7_R {
        INTR_MODE_CH7_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Configure channel 0 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch0(&mut self) -> INTR_MODE_CH0_W<0> {
        INTR_MODE_CH0_W::new(self)
    }
    #[doc = "Bits 3:5 - Configure channel 1 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch1(&mut self) -> INTR_MODE_CH1_W<3> {
        INTR_MODE_CH1_W::new(self)
    }
    #[doc = "Bits 6:8 - Configure channel 2 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch2(&mut self) -> INTR_MODE_CH2_W<6> {
        INTR_MODE_CH2_W::new(self)
    }
    #[doc = "Bits 9:11 - Configure channel 3 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch3(&mut self) -> INTR_MODE_CH3_W<9> {
        INTR_MODE_CH3_W::new(self)
    }
    #[doc = "Bits 12:14 - Configure channel 4 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch4(&mut self) -> INTR_MODE_CH4_W<12> {
        INTR_MODE_CH4_W::new(self)
    }
    #[doc = "Bits 15:17 - Configure channel 5 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch5(&mut self) -> INTR_MODE_CH5_W<15> {
        INTR_MODE_CH5_W::new(self)
    }
    #[doc = "Bits 18:20 - Configure channel 6 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch6(&mut self) -> INTR_MODE_CH6_W<18> {
        INTR_MODE_CH6_W::new(self)
    }
    #[doc = "Bits 21:23 - Configure channel 7 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger."]
    #[inline(always)]
    pub fn intr_mode_ch7(&mut self) -> INTR_MODE_CH7_W<21> {
        INTR_MODE_CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated GPIO interrupts generation mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_rcgn](index.html) module"]
pub struct INTR_RCGN_SPEC;
impl crate::RegisterSpec for INTR_RCGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_rcgn::R](R) reader structure"]
impl crate::Readable for INTR_RCGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_rcgn::W](W) writer structure"]
impl crate::Writable for INTR_RCGN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR_RCGN to value 0"]
impl crate::Resettable for INTR_RCGN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
