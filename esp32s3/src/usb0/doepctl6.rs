#[doc = "Register `DOEPCTL6` reader"]
pub struct R(crate::R<DOEPCTL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL6` writer"]
pub struct W(crate::W<DOEPCTL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL6_SPEC>;
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
impl From<crate::W<DOEPCTL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS6` reader - "]
pub type MPS6_R = crate::FieldReader<u16>;
#[doc = "Field `USBACTEP6` reader - "]
pub type USBACTEP6_R = crate::BitReader;
#[doc = "Field `NAKSTS6` reader - "]
pub type NAKSTS6_R = crate::BitReader;
#[doc = "Field `EPTYPE6` reader - "]
pub type EPTYPE6_R = crate::FieldReader;
#[doc = "Field `SNP6` reader - "]
pub type SNP6_R = crate::BitReader;
#[doc = "Field `SNP6` writer - "]
pub type SNP6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL6_SPEC, O>;
#[doc = "Field `STALL6` reader - "]
pub type STALL6_R = crate::BitReader;
#[doc = "Field `STALL6` writer - "]
pub type STALL6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL6_SPEC, O>;
#[doc = "Field `CNAK6` writer - "]
pub type CNAK6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL6_SPEC, O>;
#[doc = "Field `DO_SNAK6` writer - "]
pub type DO_SNAK6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL6_SPEC, O>;
#[doc = "Field `DO_SETD0PID6` writer - "]
pub type DO_SETD0PID6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL6_SPEC, O>;
#[doc = "Field `DO_SETD1PID6` writer - "]
pub type DO_SETD1PID6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL6_SPEC, O>;
#[doc = "Field `EPDIS6` reader - "]
pub type EPDIS6_R = crate::BitReader;
#[doc = "Field `EPENA6` reader - "]
pub type EPENA6_R = crate::BitReader;
#[doc = "Field `EPENA6` writer - "]
pub type EPENA6_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL6_SPEC, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn mps6(&self) -> MPS6_R {
        MPS6_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep6(&self) -> USBACTEP6_R {
        USBACTEP6_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts6(&self) -> NAKSTS6_R {
        NAKSTS6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype6(&self) -> EPTYPE6_R {
        EPTYPE6_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp6(&self) -> SNP6_R {
        SNP6_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall6(&self) -> STALL6_R {
        STALL6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis6(&self) -> EPDIS6_R {
        EPDIS6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena6(&self) -> EPENA6_R {
        EPENA6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL6")
            .field("mps6", &format_args!("{}", self.mps6().bits()))
            .field("usbactep6", &format_args!("{}", self.usbactep6().bit()))
            .field("naksts6", &format_args!("{}", self.naksts6().bit()))
            .field("eptype6", &format_args!("{}", self.eptype6().bits()))
            .field("snp6", &format_args!("{}", self.snp6().bit()))
            .field("stall6", &format_args!("{}", self.stall6().bit()))
            .field("epdis6", &format_args!("{}", self.epdis6().bit()))
            .field("epena6", &format_args!("{}", self.epena6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp6(&mut self) -> SNP6_W<20> {
        SNP6_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall6(&mut self) -> STALL6_W<21> {
        STALL6_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak6(&mut self) -> CNAK6_W<26> {
        CNAK6_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak6(&mut self) -> DO_SNAK6_W<27> {
        DO_SNAK6_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd0pid6(&mut self) -> DO_SETD0PID6_W<28> {
        DO_SETD0PID6_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn do_setd1pid6(&mut self) -> DO_SETD1PID6_W<29> {
        DO_SETD1PID6_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena6(&mut self) -> EPENA6_W<31> {
        EPENA6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl6](index.html) module"]
pub struct DOEPCTL6_SPEC;
impl crate::RegisterSpec for DOEPCTL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl6::R](R) reader structure"]
impl crate::Readable for DOEPCTL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl6::W](W) writer structure"]
impl crate::Writable for DOEPCTL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL6 to value 0x8000"]
impl crate::Resettable for DOEPCTL6_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
