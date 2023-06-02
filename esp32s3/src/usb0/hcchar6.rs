#[doc = "Register `HCCHAR6` reader"]
pub struct R(crate::R<HCCHAR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR6` writer"]
pub struct W(crate::W<HCCHAR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR6_SPEC>;
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
impl From<crate::W<HCCHAR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_MPS6` reader - "]
pub type H_MPS6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `H_MPS6` writer - "]
pub type H_MPS6_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR6_SPEC, 11, O, u16, u16>;
#[doc = "Field `H_EPNUM6` reader - "]
pub type H_EPNUM6_R = crate::FieldReader;
#[doc = "Field `H_EPNUM6` writer - "]
pub type H_EPNUM6_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR6_SPEC, 4, O>;
#[doc = "Field `H_EPDIR6` reader - "]
pub type H_EPDIR6_R = crate::BitReader;
#[doc = "Field `H_EPDIR6` writer - "]
pub type H_EPDIR6_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR6_SPEC, O>;
#[doc = "Field `H_LSPDDEV6` reader - "]
pub type H_LSPDDEV6_R = crate::BitReader;
#[doc = "Field `H_LSPDDEV6` writer - "]
pub type H_LSPDDEV6_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR6_SPEC, O>;
#[doc = "Field `H_EPTYPE6` reader - "]
pub type H_EPTYPE6_R = crate::FieldReader;
#[doc = "Field `H_EPTYPE6` writer - "]
pub type H_EPTYPE6_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR6_SPEC, 2, O>;
#[doc = "Field `H_EC6` reader - "]
pub type H_EC6_R = crate::BitReader;
#[doc = "Field `H_EC6` writer - "]
pub type H_EC6_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR6_SPEC, O>;
#[doc = "Field `H_DEVADDR6` reader - "]
pub type H_DEVADDR6_R = crate::FieldReader;
#[doc = "Field `H_DEVADDR6` writer - "]
pub type H_DEVADDR6_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR6_SPEC, 7, O>;
#[doc = "Field `H_ODDFRM6` reader - "]
pub type H_ODDFRM6_R = crate::BitReader;
#[doc = "Field `H_ODDFRM6` writer - "]
pub type H_ODDFRM6_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR6_SPEC, O>;
#[doc = "Field `H_CHDIS6` reader - "]
pub type H_CHDIS6_R = crate::BitReader;
#[doc = "Field `H_CHDIS6` writer - "]
pub type H_CHDIS6_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR6_SPEC, O>;
#[doc = "Field `H_CHENA6` reader - "]
pub type H_CHENA6_R = crate::BitReader;
#[doc = "Field `H_CHENA6` writer - "]
pub type H_CHENA6_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR6_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps6(&self) -> H_MPS6_R {
        H_MPS6_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum6(&self) -> H_EPNUM6_R {
        H_EPNUM6_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir6(&self) -> H_EPDIR6_R {
        H_EPDIR6_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev6(&self) -> H_LSPDDEV6_R {
        H_LSPDDEV6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype6(&self) -> H_EPTYPE6_R {
        H_EPTYPE6_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec6(&self) -> H_EC6_R {
        H_EC6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr6(&self) -> H_DEVADDR6_R {
        H_DEVADDR6_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm6(&self) -> H_ODDFRM6_R {
        H_ODDFRM6_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis6(&self) -> H_CHDIS6_R {
        H_CHDIS6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena6(&self) -> H_CHENA6_R {
        H_CHENA6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR6")
            .field("h_mps6", &format_args!("{}", self.h_mps6().bits()))
            .field("h_epnum6", &format_args!("{}", self.h_epnum6().bits()))
            .field("h_epdir6", &format_args!("{}", self.h_epdir6().bit()))
            .field("h_lspddev6", &format_args!("{}", self.h_lspddev6().bit()))
            .field("h_eptype6", &format_args!("{}", self.h_eptype6().bits()))
            .field("h_ec6", &format_args!("{}", self.h_ec6().bit()))
            .field("h_devaddr6", &format_args!("{}", self.h_devaddr6().bits()))
            .field("h_oddfrm6", &format_args!("{}", self.h_oddfrm6().bit()))
            .field("h_chdis6", &format_args!("{}", self.h_chdis6().bit()))
            .field("h_chena6", &format_args!("{}", self.h_chena6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCCHAR6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps6(&mut self) -> H_MPS6_W<0> {
        H_MPS6_W::new(self)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum6(&mut self) -> H_EPNUM6_W<11> {
        H_EPNUM6_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir6(&mut self) -> H_EPDIR6_W<15> {
        H_EPDIR6_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev6(&mut self) -> H_LSPDDEV6_W<17> {
        H_LSPDDEV6_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype6(&mut self) -> H_EPTYPE6_W<18> {
        H_EPTYPE6_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec6(&mut self) -> H_EC6_W<21> {
        H_EC6_W::new(self)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr6(&mut self) -> H_DEVADDR6_W<22> {
        H_DEVADDR6_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm6(&mut self) -> H_ODDFRM6_W<29> {
        H_ODDFRM6_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis6(&mut self) -> H_CHDIS6_W<30> {
        H_CHDIS6_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena6(&mut self) -> H_CHENA6_W<31> {
        H_CHENA6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar6](index.html) module"]
pub struct HCCHAR6_SPEC;
impl crate::RegisterSpec for HCCHAR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar6::R](R) reader structure"]
impl crate::Readable for HCCHAR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar6::W](W) writer structure"]
impl crate::Writable for HCCHAR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR6 to value 0"]
impl crate::Resettable for HCCHAR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
