#[doc = "Register `GRSTCTL` reader"]
pub struct R(crate::R<GRSTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GRSTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GRSTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GRSTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GRSTCTL` writer"]
pub struct W(crate::W<GRSTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GRSTCTL_SPEC>;
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
impl From<crate::W<GRSTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GRSTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSFTRST` reader - "]
pub type CSFTRST_R = crate::BitReader;
#[doc = "Field `CSFTRST` writer - "]
pub type CSFTRST_W<'a, const O: u8> = crate::BitWriter<'a, GRSTCTL_SPEC, O>;
#[doc = "Field `PIUFSSFTRST` reader - "]
pub type PIUFSSFTRST_R = crate::BitReader;
#[doc = "Field `PIUFSSFTRST` writer - "]
pub type PIUFSSFTRST_W<'a, const O: u8> = crate::BitWriter<'a, GRSTCTL_SPEC, O>;
#[doc = "Field `FRMCNTRRST` reader - "]
pub type FRMCNTRRST_R = crate::BitReader;
#[doc = "Field `FRMCNTRRST` writer - "]
pub type FRMCNTRRST_W<'a, const O: u8> = crate::BitWriter<'a, GRSTCTL_SPEC, O>;
#[doc = "Field `RXFFLSH` reader - "]
pub type RXFFLSH_R = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - "]
pub type RXFFLSH_W<'a, const O: u8> = crate::BitWriter<'a, GRSTCTL_SPEC, O>;
#[doc = "Field `TXFFLSH` reader - "]
pub type TXFFLSH_R = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - "]
pub type TXFFLSH_W<'a, const O: u8> = crate::BitWriter<'a, GRSTCTL_SPEC, O>;
#[doc = "Field `TXFNUM` reader - "]
pub type TXFNUM_R = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - "]
pub type TXFNUM_W<'a, const O: u8> = crate::FieldWriter<'a, GRSTCTL_SPEC, 5, O>;
#[doc = "Field `DMAREQ` reader - "]
pub type DMAREQ_R = crate::BitReader;
#[doc = "Field `AHBIDLE` reader - "]
pub type AHBIDLE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn csftrst(&self) -> CSFTRST_R {
        CSFTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn piufssftrst(&self) -> PIUFSSFTRST_R {
        PIUFSSFTRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn frmcntrrst(&self) -> FRMCNTRRST_R {
        FRMCNTRRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RXFFLSH_R {
        RXFFLSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TXFFLSH_R {
        TXFFLSH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn dmareq(&self) -> DMAREQ_R {
        DMAREQ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AHBIDLE_R {
        AHBIDLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csftrst", &format_args!("{}", self.csftrst().bit()))
            .field("piufssftrst", &format_args!("{}", self.piufssftrst().bit()))
            .field("frmcntrrst", &format_args!("{}", self.frmcntrrst().bit()))
            .field("rxfflsh", &format_args!("{}", self.rxfflsh().bit()))
            .field("txfflsh", &format_args!("{}", self.txfflsh().bit()))
            .field("txfnum", &format_args!("{}", self.txfnum().bits()))
            .field("dmareq", &format_args!("{}", self.dmareq().bit()))
            .field("ahbidle", &format_args!("{}", self.ahbidle().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GRSTCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn csftrst(&mut self) -> CSFTRST_W<0> {
        CSFTRST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn piufssftrst(&mut self) -> PIUFSSFTRST_W<1> {
        PIUFSSFTRST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn frmcntrrst(&mut self) -> FRMCNTRRST_W<2> {
        FRMCNTRRST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RXFFLSH_W<4> {
        RXFFLSH_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TXFFLSH_W<5> {
        TXFFLSH_W::new(self)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TXFNUM_W<6> {
        TXFNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstctl](index.html) module"]
pub struct GRSTCTL_SPEC;
impl crate::RegisterSpec for GRSTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [grstctl::R](R) reader structure"]
impl crate::Readable for GRSTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [grstctl::W](W) writer structure"]
impl crate::Writable for GRSTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0"]
impl crate::Resettable for GRSTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
