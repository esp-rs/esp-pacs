#[doc = "Register `DIEPCTL0` reader"]
pub struct R(crate::R<DIEPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPCTL0` writer"]
pub struct W(crate::W<DIEPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPCTL0_SPEC>;
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
impl From<crate::W<DIEPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_MPS0` reader - "]
pub type D_MPS0_R = crate::FieldReader;
#[doc = "Field `D_MPS0` writer - "]
pub type D_MPS0_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL0_SPEC, 2, O>;
#[doc = "Field `D_USBACTEP0` reader - "]
pub type D_USBACTEP0_R = crate::BitReader;
#[doc = "Field `D_NAKSTS0` reader - "]
pub type D_NAKSTS0_R = crate::BitReader;
#[doc = "Field `D_EPTYPE0` reader - "]
pub type D_EPTYPE0_R = crate::FieldReader;
#[doc = "Field `D_STALL0` reader - "]
pub type D_STALL0_R = crate::BitReader;
#[doc = "Field `D_STALL0` writer - "]
pub type D_STALL0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL0_SPEC, O>;
#[doc = "Field `D_TXFNUM0` reader - "]
pub type D_TXFNUM0_R = crate::FieldReader;
#[doc = "Field `D_TXFNUM0` writer - "]
pub type D_TXFNUM0_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL0_SPEC, 4, O>;
#[doc = "Field `D_CNAK0` writer - "]
pub type D_CNAK0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL0_SPEC, O>;
#[doc = "Field `DI_SNAK0` writer - "]
pub type DI_SNAK0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL0_SPEC, O>;
#[doc = "Field `D_EPDIS0` reader - "]
pub type D_EPDIS0_R = crate::BitReader;
#[doc = "Field `D_EPDIS0` writer - "]
pub type D_EPDIS0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL0_SPEC, O>;
#[doc = "Field `D_EPENA0` reader - "]
pub type D_EPENA0_R = crate::BitReader;
#[doc = "Field `D_EPENA0` writer - "]
pub type D_EPENA0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL0_SPEC, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn d_mps0(&self) -> D_MPS0_R {
        D_MPS0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn d_usbactep0(&self) -> D_USBACTEP0_R {
        D_USBACTEP0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn d_naksts0(&self) -> D_NAKSTS0_R {
        D_NAKSTS0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn d_eptype0(&self) -> D_EPTYPE0_R {
        D_EPTYPE0_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn d_stall0(&self) -> D_STALL0_R {
        D_STALL0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn d_txfnum0(&self) -> D_TXFNUM0_R {
        D_TXFNUM0_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn d_epdis0(&self) -> D_EPDIS0_R {
        D_EPDIS0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn d_epena0(&self) -> D_EPENA0_R {
        D_EPENA0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL0")
            .field("d_mps0", &format_args!("{}", self.d_mps0().bits()))
            .field("d_usbactep0", &format_args!("{}", self.d_usbactep0().bit()))
            .field("d_naksts0", &format_args!("{}", self.d_naksts0().bit()))
            .field("d_eptype0", &format_args!("{}", self.d_eptype0().bits()))
            .field("d_stall0", &format_args!("{}", self.d_stall0().bit()))
            .field("d_txfnum0", &format_args!("{}", self.d_txfnum0().bits()))
            .field("d_epdis0", &format_args!("{}", self.d_epdis0().bit()))
            .field("d_epena0", &format_args!("{}", self.d_epena0().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn d_mps0(&mut self) -> D_MPS0_W<0> {
        D_MPS0_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn d_stall0(&mut self) -> D_STALL0_W<21> {
        D_STALL0_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfnum0(&mut self) -> D_TXFNUM0_W<22> {
        D_TXFNUM0_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn d_cnak0(&mut self) -> D_CNAK0_W<26> {
        D_CNAK0_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak0(&mut self) -> DI_SNAK0_W<27> {
        DI_SNAK0_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdis0(&mut self) -> D_EPDIS0_W<30> {
        D_EPDIS0_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn d_epena0(&mut self) -> D_EPENA0_W<31> {
        D_EPENA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl0](index.html) module"]
pub struct DIEPCTL0_SPEC;
impl crate::RegisterSpec for DIEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepctl0::R](R) reader structure"]
impl crate::Readable for DIEPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepctl0::W](W) writer structure"]
impl crate::Writable for DIEPCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL0 to value 0x8000"]
impl crate::Resettable for DIEPCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
