#[doc = "Register `DIEPCTL6` reader"]
pub struct R(crate::R<DIEPCTL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPCTL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPCTL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPCTL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPCTL6` writer"]
pub struct W(crate::W<DIEPCTL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPCTL6_SPEC>;
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
impl From<crate::W<DIEPCTL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPCTL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_MPS6` reader - "]
pub type D_MPS6_R = crate::FieldReader;
#[doc = "Field `D_MPS6` writer - "]
pub type D_MPS6_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL6_SPEC, 2, O>;
#[doc = "Field `D_USBACTEP6` reader - "]
pub type D_USBACTEP6_R = crate::BitReader;
#[doc = "Field `D_NAKSTS6` reader - "]
pub type D_NAKSTS6_R = crate::BitReader;
#[doc = "Field `D_EPTYPE6` reader - "]
pub type D_EPTYPE6_R = crate::FieldReader;
#[doc = "Field `D_STALL6` reader - "]
pub type D_STALL6_R = crate::BitReader;
#[doc = "Field `D_STALL6` writer - "]
pub type D_STALL6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL6_SPEC, O>;
#[doc = "Field `D_TXFNUM6` reader - "]
pub type D_TXFNUM6_R = crate::FieldReader;
#[doc = "Field `D_TXFNUM6` writer - "]
pub type D_TXFNUM6_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL6_SPEC, 4, O>;
#[doc = "Field `D_CNAK6` writer - "]
pub type D_CNAK6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL6_SPEC, O>;
#[doc = "Field `DI_SNAK6` writer - "]
pub type DI_SNAK6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL6_SPEC, O>;
#[doc = "Field `DI_SETD0PID6` writer - "]
pub type DI_SETD0PID6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL6_SPEC, O>;
#[doc = "Field `DI_SETD1PID6` writer - "]
pub type DI_SETD1PID6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL6_SPEC, O>;
#[doc = "Field `D_EPDIS6` reader - "]
pub type D_EPDIS6_R = crate::BitReader;
#[doc = "Field `D_EPDIS6` writer - "]
pub type D_EPDIS6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL6_SPEC, O>;
#[doc = "Field `D_EPENA6` reader - "]
pub type D_EPENA6_R = crate::BitReader;
#[doc = "Field `D_EPENA6` writer - "]
pub type D_EPENA6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL6_SPEC, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn d_mps6(&self) -> D_MPS6_R {
        D_MPS6_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn d_usbactep6(&self) -> D_USBACTEP6_R {
        D_USBACTEP6_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn d_naksts6(&self) -> D_NAKSTS6_R {
        D_NAKSTS6_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn d_eptype6(&self) -> D_EPTYPE6_R {
        D_EPTYPE6_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn d_stall6(&self) -> D_STALL6_R {
        D_STALL6_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn d_txfnum6(&self) -> D_TXFNUM6_R {
        D_TXFNUM6_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn d_epdis6(&self) -> D_EPDIS6_R {
        D_EPDIS6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn d_epena6(&self) -> D_EPENA6_R {
        D_EPENA6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL6")
            .field("d_mps6", &format_args!("{}", self.d_mps6().bits()))
            .field("d_usbactep6", &format_args!("{}", self.d_usbactep6().bit()))
            .field("d_naksts6", &format_args!("{}", self.d_naksts6().bit()))
            .field("d_eptype6", &format_args!("{}", self.d_eptype6().bits()))
            .field("d_stall6", &format_args!("{}", self.d_stall6().bit()))
            .field("d_txfnum6", &format_args!("{}", self.d_txfnum6().bits()))
            .field("d_epdis6", &format_args!("{}", self.d_epdis6().bit()))
            .field("d_epena6", &format_args!("{}", self.d_epena6().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn d_mps6(&mut self) -> D_MPS6_W<0> {
        D_MPS6_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn d_stall6(&mut self) -> D_STALL6_W<21> {
        D_STALL6_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfnum6(&mut self) -> D_TXFNUM6_W<22> {
        D_TXFNUM6_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn d_cnak6(&mut self) -> D_CNAK6_W<26> {
        D_CNAK6_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak6(&mut self) -> DI_SNAK6_W<27> {
        DI_SNAK6_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd0pid6(&mut self) -> DI_SETD0PID6_W<28> {
        DI_SETD0PID6_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd1pid6(&mut self) -> DI_SETD1PID6_W<29> {
        DI_SETD1PID6_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdis6(&mut self) -> D_EPDIS6_W<30> {
        D_EPDIS6_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn d_epena6(&mut self) -> D_EPENA6_W<31> {
        D_EPENA6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl6](index.html) module"]
pub struct DIEPCTL6_SPEC;
impl crate::RegisterSpec for DIEPCTL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepctl6::R](R) reader structure"]
impl crate::Readable for DIEPCTL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepctl6::W](W) writer structure"]
impl crate::Writable for DIEPCTL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL6 to value 0x8000"]
impl crate::Resettable for DIEPCTL6_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
