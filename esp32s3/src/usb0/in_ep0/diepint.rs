#[doc = "Register `DIEPINT` reader"]
pub type R = crate::R<DIEPINT_SPEC>;
#[doc = "Register `DIEPINT` writer"]
pub type W = crate::W<DIEPINT_SPEC>;
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
#[doc = "Field `TIMEOUT` reader - "]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - "]
pub type TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNTXFEMP` reader - "]
pub type INTKNTXFEMP_R = crate::BitReader;
#[doc = "Field `INTKNTXFEMP` writer - "]
pub type INTKNTXFEMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTKNEPMIS` reader - "]
pub type INTKNEPMIS_R = crate::BitReader;
#[doc = "Field `INTKNEPMIS` writer - "]
pub type INTKNEPMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNAKEFF` reader - "]
pub type INEPNAKEFF_R = crate::BitReader;
#[doc = "Field `INEPNAKEFF` writer - "]
pub type INEPNAKEFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEMP` reader - "]
pub type TXFEMP_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` reader - "]
pub type TXFIFOUNDRN_R = crate::BitReader;
#[doc = "Field `TXFIFOUNDRN` writer - "]
pub type TXFIFOUNDRN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `NYETINTRPT` reader - "]
pub type NYETINTRPT_R = crate::BitReader;
#[doc = "Field `NYETINTRPT` writer - "]
pub type NYETINTRPT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn intkntxfemp(&self) -> INTKNTXFEMP_R {
        INTKNTXFEMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn intknepmis(&self) -> INTKNEPMIS_R {
        INTKNEPMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inepnakeff(&self) -> INEPNAKEFF_R {
        INEPNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn txfemp(&self) -> TXFEMP_R {
        TXFEMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txfifoundrn(&self) -> TXFIFOUNDRN_R {
        TXFIFOUNDRN_R::new(((self.bits >> 8) & 1) != 0)
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
    pub fn nyetintrpt(&self) -> NYETINTRPT_R {
        NYETINTRPT_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT")
            .field("xfercompl", &self.xfercompl())
            .field("epdisbld", &self.epdisbld())
            .field("ahberr", &self.ahberr())
            .field("timeout", &self.timeout())
            .field("intkntxfemp", &self.intkntxfemp())
            .field("intknepmis", &self.intknepmis())
            .field("inepnakeff", &self.inepnakeff())
            .field("txfemp", &self.txfemp())
            .field("txfifoundrn", &self.txfifoundrn())
            .field("bnaintr", &self.bnaintr())
            .field("pktdrpsts", &self.pktdrpsts())
            .field("bbleerr", &self.bbleerr())
            .field("nakintrpt", &self.nakintrpt())
            .field("nyetintrpt", &self.nyetintrpt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XFERCOMPL_W<DIEPINT_SPEC> {
        XFERCOMPL_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld(&mut self) -> EPDISBLD_W<DIEPINT_SPEC> {
        EPDISBLD_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AHBERR_W<DIEPINT_SPEC> {
        AHBERR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<DIEPINT_SPEC> {
        TIMEOUT_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn intkntxfemp(&mut self) -> INTKNTXFEMP_W<DIEPINT_SPEC> {
        INTKNTXFEMP_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn intknepmis(&mut self) -> INTKNEPMIS_W<DIEPINT_SPEC> {
        INTKNEPMIS_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn inepnakeff(&mut self) -> INEPNAKEFF_W<DIEPINT_SPEC> {
        INEPNAKEFF_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn txfifoundrn(&mut self) -> TXFIFOUNDRN_W<DIEPINT_SPEC> {
        TXFIFOUNDRN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr(&mut self) -> BNAINTR_W<DIEPINT_SPEC> {
        BNAINTR_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts(&mut self) -> PKTDRPSTS_W<DIEPINT_SPEC> {
        PKTDRPSTS_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr(&mut self) -> BBLEERR_W<DIEPINT_SPEC> {
        BBLEERR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt(&mut self) -> NAKINTRPT_W<DIEPINT_SPEC> {
        NAKINTRPT_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyetintrpt(&mut self) -> NYETINTRPT_W<DIEPINT_SPEC> {
        NYETINTRPT_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT_SPEC;
impl crate::RegisterSpec for DIEPINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint::R`](R) reader structure"]
impl crate::Readable for DIEPINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint::W`](W) writer structure"]
impl crate::Writable for DIEPINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT to value 0"]
impl crate::Resettable for DIEPINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
