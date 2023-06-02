#[doc = "Register `DOEPTSIZ4` reader"]
pub struct R(crate::R<DOEPTSIZ4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ4` writer"]
pub struct W(crate::W<DOEPTSIZ4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ4_SPEC>;
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
impl From<crate::W<DOEPTSIZ4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE4` reader - "]
pub type XFERSIZE4_R = crate::FieldReader;
#[doc = "Field `XFERSIZE4` writer - "]
pub type XFERSIZE4_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPTSIZ4_SPEC, 7, O>;
#[doc = "Field `PKTCNT4` reader - "]
pub type PKTCNT4_R = crate::BitReader;
#[doc = "Field `PKTCNT4` writer - "]
pub type PKTCNT4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPTSIZ4_SPEC, O>;
#[doc = "Field `SUPCNT4` reader - "]
pub type SUPCNT4_R = crate::FieldReader;
#[doc = "Field `SUPCNT4` writer - "]
pub type SUPCNT4_W<'a, const O: u8> = crate::FieldWriter<'a, DOEPTSIZ4_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize4(&self) -> XFERSIZE4_R {
        XFERSIZE4_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt4(&self) -> PKTCNT4_R {
        PKTCNT4_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt4(&self) -> SUPCNT4_R {
        SUPCNT4_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ4")
            .field("xfersize4", &format_args!("{}", self.xfersize4().bits()))
            .field("pktcnt4", &format_args!("{}", self.pktcnt4().bit()))
            .field("supcnt4", &format_args!("{}", self.supcnt4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize4(&mut self) -> XFERSIZE4_W<0> {
        XFERSIZE4_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt4(&mut self) -> PKTCNT4_W<19> {
        PKTCNT4_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt4(&mut self) -> SUPCNT4_W<29> {
        SUPCNT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz4](index.html) module"]
pub struct DOEPTSIZ4_SPEC;
impl crate::RegisterSpec for DOEPTSIZ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz4::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz4::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ4 to value 0"]
impl crate::Resettable for DOEPTSIZ4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
