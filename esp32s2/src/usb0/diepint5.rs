#[doc = "Register `DIEPINT5` reader"]
pub struct R(crate::R<DIEPINT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT5` writer"]
pub struct W(crate::W<DIEPINT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT5_SPEC>;
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
impl From<crate::W<DIEPINT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERCOMPL5` reader - "]
pub type D_XFERCOMPL5_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL5` writer - "]
pub type D_XFERCOMPL5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_EPDISBLD5` reader - "]
pub type D_EPDISBLD5_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD5` writer - "]
pub type D_EPDISBLD5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_AHBERR5` reader - "]
pub type D_AHBERR5_R = crate::BitReader;
#[doc = "Field `D_AHBERR5` writer - "]
pub type D_AHBERR5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_TIMEOUT5` reader - "]
pub type D_TIMEOUT5_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT5` writer - "]
pub type D_TIMEOUT5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_INTKNTXFEMP5` reader - "]
pub type D_INTKNTXFEMP5_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP5` writer - "]
pub type D_INTKNTXFEMP5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_INTKNEPMIS5` reader - "]
pub type D_INTKNEPMIS5_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS5` writer - "]
pub type D_INTKNEPMIS5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_INEPNAKEFF5` reader - "]
pub type D_INEPNAKEFF5_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF5` writer - "]
pub type D_INEPNAKEFF5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_TXFEMP5` reader - "]
pub type D_TXFEMP5_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN5` reader - "]
pub type D_TXFIFOUNDRN5_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN5` writer - "]
pub type D_TXFIFOUNDRN5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_BNAINTR5` reader - "]
pub type D_BNAINTR5_R = crate::BitReader;
#[doc = "Field `D_BNAINTR5` writer - "]
pub type D_BNAINTR5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_PKTDRPSTS5` reader - "]
pub type D_PKTDRPSTS5_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS5` writer - "]
pub type D_PKTDRPSTS5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_BBLEERR5` reader - "]
pub type D_BBLEERR5_R = crate::BitReader;
#[doc = "Field `D_BBLEERR5` writer - "]
pub type D_BBLEERR5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_NAKINTRPT5` reader - "]
pub type D_NAKINTRPT5_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT5` writer - "]
pub type D_NAKINTRPT5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
#[doc = "Field `D_NYETINTRPT5` reader - "]
pub type D_NYETINTRPT5_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT5` writer - "]
pub type D_NYETINTRPT5_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT5_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn d_xfercompl5(&self) -> D_XFERCOMPL5_R {
        D_XFERCOMPL5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn d_epdisbld5(&self) -> D_EPDISBLD5_R {
        D_EPDISBLD5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn d_ahberr5(&self) -> D_AHBERR5_R {
        D_AHBERR5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn d_timeout5(&self) -> D_TIMEOUT5_R {
        D_TIMEOUT5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn d_intkntxfemp5(&self) -> D_INTKNTXFEMP5_R {
        D_INTKNTXFEMP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_intknepmis5(&self) -> D_INTKNEPMIS5_R {
        D_INTKNEPMIS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn d_inepnakeff5(&self) -> D_INEPNAKEFF5_R {
        D_INEPNAKEFF5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn d_txfemp5(&self) -> D_TXFEMP5_R {
        D_TXFEMP5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn d_txfifoundrn5(&self) -> D_TXFIFOUNDRN5_R {
        D_TXFIFOUNDRN5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn d_bnaintr5(&self) -> D_BNAINTR5_R {
        D_BNAINTR5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn d_pktdrpsts5(&self) -> D_PKTDRPSTS5_R {
        D_PKTDRPSTS5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn d_bbleerr5(&self) -> D_BBLEERR5_R {
        D_BBLEERR5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn d_nakintrpt5(&self) -> D_NAKINTRPT5_R {
        D_NAKINTRPT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn d_nyetintrpt5(&self) -> D_NYETINTRPT5_R {
        D_NYETINTRPT5_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT5")
            .field(
                "d_xfercompl5",
                &format_args!("{}", self.d_xfercompl5().bit()),
            )
            .field("d_epdisbld5", &format_args!("{}", self.d_epdisbld5().bit()))
            .field("d_ahberr5", &format_args!("{}", self.d_ahberr5().bit()))
            .field("d_timeout5", &format_args!("{}", self.d_timeout5().bit()))
            .field(
                "d_intkntxfemp5",
                &format_args!("{}", self.d_intkntxfemp5().bit()),
            )
            .field(
                "d_intknepmis5",
                &format_args!("{}", self.d_intknepmis5().bit()),
            )
            .field(
                "d_inepnakeff5",
                &format_args!("{}", self.d_inepnakeff5().bit()),
            )
            .field("d_txfemp5", &format_args!("{}", self.d_txfemp5().bit()))
            .field(
                "d_txfifoundrn5",
                &format_args!("{}", self.d_txfifoundrn5().bit()),
            )
            .field("d_bnaintr5", &format_args!("{}", self.d_bnaintr5().bit()))
            .field(
                "d_pktdrpsts5",
                &format_args!("{}", self.d_pktdrpsts5().bit()),
            )
            .field("d_bbleerr5", &format_args!("{}", self.d_bbleerr5().bit()))
            .field(
                "d_nakintrpt5",
                &format_args!("{}", self.d_nakintrpt5().bit()),
            )
            .field(
                "d_nyetintrpt5",
                &format_args!("{}", self.d_nyetintrpt5().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPINT5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl5(&mut self) -> D_XFERCOMPL5_W<0> {
        D_XFERCOMPL5_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld5(&mut self) -> D_EPDISBLD5_W<1> {
        D_EPDISBLD5_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr5(&mut self) -> D_AHBERR5_W<2> {
        D_AHBERR5_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout5(&mut self) -> D_TIMEOUT5_W<3> {
        D_TIMEOUT5_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp5(&mut self) -> D_INTKNTXFEMP5_W<4> {
        D_INTKNTXFEMP5_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis5(&mut self) -> D_INTKNEPMIS5_W<5> {
        D_INTKNEPMIS5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff5(&mut self) -> D_INEPNAKEFF5_W<6> {
        D_INEPNAKEFF5_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn5(&mut self) -> D_TXFIFOUNDRN5_W<8> {
        D_TXFIFOUNDRN5_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr5(&mut self) -> D_BNAINTR5_W<9> {
        D_BNAINTR5_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts5(&mut self) -> D_PKTDRPSTS5_W<11> {
        D_PKTDRPSTS5_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr5(&mut self) -> D_BBLEERR5_W<12> {
        D_BBLEERR5_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt5(&mut self) -> D_NAKINTRPT5_W<13> {
        D_NAKINTRPT5_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt5(&mut self) -> D_NYETINTRPT5_W<14> {
        D_NYETINTRPT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint5](index.html) module"]
pub struct DIEPINT5_SPEC;
impl crate::RegisterSpec for DIEPINT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint5::R](R) reader structure"]
impl crate::Readable for DIEPINT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint5::W](W) writer structure"]
impl crate::Writable for DIEPINT5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT5 to value 0"]
impl crate::Resettable for DIEPINT5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
