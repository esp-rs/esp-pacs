#[doc = "Register `DIEPINT4` reader"]
pub type R = crate::R<DIEPINT4_SPEC>;
#[doc = "Register `DIEPINT4` writer"]
pub type W = crate::W<DIEPINT4_SPEC>;
#[doc = "Field `D_XFERCOMPL4` reader - "]
pub type D_XFERCOMPL4_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL4` writer - "]
pub type D_XFERCOMPL4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_EPDISBLD4` reader - "]
pub type D_EPDISBLD4_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD4` writer - "]
pub type D_EPDISBLD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_AHBERR4` reader - "]
pub type D_AHBERR4_R = crate::BitReader;
#[doc = "Field `D_AHBERR4` writer - "]
pub type D_AHBERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_TIMEOUT4` reader - "]
pub type D_TIMEOUT4_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT4` writer - "]
pub type D_TIMEOUT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_INTKNTXFEMP4` reader - "]
pub type D_INTKNTXFEMP4_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP4` writer - "]
pub type D_INTKNTXFEMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_INTKNEPMIS4` reader - "]
pub type D_INTKNEPMIS4_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS4` writer - "]
pub type D_INTKNEPMIS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_INEPNAKEFF4` reader - "]
pub type D_INEPNAKEFF4_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF4` writer - "]
pub type D_INEPNAKEFF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_TXFEMP4` reader - "]
pub type D_TXFEMP4_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN4` reader - "]
pub type D_TXFIFOUNDRN4_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN4` writer - "]
pub type D_TXFIFOUNDRN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_BNAINTR4` reader - "]
pub type D_BNAINTR4_R = crate::BitReader;
#[doc = "Field `D_BNAINTR4` writer - "]
pub type D_BNAINTR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_PKTDRPSTS4` reader - "]
pub type D_PKTDRPSTS4_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS4` writer - "]
pub type D_PKTDRPSTS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_BBLEERR4` reader - "]
pub type D_BBLEERR4_R = crate::BitReader;
#[doc = "Field `D_BBLEERR4` writer - "]
pub type D_BBLEERR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_NAKINTRPT4` reader - "]
pub type D_NAKINTRPT4_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT4` writer - "]
pub type D_NAKINTRPT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_NYETINTRPT4` reader - "]
pub type D_NYETINTRPT4_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT4` writer - "]
pub type D_NYETINTRPT4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn d_xfercompl4(&self) -> D_XFERCOMPL4_R {
        D_XFERCOMPL4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn d_epdisbld4(&self) -> D_EPDISBLD4_R {
        D_EPDISBLD4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn d_ahberr4(&self) -> D_AHBERR4_R {
        D_AHBERR4_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn d_timeout4(&self) -> D_TIMEOUT4_R {
        D_TIMEOUT4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn d_intkntxfemp4(&self) -> D_INTKNTXFEMP4_R {
        D_INTKNTXFEMP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_intknepmis4(&self) -> D_INTKNEPMIS4_R {
        D_INTKNEPMIS4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn d_inepnakeff4(&self) -> D_INEPNAKEFF4_R {
        D_INEPNAKEFF4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn d_txfemp4(&self) -> D_TXFEMP4_R {
        D_TXFEMP4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn d_txfifoundrn4(&self) -> D_TXFIFOUNDRN4_R {
        D_TXFIFOUNDRN4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn d_bnaintr4(&self) -> D_BNAINTR4_R {
        D_BNAINTR4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn d_pktdrpsts4(&self) -> D_PKTDRPSTS4_R {
        D_PKTDRPSTS4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn d_bbleerr4(&self) -> D_BBLEERR4_R {
        D_BBLEERR4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn d_nakintrpt4(&self) -> D_NAKINTRPT4_R {
        D_NAKINTRPT4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn d_nyetintrpt4(&self) -> D_NYETINTRPT4_R {
        D_NYETINTRPT4_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT4")
            .field(
                "d_xfercompl4",
                &format_args!("{}", self.d_xfercompl4().bit()),
            )
            .field("d_epdisbld4", &format_args!("{}", self.d_epdisbld4().bit()))
            .field("d_ahberr4", &format_args!("{}", self.d_ahberr4().bit()))
            .field("d_timeout4", &format_args!("{}", self.d_timeout4().bit()))
            .field(
                "d_intkntxfemp4",
                &format_args!("{}", self.d_intkntxfemp4().bit()),
            )
            .field(
                "d_intknepmis4",
                &format_args!("{}", self.d_intknepmis4().bit()),
            )
            .field(
                "d_inepnakeff4",
                &format_args!("{}", self.d_inepnakeff4().bit()),
            )
            .field("d_txfemp4", &format_args!("{}", self.d_txfemp4().bit()))
            .field(
                "d_txfifoundrn4",
                &format_args!("{}", self.d_txfifoundrn4().bit()),
            )
            .field("d_bnaintr4", &format_args!("{}", self.d_bnaintr4().bit()))
            .field(
                "d_pktdrpsts4",
                &format_args!("{}", self.d_pktdrpsts4().bit()),
            )
            .field("d_bbleerr4", &format_args!("{}", self.d_bbleerr4().bit()))
            .field(
                "d_nakintrpt4",
                &format_args!("{}", self.d_nakintrpt4().bit()),
            )
            .field(
                "d_nyetintrpt4",
                &format_args!("{}", self.d_nyetintrpt4().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPINT4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl4(&mut self) -> D_XFERCOMPL4_W<DIEPINT4_SPEC> {
        D_XFERCOMPL4_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld4(&mut self) -> D_EPDISBLD4_W<DIEPINT4_SPEC> {
        D_EPDISBLD4_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr4(&mut self) -> D_AHBERR4_W<DIEPINT4_SPEC> {
        D_AHBERR4_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout4(&mut self) -> D_TIMEOUT4_W<DIEPINT4_SPEC> {
        D_TIMEOUT4_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp4(&mut self) -> D_INTKNTXFEMP4_W<DIEPINT4_SPEC> {
        D_INTKNTXFEMP4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis4(&mut self) -> D_INTKNEPMIS4_W<DIEPINT4_SPEC> {
        D_INTKNEPMIS4_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff4(&mut self) -> D_INEPNAKEFF4_W<DIEPINT4_SPEC> {
        D_INEPNAKEFF4_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn4(&mut self) -> D_TXFIFOUNDRN4_W<DIEPINT4_SPEC> {
        D_TXFIFOUNDRN4_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr4(&mut self) -> D_BNAINTR4_W<DIEPINT4_SPEC> {
        D_BNAINTR4_W::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts4(&mut self) -> D_PKTDRPSTS4_W<DIEPINT4_SPEC> {
        D_PKTDRPSTS4_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr4(&mut self) -> D_BBLEERR4_W<DIEPINT4_SPEC> {
        D_BBLEERR4_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt4(&mut self) -> D_NAKINTRPT4_W<DIEPINT4_SPEC> {
        D_NAKINTRPT4_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt4(&mut self) -> D_NYETINTRPT4_W<DIEPINT4_SPEC> {
        D_NYETINTRPT4_W::new(self, 14)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepint4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepint4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT4_SPEC;
impl crate::RegisterSpec for DIEPINT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint4::R`](R) reader structure"]
impl crate::Readable for DIEPINT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepint4::W`](W) writer structure"]
impl crate::Writable for DIEPINT4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT4 to value 0"]
impl crate::Resettable for DIEPINT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
