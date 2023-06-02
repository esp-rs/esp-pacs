#[doc = "Register `FAULT_DETECT` reader"]
pub struct R(crate::R<FAULT_DETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULT_DETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULT_DETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULT_DETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAULT_DETECT` writer"]
pub struct W(crate::W<FAULT_DETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULT_DETECT_SPEC>;
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
impl From<crate::W<FAULT_DETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULT_DETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0_EN` reader - "]
pub type F0_EN_R = crate::BitReader;
#[doc = "Field `F0_EN` writer - "]
pub type F0_EN_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_DETECT_SPEC, O>;
#[doc = "Field `F1_EN` reader - "]
pub type F1_EN_R = crate::BitReader;
#[doc = "Field `F1_EN` writer - "]
pub type F1_EN_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_DETECT_SPEC, O>;
#[doc = "Field `F2_EN` reader - "]
pub type F2_EN_R = crate::BitReader;
#[doc = "Field `F2_EN` writer - "]
pub type F2_EN_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_DETECT_SPEC, O>;
#[doc = "Field `F0_POLE` reader - "]
pub type F0_POLE_R = crate::BitReader;
#[doc = "Field `F0_POLE` writer - "]
pub type F0_POLE_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_DETECT_SPEC, O>;
#[doc = "Field `F1_POLE` reader - "]
pub type F1_POLE_R = crate::BitReader;
#[doc = "Field `F1_POLE` writer - "]
pub type F1_POLE_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_DETECT_SPEC, O>;
#[doc = "Field `F2_POLE` reader - "]
pub type F2_POLE_R = crate::BitReader;
#[doc = "Field `F2_POLE` writer - "]
pub type F2_POLE_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_DETECT_SPEC, O>;
#[doc = "Field `EVENT_F0` reader - "]
pub type EVENT_F0_R = crate::BitReader;
#[doc = "Field `EVENT_F1` reader - "]
pub type EVENT_F1_R = crate::BitReader;
#[doc = "Field `EVENT_F2` reader - "]
pub type EVENT_F2_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn f0_en(&self) -> F0_EN_R {
        F0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn f1_en(&self) -> F1_EN_R {
        F1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn f2_en(&self) -> F2_EN_R {
        F2_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn f0_pole(&self) -> F0_POLE_R {
        F0_POLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn f1_pole(&self) -> F1_POLE_R {
        F1_POLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn f2_pole(&self) -> F2_POLE_R {
        F2_POLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn event_f0(&self) -> EVENT_F0_R {
        EVENT_F0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn event_f1(&self) -> EVENT_F1_R {
        EVENT_F1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn event_f2(&self) -> EVENT_F2_R {
        EVENT_F2_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FAULT_DETECT")
            .field("f0_en", &format_args!("{}", self.f0_en().bit()))
            .field("f1_en", &format_args!("{}", self.f1_en().bit()))
            .field("f2_en", &format_args!("{}", self.f2_en().bit()))
            .field("f0_pole", &format_args!("{}", self.f0_pole().bit()))
            .field("f1_pole", &format_args!("{}", self.f1_pole().bit()))
            .field("f2_pole", &format_args!("{}", self.f2_pole().bit()))
            .field("event_f0", &format_args!("{}", self.event_f0().bit()))
            .field("event_f1", &format_args!("{}", self.event_f1().bit()))
            .field("event_f2", &format_args!("{}", self.event_f2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FAULT_DETECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn f0_en(&mut self) -> F0_EN_W<0> {
        F0_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn f1_en(&mut self) -> F1_EN_W<1> {
        F1_EN_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn f2_en(&mut self) -> F2_EN_W<2> {
        F2_EN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn f0_pole(&mut self) -> F0_POLE_W<3> {
        F0_POLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn f1_pole(&mut self) -> F1_POLE_W<4> {
        F1_POLE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn f2_pole(&mut self) -> F2_POLE_W<5> {
        F2_POLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fault_detect](index.html) module"]
pub struct FAULT_DETECT_SPEC;
impl crate::RegisterSpec for FAULT_DETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fault_detect::R](R) reader structure"]
impl crate::Readable for FAULT_DETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fault_detect::W](W) writer structure"]
impl crate::Writable for FAULT_DETECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FAULT_DETECT to value 0"]
impl crate::Resettable for FAULT_DETECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
