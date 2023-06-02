#[doc = "Register `DOEPCTL4` reader"]
pub struct R(crate::R<DOEPCTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL4` writer"]
pub struct W(crate::W<DOEPCTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL4_SPEC>;
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
impl From<crate::W<DOEPCTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS4` reader - "]
pub type MPS4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USBACTEP4` reader - "]
pub type USBACTEP4_R = crate::BitReader;
#[doc = "Field `NAKSTS4` reader - "]
pub type NAKSTS4_R = crate::BitReader;
#[doc = "Field `EPTYPE4` reader - "]
pub type EPTYPE4_R = crate::FieldReader;
#[doc = "Field `SNP4` reader - "]
pub type SNP4_R = crate::BitReader;
#[doc = "Field `SNP4` writer - "]
pub type SNP4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL4_SPEC, O>;
#[doc = "Field `STALL4` reader - "]
pub type STALL4_R = crate::BitReader;
#[doc = "Field `STALL4` writer - "]
pub type STALL4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL4_SPEC, O>;
#[doc = "Field `CNAK4` writer - "]
pub type CNAK4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL4_SPEC, O>;
#[doc = "Field `DO_SNAK4` writer - "]
pub type DO_SNAK4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL4_SPEC, O>;
#[doc = "Field `DO_SETD0PID4` writer - "]
pub type DO_SETD0PID4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL4_SPEC, O>;
#[doc = "Field `DO_SETD1PID4` writer - "]
pub type DO_SETD1PID4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL4_SPEC, O>;
#[doc = "Field `EPDIS4` reader - "]
pub type EPDIS4_R = crate::BitReader;
#[doc = "Field `EPENA4` reader - "]
pub type EPENA4_R = crate::BitReader;
#[doc = "Field `EPENA4` writer - "]
pub type EPENA4_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL4_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps4(&self) -> MPS4_R {
        MPS4_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep4(&self) -> USBACTEP4_R {
        USBACTEP4_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts4(&self) -> NAKSTS4_R {
        NAKSTS4_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype4(&self) -> EPTYPE4_R {
        EPTYPE4_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp4(&self) -> SNP4_R {
        SNP4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall4(&self) -> STALL4_R {
        STALL4_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis4(&self) -> EPDIS4_R {
        EPDIS4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena4(&self) -> EPENA4_R {
        EPENA4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL4")
            .field("mps4", &format_args!("{}", self.mps4().bits()))
            .field("usbactep4", &format_args!("{}", self.usbactep4().bit()))
            .field("naksts4", &format_args!("{}", self.naksts4().bit()))
            .field("eptype4", &format_args!("{}", self.eptype4().bits()))
            .field("snp4", &format_args!("{}", self.snp4().bit()))
            .field("stall4", &format_args!("{}", self.stall4().bit()))
            .field("epdis4", &format_args!("{}", self.epdis4().bit()))
            .field("epena4", &format_args!("{}", self.epena4().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp4(&mut self) -> SNP4_W<20> {
        SNP4_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall4(&mut self) -> STALL4_W<21> {
        STALL4_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak4(&mut self) -> CNAK4_W<26> {
        CNAK4_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak4(&mut self) -> DO_SNAK4_W<27> {
        DO_SNAK4_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd0pid4(&mut self) -> DO_SETD0PID4_W<28> {
        DO_SETD0PID4_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd1pid4(&mut self) -> DO_SETD1PID4_W<29> {
        DO_SETD1PID4_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena4(&mut self) -> EPENA4_W<31> {
        EPENA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl4](index.html) module"]
pub struct DOEPCTL4_SPEC;
impl crate::RegisterSpec for DOEPCTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl4::R](R) reader structure"]
impl crate::Readable for DOEPCTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl4::W](W) writer structure"]
impl crate::Writable for DOEPCTL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL4 to value 0x8000"]
impl crate::Resettable for DOEPCTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
