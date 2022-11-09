#[doc = "Register `HCCHAR4` reader"]
pub struct R(crate::R<HCCHAR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR4` writer"]
pub struct W(crate::W<HCCHAR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR4_SPEC>;
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
impl From<crate::W<HCCHAR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_MPS4` reader - "]
pub type H_MPS4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `H_MPS4` writer - "]
pub type H_MPS4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR4_SPEC, u16, u16, 11, O>;
#[doc = "Field `H_EPNUM4` reader - "]
pub type H_EPNUM4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H_EPNUM4` writer - "]
pub type H_EPNUM4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR4_SPEC, u8, u8, 4, O>;
#[doc = "Field `H_EPDIR4` reader - "]
pub type H_EPDIR4_R = crate::BitReader<bool>;
#[doc = "Field `H_EPDIR4` writer - "]
pub type H_EPDIR4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR4_SPEC, bool, O>;
#[doc = "Field `H_LSPDDEV4` reader - "]
pub type H_LSPDDEV4_R = crate::BitReader<bool>;
#[doc = "Field `H_LSPDDEV4` writer - "]
pub type H_LSPDDEV4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR4_SPEC, bool, O>;
#[doc = "Field `H_EPTYPE4` reader - "]
pub type H_EPTYPE4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H_EPTYPE4` writer - "]
pub type H_EPTYPE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `H_EC4` reader - "]
pub type H_EC4_R = crate::BitReader<bool>;
#[doc = "Field `H_EC4` writer - "]
pub type H_EC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR4_SPEC, bool, O>;
#[doc = "Field `H_DEVADDR4` reader - "]
pub type H_DEVADDR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `H_DEVADDR4` writer - "]
pub type H_DEVADDR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCCHAR4_SPEC, u8, u8, 7, O>;
#[doc = "Field `H_ODDFRM4` reader - "]
pub type H_ODDFRM4_R = crate::BitReader<bool>;
#[doc = "Field `H_ODDFRM4` writer - "]
pub type H_ODDFRM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR4_SPEC, bool, O>;
#[doc = "Field `H_CHDIS4` reader - "]
pub type H_CHDIS4_R = crate::BitReader<bool>;
#[doc = "Field `H_CHDIS4` writer - "]
pub type H_CHDIS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR4_SPEC, bool, O>;
#[doc = "Field `H_CHENA4` reader - "]
pub type H_CHENA4_R = crate::BitReader<bool>;
#[doc = "Field `H_CHENA4` writer - "]
pub type H_CHENA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCCHAR4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps4(&self) -> H_MPS4_R {
        H_MPS4_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum4(&self) -> H_EPNUM4_R {
        H_EPNUM4_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir4(&self) -> H_EPDIR4_R {
        H_EPDIR4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev4(&self) -> H_LSPDDEV4_R {
        H_LSPDDEV4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype4(&self) -> H_EPTYPE4_R {
        H_EPTYPE4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec4(&self) -> H_EC4_R {
        H_EC4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr4(&self) -> H_DEVADDR4_R {
        H_DEVADDR4_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm4(&self) -> H_ODDFRM4_R {
        H_ODDFRM4_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis4(&self) -> H_CHDIS4_R {
        H_CHDIS4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena4(&self) -> H_CHENA4_R {
        H_CHENA4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps4(&mut self) -> H_MPS4_W<0> {
        H_MPS4_W::new(self)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum4(&mut self) -> H_EPNUM4_W<11> {
        H_EPNUM4_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir4(&mut self) -> H_EPDIR4_W<15> {
        H_EPDIR4_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev4(&mut self) -> H_LSPDDEV4_W<17> {
        H_LSPDDEV4_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype4(&mut self) -> H_EPTYPE4_W<18> {
        H_EPTYPE4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec4(&mut self) -> H_EC4_W<21> {
        H_EC4_W::new(self)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr4(&mut self) -> H_DEVADDR4_W<22> {
        H_DEVADDR4_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm4(&mut self) -> H_ODDFRM4_W<29> {
        H_ODDFRM4_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis4(&mut self) -> H_CHDIS4_W<30> {
        H_CHDIS4_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena4(&mut self) -> H_CHENA4_W<31> {
        H_CHENA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar4](index.html) module"]
pub struct HCCHAR4_SPEC;
impl crate::RegisterSpec for HCCHAR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar4::R](R) reader structure"]
impl crate::Readable for HCCHAR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar4::W](W) writer structure"]
impl crate::Writable for HCCHAR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR4 to value 0"]
impl crate::Resettable for HCCHAR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
