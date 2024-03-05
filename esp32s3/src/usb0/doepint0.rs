#[doc = "Register `DOEPINT0` reader"]
pub type R = crate::R<DOEPINT0_SPEC>;
#[doc = "Register `DOEPINT0` writer"]
pub type W = crate::W<DOEPINT0_SPEC>;
#[doc = "Field `XFERCOMPL0` reader - "]
pub type XFERCOMPL0_R = crate::BitReader;
#[doc = "Field `XFERCOMPL0` writer - "]
pub type XFERCOMPL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLD0` reader - "]
pub type EPDISBLD0_R = crate::BitReader;
#[doc = "Field `EPDISBLD0` writer - "]
pub type EPDISBLD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR0` reader - "]
pub type AHBERR0_R = crate::BitReader;
#[doc = "Field `AHBERR0` writer - "]
pub type AHBERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP0` reader - "]
pub type SETUP0_R = crate::BitReader;
#[doc = "Field `SETUP0` writer - "]
pub type SETUP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDIS0` reader - "]
pub type OUTTKNEPDIS0_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDIS0` writer - "]
pub type OUTTKNEPDIS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVD0` reader - "]
pub type STSPHSERCVD0_R = crate::BitReader;
#[doc = "Field `STSPHSERCVD0` writer - "]
pub type STSPHSERCVD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP0` reader - "]
pub type BACK2BACKSETUP0_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP0` writer - "]
pub type BACK2BACKSETUP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERR0` reader - "]
pub type OUTPKTERR0_R = crate::BitReader;
#[doc = "Field `OUTPKTERR0` writer - "]
pub type OUTPKTERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTR0` reader - "]
pub type BNAINTR0_R = crate::BitReader;
#[doc = "Field `BNAINTR0` writer - "]
pub type BNAINTR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKTDRPSTS0` reader - "]
pub type PKTDRPSTS0_R = crate::BitReader;
#[doc = "Field `PKTDRPSTS0` writer - "]
pub type PKTDRPSTS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERR0` reader - "]
pub type BBLEERR0_R = crate::BitReader;
#[doc = "Field `BBLEERR0` writer - "]
pub type BBLEERR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKINTRPT0` reader - "]
pub type NAKINTRPT0_R = crate::BitReader;
#[doc = "Field `NAKINTRPT0` writer - "]
pub type NAKINTRPT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYEPINTRPT0` reader - "]
pub type NYEPINTRPT0_R = crate::BitReader;
#[doc = "Field `NYEPINTRPT0` writer - "]
pub type NYEPINTRPT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPPKTRCVD0` reader - "]
pub type STUPPKTRCVD0_R = crate::BitReader;
#[doc = "Field `STUPPKTRCVD0` writer - "]
pub type STUPPKTRCVD0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl0(&self) -> XFERCOMPL0_R {
        XFERCOMPL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld0(&self) -> EPDISBLD0_R {
        EPDISBLD0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr0(&self) -> AHBERR0_R {
        AHBERR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup0(&self) -> SETUP0_R {
        SETUP0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis0(&self) -> OUTTKNEPDIS0_R {
        OUTTKNEPDIS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd0(&self) -> STSPHSERCVD0_R {
        STSPHSERCVD0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup0(&self) -> BACK2BACKSETUP0_R {
        BACK2BACKSETUP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr0(&self) -> OUTPKTERR0_R {
        OUTPKTERR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr0(&self) -> BNAINTR0_R {
        BNAINTR0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts0(&self) -> PKTDRPSTS0_R {
        PKTDRPSTS0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr0(&self) -> BBLEERR0_R {
        BBLEERR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt0(&self) -> NAKINTRPT0_R {
        NAKINTRPT0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt0(&self) -> NYEPINTRPT0_R {
        NYEPINTRPT0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd0(&self) -> STUPPKTRCVD0_R {
        STUPPKTRCVD0_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT0")
            .field("xfercompl0", &format_args!("{}", self.xfercompl0().bit()))
            .field("epdisbld0", &format_args!("{}", self.epdisbld0().bit()))
            .field("ahberr0", &format_args!("{}", self.ahberr0().bit()))
            .field("setup0", &format_args!("{}", self.setup0().bit()))
            .field(
                "outtknepdis0",
                &format_args!("{}", self.outtknepdis0().bit()),
            )
            .field(
                "stsphsercvd0",
                &format_args!("{}", self.stsphsercvd0().bit()),
            )
            .field(
                "back2backsetup0",
                &format_args!("{}", self.back2backsetup0().bit()),
            )
            .field("outpkterr0", &format_args!("{}", self.outpkterr0().bit()))
            .field("bnaintr0", &format_args!("{}", self.bnaintr0().bit()))
            .field("pktdrpsts0", &format_args!("{}", self.pktdrpsts0().bit()))
            .field("bbleerr0", &format_args!("{}", self.bbleerr0().bit()))
            .field("nakintrpt0", &format_args!("{}", self.nakintrpt0().bit()))
            .field("nyepintrpt0", &format_args!("{}", self.nyepintrpt0().bit()))
            .field(
                "stuppktrcvd0",
                &format_args!("{}", self.stuppktrcvd0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPINT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercompl0(&mut self) -> XFERCOMPL0_W<DOEPINT0_SPEC> {
        XFERCOMPL0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbld0(&mut self) -> EPDISBLD0_W<DOEPINT0_SPEC> {
        EPDISBLD0_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahberr0(&mut self) -> AHBERR0_W<DOEPINT0_SPEC> {
        AHBERR0_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setup0(&mut self) -> SETUP0_W<DOEPINT0_SPEC> {
        SETUP0_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdis0(&mut self) -> OUTTKNEPDIS0_W<DOEPINT0_SPEC> {
        OUTTKNEPDIS0_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvd0(&mut self) -> STSPHSERCVD0_W<DOEPINT0_SPEC> {
        STSPHSERCVD0_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup0(&mut self) -> BACK2BACKSETUP0_W<DOEPINT0_SPEC> {
        BACK2BACKSETUP0_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterr0(&mut self) -> OUTPKTERR0_W<DOEPINT0_SPEC> {
        OUTPKTERR0_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaintr0(&mut self) -> BNAINTR0_W<DOEPINT0_SPEC> {
        BNAINTR0_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn pktdrpsts0(&mut self) -> PKTDRPSTS0_W<DOEPINT0_SPEC> {
        PKTDRPSTS0_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerr0(&mut self) -> BBLEERR0_W<DOEPINT0_SPEC> {
        BBLEERR0_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakintrpt0(&mut self) -> NAKINTRPT0_W<DOEPINT0_SPEC> {
        NAKINTRPT0_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyepintrpt0(&mut self) -> NYEPINTRPT0_W<DOEPINT0_SPEC> {
        NYEPINTRPT0_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn stuppktrcvd0(&mut self) -> STUPPKTRCVD0_W<DOEPINT0_SPEC> {
        STUPPKTRCVD0_W::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepint0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepint0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT0_SPEC;
impl crate::RegisterSpec for DOEPINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint0::R`](R) reader structure"]
impl crate::Readable for DOEPINT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepint0::W`](W) writer structure"]
impl crate::Writable for DOEPINT0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT0 to value 0"]
impl crate::Resettable for DOEPINT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
