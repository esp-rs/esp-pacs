#[doc = "Register `DOEPINT6` reader"]
pub type R = crate::R<DOEPINT6_SPEC>;
#[doc = "Register `DOEPINT6` writer"]
pub type W = crate::W<DOEPINT6_SPEC>;
#[doc = "Field `XFERCOMPL6` reader - "]
pub type XFERCOMPL6_R = crate::BitReader;
#[doc = "Field `XFERCOMPL6` writer - "]
pub type XFERCOMPL6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EPDISBLD6` reader - "]
pub type EPDISBLD6_R = crate::BitReader;
#[doc = "Field `EPDISBLD6` writer - "]
pub type EPDISBLD6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AHBERR6` reader - "]
pub type AHBERR6_R = crate::BitReader;
#[doc = "Field `AHBERR6` writer - "]
pub type AHBERR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SETUP6` reader - "]
pub type SETUP6_R = crate::BitReader;
#[doc = "Field `SETUP6` writer - "]
pub type SETUP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTTKNEPDIS6` reader - "]
pub type OUTTKNEPDIS6_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS6` writer - "]
pub type OUTTKNEPDIS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STSPHSERCVD6` reader - "]
pub type STSPHSERCVD6_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD6` writer - "]
pub type STSPHSERCVD6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BACK2BACKSETUP6` reader - "]
pub type BACK2BACKSETUP6_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP6` writer - "]
pub type BACK2BACKSETUP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OUTPKTERR6` reader - "]
pub type OUTPKTERR6_R = crate::BitReader;
#[doc = "Field `OUTPKTERR6` writer - "]
pub type OUTPKTERR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BNAINTR6` reader - "]
pub type BNAINTR6_R = crate::BitReader;
#[doc = "Field `BNAINTR6` writer - "]
pub type BNAINTR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PKTDRPSTS6` reader - "]
pub type PKTDRPSTS6_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS6` writer - "]
pub type PKTDRPSTS6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BBLEERR6` reader - "]
pub type BBLEERR6_R = crate::BitReader;
#[doc = "Field `BBLEERR6` writer - "]
pub type BBLEERR6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NAKINTRPT6` reader - "]
pub type NAKINTRPT6_R = crate::BitReader;
#[doc = "Field `NAKINTRPT6` writer - "]
pub type NAKINTRPT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NYEPINTRPT6` reader - "]
pub type NYEPINTRPT6_R = crate::BitReader;
#[doc = "Field `NYEPINTRPT6` writer - "]
pub type NYEPINTRPT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `STUPPKTRCVD6` reader - "]
pub type STUPPKTRCVD6_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD6` writer - "]
pub type STUPPKTRCVD6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl6(&self) -> XFERCOMPL6_R {
        XFERCOMPL6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld6(&self) -> EPDISBLD6_R {
        EPDISBLD6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr6(&self) -> AHBERR6_R {
        AHBERR6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup6(&self) -> SETUP6_R {
        SETUP6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis6(&self) -> OUTTKNEPDIS6_R {
        OUTTKNEPDIS6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd6(&self) -> STSPHSERCVD6_R {
        STSPHSERCVD6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup6(&self) -> BACK2BACKSETUP6_R {
        BACK2BACKSETUP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr6(&self) -> OUTPKTERR6_R {
        OUTPKTERR6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr6(&self) -> BNAINTR6_R {
        BNAINTR6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts6(&self) -> PKTDRPSTS6_R {
        PKTDRPSTS6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr6(&self) -> BBLEERR6_R {
        BBLEERR6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt6(&self) -> NAKINTRPT6_R {
        NAKINTRPT6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt6(&self) -> NYEPINTRPT6_R {
        NYEPINTRPT6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd6(&self) -> STUPPKTRCVD6_R {
        STUPPKTRCVD6_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT6")
            .field("xfercompl6", &format_args!("{}", self.xfercompl6().bit()))
            .field("epdisbld6", &format_args!("{}", self.epdisbld6().bit()))
            .field("ahberr6", &format_args!("{}", self.ahberr6().bit()))
            .field("setup6", &format_args!("{}", self.setup6().bit()))
            .field(
                "outtknepdis6",
                &format_args!("{}", self.outtknepdis6().bit()),
            )
            .field(
                "stsphsercvd6",
                &format_args!("{}", self.stsphsercvd6().bit()),
            )
            .field(
                "back2backsetup6",
                &format_args!("{}", self.back2backsetup6().bit()),
            )
            .field("outpkterr6", &format_args!("{}", self.outpkterr6().bit()))
            .field("bnaintr6", &format_args!("{}", self.bnaintr6().bit()))
            .field("pktdrpsts6", &format_args!("{}", self.pktdrpsts6().bit()))
            .field("bbleerr6", &format_args!("{}", self.bbleerr6().bit()))
            .field("nakintrpt6", &format_args!("{}", self.nakintrpt6().bit()))
            .field("nyepintrpt6", &format_args!("{}", self.nyepintrpt6().bit()))
            .field(
                "stuppktrcvd6",
                &format_args!("{}", self.stuppktrcvd6().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPINT6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl6(&mut self) -> XFERCOMPL6_W<DOEPINT6_SPEC, 0> {
        XFERCOMPL6_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld6(&mut self) -> EPDISBLD6_W<DOEPINT6_SPEC, 1> {
        EPDISBLD6_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr6(&mut self) -> AHBERR6_W<DOEPINT6_SPEC, 2> {
        AHBERR6_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setup6(&mut self) -> SETUP6_W<DOEPINT6_SPEC, 3> {
        SETUP6_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis6(&mut self) -> OUTTKNEPDIS6_W<DOEPINT6_SPEC, 4> {
        OUTTKNEPDIS6_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd6(&mut self) -> STSPHSERCVD6_W<DOEPINT6_SPEC, 5> {
        STSPHSERCVD6_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup6(&mut self) -> BACK2BACKSETUP6_W<DOEPINT6_SPEC, 6> {
        BACK2BACKSETUP6_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr6(&mut self) -> OUTPKTERR6_W<DOEPINT6_SPEC, 8> {
        OUTPKTERR6_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr6(&mut self) -> BNAINTR6_W<DOEPINT6_SPEC, 9> {
        BNAINTR6_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts6(&mut self) -> PKTDRPSTS6_W<DOEPINT6_SPEC, 11> {
        PKTDRPSTS6_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr6(&mut self) -> BBLEERR6_W<DOEPINT6_SPEC, 12> {
        BBLEERR6_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt6(&mut self) -> NAKINTRPT6_W<DOEPINT6_SPEC, 13> {
        NAKINTRPT6_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyepintrpt6(&mut self) -> NYEPINTRPT6_W<DOEPINT6_SPEC, 14> {
        NYEPINTRPT6_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd6(&mut self) -> STUPPKTRCVD6_W<DOEPINT6_SPEC, 15> {
        STUPPKTRCVD6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT6_SPEC;
impl crate::RegisterSpec for DOEPINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint6::R`](R) reader structure"]
impl crate::Readable for DOEPINT6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint6::W`](W) writer structure"]
impl crate::Writable for DOEPINT6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPINT6 to value 0"]
impl crate::Resettable for DOEPINT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
