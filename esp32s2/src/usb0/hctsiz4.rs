#[doc = "Register `HCTSIZ4` reader"]
pub struct R(crate::R<HCTSIZ4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCTSIZ4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCTSIZ4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCTSIZ4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCTSIZ4` writer"]
pub struct W(crate::W<HCTSIZ4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCTSIZ4_SPEC>;
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
impl From<crate::W<HCTSIZ4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCTSIZ4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERSIZE4` reader - "]
pub type H_XFERSIZE4_R = crate::FieldReader<u32>;
#[doc = "Field `H_XFERSIZE4` writer - "]
pub type H_XFERSIZE4_W<'a, const O: u8> = crate::FieldWriter<'a, HCTSIZ4_SPEC, 19, O, u32>;
#[doc = "Field `H_PKTCNT4` reader - "]
pub type H_PKTCNT4_R = crate::FieldReader<u16>;
#[doc = "Field `H_PKTCNT4` writer - "]
pub type H_PKTCNT4_W<'a, const O: u8> = crate::FieldWriter<'a, HCTSIZ4_SPEC, 10, O, u16>;
#[doc = "Field `H_PID4` reader - "]
pub type H_PID4_R = crate::FieldReader;
#[doc = "Field `H_PID4` writer - "]
pub type H_PID4_W<'a, const O: u8> = crate::FieldWriter<'a, HCTSIZ4_SPEC, 2, O>;
#[doc = "Field `H_DOPNG4` reader - "]
pub type H_DOPNG4_R = crate::BitReader;
#[doc = "Field `H_DOPNG4` writer - "]
pub type H_DOPNG4_W<'a, const O: u8> = crate::BitWriter<'a, HCTSIZ4_SPEC, O>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn h_xfersize4(&self) -> H_XFERSIZE4_R {
        H_XFERSIZE4_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn h_pktcnt4(&self) -> H_PKTCNT4_R {
        H_PKTCNT4_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn h_pid4(&self) -> H_PID4_R {
        H_PID4_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_dopng4(&self) -> H_DOPNG4_R {
        H_DOPNG4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ4")
            .field(
                "h_xfersize4",
                &format_args!("{}", self.h_xfersize4().bits()),
            )
            .field("h_pktcnt4", &format_args!("{}", self.h_pktcnt4().bits()))
            .field("h_pid4", &format_args!("{}", self.h_pid4().bits()))
            .field("h_dopng4", &format_args!("{}", self.h_dopng4().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCTSIZ4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfersize4(&mut self) -> H_XFERSIZE4_W<0> {
        H_XFERSIZE4_W::new(self)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_pktcnt4(&mut self) -> H_PKTCNT4_W<19> {
        H_PKTCNT4_W::new(self)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn h_pid4(&mut self) -> H_PID4_W<29> {
        H_PID4_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_dopng4(&mut self) -> H_DOPNG4_W<31> {
        H_DOPNG4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hctsiz4](index.html) module"]
pub struct HCTSIZ4_SPEC;
impl crate::RegisterSpec for HCTSIZ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hctsiz4::R](R) reader structure"]
impl crate::Readable for HCTSIZ4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hctsiz4::W](W) writer structure"]
impl crate::Writable for HCTSIZ4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCTSIZ4 to value 0"]
impl crate::Resettable for HCTSIZ4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
