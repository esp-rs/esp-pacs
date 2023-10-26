#[doc = "Register `DOEPINT4` reader"]
pub type R = crate::R<DOEPINT4_SPEC>;
#[doc = "Register `DOEPINT4` writer"]
pub type W = crate::W<DOEPINT4_SPEC>;
#[doc = "Field `XFERCOMPL4` reader - "]
pub type XFERCOMPL4_R = crate::BitReader;
#[doc = "Field `XFERCOMPL4` writer - "]
pub type XFERCOMPL4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISBLD4` reader - "]
pub type EPDISBLD4_R = crate::BitReader;
#[doc = "Field `EPDISBLD4` writer - "]
pub type EPDISBLD4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHBERR4` reader - "]
pub type AHBERR4_R = crate::BitReader;
#[doc = "Field `AHBERR4` writer - "]
pub type AHBERR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETUP4` reader - "]
pub type SETUP4_R = crate::BitReader;
#[doc = "Field `SETUP4` writer - "]
pub type SETUP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTTKNEPDIS4` reader - "]
pub type OUTTKNEPDIS4_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS4` writer - "]
pub type OUTTKNEPDIS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STSPHSERCVD4` reader - "]
pub type STSPHSERCVD4_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD4` writer - "]
pub type STSPHSERCVD4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BACK2BACKSETUP4` reader - "]
pub type BACK2BACKSETUP4_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP4` writer - "]
pub type BACK2BACKSETUP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTPKTERR4` reader - "]
pub type OUTPKTERR4_R = crate::BitReader;
#[doc = "Field `OUTPKTERR4` writer - "]
pub type OUTPKTERR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BNAINTR4` reader - "]
pub type BNAINTR4_R = crate::BitReader;
#[doc = "Field `BNAINTR4` writer - "]
pub type BNAINTR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PKTDRPSTS4` reader - "]
pub type PKTDRPSTS4_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS4` writer - "]
pub type PKTDRPSTS4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBLEERR4` reader - "]
pub type BBLEERR4_R = crate::BitReader;
#[doc = "Field `BBLEERR4` writer - "]
pub type BBLEERR4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKINTRPT4` reader - "]
pub type NAKINTRPT4_R = crate::BitReader;
#[doc = "Field `NAKINTRPT4` writer - "]
pub type NAKINTRPT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYEPINTRPT4` reader - "]
pub type NYEPINTRPT4_R = crate::BitReader;
#[doc = "Field `NYEPINTRPT4` writer - "]
pub type NYEPINTRPT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STUPPKTRCVD4` reader - "]
pub type STUPPKTRCVD4_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD4` writer - "]
pub type STUPPKTRCVD4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl4(&self) -> XFERCOMPL4_R {
        XFERCOMPL4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld4(&self) -> EPDISBLD4_R {
        EPDISBLD4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr4(&self) -> AHBERR4_R {
        AHBERR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup4(&self) -> SETUP4_R {
        SETUP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis4(&self) -> OUTTKNEPDIS4_R {
        OUTTKNEPDIS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd4(&self) -> STSPHSERCVD4_R {
        STSPHSERCVD4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup4(&self) -> BACK2BACKSETUP4_R {
        BACK2BACKSETUP4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr4(&self) -> OUTPKTERR4_R {
        OUTPKTERR4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr4(&self) -> BNAINTR4_R {
        BNAINTR4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts4(&self) -> PKTDRPSTS4_R {
        PKTDRPSTS4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr4(&self) -> BBLEERR4_R {
        BBLEERR4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt4(&self) -> NAKINTRPT4_R {
        NAKINTRPT4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt4(&self) -> NYEPINTRPT4_R {
        NYEPINTRPT4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd4(&self) -> STUPPKTRCVD4_R {
        STUPPKTRCVD4_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT4")
            .field("xfercompl4", &format_args!("{}", self.xfercompl4().bit()))
            .field("epdisbld4", &format_args!("{}", self.epdisbld4().bit()))
            .field("ahberr4", &format_args!("{}", self.ahberr4().bit()))
            .field("setup4", &format_args!("{}", self.setup4().bit()))
            .field(
                "outtknepdis4",
                &format_args!("{}", self.outtknepdis4().bit()),
            )
            .field(
                "stsphsercvd4",
                &format_args!("{}", self.stsphsercvd4().bit()),
            )
            .field(
                "back2backsetup4",
                &format_args!("{}", self.back2backsetup4().bit()),
            )
            .field("outpkterr4", &format_args!("{}", self.outpkterr4().bit()))
            .field("bnaintr4", &format_args!("{}", self.bnaintr4().bit()))
            .field("pktdrpsts4", &format_args!("{}", self.pktdrpsts4().bit()))
            .field("bbleerr4", &format_args!("{}", self.bbleerr4().bit()))
            .field("nakintrpt4", &format_args!("{}", self.nakintrpt4().bit()))
            .field("nyepintrpt4", &format_args!("{}", self.nyepintrpt4().bit()))
            .field(
                "stuppktrcvd4",
                &format_args!("{}", self.stuppktrcvd4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPINT4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl4(&mut self) -> XFERCOMPL4_W<DOEPINT4_SPEC, 0> {
        XFERCOMPL4_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld4(&mut self) -> EPDISBLD4_W<DOEPINT4_SPEC, 1> {
        EPDISBLD4_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr4(&mut self) -> AHBERR4_W<DOEPINT4_SPEC, 2> {
        AHBERR4_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setup4(&mut self) -> SETUP4_W<DOEPINT4_SPEC, 3> {
        SETUP4_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis4(&mut self) -> OUTTKNEPDIS4_W<DOEPINT4_SPEC, 4> {
        OUTTKNEPDIS4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd4(&mut self) -> STSPHSERCVD4_W<DOEPINT4_SPEC, 5> {
        STSPHSERCVD4_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup4(&mut self) -> BACK2BACKSETUP4_W<DOEPINT4_SPEC, 6> {
        BACK2BACKSETUP4_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr4(&mut self) -> OUTPKTERR4_W<DOEPINT4_SPEC, 8> {
        OUTPKTERR4_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr4(&mut self) -> BNAINTR4_W<DOEPINT4_SPEC, 9> {
        BNAINTR4_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts4(&mut self) -> PKTDRPSTS4_W<DOEPINT4_SPEC, 11> {
        PKTDRPSTS4_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr4(&mut self) -> BBLEERR4_W<DOEPINT4_SPEC, 12> {
        BBLEERR4_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt4(&mut self) -> NAKINTRPT4_W<DOEPINT4_SPEC, 13> {
        NAKINTRPT4_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyepintrpt4(&mut self) -> NYEPINTRPT4_W<DOEPINT4_SPEC, 14> {
        NYEPINTRPT4_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd4(&mut self) -> STUPPKTRCVD4_W<DOEPINT4_SPEC, 15> {
        STUPPKTRCVD4_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT4_SPEC;
impl crate::RegisterSpec for DOEPINT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint4::R`](R) reader structure"]
impl crate::Readable for DOEPINT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint4::W`](W) writer structure"]
impl crate::Writable for DOEPINT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPINT4 to value 0"]
impl crate::Resettable for DOEPINT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
