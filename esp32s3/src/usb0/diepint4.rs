#[doc = "Register `DIEPINT4` reader"]
pub struct R(crate::R<DIEPINT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT4` writer"]
pub struct W(crate::W<DIEPINT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT4_SPEC>;
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
impl From<crate::W<DIEPINT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERCOMPL4` reader - "]
pub type D_XFERCOMPL4_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL4` writer - "]
pub type D_XFERCOMPL4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_EPDISBLD4` reader - "]
pub type D_EPDISBLD4_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD4` writer - "]
pub type D_EPDISBLD4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_AHBERR4` reader - "]
pub type D_AHBERR4_R = crate::BitReader;
#[doc = "Field `D_AHBERR4` writer - "]
pub type D_AHBERR4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_TIMEOUT4` reader - "]
pub type D_TIMEOUT4_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT4` writer - "]
pub type D_TIMEOUT4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_INTKNTXFEMP4` reader - "]
pub type D_INTKNTXFEMP4_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP4` writer - "]
pub type D_INTKNTXFEMP4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_INTKNEPMIS4` reader - "]
pub type D_INTKNEPMIS4_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS4` writer - "]
pub type D_INTKNEPMIS4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_INEPNAKEFF4` reader - "]
pub type D_INEPNAKEFF4_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF4` writer - "]
pub type D_INEPNAKEFF4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_TXFEMP4` reader - "]
pub type D_TXFEMP4_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN4` reader - "]
pub type D_TXFIFOUNDRN4_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN4` writer - "]
pub type D_TXFIFOUNDRN4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_BNAINTR4` reader - "]
pub type D_BNAINTR4_R = crate::BitReader;
#[doc = "Field `D_BNAINTR4` writer - "]
pub type D_BNAINTR4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_PKTDRPSTS4` reader - "]
pub type D_PKTDRPSTS4_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS4` writer - "]
pub type D_PKTDRPSTS4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_BBLEERR4` reader - "]
pub type D_BBLEERR4_R = crate::BitReader;
#[doc = "Field `D_BBLEERR4` writer - "]
pub type D_BBLEERR4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_NAKINTRPT4` reader - "]
pub type D_NAKINTRPT4_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT4` writer - "]
pub type D_NAKINTRPT4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
#[doc = "Field `D_NYETINTRPT4` reader - "]
pub type D_NYETINTRPT4_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT4` writer - "]
pub type D_NYETINTRPT4_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT4_SPEC, O>;
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl4(&mut self) -> D_XFERCOMPL4_W<0> {
        D_XFERCOMPL4_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld4(&mut self) -> D_EPDISBLD4_W<1> {
        D_EPDISBLD4_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr4(&mut self) -> D_AHBERR4_W<2> {
        D_AHBERR4_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout4(&mut self) -> D_TIMEOUT4_W<3> {
        D_TIMEOUT4_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp4(&mut self) -> D_INTKNTXFEMP4_W<4> {
        D_INTKNTXFEMP4_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis4(&mut self) -> D_INTKNEPMIS4_W<5> {
        D_INTKNEPMIS4_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff4(&mut self) -> D_INEPNAKEFF4_W<6> {
        D_INEPNAKEFF4_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn4(&mut self) -> D_TXFIFOUNDRN4_W<8> {
        D_TXFIFOUNDRN4_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr4(&mut self) -> D_BNAINTR4_W<9> {
        D_BNAINTR4_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts4(&mut self) -> D_PKTDRPSTS4_W<11> {
        D_PKTDRPSTS4_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr4(&mut self) -> D_BBLEERR4_W<12> {
        D_BBLEERR4_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt4(&mut self) -> D_NAKINTRPT4_W<13> {
        D_NAKINTRPT4_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt4(&mut self) -> D_NYETINTRPT4_W<14> {
        D_NYETINTRPT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint4](index.html) module"]
pub struct DIEPINT4_SPEC;
impl crate::RegisterSpec for DIEPINT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint4::R](R) reader structure"]
impl crate::Readable for DIEPINT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint4::W](W) writer structure"]
impl crate::Writable for DIEPINT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT4 to value 0"]
impl crate::Resettable for DIEPINT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
