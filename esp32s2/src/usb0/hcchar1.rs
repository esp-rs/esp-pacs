#[doc = "Register `HCCHAR1` reader"]
pub struct R(crate::R<HCCHAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR1` writer"]
pub struct W(crate::W<HCCHAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR1_SPEC>;
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
impl From<crate::W<HCCHAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_MPS1` reader - "]
pub type H_MPS1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `H_MPS1` writer - "]
pub type H_MPS1_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR1_SPEC, 11, O, u16, u16>;
#[doc = "Field `H_EPNUM1` reader - "]
pub type H_EPNUM1_R = crate::FieldReader;
#[doc = "Field `H_EPNUM1` writer - "]
pub type H_EPNUM1_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR1_SPEC, 4, O>;
#[doc = "Field `H_EPDIR1` reader - "]
pub type H_EPDIR1_R = crate::BitReader;
#[doc = "Field `H_EPDIR1` writer - "]
pub type H_EPDIR1_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR1_SPEC, O>;
#[doc = "Field `H_LSPDDEV1` reader - "]
pub type H_LSPDDEV1_R = crate::BitReader;
#[doc = "Field `H_LSPDDEV1` writer - "]
pub type H_LSPDDEV1_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR1_SPEC, O>;
#[doc = "Field `H_EPTYPE1` reader - "]
pub type H_EPTYPE1_R = crate::FieldReader;
#[doc = "Field `H_EPTYPE1` writer - "]
pub type H_EPTYPE1_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR1_SPEC, 2, O>;
#[doc = "Field `H_EC1` reader - "]
pub type H_EC1_R = crate::BitReader;
#[doc = "Field `H_EC1` writer - "]
pub type H_EC1_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR1_SPEC, O>;
#[doc = "Field `H_DEVADDR1` reader - "]
pub type H_DEVADDR1_R = crate::FieldReader;
#[doc = "Field `H_DEVADDR1` writer - "]
pub type H_DEVADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR1_SPEC, 7, O>;
#[doc = "Field `H_ODDFRM1` reader - "]
pub type H_ODDFRM1_R = crate::BitReader;
#[doc = "Field `H_ODDFRM1` writer - "]
pub type H_ODDFRM1_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR1_SPEC, O>;
#[doc = "Field `H_CHDIS1` reader - "]
pub type H_CHDIS1_R = crate::BitReader;
#[doc = "Field `H_CHDIS1` writer - "]
pub type H_CHDIS1_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR1_SPEC, O>;
#[doc = "Field `H_CHENA1` reader - "]
pub type H_CHENA1_R = crate::BitReader;
#[doc = "Field `H_CHENA1` writer - "]
pub type H_CHENA1_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR1_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps1(&self) -> H_MPS1_R {
        H_MPS1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum1(&self) -> H_EPNUM1_R {
        H_EPNUM1_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir1(&self) -> H_EPDIR1_R {
        H_EPDIR1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev1(&self) -> H_LSPDDEV1_R {
        H_LSPDDEV1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype1(&self) -> H_EPTYPE1_R {
        H_EPTYPE1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec1(&self) -> H_EC1_R {
        H_EC1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr1(&self) -> H_DEVADDR1_R {
        H_DEVADDR1_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm1(&self) -> H_ODDFRM1_R {
        H_ODDFRM1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis1(&self) -> H_CHDIS1_R {
        H_CHDIS1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena1(&self) -> H_CHENA1_R {
        H_CHENA1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR1")
            .field("h_mps1", &format_args!("{}", self.h_mps1().bits()))
            .field("h_epnum1", &format_args!("{}", self.h_epnum1().bits()))
            .field("h_epdir1", &format_args!("{}", self.h_epdir1().bit()))
            .field("h_lspddev1", &format_args!("{}", self.h_lspddev1().bit()))
            .field("h_eptype1", &format_args!("{}", self.h_eptype1().bits()))
            .field("h_ec1", &format_args!("{}", self.h_ec1().bit()))
            .field("h_devaddr1", &format_args!("{}", self.h_devaddr1().bits()))
            .field("h_oddfrm1", &format_args!("{}", self.h_oddfrm1().bit()))
            .field("h_chdis1", &format_args!("{}", self.h_chdis1().bit()))
            .field("h_chena1", &format_args!("{}", self.h_chena1().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCCHAR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps1(&mut self) -> H_MPS1_W<0> {
        H_MPS1_W::new(self)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum1(&mut self) -> H_EPNUM1_W<11> {
        H_EPNUM1_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir1(&mut self) -> H_EPDIR1_W<15> {
        H_EPDIR1_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev1(&mut self) -> H_LSPDDEV1_W<17> {
        H_LSPDDEV1_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype1(&mut self) -> H_EPTYPE1_W<18> {
        H_EPTYPE1_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec1(&mut self) -> H_EC1_W<21> {
        H_EC1_W::new(self)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr1(&mut self) -> H_DEVADDR1_W<22> {
        H_DEVADDR1_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm1(&mut self) -> H_ODDFRM1_W<29> {
        H_ODDFRM1_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis1(&mut self) -> H_CHDIS1_W<30> {
        H_CHDIS1_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena1(&mut self) -> H_CHENA1_W<31> {
        H_CHENA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar1](index.html) module"]
pub struct HCCHAR1_SPEC;
impl crate::RegisterSpec for HCCHAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar1::R](R) reader structure"]
impl crate::Readable for HCCHAR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar1::W](W) writer structure"]
impl crate::Writable for HCCHAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR1 to value 0"]
impl crate::Resettable for HCCHAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
