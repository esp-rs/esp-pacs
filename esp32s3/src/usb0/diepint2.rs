#[doc = "Register `DIEPINT2` reader"]
pub struct R(crate::R<DIEPINT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT2` writer"]
pub struct W(crate::W<DIEPINT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT2_SPEC>;
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
impl From<crate::W<DIEPINT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERCOMPL2` reader - "]
pub type D_XFERCOMPL2_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL2` writer - "]
pub type D_XFERCOMPL2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_EPDISBLD2` reader - "]
pub type D_EPDISBLD2_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD2` writer - "]
pub type D_EPDISBLD2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_AHBERR2` reader - "]
pub type D_AHBERR2_R = crate::BitReader;
#[doc = "Field `D_AHBERR2` writer - "]
pub type D_AHBERR2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_TIMEOUT2` reader - "]
pub type D_TIMEOUT2_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT2` writer - "]
pub type D_TIMEOUT2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_INTKNTXFEMP2` reader - "]
pub type D_INTKNTXFEMP2_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP2` writer - "]
pub type D_INTKNTXFEMP2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_INTKNEPMIS2` reader - "]
pub type D_INTKNEPMIS2_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS2` writer - "]
pub type D_INTKNEPMIS2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_INEPNAKEFF2` reader - "]
pub type D_INEPNAKEFF2_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF2` writer - "]
pub type D_INEPNAKEFF2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_TXFEMP2` reader - "]
pub type D_TXFEMP2_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN2` reader - "]
pub type D_TXFIFOUNDRN2_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN2` writer - "]
pub type D_TXFIFOUNDRN2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_BNAINTR2` reader - "]
pub type D_BNAINTR2_R = crate::BitReader;
#[doc = "Field `D_BNAINTR2` writer - "]
pub type D_BNAINTR2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_PKTDRPSTS2` reader - "]
pub type D_PKTDRPSTS2_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS2` writer - "]
pub type D_PKTDRPSTS2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_BBLEERR2` reader - "]
pub type D_BBLEERR2_R = crate::BitReader;
#[doc = "Field `D_BBLEERR2` writer - "]
pub type D_BBLEERR2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_NAKINTRPT2` reader - "]
pub type D_NAKINTRPT2_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT2` writer - "]
pub type D_NAKINTRPT2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
#[doc = "Field `D_NYETINTRPT2` reader - "]
pub type D_NYETINTRPT2_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT2` writer - "]
pub type D_NYETINTRPT2_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT2_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn d_xfercompl2(&self) -> D_XFERCOMPL2_R {
        D_XFERCOMPL2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn d_epdisbld2(&self) -> D_EPDISBLD2_R {
        D_EPDISBLD2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn d_ahberr2(&self) -> D_AHBERR2_R {
        D_AHBERR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn d_timeout2(&self) -> D_TIMEOUT2_R {
        D_TIMEOUT2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn d_intkntxfemp2(&self) -> D_INTKNTXFEMP2_R {
        D_INTKNTXFEMP2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_intknepmis2(&self) -> D_INTKNEPMIS2_R {
        D_INTKNEPMIS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn d_inepnakeff2(&self) -> D_INEPNAKEFF2_R {
        D_INEPNAKEFF2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn d_txfemp2(&self) -> D_TXFEMP2_R {
        D_TXFEMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn d_txfifoundrn2(&self) -> D_TXFIFOUNDRN2_R {
        D_TXFIFOUNDRN2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn d_bnaintr2(&self) -> D_BNAINTR2_R {
        D_BNAINTR2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn d_pktdrpsts2(&self) -> D_PKTDRPSTS2_R {
        D_PKTDRPSTS2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn d_bbleerr2(&self) -> D_BBLEERR2_R {
        D_BBLEERR2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn d_nakintrpt2(&self) -> D_NAKINTRPT2_R {
        D_NAKINTRPT2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn d_nyetintrpt2(&self) -> D_NYETINTRPT2_R {
        D_NYETINTRPT2_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT2")
            .field(
                "d_xfercompl2",
                &format_args!("{}", self.d_xfercompl2().bit()),
            )
            .field("d_epdisbld2", &format_args!("{}", self.d_epdisbld2().bit()))
            .field("d_ahberr2", &format_args!("{}", self.d_ahberr2().bit()))
            .field("d_timeout2", &format_args!("{}", self.d_timeout2().bit()))
            .field(
                "d_intkntxfemp2",
                &format_args!("{}", self.d_intkntxfemp2().bit()),
            )
            .field(
                "d_intknepmis2",
                &format_args!("{}", self.d_intknepmis2().bit()),
            )
            .field(
                "d_inepnakeff2",
                &format_args!("{}", self.d_inepnakeff2().bit()),
            )
            .field("d_txfemp2", &format_args!("{}", self.d_txfemp2().bit()))
            .field(
                "d_txfifoundrn2",
                &format_args!("{}", self.d_txfifoundrn2().bit()),
            )
            .field("d_bnaintr2", &format_args!("{}", self.d_bnaintr2().bit()))
            .field(
                "d_pktdrpsts2",
                &format_args!("{}", self.d_pktdrpsts2().bit()),
            )
            .field("d_bbleerr2", &format_args!("{}", self.d_bbleerr2().bit()))
            .field(
                "d_nakintrpt2",
                &format_args!("{}", self.d_nakintrpt2().bit()),
            )
            .field(
                "d_nyetintrpt2",
                &format_args!("{}", self.d_nyetintrpt2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPINT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl2(&mut self) -> D_XFERCOMPL2_W<0> {
        D_XFERCOMPL2_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld2(&mut self) -> D_EPDISBLD2_W<1> {
        D_EPDISBLD2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr2(&mut self) -> D_AHBERR2_W<2> {
        D_AHBERR2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout2(&mut self) -> D_TIMEOUT2_W<3> {
        D_TIMEOUT2_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp2(&mut self) -> D_INTKNTXFEMP2_W<4> {
        D_INTKNTXFEMP2_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis2(&mut self) -> D_INTKNEPMIS2_W<5> {
        D_INTKNEPMIS2_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff2(&mut self) -> D_INEPNAKEFF2_W<6> {
        D_INEPNAKEFF2_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn2(&mut self) -> D_TXFIFOUNDRN2_W<8> {
        D_TXFIFOUNDRN2_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr2(&mut self) -> D_BNAINTR2_W<9> {
        D_BNAINTR2_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts2(&mut self) -> D_PKTDRPSTS2_W<11> {
        D_PKTDRPSTS2_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr2(&mut self) -> D_BBLEERR2_W<12> {
        D_BBLEERR2_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt2(&mut self) -> D_NAKINTRPT2_W<13> {
        D_NAKINTRPT2_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt2(&mut self) -> D_NYETINTRPT2_W<14> {
        D_NYETINTRPT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint2](index.html) module"]
pub struct DIEPINT2_SPEC;
impl crate::RegisterSpec for DIEPINT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint2::R](R) reader structure"]
impl crate::Readable for DIEPINT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint2::W](W) writer structure"]
impl crate::Writable for DIEPINT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT2 to value 0"]
impl crate::Resettable for DIEPINT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
