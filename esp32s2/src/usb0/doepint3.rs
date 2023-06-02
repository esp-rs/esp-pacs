#[doc = "Register `DOEPINT3` reader"]
pub struct R(crate::R<DOEPINT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT3` writer"]
pub struct W(crate::W<DOEPINT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT3_SPEC>;
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
impl From<crate::W<DOEPINT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPL3` reader - "]
pub type XFERCOMPL3_R = crate::BitReader;
#[doc = "Field `XFERCOMPL3` writer - "]
pub type XFERCOMPL3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `EPDISBLD3` reader - "]
pub type EPDISBLD3_R = crate::BitReader;
#[doc = "Field `EPDISBLD3` writer - "]
pub type EPDISBLD3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `AHBERR3` reader - "]
pub type AHBERR3_R = crate::BitReader;
#[doc = "Field `AHBERR3` writer - "]
pub type AHBERR3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `SETUP3` reader - "]
pub type SETUP3_R = crate::BitReader;
#[doc = "Field `SETUP3` writer - "]
pub type SETUP3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `OUTTKNEPDIS3` reader - "]
pub type OUTTKNEPDIS3_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS3` writer - "]
pub type OUTTKNEPDIS3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `STSPHSERCVD3` reader - "]
pub type STSPHSERCVD3_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD3` writer - "]
pub type STSPHSERCVD3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `BACK2BACKSETUP3` reader - "]
pub type BACK2BACKSETUP3_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP3` writer - "]
pub type BACK2BACKSETUP3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `OUTPKTERR3` reader - "]
pub type OUTPKTERR3_R = crate::BitReader;
#[doc = "Field `OUTPKTERR3` writer - "]
pub type OUTPKTERR3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `BNAINTR3` reader - "]
pub type BNAINTR3_R = crate::BitReader;
#[doc = "Field `BNAINTR3` writer - "]
pub type BNAINTR3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `PKTDRPSTS3` reader - "]
pub type PKTDRPSTS3_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS3` writer - "]
pub type PKTDRPSTS3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `BBLEERR3` reader - "]
pub type BBLEERR3_R = crate::BitReader;
#[doc = "Field `BBLEERR3` writer - "]
pub type BBLEERR3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `NAKINTRPT3` reader - "]
pub type NAKINTRPT3_R = crate::BitReader;
#[doc = "Field `NAKINTRPT3` writer - "]
pub type NAKINTRPT3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `NYEPINTRPT3` reader - "]
pub type NYEPINTRPT3_R = crate::BitReader;
#[doc = "Field `NYEPINTRPT3` writer - "]
pub type NYEPINTRPT3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
#[doc = "Field `STUPPKTRCVD3` reader - "]
pub type STUPPKTRCVD3_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD3` writer - "]
pub type STUPPKTRCVD3_W<'a, const O: u8> = crate::BitWriter<'a, DOEPINT3_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl3(&self) -> XFERCOMPL3_R {
        XFERCOMPL3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld3(&self) -> EPDISBLD3_R {
        EPDISBLD3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr3(&self) -> AHBERR3_R {
        AHBERR3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup3(&self) -> SETUP3_R {
        SETUP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis3(&self) -> OUTTKNEPDIS3_R {
        OUTTKNEPDIS3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd3(&self) -> STSPHSERCVD3_R {
        STSPHSERCVD3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup3(&self) -> BACK2BACKSETUP3_R {
        BACK2BACKSETUP3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr3(&self) -> OUTPKTERR3_R {
        OUTPKTERR3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr3(&self) -> BNAINTR3_R {
        BNAINTR3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts3(&self) -> PKTDRPSTS3_R {
        PKTDRPSTS3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr3(&self) -> BBLEERR3_R {
        BBLEERR3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt3(&self) -> NAKINTRPT3_R {
        NAKINTRPT3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt3(&self) -> NYEPINTRPT3_R {
        NYEPINTRPT3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd3(&self) -> STUPPKTRCVD3_R {
        STUPPKTRCVD3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT3")
            .field("xfercompl3", &format_args!("{}", self.xfercompl3().bit()))
            .field("epdisbld3", &format_args!("{}", self.epdisbld3().bit()))
            .field("ahberr3", &format_args!("{}", self.ahberr3().bit()))
            .field("setup3", &format_args!("{}", self.setup3().bit()))
            .field(
                "outtknepdis3",
                &format_args!("{}", self.outtknepdis3().bit()),
            )
            .field(
                "stsphsercvd3",
                &format_args!("{}", self.stsphsercvd3().bit()),
            )
            .field(
                "back2backsetup3",
                &format_args!("{}", self.back2backsetup3().bit()),
            )
            .field("outpkterr3", &format_args!("{}", self.outpkterr3().bit()))
            .field("bnaintr3", &format_args!("{}", self.bnaintr3().bit()))
            .field("pktdrpsts3", &format_args!("{}", self.pktdrpsts3().bit()))
            .field("bbleerr3", &format_args!("{}", self.bbleerr3().bit()))
            .field("nakintrpt3", &format_args!("{}", self.nakintrpt3().bit()))
            .field("nyepintrpt3", &format_args!("{}", self.nyepintrpt3().bit()))
            .field(
                "stuppktrcvd3",
                &format_args!("{}", self.stuppktrcvd3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPINT3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl3(&mut self) -> XFERCOMPL3_W<0> {
        XFERCOMPL3_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld3(&mut self) -> EPDISBLD3_W<1> {
        EPDISBLD3_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr3(&mut self) -> AHBERR3_W<2> {
        AHBERR3_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setup3(&mut self) -> SETUP3_W<3> {
        SETUP3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis3(&mut self) -> OUTTKNEPDIS3_W<4> {
        OUTTKNEPDIS3_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd3(&mut self) -> STSPHSERCVD3_W<5> {
        STSPHSERCVD3_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup3(&mut self) -> BACK2BACKSETUP3_W<6> {
        BACK2BACKSETUP3_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr3(&mut self) -> OUTPKTERR3_W<8> {
        OUTPKTERR3_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr3(&mut self) -> BNAINTR3_W<9> {
        BNAINTR3_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts3(&mut self) -> PKTDRPSTS3_W<11> {
        PKTDRPSTS3_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr3(&mut self) -> BBLEERR3_W<12> {
        BBLEERR3_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt3(&mut self) -> NAKINTRPT3_W<13> {
        NAKINTRPT3_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyepintrpt3(&mut self) -> NYEPINTRPT3_W<14> {
        NYEPINTRPT3_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd3(&mut self) -> STUPPKTRCVD3_W<15> {
        STUPPKTRCVD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint3](index.html) module"]
pub struct DOEPINT3_SPEC;
impl crate::RegisterSpec for DOEPINT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint3::R](R) reader structure"]
impl crate::Readable for DOEPINT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint3::W](W) writer structure"]
impl crate::Writable for DOEPINT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPINT3 to value 0"]
impl crate::Resettable for DOEPINT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
