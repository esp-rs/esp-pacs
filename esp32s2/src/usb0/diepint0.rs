#[doc = "Register `DIEPINT0` reader"]
pub struct R(crate::R<DIEPINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT0` writer"]
pub struct W(crate::W<DIEPINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT0_SPEC>;
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
impl From<crate::W<DIEPINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERCOMPL0` reader - "]
pub type D_XFERCOMPL0_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL0` writer - "]
pub type D_XFERCOMPL0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_EPDISBLD0` reader - "]
pub type D_EPDISBLD0_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD0` writer - "]
pub type D_EPDISBLD0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_AHBERR0` reader - "]
pub type D_AHBERR0_R = crate::BitReader;
#[doc = "Field `D_AHBERR0` writer - "]
pub type D_AHBERR0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_TIMEOUT0` reader - "]
pub type D_TIMEOUT0_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT0` writer - "]
pub type D_TIMEOUT0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_INTKNTXFEMP0` reader - "]
pub type D_INTKNTXFEMP0_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP0` writer - "]
pub type D_INTKNTXFEMP0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_INTKNEPMIS0` reader - "]
pub type D_INTKNEPMIS0_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS0` writer - "]
pub type D_INTKNEPMIS0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_INEPNAKEFF0` reader - "]
pub type D_INEPNAKEFF0_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF0` writer - "]
pub type D_INEPNAKEFF0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_TXFEMP0` reader - "]
pub type D_TXFEMP0_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN0` reader - "]
pub type D_TXFIFOUNDRN0_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN0` writer - "]
pub type D_TXFIFOUNDRN0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_BNAINTR0` reader - "]
pub type D_BNAINTR0_R = crate::BitReader;
#[doc = "Field `D_BNAINTR0` writer - "]
pub type D_BNAINTR0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_PKTDRPSTS0` reader - "]
pub type D_PKTDRPSTS0_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS0` writer - "]
pub type D_PKTDRPSTS0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_BBLEERR0` reader - "]
pub type D_BBLEERR0_R = crate::BitReader;
#[doc = "Field `D_BBLEERR0` writer - "]
pub type D_BBLEERR0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_NAKINTRPT0` reader - "]
pub type D_NAKINTRPT0_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT0` writer - "]
pub type D_NAKINTRPT0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
#[doc = "Field `D_NYETINTRPT0` reader - "]
pub type D_NYETINTRPT0_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT0` writer - "]
pub type D_NYETINTRPT0_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT0_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn d_xfercompl0(&self) -> D_XFERCOMPL0_R {
        D_XFERCOMPL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn d_epdisbld0(&self) -> D_EPDISBLD0_R {
        D_EPDISBLD0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn d_ahberr0(&self) -> D_AHBERR0_R {
        D_AHBERR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn d_timeout0(&self) -> D_TIMEOUT0_R {
        D_TIMEOUT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn d_intkntxfemp0(&self) -> D_INTKNTXFEMP0_R {
        D_INTKNTXFEMP0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_intknepmis0(&self) -> D_INTKNEPMIS0_R {
        D_INTKNEPMIS0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn d_inepnakeff0(&self) -> D_INEPNAKEFF0_R {
        D_INEPNAKEFF0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn d_txfemp0(&self) -> D_TXFEMP0_R {
        D_TXFEMP0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn d_txfifoundrn0(&self) -> D_TXFIFOUNDRN0_R {
        D_TXFIFOUNDRN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn d_bnaintr0(&self) -> D_BNAINTR0_R {
        D_BNAINTR0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn d_pktdrpsts0(&self) -> D_PKTDRPSTS0_R {
        D_PKTDRPSTS0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn d_bbleerr0(&self) -> D_BBLEERR0_R {
        D_BBLEERR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn d_nakintrpt0(&self) -> D_NAKINTRPT0_R {
        D_NAKINTRPT0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn d_nyetintrpt0(&self) -> D_NYETINTRPT0_R {
        D_NYETINTRPT0_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT0")
            .field(
                "d_xfercompl0",
                &format_args!("{}", self.d_xfercompl0().bit()),
            )
            .field("d_epdisbld0", &format_args!("{}", self.d_epdisbld0().bit()))
            .field("d_ahberr0", &format_args!("{}", self.d_ahberr0().bit()))
            .field("d_timeout0", &format_args!("{}", self.d_timeout0().bit()))
            .field(
                "d_intkntxfemp0",
                &format_args!("{}", self.d_intkntxfemp0().bit()),
            )
            .field(
                "d_intknepmis0",
                &format_args!("{}", self.d_intknepmis0().bit()),
            )
            .field(
                "d_inepnakeff0",
                &format_args!("{}", self.d_inepnakeff0().bit()),
            )
            .field("d_txfemp0", &format_args!("{}", self.d_txfemp0().bit()))
            .field(
                "d_txfifoundrn0",
                &format_args!("{}", self.d_txfifoundrn0().bit()),
            )
            .field("d_bnaintr0", &format_args!("{}", self.d_bnaintr0().bit()))
            .field(
                "d_pktdrpsts0",
                &format_args!("{}", self.d_pktdrpsts0().bit()),
            )
            .field("d_bbleerr0", &format_args!("{}", self.d_bbleerr0().bit()))
            .field(
                "d_nakintrpt0",
                &format_args!("{}", self.d_nakintrpt0().bit()),
            )
            .field(
                "d_nyetintrpt0",
                &format_args!("{}", self.d_nyetintrpt0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPINT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl0(&mut self) -> D_XFERCOMPL0_W<0> {
        D_XFERCOMPL0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld0(&mut self) -> D_EPDISBLD0_W<1> {
        D_EPDISBLD0_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr0(&mut self) -> D_AHBERR0_W<2> {
        D_AHBERR0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout0(&mut self) -> D_TIMEOUT0_W<3> {
        D_TIMEOUT0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp0(&mut self) -> D_INTKNTXFEMP0_W<4> {
        D_INTKNTXFEMP0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis0(&mut self) -> D_INTKNEPMIS0_W<5> {
        D_INTKNEPMIS0_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff0(&mut self) -> D_INEPNAKEFF0_W<6> {
        D_INEPNAKEFF0_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn0(&mut self) -> D_TXFIFOUNDRN0_W<8> {
        D_TXFIFOUNDRN0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr0(&mut self) -> D_BNAINTR0_W<9> {
        D_BNAINTR0_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts0(&mut self) -> D_PKTDRPSTS0_W<11> {
        D_PKTDRPSTS0_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr0(&mut self) -> D_BBLEERR0_W<12> {
        D_BBLEERR0_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt0(&mut self) -> D_NAKINTRPT0_W<13> {
        D_NAKINTRPT0_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt0(&mut self) -> D_NYETINTRPT0_W<14> {
        D_NYETINTRPT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint0](index.html) module"]
pub struct DIEPINT0_SPEC;
impl crate::RegisterSpec for DIEPINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint0::R](R) reader structure"]
impl crate::Readable for DIEPINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint0::W](W) writer structure"]
impl crate::Writable for DIEPINT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT0 to value 0"]
impl crate::Resettable for DIEPINT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
