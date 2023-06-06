#[doc = "Register `DOEPCTL1` reader"]
pub struct R(crate::R<DOEPCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL1` writer"]
pub struct W(crate::W<DOEPCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL1_SPEC>;
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
impl From<crate::W<DOEPCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS1` reader - "]
pub type MPS1_R = crate::FieldReader<u16>;
#[doc = "Field `USBACTEP1` reader - "]
pub type USBACTEP1_R = crate::BitReader;
#[doc = "Field `NAKSTS1` reader - "]
pub type NAKSTS1_R = crate::BitReader;
#[doc = "Field `EPTYPE1` reader - "]
pub type EPTYPE1_R = crate::FieldReader;
#[doc = "Field `SNP1` reader - "]
pub type SNP1_R = crate::BitReader;
#[doc = "Field `SNP1` writer - "]
pub type SNP1_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL1_SPEC, O>;
#[doc = "Field `STALL1` reader - "]
pub type STALL1_R = crate::BitReader;
#[doc = "Field `STALL1` writer - "]
pub type STALL1_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL1_SPEC, O>;
#[doc = "Field `CNAK1` writer - "]
pub type CNAK1_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL1_SPEC, O>;
#[doc = "Field `DO_SNAK1` writer - "]
pub type DO_SNAK1_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL1_SPEC, O>;
#[doc = "Field `DO_SETD0PID1` writer - "]
pub type DO_SETD0PID1_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL1_SPEC, O>;
#[doc = "Field `DO_SETD1PID1` writer - "]
pub type DO_SETD1PID1_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL1_SPEC, O>;
#[doc = "Field `EPDIS1` reader - "]
pub type EPDIS1_R = crate::BitReader;
#[doc = "Field `EPENA1` reader - "]
pub type EPENA1_R = crate::BitReader;
#[doc = "Field `EPENA1` writer - "]
pub type EPENA1_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL1_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps1(&self) -> MPS1_R {
        MPS1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep1(&self) -> USBACTEP1_R {
        USBACTEP1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts1(&self) -> NAKSTS1_R {
        NAKSTS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype1(&self) -> EPTYPE1_R {
        EPTYPE1_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp1(&self) -> SNP1_R {
        SNP1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall1(&self) -> STALL1_R {
        STALL1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis1(&self) -> EPDIS1_R {
        EPDIS1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena1(&self) -> EPENA1_R {
        EPENA1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL1")
            .field("mps1", &format_args!("{}", self.mps1().bits()))
            .field("usbactep1", &format_args!("{}", self.usbactep1().bit()))
            .field("naksts1", &format_args!("{}", self.naksts1().bit()))
            .field("eptype1", &format_args!("{}", self.eptype1().bits()))
            .field("snp1", &format_args!("{}", self.snp1().bit()))
            .field("stall1", &format_args!("{}", self.stall1().bit()))
            .field("epdis1", &format_args!("{}", self.epdis1().bit()))
            .field("epena1", &format_args!("{}", self.epena1().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp1(&mut self) -> SNP1_W<20> {
        SNP1_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall1(&mut self) -> STALL1_W<21> {
        STALL1_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak1(&mut self) -> CNAK1_W<26> {
        CNAK1_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak1(&mut self) -> DO_SNAK1_W<27> {
        DO_SNAK1_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd0pid1(&mut self) -> DO_SETD0PID1_W<28> {
        DO_SETD0PID1_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd1pid1(&mut self) -> DO_SETD1PID1_W<29> {
        DO_SETD1PID1_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena1(&mut self) -> EPENA1_W<31> {
        EPENA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl1](index.html) module"]
pub struct DOEPCTL1_SPEC;
impl crate::RegisterSpec for DOEPCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl1::R](R) reader structure"]
impl crate::Readable for DOEPCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl1::W](W) writer structure"]
impl crate::Writable for DOEPCTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL1 to value 0x8000"]
impl crate::Resettable for DOEPCTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
