#[doc = "Register `DOEPTSIZ6` reader"]
pub struct R(crate::R<DOEPTSIZ6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ6` writer"]
pub struct W(crate::W<DOEPTSIZ6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ6_SPEC>;
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
impl From<crate::W<DOEPTSIZ6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE6` reader - "]
pub type XFERSIZE6_R = crate::FieldReader;
#[doc = "Field `XFERSIZE6` writer - "]
pub type XFERSIZE6_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPTSIZ6_SPEC, 7, O>;
#[doc = "Field `PKTCNT6` reader - "]
pub type PKTCNT6_R = crate::BitReader;
#[doc = "Field `PKTCNT6` writer - "]
pub type PKTCNT6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPTSIZ6_SPEC, O>;
#[doc = "Field `SUPCNT6` reader - "]
pub type SUPCNT6_R = crate::FieldReader;
#[doc = "Field `SUPCNT6` writer - "]
pub type SUPCNT6_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPTSIZ6_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize6(&self) -> XFERSIZE6_R {
        XFERSIZE6_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt6(&self) -> PKTCNT6_R {
        PKTCNT6_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt6(&self) -> SUPCNT6_R {
        SUPCNT6_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ6")
            .field("xfersize6", &format_args!("{}", self.xfersize6().bits()))
            .field("pktcnt6", &format_args!("{}", self.pktcnt6().bit()))
            .field("supcnt6", &format_args!("{}", self.supcnt6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize6(&mut self) -> XFERSIZE6_W<0> {
        XFERSIZE6_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt6(&mut self) -> PKTCNT6_W<19> {
        PKTCNT6_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt6(&mut self) -> SUPCNT6_W<29> {
        SUPCNT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz6](index.html) module"]
pub struct DOEPTSIZ6_SPEC;
impl crate::RegisterSpec for DOEPTSIZ6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz6::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz6::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ6 to value 0"]
impl crate::Resettable for DOEPTSIZ6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
