#[doc = "Register `DAINTMSK` reader"]
pub struct R(crate::R<DAINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAINTMSK` writer"]
pub struct W(crate::W<DAINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAINTMSK_SPEC>;
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
impl From<crate::W<DAINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPMSK0` reader - "]
pub type INEPMSK0_R = crate::BitReader;
#[doc = "Field `INEPMSK0` writer - "]
pub type INEPMSK0_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `INEPMSK1` reader - "]
pub type INEPMSK1_R = crate::BitReader;
#[doc = "Field `INEPMSK1` writer - "]
pub type INEPMSK1_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `INEPMSK2` reader - "]
pub type INEPMSK2_R = crate::BitReader;
#[doc = "Field `INEPMSK2` writer - "]
pub type INEPMSK2_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `INEPMSK3` reader - "]
pub type INEPMSK3_R = crate::BitReader;
#[doc = "Field `INEPMSK3` writer - "]
pub type INEPMSK3_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `INEPMSK4` reader - "]
pub type INEPMSK4_R = crate::BitReader;
#[doc = "Field `INEPMSK4` writer - "]
pub type INEPMSK4_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `INEPMSK5` reader - "]
pub type INEPMSK5_R = crate::BitReader;
#[doc = "Field `INEPMSK5` writer - "]
pub type INEPMSK5_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `INEPMSK6` reader - "]
pub type INEPMSK6_R = crate::BitReader;
#[doc = "Field `INEPMSK6` writer - "]
pub type INEPMSK6_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `OUTEPMSK0` reader - "]
pub type OUTEPMSK0_R = crate::BitReader;
#[doc = "Field `OUTEPMSK0` writer - "]
pub type OUTEPMSK0_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `OUTEPMSK1` reader - "]
pub type OUTEPMSK1_R = crate::BitReader;
#[doc = "Field `OUTEPMSK1` writer - "]
pub type OUTEPMSK1_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `OUTEPMSK2` reader - "]
pub type OUTEPMSK2_R = crate::BitReader;
#[doc = "Field `OUTEPMSK2` writer - "]
pub type OUTEPMSK2_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `OUTEPMSK3` reader - "]
pub type OUTEPMSK3_R = crate::BitReader;
#[doc = "Field `OUTEPMSK3` writer - "]
pub type OUTEPMSK3_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `OUTEPMSK4` reader - "]
pub type OUTEPMSK4_R = crate::BitReader;
#[doc = "Field `OUTEPMSK4` writer - "]
pub type OUTEPMSK4_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `OUTEPMSK5` reader - "]
pub type OUTEPMSK5_R = crate::BitReader;
#[doc = "Field `OUTEPMSK5` writer - "]
pub type OUTEPMSK5_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
#[doc = "Field `OUTEPMSK6` reader - "]
pub type OUTEPMSK6_R = crate::BitReader;
#[doc = "Field `OUTEPMSK6` writer - "]
pub type OUTEPMSK6_W<'a, const O: u8> = crate::BitWriter<'a, DAINTMSK_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn inepmsk0(&self) -> INEPMSK0_R {
        INEPMSK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn inepmsk1(&self) -> INEPMSK1_R {
        INEPMSK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn inepmsk2(&self) -> INEPMSK2_R {
        INEPMSK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn inepmsk3(&self) -> INEPMSK3_R {
        INEPMSK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inepmsk4(&self) -> INEPMSK4_R {
        INEPMSK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn inepmsk5(&self) -> INEPMSK5_R {
        INEPMSK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inepmsk6(&self) -> INEPMSK6_R {
        INEPMSK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn outepmsk0(&self) -> OUTEPMSK0_R {
        OUTEPMSK0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn outepmsk1(&self) -> OUTEPMSK1_R {
        OUTEPMSK1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn outepmsk2(&self) -> OUTEPMSK2_R {
        OUTEPMSK2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn outepmsk3(&self) -> OUTEPMSK3_R {
        OUTEPMSK3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn outepmsk4(&self) -> OUTEPMSK4_R {
        OUTEPMSK4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn outepmsk5(&self) -> OUTEPMSK5_R {
        OUTEPMSK5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn outepmsk6(&self) -> OUTEPMSK6_R {
        OUTEPMSK6_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINTMSK")
            .field("inepmsk0", &format_args!("{}", self.inepmsk0().bit()))
            .field("inepmsk1", &format_args!("{}", self.inepmsk1().bit()))
            .field("inepmsk2", &format_args!("{}", self.inepmsk2().bit()))
            .field("inepmsk3", &format_args!("{}", self.inepmsk3().bit()))
            .field("inepmsk4", &format_args!("{}", self.inepmsk4().bit()))
            .field("inepmsk5", &format_args!("{}", self.inepmsk5().bit()))
            .field("inepmsk6", &format_args!("{}", self.inepmsk6().bit()))
            .field("outepmsk0", &format_args!("{}", self.outepmsk0().bit()))
            .field("outepmsk1", &format_args!("{}", self.outepmsk1().bit()))
            .field("outepmsk2", &format_args!("{}", self.outepmsk2().bit()))
            .field("outepmsk3", &format_args!("{}", self.outepmsk3().bit()))
            .field("outepmsk4", &format_args!("{}", self.outepmsk4().bit()))
            .field("outepmsk5", &format_args!("{}", self.outepmsk5().bit()))
            .field("outepmsk6", &format_args!("{}", self.outepmsk6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DAINTMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk0(&mut self) -> INEPMSK0_W<0> {
        INEPMSK0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk1(&mut self) -> INEPMSK1_W<1> {
        INEPMSK1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk2(&mut self) -> INEPMSK2_W<2> {
        INEPMSK2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk3(&mut self) -> INEPMSK3_W<3> {
        INEPMSK3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk4(&mut self) -> INEPMSK4_W<4> {
        INEPMSK4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk5(&mut self) -> INEPMSK5_W<5> {
        INEPMSK5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn inepmsk6(&mut self) -> INEPMSK6_W<6> {
        INEPMSK6_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk0(&mut self) -> OUTEPMSK0_W<16> {
        OUTEPMSK0_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk1(&mut self) -> OUTEPMSK1_W<17> {
        OUTEPMSK1_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk2(&mut self) -> OUTEPMSK2_W<18> {
        OUTEPMSK2_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk3(&mut self) -> OUTEPMSK3_W<19> {
        OUTEPMSK3_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk4(&mut self) -> OUTEPMSK4_W<20> {
        OUTEPMSK4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk5(&mut self) -> OUTEPMSK5_W<21> {
        OUTEPMSK5_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn outepmsk6(&mut self) -> OUTEPMSK6_W<22> {
        OUTEPMSK6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daintmsk](index.html) module"]
pub struct DAINTMSK_SPEC;
impl crate::RegisterSpec for DAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daintmsk::R](R) reader structure"]
impl crate::Readable for DAINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](W) writer structure"]
impl crate::Writable for DAINTMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAINTMSK to value 0"]
impl crate::Resettable for DAINTMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
