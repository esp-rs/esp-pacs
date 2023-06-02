#[doc = "Register `HCTSIZ1` reader"]
pub struct R(crate::R<HCTSIZ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTSIZ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTSIZ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTSIZ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTSIZ1` writer"]
pub struct W(crate::W<HCTSIZ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTSIZ1_SPEC>;
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
impl From<crate::W<HCTSIZ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTSIZ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERSIZE1` reader - "]
pub type H_XFERSIZE1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `H_XFERSIZE1` writer - "]
pub type H_XFERSIZE1_W<'a, const O: u8> = crate::FieldWriter<'a, HCTSIZ1_SPEC, 19, O, u32, u32>;
#[doc = "Field `H_PKTCNT1` reader - "]
pub type H_PKTCNT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `H_PKTCNT1` writer - "]
pub type H_PKTCNT1_W<'a, const O: u8> = crate::FieldWriter<'a, HCTSIZ1_SPEC, 10, O, u16, u16>;
#[doc = "Field `H_PID1` reader - "]
pub type H_PID1_R = crate::FieldReader;
#[doc = "Field `H_PID1` writer - "]
pub type H_PID1_W<'a, const O: u8> = crate::FieldWriter<'a, HCTSIZ1_SPEC, 2, O>;
#[doc = "Field `H_DOPNG1` reader - "]
pub type H_DOPNG1_R = crate::BitReader;
#[doc = "Field `H_DOPNG1` writer - "]
pub type H_DOPNG1_W<'a, const O: u8> = crate::BitWriter<'a, HCTSIZ1_SPEC, O>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize1(&self) -> H_XFERSIZE1_R {
        H_XFERSIZE1_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt1(&self) -> H_PKTCNT1_R {
        H_PKTCNT1_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid1(&self) -> H_PID1_R {
        H_PID1_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng1(&self) -> H_DOPNG1_R {
        H_DOPNG1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ1")
            .field(
                "h_xfersize1",
                &format_args!("{}", self.h_xfersize1().bits()),
            )
            .field("h_pktcnt1", &format_args!("{}", self.h_pktcnt1().bits()))
            .field("h_pid1", &format_args!("{}", self.h_pid1().bits()))
            .field("h_dopng1", &format_args!("{}", self.h_dopng1().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCTSIZ1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfersize1(&mut self) -> H_XFERSIZE1_W<0> {
        H_XFERSIZE1_W::new(self)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_pktcnt1(&mut self) -> H_PKTCNT1_W<19> {
        H_PKTCNT1_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn h_pid1(&mut self) -> H_PID1_W<29> {
        H_PID1_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dopng1(&mut self) -> H_DOPNG1_W<31> {
        H_DOPNG1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz1](index.html) module"]
pub struct HCTSIZ1_SPEC;
impl crate::RegisterSpec for HCTSIZ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctsiz1::R](R) reader structure"]
impl crate::Readable for HCTSIZ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctsiz1::W](W) writer structure"]
impl crate::Writable for HCTSIZ1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTSIZ1 to value 0"]
impl crate::Resettable for HCTSIZ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
