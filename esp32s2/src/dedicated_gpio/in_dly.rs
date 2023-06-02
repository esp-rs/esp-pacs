#[doc = "Register `IN_DLY` reader"]
pub struct R(crate::R<IN_DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_DLY` writer"]
pub struct W(crate::W<IN_DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_DLY_SPEC>;
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
impl From<crate::W<IN_DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH0_R = crate::FieldReader;
#[doc = "Field `CH0` writer - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH0_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
#[doc = "Field `CH1` reader - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH1_R = crate::FieldReader;
#[doc = "Field `CH1` writer - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH1_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
#[doc = "Field `CH2` reader - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH2_R = crate::FieldReader;
#[doc = "Field `CH2` writer - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH2_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
#[doc = "Field `CH3` reader - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH3_R = crate::FieldReader;
#[doc = "Field `CH3` writer - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH3_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
#[doc = "Field `CH4` reader - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH4_R = crate::FieldReader;
#[doc = "Field `CH4` writer - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH4_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
#[doc = "Field `CH5` reader - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH5_R = crate::FieldReader;
#[doc = "Field `CH5` writer - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH5_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
#[doc = "Field `CH6` reader - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH6_R = crate::FieldReader;
#[doc = "Field `CH6` writer - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH6_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
#[doc = "Field `CH7` reader - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH7_R = crate::FieldReader;
#[doc = "Field `CH7` writer - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
pub type CH7_W<'a, const O: u8> = crate::FieldWriter<'a, IN_DLY_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DLY")
            .field("ch0", &format_args!("{}", self.ch0().bits()))
            .field("ch1", &format_args!("{}", self.ch1().bits()))
            .field("ch2", &format_args!("{}", self.ch2().bits()))
            .field("ch3", &format_args!("{}", self.ch3().bits()))
            .field("ch4", &format_args!("{}", self.ch4().bits()))
            .field("ch5", &format_args!("{}", self.ch5().bits()))
            .field("ch6", &format_args!("{}", self.ch6().bits()))
            .field("ch7", &format_args!("{}", self.ch7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_DLY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Configure GPIO0 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bits 2:3 - Configure GPIO1 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<2> {
        CH1_W::new(self)
    }
    #[doc = "Bits 4:5 - Configure GPIO2 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<4> {
        CH2_W::new(self)
    }
    #[doc = "Bits 6:7 - Configure GPIO3 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<6> {
        CH3_W::new(self)
    }
    #[doc = "Bits 8:9 - Configure GPIO4 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<8> {
        CH4_W::new(self)
    }
    #[doc = "Bits 10:11 - Configure GPIO5 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<10> {
        CH5_W::new(self)
    }
    #[doc = "Bits 12:13 - Configure GPIO6 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<12> {
        CH6_W::new(self)
    }
    #[doc = "Bits 14:15 - Configure GPIO7 input delay. 0: no delay. 1: one clock delay. 2: two clock delay. 3: three clock delay."]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<14> {
        CH7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated GPIO input delay configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_dly](index.html) module"]
pub struct IN_DLY_SPEC;
impl crate::RegisterSpec for IN_DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_dly::R](R) reader structure"]
impl crate::Readable for IN_DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_dly::W](W) writer structure"]
impl crate::Writable for IN_DLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_DLY to value 0"]
impl crate::Resettable for IN_DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
