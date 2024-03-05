#[doc = "Register `DIEPINT3` reader"]
pub type R = crate::R<DIEPINT3_SPEC>;
#[doc = "Register `DIEPINT3` writer"]
pub type W = crate::W<DIEPINT3_SPEC>;
#[doc = "Field `D_XFERCOMPL3` reader - "]
pub type D_XFERCOMPL3_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL3` writer - "]
pub type D_XFERCOMPL3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_EPDISBLD3` reader - "]
pub type D_EPDISBLD3_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD3` writer - "]
pub type D_EPDISBLD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_AHBERR3` reader - "]
pub type D_AHBERR3_R = crate::BitReader;
#[doc = "Field `D_AHBERR3` writer - "]
pub type D_AHBERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_TIMEOUT3` reader - "]
pub type D_TIMEOUT3_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT3` writer - "]
pub type D_TIMEOUT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_INTKNTXFEMP3` reader - "]
pub type D_INTKNTXFEMP3_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP3` writer - "]
pub type D_INTKNTXFEMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_INTKNEPMIS3` reader - "]
pub type D_INTKNEPMIS3_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS3` writer - "]
pub type D_INTKNEPMIS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_INEPNAKEFF3` reader - "]
pub type D_INEPNAKEFF3_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF3` writer - "]
pub type D_INEPNAKEFF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_TXFEMP3` reader - "]
pub type D_TXFEMP3_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN3` reader - "]
pub type D_TXFIFOUNDRN3_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN3` writer - "]
pub type D_TXFIFOUNDRN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_BNAINTR3` reader - "]
pub type D_BNAINTR3_R = crate::BitReader;
#[doc = "Field `D_BNAINTR3` writer - "]
pub type D_BNAINTR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_PKTDRPSTS3` reader - "]
pub type D_PKTDRPSTS3_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS3` writer - "]
pub type D_PKTDRPSTS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_BBLEERR3` reader - "]
pub type D_BBLEERR3_R = crate::BitReader;
#[doc = "Field `D_BBLEERR3` writer - "]
pub type D_BBLEERR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_NAKINTRPT3` reader - "]
pub type D_NAKINTRPT3_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT3` writer - "]
pub type D_NAKINTRPT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_NYETINTRPT3` reader - "]
pub type D_NYETINTRPT3_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT3` writer - "]
pub type D_NYETINTRPT3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn d_xfercompl3(&self) -> D_XFERCOMPL3_R {
        D_XFERCOMPL3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn d_epdisbld3(&self) -> D_EPDISBLD3_R {
        D_EPDISBLD3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn d_ahberr3(&self) -> D_AHBERR3_R {
        D_AHBERR3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn d_timeout3(&self) -> D_TIMEOUT3_R {
        D_TIMEOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn d_intkntxfemp3(&self) -> D_INTKNTXFEMP3_R {
        D_INTKNTXFEMP3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_intknepmis3(&self) -> D_INTKNEPMIS3_R {
        D_INTKNEPMIS3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn d_inepnakeff3(&self) -> D_INEPNAKEFF3_R {
        D_INEPNAKEFF3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn d_txfemp3(&self) -> D_TXFEMP3_R {
        D_TXFEMP3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn d_txfifoundrn3(&self) -> D_TXFIFOUNDRN3_R {
        D_TXFIFOUNDRN3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn d_bnaintr3(&self) -> D_BNAINTR3_R {
        D_BNAINTR3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn d_pktdrpsts3(&self) -> D_PKTDRPSTS3_R {
        D_PKTDRPSTS3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn d_bbleerr3(&self) -> D_BBLEERR3_R {
        D_BBLEERR3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn d_nakintrpt3(&self) -> D_NAKINTRPT3_R {
        D_NAKINTRPT3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn d_nyetintrpt3(&self) -> D_NYETINTRPT3_R {
        D_NYETINTRPT3_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT3")
            .field(
                "d_xfercompl3",
                &format_args!("{}", self.d_xfercompl3().bit()),
            )
            .field("d_epdisbld3", &format_args!("{}", self.d_epdisbld3().bit()))
            .field("d_ahberr3", &format_args!("{}", self.d_ahberr3().bit()))
            .field("d_timeout3", &format_args!("{}", self.d_timeout3().bit()))
            .field(
                "d_intkntxfemp3",
                &format_args!("{}", self.d_intkntxfemp3().bit()),
            )
            .field(
                "d_intknepmis3",
                &format_args!("{}", self.d_intknepmis3().bit()),
            )
            .field(
                "d_inepnakeff3",
                &format_args!("{}", self.d_inepnakeff3().bit()),
            )
            .field("d_txfemp3", &format_args!("{}", self.d_txfemp3().bit()))
            .field(
                "d_txfifoundrn3",
                &format_args!("{}", self.d_txfifoundrn3().bit()),
            )
            .field("d_bnaintr3", &format_args!("{}", self.d_bnaintr3().bit()))
            .field(
                "d_pktdrpsts3",
                &format_args!("{}", self.d_pktdrpsts3().bit()),
            )
            .field("d_bbleerr3", &format_args!("{}", self.d_bbleerr3().bit()))
            .field(
                "d_nakintrpt3",
                &format_args!("{}", self.d_nakintrpt3().bit()),
            )
            .field(
                "d_nyetintrpt3",
                &format_args!("{}", self.d_nyetintrpt3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPINT3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl3(&mut self) -> D_XFERCOMPL3_W<DIEPINT3_SPEC> {
        D_XFERCOMPL3_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld3(&mut self) -> D_EPDISBLD3_W<DIEPINT3_SPEC> {
        D_EPDISBLD3_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr3(&mut self) -> D_AHBERR3_W<DIEPINT3_SPEC> {
        D_AHBERR3_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout3(&mut self) -> D_TIMEOUT3_W<DIEPINT3_SPEC> {
        D_TIMEOUT3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp3(&mut self) -> D_INTKNTXFEMP3_W<DIEPINT3_SPEC> {
        D_INTKNTXFEMP3_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis3(&mut self) -> D_INTKNEPMIS3_W<DIEPINT3_SPEC> {
        D_INTKNEPMIS3_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff3(&mut self) -> D_INEPNAKEFF3_W<DIEPINT3_SPEC> {
        D_INEPNAKEFF3_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn3(&mut self) -> D_TXFIFOUNDRN3_W<DIEPINT3_SPEC> {
        D_TXFIFOUNDRN3_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr3(&mut self) -> D_BNAINTR3_W<DIEPINT3_SPEC> {
        D_BNAINTR3_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts3(&mut self) -> D_PKTDRPSTS3_W<DIEPINT3_SPEC> {
        D_PKTDRPSTS3_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr3(&mut self) -> D_BBLEERR3_W<DIEPINT3_SPEC> {
        D_BBLEERR3_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt3(&mut self) -> D_NAKINTRPT3_W<DIEPINT3_SPEC> {
        D_NAKINTRPT3_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt3(&mut self) -> D_NYETINTRPT3_W<DIEPINT3_SPEC> {
        D_NYETINTRPT3_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT3_SPEC;
impl crate::RegisterSpec for DIEPINT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint3::R`](R) reader structure"]
impl crate::Readable for DIEPINT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint3::W`](W) writer structure"]
impl crate::Writable for DIEPINT3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT3 to value 0"]
impl crate::Resettable for DIEPINT3_SPEC {
    const RESET_VALUE: u32 = 0;
}
