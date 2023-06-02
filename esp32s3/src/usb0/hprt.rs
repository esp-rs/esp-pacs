#[doc = "Register `HPRT` reader"]
pub struct R(crate::R<HPRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPRT` writer"]
pub struct W(crate::W<HPRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPRT_SPEC>;
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
impl From<crate::W<HPRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRTCONNSTS` reader - "]
pub type PRTCONNSTS_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` reader - "]
pub type PRTCONNDET_R = crate::BitReader;
#[doc = "Field `PRTCONNDET` writer - "]
pub type PRTCONNDET_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTENA` reader - "]
pub type PRTENA_R = crate::BitReader;
#[doc = "Field `PRTENA` writer - "]
pub type PRTENA_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTENCHNG` reader - "]
pub type PRTENCHNG_R = crate::BitReader;
#[doc = "Field `PRTENCHNG` writer - "]
pub type PRTENCHNG_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTOVRCURRACT` reader - "]
pub type PRTOVRCURRACT_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` reader - "]
pub type PRTOVRCURRCHNG_R = crate::BitReader;
#[doc = "Field `PRTOVRCURRCHNG` writer - "]
pub type PRTOVRCURRCHNG_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTRES` reader - "]
pub type PRTRES_R = crate::BitReader;
#[doc = "Field `PRTRES` writer - "]
pub type PRTRES_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTSUSP` reader - "]
pub type PRTSUSP_R = crate::BitReader;
#[doc = "Field `PRTSUSP` writer - "]
pub type PRTSUSP_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTRST` reader - "]
pub type PRTRST_R = crate::BitReader;
#[doc = "Field `PRTRST` writer - "]
pub type PRTRST_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTLNSTS` reader - "]
pub type PRTLNSTS_R = crate::FieldReader;
#[doc = "Field `PRTPWR` reader - "]
pub type PRTPWR_R = crate::BitReader;
#[doc = "Field `PRTPWR` writer - "]
pub type PRTPWR_W<'a, const O: u8> = crate::BitWriter<'a, HPRT_SPEC, O>;
#[doc = "Field `PRTTSTCTL` reader - "]
pub type PRTTSTCTL_R = crate::FieldReader;
#[doc = "Field `PRTTSTCTL` writer - "]
pub type PRTTSTCTL_W<'a, const O: u8> = crate::FieldWriter<'a, HPRT_SPEC, 4, O>;
#[doc = "Field `PRTSPD` reader - "]
pub type PRTSPD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn prtconnsts(&self) -> PRTCONNSTS_R {
        PRTCONNSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn prtconndet(&self) -> PRTCONNDET_R {
        PRTCONNDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn prtena(&self) -> PRTENA_R {
        PRTENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PRTENCHNG_R {
        PRTENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prtovrcurract(&self) -> PRTOVRCURRACT_R {
        PRTOVRCURRACT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn prtovrcurrchng(&self) -> PRTOVRCURRCHNG_R {
        PRTOVRCURRCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn prtres(&self) -> PRTRES_R {
        PRTRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PRTSUSP_R {
        PRTSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn prtrst(&self) -> PRTRST_R {
        PRTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PRTLNSTS_R {
        PRTLNSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PRTPWR_R {
        PRTPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PRTTSTCTL_R {
        PRTTSTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn prtspd(&self) -> PRTSPD_R {
        PRTSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("prtconnsts", &format_args!("{}", self.prtconnsts().bit()))
            .field("prtconndet", &format_args!("{}", self.prtconndet().bit()))
            .field("prtena", &format_args!("{}", self.prtena().bit()))
            .field("prtenchng", &format_args!("{}", self.prtenchng().bit()))
            .field(
                "prtovrcurract",
                &format_args!("{}", self.prtovrcurract().bit()),
            )
            .field(
                "prtovrcurrchng",
                &format_args!("{}", self.prtovrcurrchng().bit()),
            )
            .field("prtres", &format_args!("{}", self.prtres().bit()))
            .field("prtsusp", &format_args!("{}", self.prtsusp().bit()))
            .field("prtrst", &format_args!("{}", self.prtrst().bit()))
            .field("prtlnsts", &format_args!("{}", self.prtlnsts().bits()))
            .field("prtpwr", &format_args!("{}", self.prtpwr().bit()))
            .field("prttstctl", &format_args!("{}", self.prttstctl().bits()))
            .field("prtspd", &format_args!("{}", self.prtspd().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HPRT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn prtconndet(&mut self) -> PRTCONNDET_W<1> {
        PRTCONNDET_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn prtena(&mut self) -> PRTENA_W<2> {
        PRTENA_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn prtenchng(&mut self) -> PRTENCHNG_W<3> {
        PRTENCHNG_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn prtovrcurrchng(&mut self) -> PRTOVRCURRCHNG_W<5> {
        PRTOVRCURRCHNG_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn prtres(&mut self) -> PRTRES_W<6> {
        PRTRES_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn prtsusp(&mut self) -> PRTSUSP_W<7> {
        PRTSUSP_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn prtrst(&mut self) -> PRTRST_W<8> {
        PRTRST_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn prtpwr(&mut self) -> PRTPWR_W<12> {
        PRTPWR_W::new(self)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    #[must_use]
    pub fn prttstctl(&mut self) -> PRTTSTCTL_W<13> {
        PRTTSTCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprt](index.html) module"]
pub struct HPRT_SPEC;
impl crate::RegisterSpec for HPRT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hprt::R](R) reader structure"]
impl crate::Readable for HPRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hprt::W](W) writer structure"]
impl crate::Writable for HPRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HPRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
