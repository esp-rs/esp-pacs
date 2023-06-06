#[doc = "Register `HCCHAR3` reader"]
pub struct R(crate::R<HCCHAR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR3` writer"]
pub struct W(crate::W<HCCHAR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR3_SPEC>;
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
impl From<crate::W<HCCHAR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_MPS3` reader - "]
pub type H_MPS3_R = crate::FieldReader<u16>;
#[doc = "Field `H_MPS3` writer - "]
pub type H_MPS3_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR3_SPEC, 11, O, u16>;
#[doc = "Field `H_EPNUM3` reader - "]
pub type H_EPNUM3_R = crate::FieldReader;
#[doc = "Field `H_EPNUM3` writer - "]
pub type H_EPNUM3_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR3_SPEC, 4, O>;
#[doc = "Field `H_EPDIR3` reader - "]
pub type H_EPDIR3_R = crate::BitReader;
#[doc = "Field `H_EPDIR3` writer - "]
pub type H_EPDIR3_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR3_SPEC, O>;
#[doc = "Field `H_LSPDDEV3` reader - "]
pub type H_LSPDDEV3_R = crate::BitReader;
#[doc = "Field `H_LSPDDEV3` writer - "]
pub type H_LSPDDEV3_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR3_SPEC, O>;
#[doc = "Field `H_EPTYPE3` reader - "]
pub type H_EPTYPE3_R = crate::FieldReader;
#[doc = "Field `H_EPTYPE3` writer - "]
pub type H_EPTYPE3_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR3_SPEC, 2, O>;
#[doc = "Field `H_EC3` reader - "]
pub type H_EC3_R = crate::BitReader;
#[doc = "Field `H_EC3` writer - "]
pub type H_EC3_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR3_SPEC, O>;
#[doc = "Field `H_DEVADDR3` reader - "]
pub type H_DEVADDR3_R = crate::FieldReader;
#[doc = "Field `H_DEVADDR3` writer - "]
pub type H_DEVADDR3_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR3_SPEC, 7, O>;
#[doc = "Field `H_ODDFRM3` reader - "]
pub type H_ODDFRM3_R = crate::BitReader;
#[doc = "Field `H_ODDFRM3` writer - "]
pub type H_ODDFRM3_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR3_SPEC, O>;
#[doc = "Field `H_CHDIS3` reader - "]
pub type H_CHDIS3_R = crate::BitReader;
#[doc = "Field `H_CHDIS3` writer - "]
pub type H_CHDIS3_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR3_SPEC, O>;
#[doc = "Field `H_CHENA3` reader - "]
pub type H_CHENA3_R = crate::BitReader;
#[doc = "Field `H_CHENA3` writer - "]
pub type H_CHENA3_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR3_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps3(&self) -> H_MPS3_R {
        H_MPS3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum3(&self) -> H_EPNUM3_R {
        H_EPNUM3_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir3(&self) -> H_EPDIR3_R {
        H_EPDIR3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev3(&self) -> H_LSPDDEV3_R {
        H_LSPDDEV3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype3(&self) -> H_EPTYPE3_R {
        H_EPTYPE3_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec3(&self) -> H_EC3_R {
        H_EC3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr3(&self) -> H_DEVADDR3_R {
        H_DEVADDR3_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm3(&self) -> H_ODDFRM3_R {
        H_ODDFRM3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis3(&self) -> H_CHDIS3_R {
        H_CHDIS3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena3(&self) -> H_CHENA3_R {
        H_CHENA3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR3")
            .field("h_mps3", &format_args!("{}", self.h_mps3().bits()))
            .field("h_epnum3", &format_args!("{}", self.h_epnum3().bits()))
            .field("h_epdir3", &format_args!("{}", self.h_epdir3().bit()))
            .field("h_lspddev3", &format_args!("{}", self.h_lspddev3().bit()))
            .field("h_eptype3", &format_args!("{}", self.h_eptype3().bits()))
            .field("h_ec3", &format_args!("{}", self.h_ec3().bit()))
            .field("h_devaddr3", &format_args!("{}", self.h_devaddr3().bits()))
            .field("h_oddfrm3", &format_args!("{}", self.h_oddfrm3().bit()))
            .field("h_chdis3", &format_args!("{}", self.h_chdis3().bit()))
            .field("h_chena3", &format_args!("{}", self.h_chena3().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCCHAR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps3(&mut self) -> H_MPS3_W<0> {
        H_MPS3_W::new(self)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum3(&mut self) -> H_EPNUM3_W<11> {
        H_EPNUM3_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir3(&mut self) -> H_EPDIR3_W<15> {
        H_EPDIR3_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev3(&mut self) -> H_LSPDDEV3_W<17> {
        H_LSPDDEV3_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype3(&mut self) -> H_EPTYPE3_W<18> {
        H_EPTYPE3_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec3(&mut self) -> H_EC3_W<21> {
        H_EC3_W::new(self)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr3(&mut self) -> H_DEVADDR3_W<22> {
        H_DEVADDR3_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm3(&mut self) -> H_ODDFRM3_W<29> {
        H_ODDFRM3_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis3(&mut self) -> H_CHDIS3_W<30> {
        H_CHDIS3_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena3(&mut self) -> H_CHENA3_W<31> {
        H_CHENA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar3](index.html) module"]
pub struct HCCHAR3_SPEC;
impl crate::RegisterSpec for HCCHAR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar3::R](R) reader structure"]
impl crate::Readable for HCCHAR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar3::W](W) writer structure"]
impl crate::Writable for HCCHAR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR3 to value 0"]
impl crate::Resettable for HCCHAR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
