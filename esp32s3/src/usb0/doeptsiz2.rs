#[doc = "Register `DOEPTSIZ2` reader"]
pub struct R(crate::R<DOEPTSIZ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPTSIZ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPTSIZ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPTSIZ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPTSIZ2` writer"]
pub struct W(crate::W<DOEPTSIZ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPTSIZ2_SPEC>;
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
impl From<crate::W<DOEPTSIZ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPTSIZ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE2` reader - "]
pub type XFERSIZE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XFERSIZE2` writer - "]
pub type XFERSIZE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ2_SPEC, u8, u8, 7, O>;
#[doc = "Field `PKTCNT2` reader - "]
pub type PKTCNT2_R = crate::BitReader<bool>;
#[doc = "Field `PKTCNT2` writer - "]
pub type PKTCNT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPTSIZ2_SPEC, bool, O>;
#[doc = "Field `SUPCNT2` reader - "]
pub type SUPCNT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUPCNT2` writer - "]
pub type SUPCNT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DOEPTSIZ2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize2(&self) -> XFERSIZE2_R {
        XFERSIZE2_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt2(&self) -> PKTCNT2_R {
        PKTCNT2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt2(&self) -> SUPCNT2_R {
        SUPCNT2_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize2(&mut self) -> XFERSIZE2_W<0> {
        XFERSIZE2_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt2(&mut self) -> PKTCNT2_W<19> {
        PKTCNT2_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt2(&mut self) -> SUPCNT2_W<29> {
        SUPCNT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doeptsiz2](index.html) module"]
pub struct DOEPTSIZ2_SPEC;
impl crate::RegisterSpec for DOEPTSIZ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doeptsiz2::R](R) reader structure"]
impl crate::Readable for DOEPTSIZ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doeptsiz2::W](W) writer structure"]
impl crate::Writable for DOEPTSIZ2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPTSIZ2 to value 0"]
impl crate::Resettable for DOEPTSIZ2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
