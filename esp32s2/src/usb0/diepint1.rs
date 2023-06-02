#[doc = "Register `DIEPINT1` reader"]
pub struct R(crate::R<DIEPINT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT1` writer"]
pub struct W(crate::W<DIEPINT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT1_SPEC>;
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
impl From<crate::W<DIEPINT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERCOMPL1` reader - "]
pub type D_XFERCOMPL1_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL1` writer - "]
pub type D_XFERCOMPL1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_EPDISBLD1` reader - "]
pub type D_EPDISBLD1_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD1` writer - "]
pub type D_EPDISBLD1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_AHBERR1` reader - "]
pub type D_AHBERR1_R = crate::BitReader;
#[doc = "Field `D_AHBERR1` writer - "]
pub type D_AHBERR1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_TIMEOUT1` reader - "]
pub type D_TIMEOUT1_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT1` writer - "]
pub type D_TIMEOUT1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_INTKNTXFEMP1` reader - "]
pub type D_INTKNTXFEMP1_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP1` writer - "]
pub type D_INTKNTXFEMP1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_INTKNEPMIS1` reader - "]
pub type D_INTKNEPMIS1_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS1` writer - "]
pub type D_INTKNEPMIS1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_INEPNAKEFF1` reader - "]
pub type D_INEPNAKEFF1_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF1` writer - "]
pub type D_INEPNAKEFF1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_TXFEMP1` reader - "]
pub type D_TXFEMP1_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN1` reader - "]
pub type D_TXFIFOUNDRN1_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN1` writer - "]
pub type D_TXFIFOUNDRN1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_BNAINTR1` reader - "]
pub type D_BNAINTR1_R = crate::BitReader;
#[doc = "Field `D_BNAINTR1` writer - "]
pub type D_BNAINTR1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_PKTDRPSTS1` reader - "]
pub type D_PKTDRPSTS1_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS1` writer - "]
pub type D_PKTDRPSTS1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_BBLEERR1` reader - "]
pub type D_BBLEERR1_R = crate::BitReader;
#[doc = "Field `D_BBLEERR1` writer - "]
pub type D_BBLEERR1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_NAKINTRPT1` reader - "]
pub type D_NAKINTRPT1_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT1` writer - "]
pub type D_NAKINTRPT1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
#[doc = "Field `D_NYETINTRPT1` reader - "]
pub type D_NYETINTRPT1_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT1` writer - "]
pub type D_NYETINTRPT1_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT1_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn d_xfercompl1(&self) -> D_XFERCOMPL1_R {
        D_XFERCOMPL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn d_epdisbld1(&self) -> D_EPDISBLD1_R {
        D_EPDISBLD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn d_ahberr1(&self) -> D_AHBERR1_R {
        D_AHBERR1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn d_timeout1(&self) -> D_TIMEOUT1_R {
        D_TIMEOUT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn d_intkntxfemp1(&self) -> D_INTKNTXFEMP1_R {
        D_INTKNTXFEMP1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_intknepmis1(&self) -> D_INTKNEPMIS1_R {
        D_INTKNEPMIS1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn d_inepnakeff1(&self) -> D_INEPNAKEFF1_R {
        D_INEPNAKEFF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn d_txfemp1(&self) -> D_TXFEMP1_R {
        D_TXFEMP1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn d_txfifoundrn1(&self) -> D_TXFIFOUNDRN1_R {
        D_TXFIFOUNDRN1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn d_bnaintr1(&self) -> D_BNAINTR1_R {
        D_BNAINTR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn d_pktdrpsts1(&self) -> D_PKTDRPSTS1_R {
        D_PKTDRPSTS1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn d_bbleerr1(&self) -> D_BBLEERR1_R {
        D_BBLEERR1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn d_nakintrpt1(&self) -> D_NAKINTRPT1_R {
        D_NAKINTRPT1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn d_nyetintrpt1(&self) -> D_NYETINTRPT1_R {
        D_NYETINTRPT1_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT1")
            .field(
                "d_xfercompl1",
                &format_args!("{}", self.d_xfercompl1().bit()),
            )
            .field("d_epdisbld1", &format_args!("{}", self.d_epdisbld1().bit()))
            .field("d_ahberr1", &format_args!("{}", self.d_ahberr1().bit()))
            .field("d_timeout1", &format_args!("{}", self.d_timeout1().bit()))
            .field(
                "d_intkntxfemp1",
                &format_args!("{}", self.d_intkntxfemp1().bit()),
            )
            .field(
                "d_intknepmis1",
                &format_args!("{}", self.d_intknepmis1().bit()),
            )
            .field(
                "d_inepnakeff1",
                &format_args!("{}", self.d_inepnakeff1().bit()),
            )
            .field("d_txfemp1", &format_args!("{}", self.d_txfemp1().bit()))
            .field(
                "d_txfifoundrn1",
                &format_args!("{}", self.d_txfifoundrn1().bit()),
            )
            .field("d_bnaintr1", &format_args!("{}", self.d_bnaintr1().bit()))
            .field(
                "d_pktdrpsts1",
                &format_args!("{}", self.d_pktdrpsts1().bit()),
            )
            .field("d_bbleerr1", &format_args!("{}", self.d_bbleerr1().bit()))
            .field(
                "d_nakintrpt1",
                &format_args!("{}", self.d_nakintrpt1().bit()),
            )
            .field(
                "d_nyetintrpt1",
                &format_args!("{}", self.d_nyetintrpt1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPINT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl1(&mut self) -> D_XFERCOMPL1_W<0> {
        D_XFERCOMPL1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld1(&mut self) -> D_EPDISBLD1_W<1> {
        D_EPDISBLD1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr1(&mut self) -> D_AHBERR1_W<2> {
        D_AHBERR1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout1(&mut self) -> D_TIMEOUT1_W<3> {
        D_TIMEOUT1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp1(&mut self) -> D_INTKNTXFEMP1_W<4> {
        D_INTKNTXFEMP1_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis1(&mut self) -> D_INTKNEPMIS1_W<5> {
        D_INTKNEPMIS1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff1(&mut self) -> D_INEPNAKEFF1_W<6> {
        D_INEPNAKEFF1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn1(&mut self) -> D_TXFIFOUNDRN1_W<8> {
        D_TXFIFOUNDRN1_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr1(&mut self) -> D_BNAINTR1_W<9> {
        D_BNAINTR1_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts1(&mut self) -> D_PKTDRPSTS1_W<11> {
        D_PKTDRPSTS1_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr1(&mut self) -> D_BBLEERR1_W<12> {
        D_BBLEERR1_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt1(&mut self) -> D_NAKINTRPT1_W<13> {
        D_NAKINTRPT1_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt1(&mut self) -> D_NYETINTRPT1_W<14> {
        D_NYETINTRPT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint1](index.html) module"]
pub struct DIEPINT1_SPEC;
impl crate::RegisterSpec for DIEPINT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint1::R](R) reader structure"]
impl crate::Readable for DIEPINT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint1::W](W) writer structure"]
impl crate::Writable for DIEPINT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT1 to value 0"]
impl crate::Resettable for DIEPINT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
