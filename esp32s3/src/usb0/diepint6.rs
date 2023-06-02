#[doc = "Register `DIEPINT6` reader"]
pub struct R(crate::R<DIEPINT6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPINT6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPINT6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPINT6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPINT6` writer"]
pub struct W(crate::W<DIEPINT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPINT6_SPEC>;
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
impl From<crate::W<DIEPINT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPINT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_XFERCOMPL6` reader - "]
pub type D_XFERCOMPL6_R = crate::BitReader;
#[doc = "Field `D_XFERCOMPL6` writer - "]
pub type D_XFERCOMPL6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_EPDISBLD6` reader - "]
pub type D_EPDISBLD6_R = crate::BitReader;
#[doc = "Field `D_EPDISBLD6` writer - "]
pub type D_EPDISBLD6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_AHBERR6` reader - "]
pub type D_AHBERR6_R = crate::BitReader;
#[doc = "Field `D_AHBERR6` writer - "]
pub type D_AHBERR6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_TIMEOUT6` reader - "]
pub type D_TIMEOUT6_R = crate::BitReader;
#[doc = "Field `D_TIMEOUT6` writer - "]
pub type D_TIMEOUT6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_INTKNTXFEMP6` reader - "]
pub type D_INTKNTXFEMP6_R = crate::BitReader;
#[doc = "Field `D_INTKNTXFEMP6` writer - "]
pub type D_INTKNTXFEMP6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_INTKNEPMIS6` reader - "]
pub type D_INTKNEPMIS6_R = crate::BitReader;
#[doc = "Field `D_INTKNEPMIS6` writer - "]
pub type D_INTKNEPMIS6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_INEPNAKEFF6` reader - "]
pub type D_INEPNAKEFF6_R = crate::BitReader;
#[doc = "Field `D_INEPNAKEFF6` writer - "]
pub type D_INEPNAKEFF6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_TXFEMP6` reader - "]
pub type D_TXFEMP6_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN6` reader - "]
pub type D_TXFIFOUNDRN6_R = crate::BitReader;
#[doc = "Field `D_TXFIFOUNDRN6` writer - "]
pub type D_TXFIFOUNDRN6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_BNAINTR6` reader - "]
pub type D_BNAINTR6_R = crate::BitReader;
#[doc = "Field `D_BNAINTR6` writer - "]
pub type D_BNAINTR6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_PKTDRPSTS6` reader - "]
pub type D_PKTDRPSTS6_R = crate::BitReader;
#[doc = "Field `D_PKTDRPSTS6` writer - "]
pub type D_PKTDRPSTS6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_BBLEERR6` reader - "]
pub type D_BBLEERR6_R = crate::BitReader;
#[doc = "Field `D_BBLEERR6` writer - "]
pub type D_BBLEERR6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_NAKINTRPT6` reader - "]
pub type D_NAKINTRPT6_R = crate::BitReader;
#[doc = "Field `D_NAKINTRPT6` writer - "]
pub type D_NAKINTRPT6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
#[doc = "Field `D_NYETINTRPT6` reader - "]
pub type D_NYETINTRPT6_R = crate::BitReader;
#[doc = "Field `D_NYETINTRPT6` writer - "]
pub type D_NYETINTRPT6_W<'a, const O: u8> = crate::BitWriter<'a, DIEPINT6_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn d_xfercompl6(&self) -> D_XFERCOMPL6_R {
        D_XFERCOMPL6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn d_epdisbld6(&self) -> D_EPDISBLD6_R {
        D_EPDISBLD6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn d_ahberr6(&self) -> D_AHBERR6_R {
        D_AHBERR6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn d_timeout6(&self) -> D_TIMEOUT6_R {
        D_TIMEOUT6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn d_intkntxfemp6(&self) -> D_INTKNTXFEMP6_R {
        D_INTKNTXFEMP6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn d_intknepmis6(&self) -> D_INTKNEPMIS6_R {
        D_INTKNEPMIS6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn d_inepnakeff6(&self) -> D_INEPNAKEFF6_R {
        D_INEPNAKEFF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn d_txfemp6(&self) -> D_TXFEMP6_R {
        D_TXFEMP6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn d_txfifoundrn6(&self) -> D_TXFIFOUNDRN6_R {
        D_TXFIFOUNDRN6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn d_bnaintr6(&self) -> D_BNAINTR6_R {
        D_BNAINTR6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn d_pktdrpsts6(&self) -> D_PKTDRPSTS6_R {
        D_PKTDRPSTS6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn d_bbleerr6(&self) -> D_BBLEERR6_R {
        D_BBLEERR6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn d_nakintrpt6(&self) -> D_NAKINTRPT6_R {
        D_NAKINTRPT6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn d_nyetintrpt6(&self) -> D_NYETINTRPT6_R {
        D_NYETINTRPT6_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT6")
            .field(
                "d_xfercompl6",
                &format_args!("{}", self.d_xfercompl6().bit()),
            )
            .field("d_epdisbld6", &format_args!("{}", self.d_epdisbld6().bit()))
            .field("d_ahberr6", &format_args!("{}", self.d_ahberr6().bit()))
            .field("d_timeout6", &format_args!("{}", self.d_timeout6().bit()))
            .field(
                "d_intkntxfemp6",
                &format_args!("{}", self.d_intkntxfemp6().bit()),
            )
            .field(
                "d_intknepmis6",
                &format_args!("{}", self.d_intknepmis6().bit()),
            )
            .field(
                "d_inepnakeff6",
                &format_args!("{}", self.d_inepnakeff6().bit()),
            )
            .field("d_txfemp6", &format_args!("{}", self.d_txfemp6().bit()))
            .field(
                "d_txfifoundrn6",
                &format_args!("{}", self.d_txfifoundrn6().bit()),
            )
            .field("d_bnaintr6", &format_args!("{}", self.d_bnaintr6().bit()))
            .field(
                "d_pktdrpsts6",
                &format_args!("{}", self.d_pktdrpsts6().bit()),
            )
            .field("d_bbleerr6", &format_args!("{}", self.d_bbleerr6().bit()))
            .field(
                "d_nakintrpt6",
                &format_args!("{}", self.d_nakintrpt6().bit()),
            )
            .field(
                "d_nyetintrpt6",
                &format_args!("{}", self.d_nyetintrpt6().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPINT6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn d_xfercompl6(&mut self) -> D_XFERCOMPL6_W<0> {
        D_XFERCOMPL6_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn d_epdisbld6(&mut self) -> D_EPDISBLD6_W<1> {
        D_EPDISBLD6_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn d_ahberr6(&mut self) -> D_AHBERR6_W<2> {
        D_AHBERR6_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn d_timeout6(&mut self) -> D_TIMEOUT6_W<3> {
        D_TIMEOUT6_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn d_intkntxfemp6(&mut self) -> D_INTKNTXFEMP6_W<4> {
        D_INTKNTXFEMP6_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn d_intknepmis6(&mut self) -> D_INTKNEPMIS6_W<5> {
        D_INTKNEPMIS6_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn d_inepnakeff6(&mut self) -> D_INEPNAKEFF6_W<6> {
        D_INEPNAKEFF6_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn d_txfifoundrn6(&mut self) -> D_TXFIFOUNDRN6_W<8> {
        D_TXFIFOUNDRN6_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn d_bnaintr6(&mut self) -> D_BNAINTR6_W<9> {
        D_BNAINTR6_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn d_pktdrpsts6(&mut self) -> D_PKTDRPSTS6_W<11> {
        D_PKTDRPSTS6_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn d_bbleerr6(&mut self) -> D_BBLEERR6_W<12> {
        D_BBLEERR6_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn d_nakintrpt6(&mut self) -> D_NAKINTRPT6_W<13> {
        D_NAKINTRPT6_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn d_nyetintrpt6(&mut self) -> D_NYETINTRPT6_W<14> {
        D_NYETINTRPT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepint6](index.html) module"]
pub struct DIEPINT6_SPEC;
impl crate::RegisterSpec for DIEPINT6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diepint6::R](R) reader structure"]
impl crate::Readable for DIEPINT6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diepint6::W](W) writer structure"]
impl crate::Writable for DIEPINT6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPINT6 to value 0"]
impl crate::Resettable for DIEPINT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
