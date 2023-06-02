#[doc = "Register `DIEPMSK` reader"]
pub struct R(crate::R<DIEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPMSK` writer"]
pub struct W(crate::W<DIEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPMSK_SPEC>;
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
impl From<crate::W<DIEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DI_XFERCOMPLMSK` reader - "]
pub type DI_XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `DI_XFERCOMPLMSK` writer - "]
pub type DI_XFERCOMPLMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `DI_EPDISBLDMSK` reader - "]
pub type DI_EPDISBLDMSK_R = crate::BitReader;
#[doc = "Field `DI_EPDISBLDMSK` writer - "]
pub type DI_EPDISBLDMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `DI_AHBERMSK` reader - "]
pub type DI_AHBERMSK_R = crate::BitReader;
#[doc = "Field `DI_AHBERMSK` writer - "]
pub type DI_AHBERMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `TIMEOUTMSK` reader - "]
pub type TIMEOUTMSK_R = crate::BitReader;
#[doc = "Field `TIMEOUTMSK` writer - "]
pub type TIMEOUTMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `INTKNTXFEMPMSK` reader - "]
pub type INTKNTXFEMPMSK_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMPMSK` writer - "]
pub type INTKNTXFEMPMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `INTKNEPMISMSK` reader - "]
pub type INTKNEPMISMSK_R = crate::BitReader;
#[doc = "Field `INTKNEPMISMSK` writer - "]
pub type INTKNEPMISMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `INEPNAKEFFMSK` reader - "]
pub type INEPNAKEFFMSK_R = crate::BitReader;
#[doc = "Field `INEPNAKEFFMSK` writer - "]
pub type INEPNAKEFFMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `TXFIFOUNDRNMSK` reader - "]
pub type TXFIFOUNDRNMSK_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRNMSK` writer - "]
pub type TXFIFOUNDRNMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `BNAININTRMSK` reader - "]
pub type BNAININTRMSK_R = crate::BitReader;
#[doc = "Field `BNAININTRMSK` writer - "]
pub type BNAININTRMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
#[doc = "Field `DI_NAKMSK` reader - "]
pub type DI_NAKMSK_R = crate::BitReader;
#[doc = "Field `DI_NAKMSK` writer - "]
pub type DI_NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, DIEPMSK_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn di_xfercomplmsk(&self) -> DI_XFERCOMPLMSK_R {
        DI_XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn di_epdisbldmsk(&self) -> DI_EPDISBLDMSK_R {
        DI_EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn di_ahbermsk(&self) -> DI_AHBERMSK_R {
        DI_AHBERMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timeoutmsk(&self) -> TIMEOUTMSK_R {
        TIMEOUTMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn intkntxfempmsk(&self) -> INTKNTXFEMPMSK_R {
        INTKNTXFEMPMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn intknepmismsk(&self) -> INTKNEPMISMSK_R {
        INTKNEPMISMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inepnakeffmsk(&self) -> INEPNAKEFFMSK_R {
        INEPNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txfifoundrnmsk(&self) -> TXFIFOUNDRNMSK_R {
        TXFIFOUNDRNMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnainintrmsk(&self) -> BNAININTRMSK_R {
        BNAININTRMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn di_nakmsk(&self) -> DI_NAKMSK_R {
        DI_NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPMSK")
            .field(
                "di_xfercomplmsk",
                &format_args!("{}", self.di_xfercomplmsk().bit()),
            )
            .field(
                "di_epdisbldmsk",
                &format_args!("{}", self.di_epdisbldmsk().bit()),
            )
            .field("di_ahbermsk", &format_args!("{}", self.di_ahbermsk().bit()))
            .field("timeoutmsk", &format_args!("{}", self.timeoutmsk().bit()))
            .field(
                "intkntxfempmsk",
                &format_args!("{}", self.intkntxfempmsk().bit()),
            )
            .field(
                "intknepmismsk",
                &format_args!("{}", self.intknepmismsk().bit()),
            )
            .field(
                "inepnakeffmsk",
                &format_args!("{}", self.inepnakeffmsk().bit()),
            )
            .field(
                "txfifoundrnmsk",
                &format_args!("{}", self.txfifoundrnmsk().bit()),
            )
            .field(
                "bnainintrmsk",
                &format_args!("{}", self.bnainintrmsk().bit()),
            )
            .field("di_nakmsk", &format_args!("{}", self.di_nakmsk().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn di_xfercomplmsk(&mut self) -> DI_XFERCOMPLMSK_W<0> {
        DI_XFERCOMPLMSK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn di_epdisbldmsk(&mut self) -> DI_EPDISBLDMSK_W<1> {
        DI_EPDISBLDMSK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn di_ahbermsk(&mut self) -> DI_AHBERMSK_W<2> {
        DI_AHBERMSK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn timeoutmsk(&mut self) -> TIMEOUTMSK_W<3> {
        TIMEOUTMSK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn intkntxfempmsk(&mut self) -> INTKNTXFEMPMSK_W<4> {
        INTKNTXFEMPMSK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn intknepmismsk(&mut self) -> INTKNEPMISMSK_W<5> {
        INTKNEPMISMSK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn inepnakeffmsk(&mut self) -> INEPNAKEFFMSK_W<6> {
        INEPNAKEFFMSK_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn txfifoundrnmsk(&mut self) -> TXFIFOUNDRNMSK_W<8> {
        TXFIFOUNDRNMSK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnainintrmsk(&mut self) -> BNAININTRMSK_W<9> {
        BNAININTRMSK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn di_nakmsk(&mut self) -> DI_NAKMSK_W<13> {
        DI_NAKMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepmsk](index.html) module"]
pub struct DIEPMSK_SPEC;
impl crate::RegisterSpec for DIEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepmsk::R](R) reader structure"]
impl crate::Readable for DIEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](W) writer structure"]
impl crate::Writable for DIEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPMSK to value 0"]
impl crate::Resettable for DIEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
