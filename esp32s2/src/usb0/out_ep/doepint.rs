#[doc = "Register `DOEPINT` reader"]
pub type R = crate::R<DOEPINT_SPEC>;
#[doc = "Register `DOEPINT` writer"]
pub type W = crate::W<DOEPINT_SPEC>;
#[doc = "Field `XFERCOMPL` reader - "]
pub type XFERCOMPL_R = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - "]
pub type XFERCOMPL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLD` reader - "]
pub type EPDISBLD_R = crate::BitReader;
#[doc = "Field `EPDISBLD` writer - "]
pub type EPDISBLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - "]
pub type AHBERR_R = crate::BitReader;
#[doc = "Field `AHBERR` writer - "]
pub type AHBERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP` reader - "]
pub type SETUP_R = crate::BitReader;
#[doc = "Field `SETUP` writer - "]
pub type SETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDIS` reader - "]
pub type OUTTKNEPDIS_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS` writer - "]
pub type OUTTKNEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVD` reader - "]
pub type STSPHSERCVD_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD` writer - "]
pub type STSPHSERCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP` reader - "]
pub type BACK2BACKSETUP_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP` writer - "]
pub type BACK2BACKSETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERR` reader - "]
pub type OUTPKTERR_R = crate::BitReader;
#[doc = "Field `OUTPKTERR` writer - "]
pub type OUTPKTERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTR` reader - "]
pub type BNAINTR_R = crate::BitReader;
#[doc = "Field `BNAINTR` writer - "]
pub type BNAINTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS` reader - "]
pub type PKTDRPSTS_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS` writer - "]
pub type PKTDRPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR` reader - "]
pub type BBLEERR_R = crate::BitReader;
#[doc = "Field `BBLEERR` writer - "]
pub type BBLEERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT` reader - "]
pub type NAKINTRPT_R = crate::BitReader;
#[doc = "Field `NAKINTRPT` writer - "]
pub type NAKINTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYEPINTRPT` reader - "]
pub type NYEPINTRPT_R = crate::BitReader;
#[doc = "Field `NYEPINTRPT` writer - "]
pub type NYEPINTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPPKTRCVD` reader - "]
pub type STUPPKTRCVD_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD` writer - "]
pub type STUPPKTRCVD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XFERCOMPL_R {
        XFERCOMPL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld(&self) -> EPDISBLD_R {
        EPDISBLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr(&self) -> AHBERR_R {
        AHBERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis(&self) -> OUTTKNEPDIS_R {
        OUTTKNEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd(&self) -> STSPHSERCVD_R {
        STSPHSERCVD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr(&self) -> OUTPKTERR_R {
        OUTPKTERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr(&self) -> BNAINTR_R {
        BNAINTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts(&self) -> PKTDRPSTS_R {
        PKTDRPSTS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr(&self) -> BBLEERR_R {
        BBLEERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt(&self) -> NAKINTRPT_R {
        NAKINTRPT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt(&self) -> NYEPINTRPT_R {
        NYEPINTRPT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd(&self) -> STUPPKTRCVD_R {
        STUPPKTRCVD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT")
            .field("xfercompl", &format_args!("{}", self.xfercompl().bit()))
            .field("epdisbld", &format_args!("{}", self.epdisbld().bit()))
            .field("ahberr", &format_args!("{}", self.ahberr().bit()))
            .field("setup", &format_args!("{}", self.setup().bit()))
            .field("outtknepdis", &format_args!("{}", self.outtknepdis().bit()))
            .field("stsphsercvd", &format_args!("{}", self.stsphsercvd().bit()))
            .field(
                "back2backsetup",
                &format_args!("{}", self.back2backsetup().bit()),
            )
            .field("outpkterr", &format_args!("{}", self.outpkterr().bit()))
            .field("bnaintr", &format_args!("{}", self.bnaintr().bit()))
            .field("pktdrpsts", &format_args!("{}", self.pktdrpsts().bit()))
            .field("bbleerr", &format_args!("{}", self.bbleerr().bit()))
            .field("nakintrpt", &format_args!("{}", self.nakintrpt().bit()))
            .field("nyepintrpt", &format_args!("{}", self.nyepintrpt().bit()))
            .field("stuppktrcvd", &format_args!("{}", self.stuppktrcvd().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<DOEPINT_SPEC> {
        XFERCOMPL_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<DOEPINT_SPEC> {
        EPDISBLD_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr(&mut self) -> AHBERR_W<DOEPINT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<DOEPINT_SPEC> {
        SETUP_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis(&mut self) -> OUTTKNEPDIS_W<DOEPINT_SPEC> {
        OUTTKNEPDIS_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd(&mut self) -> STSPHSERCVD_W<DOEPINT_SPEC> {
        STSPHSERCVD_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W<DOEPINT_SPEC> {
        BACK2BACKSETUP_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr(&mut self) -> OUTPKTERR_W<DOEPINT_SPEC> {
        OUTPKTERR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr(&mut self) -> BNAINTR_W<DOEPINT_SPEC> {
        BNAINTR_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DOEPINT_SPEC> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr(&mut self) -> BBLEERR_W<DOEPINT_SPEC> {
        BBLEERR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W<DOEPINT_SPEC> {
        NAKINTRPT_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyepintrpt(&mut self) -> NYEPINTRPT_W<DOEPINT_SPEC> {
        NYEPINTRPT_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd(&mut self) -> STUPPKTRCVD_W<DOEPINT_SPEC> {
        STUPPKTRCVD_W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT_SPEC;
impl crate::RegisterSpec for DOEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint::R`](R) reader structure"]
impl crate::Readable for DOEPINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint::W`](W) writer structure"]
impl crate::Writable for DOEPINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT to value 0"]
impl crate::Resettable for DOEPINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
