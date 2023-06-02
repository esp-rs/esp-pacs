#[doc = "Register `DIEPCTL3` reader"]
pub struct R(crate::R<DIEPCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPCTL3` writer"]
pub struct W(crate::W<DIEPCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPCTL3_SPEC>;
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
impl From<crate::W<DIEPCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DI_MPS3` reader - "]
pub type DI_MPS3_R = crate::FieldReader;
#[doc = "Field `DI_MPS3` writer - "]
pub type DI_MPS3_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL3_SPEC, 2, O>;
#[doc = "Field `DI_USBACTEP3` reader - "]
pub type DI_USBACTEP3_R = crate::BitReader;
#[doc = "Field `DI_NAKSTS3` reader - "]
pub type DI_NAKSTS3_R = crate::BitReader;
#[doc = "Field `DI_EPTYPE3` reader - "]
pub type DI_EPTYPE3_R = crate::FieldReader;
#[doc = "Field `DI_STALL3` reader - "]
pub type DI_STALL3_R = crate::BitReader;
#[doc = "Field `DI_STALL3` writer - "]
pub type DI_STALL3_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL3_SPEC, O>;
#[doc = "Field `DI_TXFNUM3` reader - "]
pub type DI_TXFNUM3_R = crate::FieldReader;
#[doc = "Field `DI_TXFNUM3` writer - "]
pub type DI_TXFNUM3_W<'a, const O: u8> = crate::FieldWriter<'a, DIEPCTL3_SPEC, 4, O>;
#[doc = "Field `DI_CNAK3` writer - "]
pub type DI_CNAK3_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL3_SPEC, O>;
#[doc = "Field `DI_SNAK3` writer - "]
pub type DI_SNAK3_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL3_SPEC, O>;
#[doc = "Field `DI_SETD0PID3` writer - "]
pub type DI_SETD0PID3_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL3_SPEC, O>;
#[doc = "Field `DI_SETD1PID3` writer - "]
pub type DI_SETD1PID3_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL3_SPEC, O>;
#[doc = "Field `DI_EPDIS3` reader - "]
pub type DI_EPDIS3_R = crate::BitReader;
#[doc = "Field `DI_EPDIS3` writer - "]
pub type DI_EPDIS3_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL3_SPEC, O>;
#[doc = "Field `DI_EPENA3` reader - "]
pub type DI_EPENA3_R = crate::BitReader;
#[doc = "Field `DI_EPENA3` writer - "]
pub type DI_EPENA3_W<'a, const O: u8> = crate::BitWriter<'a, DIEPCTL3_SPEC, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn di_mps3(&self) -> DI_MPS3_R {
        DI_MPS3_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn di_usbactep3(&self) -> DI_USBACTEP3_R {
        DI_USBACTEP3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn di_naksts3(&self) -> DI_NAKSTS3_R {
        DI_NAKSTS3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn di_eptype3(&self) -> DI_EPTYPE3_R {
        DI_EPTYPE3_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn di_stall3(&self) -> DI_STALL3_R {
        DI_STALL3_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    pub fn di_txfnum3(&self) -> DI_TXFNUM3_R {
        DI_TXFNUM3_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn di_epdis3(&self) -> DI_EPDIS3_R {
        DI_EPDIS3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn di_epena3(&self) -> DI_EPENA3_R {
        DI_EPENA3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPCTL3")
            .field("di_mps3", &format_args!("{}", self.di_mps3().bits()))
            .field(
                "di_usbactep3",
                &format_args!("{}", self.di_usbactep3().bit()),
            )
            .field("di_naksts3", &format_args!("{}", self.di_naksts3().bit()))
            .field("di_eptype3", &format_args!("{}", self.di_eptype3().bits()))
            .field("di_stall3", &format_args!("{}", self.di_stall3().bit()))
            .field("di_txfnum3", &format_args!("{}", self.di_txfnum3().bits()))
            .field("di_epdis3", &format_args!("{}", self.di_epdis3().bit()))
            .field("di_epena3", &format_args!("{}", self.di_epena3().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPCTL3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn di_mps3(&mut self) -> DI_MPS3_W<0> {
        DI_MPS3_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn di_stall3(&mut self) -> DI_STALL3_W<21> {
        DI_STALL3_W::new(self)
    }
    #[doc = "Bits 22:25"]
    #[inline(always)]
    #[must_use]
    pub fn di_txfnum3(&mut self) -> DI_TXFNUM3_W<22> {
        DI_TXFNUM3_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn di_cnak3(&mut self) -> DI_CNAK3_W<26> {
        DI_CNAK3_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn di_snak3(&mut self) -> DI_SNAK3_W<27> {
        DI_SNAK3_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd0pid3(&mut self) -> DI_SETD0PID3_W<28> {
        DI_SETD0PID3_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn di_setd1pid3(&mut self) -> DI_SETD1PID3_W<29> {
        DI_SETD1PID3_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn di_epdis3(&mut self) -> DI_EPDIS3_W<30> {
        DI_EPDIS3_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn di_epena3(&mut self) -> DI_EPENA3_W<31> {
        DI_EPENA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepctl3](index.html) module"]
pub struct DIEPCTL3_SPEC;
impl crate::RegisterSpec for DIEPCTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepctl3::R](R) reader structure"]
impl crate::Readable for DIEPCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepctl3::W](W) writer structure"]
impl crate::Writable for DIEPCTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPCTL3 to value 0x8000"]
impl crate::Resettable for DIEPCTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
