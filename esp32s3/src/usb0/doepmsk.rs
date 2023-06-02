#[doc = "Register `DOEPMSK` reader"]
pub struct R(crate::R<DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPMSK` writer"]
pub struct W(crate::W<DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPMSK_SPEC>;
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
impl From<crate::W<DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPLMSK` reader - "]
pub type XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - "]
pub type XFERCOMPLMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `EPDISBLDMSK` reader - "]
pub type EPDISBLDMSK_R = crate::BitReader;
#[doc = "Field `EPDISBLDMSK` writer - "]
pub type EPDISBLDMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `AHBERMSK` reader - "]
pub type AHBERMSK_R = crate::BitReader;
#[doc = "Field `AHBERMSK` writer - "]
pub type AHBERMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `SETUPMSK` reader - "]
pub type SETUPMSK_R = crate::BitReader;
#[doc = "Field `SETUPMSK` writer - "]
pub type SETUPMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `OUTTKNEPDISMSK` reader - "]
pub type OUTTKNEPDISMSK_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDISMSK` writer - "]
pub type OUTTKNEPDISMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `STSPHSERCVDMSK` reader - "]
pub type STSPHSERCVDMSK_R = crate::BitReader;
#[doc = "Field `STSPHSERCVDMSK` writer - "]
pub type STSPHSERCVDMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `BACK2BACKSETUP` reader - "]
pub type BACK2BACKSETUP_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP` writer - "]
pub type BACK2BACKSETUP_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `OUTPKTERRMSK` reader - "]
pub type OUTPKTERRMSK_R = crate::BitReader;
#[doc = "Field `OUTPKTERRMSK` writer - "]
pub type OUTPKTERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `BNAOUTINTRMSK` reader - "]
pub type BNAOUTINTRMSK_R = crate::BitReader;
#[doc = "Field `BNAOUTINTRMSK` writer - "]
pub type BNAOUTINTRMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `BBLEERRMSK` reader - "]
pub type BBLEERRMSK_R = crate::BitReader;
#[doc = "Field `BBLEERRMSK` writer - "]
pub type BBLEERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `NAKMSK` reader - "]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - "]
pub type NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
#[doc = "Field `NYETMSK` reader - "]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMSK` writer - "]
pub type NYETMSK_W<'a, const O: u8> = crate::BitWriter<'a, DOEPMSK_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahbermsk(&self) -> AHBERMSK_R {
        AHBERMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setupmsk(&self) -> SETUPMSK_R {
        SETUPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdismsk(&self) -> OUTTKNEPDISMSK_R {
        OUTTKNEPDISMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvdmsk(&self) -> STSPHSERCVDMSK_R {
        STSPHSERCVDMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterrmsk(&self) -> OUTPKTERRMSK_R {
        OUTPKTERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaoutintrmsk(&self) -> BNAOUTINTRMSK_R {
        BNAOUTINTRMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerrmsk(&self) -> BBLEERRMSK_R {
        BBLEERRMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPMSK")
            .field(
                "xfercomplmsk",
                &format_args!("{}", self.xfercomplmsk().bit()),
            )
            .field("epdisbldmsk", &format_args!("{}", self.epdisbldmsk().bit()))
            .field("ahbermsk", &format_args!("{}", self.ahbermsk().bit()))
            .field("setupmsk", &format_args!("{}", self.setupmsk().bit()))
            .field(
                "outtknepdismsk",
                &format_args!("{}", self.outtknepdismsk().bit()),
            )
            .field(
                "stsphsercvdmsk",
                &format_args!("{}", self.stsphsercvdmsk().bit()),
            )
            .field(
                "back2backsetup",
                &format_args!("{}", self.back2backsetup().bit()),
            )
            .field(
                "outpkterrmsk",
                &format_args!("{}", self.outpkterrmsk().bit()),
            )
            .field(
                "bnaoutintrmsk",
                &format_args!("{}", self.bnaoutintrmsk().bit()),
            )
            .field("bbleerrmsk", &format_args!("{}", self.bbleerrmsk().bit()))
            .field("nakmsk", &format_args!("{}", self.nakmsk().bit()))
            .field("nyetmsk", &format_args!("{}", self.nyetmsk().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPMSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<0> {
        XFERCOMPLMSK_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbldmsk(&mut self) -> EPDISBLDMSK_W<1> {
        EPDISBLDMSK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahbermsk(&mut self) -> AHBERMSK_W<2> {
        AHBERMSK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setupmsk(&mut self) -> SETUPMSK_W<3> {
        SETUPMSK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdismsk(&mut self) -> OUTTKNEPDISMSK_W<4> {
        OUTTKNEPDISMSK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvdmsk(&mut self) -> STSPHSERCVDMSK_W<5> {
        STSPHSERCVDMSK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W<6> {
        BACK2BACKSETUP_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterrmsk(&mut self) -> OUTPKTERRMSK_W<8> {
        OUTPKTERRMSK_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaoutintrmsk(&mut self) -> BNAOUTINTRMSK_W<9> {
        BNAOUTINTRMSK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerrmsk(&mut self) -> BBLEERRMSK_W<12> {
        BBLEERRMSK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<13> {
        NAKMSK_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<14> {
        NYETMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](index.html) module"]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepmsk::R](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
