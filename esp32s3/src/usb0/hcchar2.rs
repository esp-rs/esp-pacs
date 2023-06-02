#[doc = "Register `HCCHAR2` reader"]
pub struct R(crate::R<HCCHAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCHAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCCHAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCCHAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCHAR2` writer"]
pub struct W(crate::W<HCCHAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCHAR2_SPEC>;
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
impl From<crate::W<HCCHAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCCHAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_MPS2` reader - "]
pub type H_MPS2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `H_MPS2` writer - "]
pub type H_MPS2_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR2_SPEC, 11, O, u16, u16>;
#[doc = "Field `H_EPNUM2` reader - "]
pub type H_EPNUM2_R = crate::FieldReader;
#[doc = "Field `H_EPNUM2` writer - "]
pub type H_EPNUM2_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR2_SPEC, 4, O>;
#[doc = "Field `H_EPDIR2` reader - "]
pub type H_EPDIR2_R = crate::BitReader;
#[doc = "Field `H_EPDIR2` writer - "]
pub type H_EPDIR2_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR2_SPEC, O>;
#[doc = "Field `H_LSPDDEV2` reader - "]
pub type H_LSPDDEV2_R = crate::BitReader;
#[doc = "Field `H_LSPDDEV2` writer - "]
pub type H_LSPDDEV2_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR2_SPEC, O>;
#[doc = "Field `H_EPTYPE2` reader - "]
pub type H_EPTYPE2_R = crate::FieldReader;
#[doc = "Field `H_EPTYPE2` writer - "]
pub type H_EPTYPE2_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR2_SPEC, 2, O>;
#[doc = "Field `H_EC2` reader - "]
pub type H_EC2_R = crate::BitReader;
#[doc = "Field `H_EC2` writer - "]
pub type H_EC2_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR2_SPEC, O>;
#[doc = "Field `H_DEVADDR2` reader - "]
pub type H_DEVADDR2_R = crate::FieldReader;
#[doc = "Field `H_DEVADDR2` writer - "]
pub type H_DEVADDR2_W<'a, const O: u8> = crate::FieldWriter<'a, HCCHAR2_SPEC, 7, O>;
#[doc = "Field `H_ODDFRM2` reader - "]
pub type H_ODDFRM2_R = crate::BitReader;
#[doc = "Field `H_ODDFRM2` writer - "]
pub type H_ODDFRM2_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR2_SPEC, O>;
#[doc = "Field `H_CHDIS2` reader - "]
pub type H_CHDIS2_R = crate::BitReader;
#[doc = "Field `H_CHDIS2` writer - "]
pub type H_CHDIS2_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR2_SPEC, O>;
#[doc = "Field `H_CHENA2` reader - "]
pub type H_CHENA2_R = crate::BitReader;
#[doc = "Field `H_CHENA2` writer - "]
pub type H_CHENA2_W<'a, const O: u8> = crate::BitWriter<'a, HCCHAR2_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn h_mps2(&self) -> H_MPS2_R {
        H_MPS2_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    pub fn h_epnum2(&self) -> H_EPNUM2_R {
        H_EPNUM2_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn h_epdir2(&self) -> H_EPDIR2_R {
        H_EPDIR2_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn h_lspddev2(&self) -> H_LSPDDEV2_R {
        H_LSPDDEV2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn h_eptype2(&self) -> H_EPTYPE2_R {
        H_EPTYPE2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn h_ec2(&self) -> H_EC2_R {
        H_EC2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    pub fn h_devaddr2(&self) -> H_DEVADDR2_R {
        H_DEVADDR2_R::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn h_oddfrm2(&self) -> H_ODDFRM2_R {
        H_ODDFRM2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn h_chdis2(&self) -> H_CHDIS2_R {
        H_CHDIS2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn h_chena2(&self) -> H_CHENA2_R {
        H_CHENA2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCCHAR2")
            .field("h_mps2", &format_args!("{}", self.h_mps2().bits()))
            .field("h_epnum2", &format_args!("{}", self.h_epnum2().bits()))
            .field("h_epdir2", &format_args!("{}", self.h_epdir2().bit()))
            .field("h_lspddev2", &format_args!("{}", self.h_lspddev2().bit()))
            .field("h_eptype2", &format_args!("{}", self.h_eptype2().bits()))
            .field("h_ec2", &format_args!("{}", self.h_ec2().bit()))
            .field("h_devaddr2", &format_args!("{}", self.h_devaddr2().bits()))
            .field("h_oddfrm2", &format_args!("{}", self.h_oddfrm2().bit()))
            .field("h_chdis2", &format_args!("{}", self.h_chdis2().bit()))
            .field("h_chena2", &format_args!("{}", self.h_chena2().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCCHAR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn h_mps2(&mut self) -> H_MPS2_W<0> {
        H_MPS2_W::new(self)
    }
    #[doc = "Bits 11:14"]
    #[inline(always)]
    #[must_use]
    pub fn h_epnum2(&mut self) -> H_EPNUM2_W<11> {
        H_EPNUM2_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn h_epdir2(&mut self) -> H_EPDIR2_W<15> {
        H_EPDIR2_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn h_lspddev2(&mut self) -> H_LSPDDEV2_W<17> {
        H_LSPDDEV2_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn h_eptype2(&mut self) -> H_EPTYPE2_W<18> {
        H_EPTYPE2_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn h_ec2(&mut self) -> H_EC2_W<21> {
        H_EC2_W::new(self)
    }
    #[doc = "Bits 22:28"]
    #[inline(always)]
    #[must_use]
    pub fn h_devaddr2(&mut self) -> H_DEVADDR2_W<22> {
        H_DEVADDR2_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn h_oddfrm2(&mut self) -> H_ODDFRM2_W<29> {
        H_ODDFRM2_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn h_chdis2(&mut self) -> H_CHDIS2_W<30> {
        H_CHDIS2_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn h_chena2(&mut self) -> H_CHENA2_W<31> {
        H_CHENA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcchar2](index.html) module"]
pub struct HCCHAR2_SPEC;
impl crate::RegisterSpec for HCCHAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcchar2::R](R) reader structure"]
impl crate::Readable for HCCHAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcchar2::W](W) writer structure"]
impl crate::Writable for HCCHAR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCCHAR2 to value 0"]
impl crate::Resettable for HCCHAR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
