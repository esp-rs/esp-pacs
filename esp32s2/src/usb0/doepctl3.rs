#[doc = "Register `DOEPCTL3` reader"]
pub struct R(crate::R<DOEPCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL3` writer"]
pub struct W(crate::W<DOEPCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL3_SPEC>;
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
impl From<crate::W<DOEPCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS3` reader - "]
pub type MPS3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `USBACTEP3` reader - "]
pub type USBACTEP3_R = crate::BitReader;
#[doc = "Field `NAKSTS3` reader - "]
pub type NAKSTS3_R = crate::BitReader;
#[doc = "Field `EPTYPE3` reader - "]
pub type EPTYPE3_R = crate::FieldReader;
#[doc = "Field `SNP3` reader - "]
pub type SNP3_R = crate::BitReader;
#[doc = "Field `SNP3` writer - "]
pub type SNP3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL3_SPEC, O>;
#[doc = "Field `STALL3` reader - "]
pub type STALL3_R = crate::BitReader;
#[doc = "Field `STALL3` writer - "]
pub type STALL3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL3_SPEC, O>;
#[doc = "Field `CNAK3` writer - "]
pub type CNAK3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL3_SPEC, O>;
#[doc = "Field `DO_SNAK3` writer - "]
pub type DO_SNAK3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL3_SPEC, O>;
#[doc = "Field `DO_SETD0PID3` writer - "]
pub type DO_SETD0PID3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL3_SPEC, O>;
#[doc = "Field `DO_SETD1PID3` writer - "]
pub type DO_SETD1PID3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL3_SPEC, O>;
#[doc = "Field `EPDIS3` reader - "]
pub type EPDIS3_R = crate::BitReader;
#[doc = "Field `EPENA3` reader - "]
pub type EPENA3_R = crate::BitReader;
#[doc = "Field `EPENA3` writer - "]
pub type EPENA3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL3_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps3(&self) -> MPS3_R {
        MPS3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep3(&self) -> USBACTEP3_R {
        USBACTEP3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts3(&self) -> NAKSTS3_R {
        NAKSTS3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype3(&self) -> EPTYPE3_R {
        EPTYPE3_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp3(&self) -> SNP3_R {
        SNP3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall3(&self) -> STALL3_R {
        STALL3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis3(&self) -> EPDIS3_R {
        EPDIS3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena3(&self) -> EPENA3_R {
        EPENA3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL3")
            .field("mps3", &format_args!("{}", self.mps3().bits()))
            .field("usbactep3", &format_args!("{}", self.usbactep3().bit()))
            .field("naksts3", &format_args!("{}", self.naksts3().bit()))
            .field("eptype3", &format_args!("{}", self.eptype3().bits()))
            .field("snp3", &format_args!("{}", self.snp3().bit()))
            .field("stall3", &format_args!("{}", self.stall3().bit()))
            .field("epdis3", &format_args!("{}", self.epdis3().bit()))
            .field("epena3", &format_args!("{}", self.epena3().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp3(&mut self) -> SNP3_W<20> {
        SNP3_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall3(&mut self) -> STALL3_W<21> {
        STALL3_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak3(&mut self) -> CNAK3_W<26> {
        CNAK3_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak3(&mut self) -> DO_SNAK3_W<27> {
        DO_SNAK3_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd0pid3(&mut self) -> DO_SETD0PID3_W<28> {
        DO_SETD0PID3_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd1pid3(&mut self) -> DO_SETD1PID3_W<29> {
        DO_SETD1PID3_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena3(&mut self) -> EPENA3_W<31> {
        EPENA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl3](index.html) module"]
pub struct DOEPCTL3_SPEC;
impl crate::RegisterSpec for DOEPCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl3::R](R) reader structure"]
impl crate::Readable for DOEPCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl3::W](W) writer structure"]
impl crate::Writable for DOEPCTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL3 to value 0x8000"]
impl crate::Resettable for DOEPCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
