#[doc = "Register `DOEPINT2` reader"]
pub type R = crate::R<DOEPINT2_SPEC>;
#[doc = "Register `DOEPINT2` writer"]
pub type W = crate::W<DOEPINT2_SPEC>;
#[doc = "Field `XFERCOMPL2` reader - "]
pub type XFERCOMPL2_R = crate::BitReader;
#[doc = "Field `XFERCOMPL2` writer - "]
pub type XFERCOMPL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLD2` reader - "]
pub type EPDISBLD2_R = crate::BitReader;
#[doc = "Field `EPDISBLD2` writer - "]
pub type EPDISBLD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR2` reader - "]
pub type AHBERR2_R = crate::BitReader;
#[doc = "Field `AHBERR2` writer - "]
pub type AHBERR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP2` reader - "]
pub type SETUP2_R = crate::BitReader;
#[doc = "Field `SETUP2` writer - "]
pub type SETUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDIS2` reader - "]
pub type OUTTKNEPDIS2_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS2` writer - "]
pub type OUTTKNEPDIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVD2` reader - "]
pub type STSPHSERCVD2_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD2` writer - "]
pub type STSPHSERCVD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP2` reader - "]
pub type BACK2BACKSETUP2_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP2` writer - "]
pub type BACK2BACKSETUP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERR2` reader - "]
pub type OUTPKTERR2_R = crate::BitReader;
#[doc = "Field `OUTPKTERR2` writer - "]
pub type OUTPKTERR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTR2` reader - "]
pub type BNAINTR2_R = crate::BitReader;
#[doc = "Field `BNAINTR2` writer - "]
pub type BNAINTR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS2` reader - "]
pub type PKTDRPSTS2_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS2` writer - "]
pub type PKTDRPSTS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR2` reader - "]
pub type BBLEERR2_R = crate::BitReader;
#[doc = "Field `BBLEERR2` writer - "]
pub type BBLEERR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT2` reader - "]
pub type NAKINTRPT2_R = crate::BitReader;
#[doc = "Field `NAKINTRPT2` writer - "]
pub type NAKINTRPT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYEPINTRPT2` reader - "]
pub type NYEPINTRPT2_R = crate::BitReader;
#[doc = "Field `NYEPINTRPT2` writer - "]
pub type NYEPINTRPT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPPKTRCVD2` reader - "]
pub type STUPPKTRCVD2_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD2` writer - "]
pub type STUPPKTRCVD2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl2(&self) -> XFERCOMPL2_R {
        XFERCOMPL2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld2(&self) -> EPDISBLD2_R {
        EPDISBLD2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr2(&self) -> AHBERR2_R {
        AHBERR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup2(&self) -> SETUP2_R {
        SETUP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis2(&self) -> OUTTKNEPDIS2_R {
        OUTTKNEPDIS2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd2(&self) -> STSPHSERCVD2_R {
        STSPHSERCVD2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup2(&self) -> BACK2BACKSETUP2_R {
        BACK2BACKSETUP2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr2(&self) -> OUTPKTERR2_R {
        OUTPKTERR2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr2(&self) -> BNAINTR2_R {
        BNAINTR2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts2(&self) -> PKTDRPSTS2_R {
        PKTDRPSTS2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr2(&self) -> BBLEERR2_R {
        BBLEERR2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt2(&self) -> NAKINTRPT2_R {
        NAKINTRPT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt2(&self) -> NYEPINTRPT2_R {
        NYEPINTRPT2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd2(&self) -> STUPPKTRCVD2_R {
        STUPPKTRCVD2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT2")
            .field("xfercompl2", &format_args!("{}", self.xfercompl2().bit()))
            .field("epdisbld2", &format_args!("{}", self.epdisbld2().bit()))
            .field("ahberr2", &format_args!("{}", self.ahberr2().bit()))
            .field("setup2", &format_args!("{}", self.setup2().bit()))
            .field(
                "outtknepdis2",
                &format_args!("{}", self.outtknepdis2().bit()),
            )
            .field(
                "stsphsercvd2",
                &format_args!("{}", self.stsphsercvd2().bit()),
            )
            .field(
                "back2backsetup2",
                &format_args!("{}", self.back2backsetup2().bit()),
            )
            .field("outpkterr2", &format_args!("{}", self.outpkterr2().bit()))
            .field("bnaintr2", &format_args!("{}", self.bnaintr2().bit()))
            .field("pktdrpsts2", &format_args!("{}", self.pktdrpsts2().bit()))
            .field("bbleerr2", &format_args!("{}", self.bbleerr2().bit()))
            .field("nakintrpt2", &format_args!("{}", self.nakintrpt2().bit()))
            .field("nyepintrpt2", &format_args!("{}", self.nyepintrpt2().bit()))
            .field(
                "stuppktrcvd2",
                &format_args!("{}", self.stuppktrcvd2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPINT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl2(&mut self) -> XFERCOMPL2_W<DOEPINT2_SPEC> {
        XFERCOMPL2_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld2(&mut self) -> EPDISBLD2_W<DOEPINT2_SPEC> {
        EPDISBLD2_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr2(&mut self) -> AHBERR2_W<DOEPINT2_SPEC> {
        AHBERR2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setup2(&mut self) -> SETUP2_W<DOEPINT2_SPEC> {
        SETUP2_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis2(&mut self) -> OUTTKNEPDIS2_W<DOEPINT2_SPEC> {
        OUTTKNEPDIS2_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd2(&mut self) -> STSPHSERCVD2_W<DOEPINT2_SPEC> {
        STSPHSERCVD2_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup2(&mut self) -> BACK2BACKSETUP2_W<DOEPINT2_SPEC> {
        BACK2BACKSETUP2_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr2(&mut self) -> OUTPKTERR2_W<DOEPINT2_SPEC> {
        OUTPKTERR2_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr2(&mut self) -> BNAINTR2_W<DOEPINT2_SPEC> {
        BNAINTR2_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts2(&mut self) -> PKTDRPSTS2_W<DOEPINT2_SPEC> {
        PKTDRPSTS2_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr2(&mut self) -> BBLEERR2_W<DOEPINT2_SPEC> {
        BBLEERR2_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt2(&mut self) -> NAKINTRPT2_W<DOEPINT2_SPEC> {
        NAKINTRPT2_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyepintrpt2(&mut self) -> NYEPINTRPT2_W<DOEPINT2_SPEC> {
        NYEPINTRPT2_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd2(&mut self) -> STUPPKTRCVD2_W<DOEPINT2_SPEC> {
        STUPPKTRCVD2_W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT2_SPEC;
impl crate::RegisterSpec for DOEPINT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint2::R`](R) reader structure"]
impl crate::Readable for DOEPINT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint2::W`](W) writer structure"]
impl crate::Writable for DOEPINT2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT2 to value 0"]
impl crate::Resettable for DOEPINT2_SPEC {
    const RESET_VALUE: u32 = 0;
}
