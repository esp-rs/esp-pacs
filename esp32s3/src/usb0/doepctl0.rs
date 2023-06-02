#[doc = "Register `DOEPCTL0` reader"]
pub struct R(crate::R<DOEPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPCTL0` writer"]
pub struct W(crate::W<DOEPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPCTL0_SPEC>;
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
impl From<crate::W<DOEPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPS0` reader - "]
pub type MPS0_R = crate::FieldReader;
#[doc = "Field `USBACTEP0` reader - "]
pub type USBACTEP0_R = crate::BitReader;
#[doc = "Field `NAKSTS0` reader - "]
pub type NAKSTS0_R = crate::BitReader;
#[doc = "Field `EPTYPE0` reader - "]
pub type EPTYPE0_R = crate::FieldReader;
#[doc = "Field `SNP0` reader - "]
pub type SNP0_R = crate::BitReader;
#[doc = "Field `SNP0` writer - "]
pub type SNP0_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL0_SPEC, O>;
#[doc = "Field `STALL0` reader - "]
pub type STALL0_R = crate::BitReader;
#[doc = "Field `STALL0` writer - "]
pub type STALL0_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL0_SPEC, O>;
#[doc = "Field `CNAK0` writer - "]
pub type CNAK0_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL0_SPEC, O>;
#[doc = "Field `DO_SNAK0` writer - "]
pub type DO_SNAK0_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL0_SPEC, O>;
#[doc = "Field `EPDIS0` reader - "]
pub type EPDIS0_R = crate::BitReader;
#[doc = "Field `EPENA0` reader - "]
pub type EPENA0_R = crate::BitReader;
#[doc = "Field `EPENA0` writer - "]
pub type EPENA0_W<'a, const O: u8> = crate::BitWriter<'a, DOEPCTL0_SPEC, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mps0(&self) -> MPS0_R {
        MPS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn usbactep0(&self) -> USBACTEP0_R {
        USBACTEP0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn naksts0(&self) -> NAKSTS0_R {
        NAKSTS0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn eptype0(&self) -> EPTYPE0_R {
        EPTYPE0_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn snp0(&self) -> SNP0_R {
        SNP0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn stall0(&self) -> STALL0_R {
        STALL0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn epdis0(&self) -> EPDIS0_R {
        EPDIS0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn epena0(&self) -> EPENA0_R {
        EPENA0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL0")
            .field("mps0", &format_args!("{}", self.mps0().bits()))
            .field("usbactep0", &format_args!("{}", self.usbactep0().bit()))
            .field("naksts0", &format_args!("{}", self.naksts0().bit()))
            .field("eptype0", &format_args!("{}", self.eptype0().bits()))
            .field("snp0", &format_args!("{}", self.snp0().bit()))
            .field("stall0", &format_args!("{}", self.stall0().bit()))
            .field("epdis0", &format_args!("{}", self.epdis0().bit()))
            .field("epena0", &format_args!("{}", self.epena0().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPCTL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn snp0(&mut self) -> SNP0_W<20> {
        SNP0_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn stall0(&mut self) -> STALL0_W<21> {
        STALL0_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn cnak0(&mut self) -> CNAK0_W<26> {
        CNAK0_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn do_snak0(&mut self) -> DO_SNAK0_W<27> {
        DO_SNAK0_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn epena0(&mut self) -> EPENA0_W<31> {
        EPENA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepctl0](index.html) module"]
pub struct DOEPCTL0_SPEC;
impl crate::RegisterSpec for DOEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepctl0::R](R) reader structure"]
impl crate::Readable for DOEPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepctl0::W](W) writer structure"]
impl crate::Writable for DOEPCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPCTL0 to value 0x8000"]
impl crate::Resettable for DOEPCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
