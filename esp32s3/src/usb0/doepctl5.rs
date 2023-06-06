#[doc = "Register `DOEPCTL5` reader"]
pub struct R(crate::R<DOEPCTL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL5` writer"]
pub struct W(crate::W<DOEPCTL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL5_SPEC>;
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
impl From<crate::W<DOEPCTL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS5` reader - "]
pub type MPS5_R = crate::FieldReader<u16>;
#[doc = "Field `USBACTEP5` reader - "]
pub type USBACTEP5_R = crate::BitReader;
#[doc = "Field `NAKSTS5` reader - "]
pub type NAKSTS5_R = crate::BitReader;
#[doc = "Field `EPTYPE5` reader - "]
pub type EPTYPE5_R = crate::FieldReader;
#[doc = "Field `SNP5` reader - "]
pub type SNP5_R = crate::BitReader;
#[doc = "Field `SNP5` writer - "]
pub type SNP5_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL5_SPEC, O>;
#[doc = "Field `STALL5` reader - "]
pub type STALL5_R = crate::BitReader;
#[doc = "Field `STALL5` writer - "]
pub type STALL5_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL5_SPEC, O>;
#[doc = "Field `CNAK5` writer - "]
pub type CNAK5_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL5_SPEC, O>;
#[doc = "Field `DO_SNAK5` writer - "]
pub type DO_SNAK5_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL5_SPEC, O>;
#[doc = "Field `DO_SETD0PID5` writer - "]
pub type DO_SETD0PID5_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL5_SPEC, O>;
#[doc = "Field `DO_SETD1PID5` writer - "]
pub type DO_SETD1PID5_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL5_SPEC, O>;
#[doc = "Field `EPDIS5` reader - "]
pub type EPDIS5_R = crate::BitReader;
#[doc = "Field `EPENA5` reader - "]
pub type EPENA5_R = crate::BitReader;
#[doc = "Field `EPENA5` writer - "]
pub type EPENA5_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL5_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps5(&self) -> MPS5_R {
        MPS5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep5(&self) -> USBACTEP5_R {
        USBACTEP5_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts5(&self) -> NAKSTS5_R {
        NAKSTS5_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype5(&self) -> EPTYPE5_R {
        EPTYPE5_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp5(&self) -> SNP5_R {
        SNP5_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall5(&self) -> STALL5_R {
        STALL5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis5(&self) -> EPDIS5_R {
        EPDIS5_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena5(&self) -> EPENA5_R {
        EPENA5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL5")
            .field("mps5", &format_args!("{}", self.mps5().bits()))
            .field("usbactep5", &format_args!("{}", self.usbactep5().bit()))
            .field("naksts5", &format_args!("{}", self.naksts5().bit()))
            .field("eptype5", &format_args!("{}", self.eptype5().bits()))
            .field("snp5", &format_args!("{}", self.snp5().bit()))
            .field("stall5", &format_args!("{}", self.stall5().bit()))
            .field("epdis5", &format_args!("{}", self.epdis5().bit()))
            .field("epena5", &format_args!("{}", self.epena5().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp5(&mut self) -> SNP5_W<20> {
        SNP5_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall5(&mut self) -> STALL5_W<21> {
        STALL5_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak5(&mut self) -> CNAK5_W<26> {
        CNAK5_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak5(&mut self) -> DO_SNAK5_W<27> {
        DO_SNAK5_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd0pid5(&mut self) -> DO_SETD0PID5_W<28> {
        DO_SETD0PID5_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd1pid5(&mut self) -> DO_SETD1PID5_W<29> {
        DO_SETD1PID5_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena5(&mut self) -> EPENA5_W<31> {
        EPENA5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl5](index.html) module"]
pub struct DOEPCTL5_SPEC;
impl crate::RegisterSpec for DOEPCTL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl5::R](R) reader structure"]
impl crate::Readable for DOEPCTL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl5::W](W) writer structure"]
impl crate::Writable for DOEPCTL5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL5 to value 0x8000"]
impl crate::Resettable for DOEPCTL5_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
