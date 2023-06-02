#[doc = "Register `GOTGCTL` reader"]
pub struct R(crate::R<GOTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GOTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GOTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GOTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GOTGCTL` writer"]
pub struct W(crate::W<GOTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GOTGCTL_SPEC>;
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
impl From<crate::W<GOTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GOTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SESREQSCS` reader - "]
pub type SESREQSCS_R = crate::BitReader;
#[doc = "Field `SESREQ` reader - "]
pub type SESREQ_R = crate::BitReader;
#[doc = "Field `SESREQ` writer - "]
pub type SESREQ_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `VBVALIDOVEN` reader - "]
pub type VBVALIDOVEN_R = crate::BitReader;
#[doc = "Field `VBVALIDOVEN` writer - "]
pub type VBVALIDOVEN_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `VBVALIDOVVAL` reader - "]
pub type VBVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `VBVALIDOVVAL` writer - "]
pub type VBVALIDOVVAL_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `AVALIDOVEN` reader - "]
pub type AVALIDOVEN_R = crate::BitReader;
#[doc = "Field `AVALIDOVEN` writer - "]
pub type AVALIDOVEN_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `AVALIDOVVAL` reader - "]
pub type AVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `AVALIDOVVAL` writer - "]
pub type AVALIDOVVAL_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `BVALIDOVEN` reader - "]
pub type BVALIDOVEN_R = crate::BitReader;
#[doc = "Field `BVALIDOVEN` writer - "]
pub type BVALIDOVEN_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `BVALIDOVVAL` reader - "]
pub type BVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `BVALIDOVVAL` writer - "]
pub type BVALIDOVVAL_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `HSTNEGSCS` reader - "]
pub type HSTNEGSCS_R = crate::BitReader;
#[doc = "Field `HNPREQ` reader - "]
pub type HNPREQ_R = crate::BitReader;
#[doc = "Field `HNPREQ` writer - "]
pub type HNPREQ_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `HSTSETHNPEN` reader - "]
pub type HSTSETHNPEN_R = crate::BitReader;
#[doc = "Field `HSTSETHNPEN` writer - "]
pub type HSTSETHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `DEVHNPEN` reader - "]
pub type DEVHNPEN_R = crate::BitReader;
#[doc = "Field `DEVHNPEN` writer - "]
pub type DEVHNPEN_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `EHEN` reader - "]
pub type EHEN_R = crate::BitReader;
#[doc = "Field `EHEN` writer - "]
pub type EHEN_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `DBNCEFLTRBYPASS` reader - "]
pub type DBNCEFLTRBYPASS_R = crate::BitReader;
#[doc = "Field `DBNCEFLTRBYPASS` writer - "]
pub type DBNCEFLTRBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `CONIDSTS` reader - "]
pub type CONIDSTS_R = crate::BitReader;
#[doc = "Field `DBNCTIME` reader - "]
pub type DBNCTIME_R = crate::BitReader;
#[doc = "Field `ASESVLD` reader - "]
pub type ASESVLD_R = crate::BitReader;
#[doc = "Field `BSESVLD` reader - "]
pub type BSESVLD_R = crate::BitReader;
#[doc = "Field `OTGVER` reader - "]
pub type OTGVER_R = crate::BitReader;
#[doc = "Field `OTGVER` writer - "]
pub type OTGVER_W<'a, const O: u8> = crate::BitWriter<'a, GOTGCTL_SPEC, O>;
#[doc = "Field `CURMOD` reader - "]
pub type CURMOD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sesreqscs(&self) -> SESREQSCS_R {
        SESREQSCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sesreq(&self) -> SESREQ_R {
        SESREQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn vbvalidoven(&self) -> VBVALIDOVEN_R {
        VBVALIDOVEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn vbvalidovval(&self) -> VBVALIDOVVAL_R {
        VBVALIDOVVAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn avalidoven(&self) -> AVALIDOVEN_R {
        AVALIDOVEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn avalidovval(&self) -> AVALIDOVVAL_R {
        AVALIDOVVAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bvalidoven(&self) -> BVALIDOVEN_R {
        BVALIDOVEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bvalidovval(&self) -> BVALIDOVVAL_R {
        BVALIDOVVAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hstnegscs(&self) -> HSTNEGSCS_R {
        HSTNEGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hnpreq(&self) -> HNPREQ_R {
        HNPREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn hstsethnpen(&self) -> HSTSETHNPEN_R {
        HSTSETHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn devhnpen(&self) -> DEVHNPEN_R {
        DEVHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ehen(&self) -> EHEN_R {
        EHEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dbncefltrbypass(&self) -> DBNCEFLTRBYPASS_R {
        DBNCEFLTRBYPASS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn conidsts(&self) -> CONIDSTS_R {
        CONIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn dbnctime(&self) -> DBNCTIME_R {
        DBNCTIME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn asesvld(&self) -> ASESVLD_R {
        ASESVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn bsesvld(&self) -> BSESVLD_R {
        BSESVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn otgver(&self) -> OTGVER_R {
        OTGVER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGCTL")
            .field("sesreqscs", &format_args!("{}", self.sesreqscs().bit()))
            .field("sesreq", &format_args!("{}", self.sesreq().bit()))
            .field("vbvalidoven", &format_args!("{}", self.vbvalidoven().bit()))
            .field(
                "vbvalidovval",
                &format_args!("{}", self.vbvalidovval().bit()),
            )
            .field("avalidoven", &format_args!("{}", self.avalidoven().bit()))
            .field("avalidovval", &format_args!("{}", self.avalidovval().bit()))
            .field("bvalidoven", &format_args!("{}", self.bvalidoven().bit()))
            .field("bvalidovval", &format_args!("{}", self.bvalidovval().bit()))
            .field("hstnegscs", &format_args!("{}", self.hstnegscs().bit()))
            .field("hnpreq", &format_args!("{}", self.hnpreq().bit()))
            .field("hstsethnpen", &format_args!("{}", self.hstsethnpen().bit()))
            .field("devhnpen", &format_args!("{}", self.devhnpen().bit()))
            .field("ehen", &format_args!("{}", self.ehen().bit()))
            .field(
                "dbncefltrbypass",
                &format_args!("{}", self.dbncefltrbypass().bit()),
            )
            .field("conidsts", &format_args!("{}", self.conidsts().bit()))
            .field("dbnctime", &format_args!("{}", self.dbnctime().bit()))
            .field("asesvld", &format_args!("{}", self.asesvld().bit()))
            .field("bsesvld", &format_args!("{}", self.bsesvld().bit()))
            .field("otgver", &format_args!("{}", self.otgver().bit()))
            .field("curmod", &format_args!("{}", self.curmod().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GOTGCTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sesreq(&mut self) -> SESREQ_W<1> {
        SESREQ_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalidoven(&mut self) -> VBVALIDOVEN_W<2> {
        VBVALIDOVEN_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalidovval(&mut self) -> VBVALIDOVVAL_W<3> {
        VBVALIDOVVAL_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn avalidoven(&mut self) -> AVALIDOVEN_W<4> {
        AVALIDOVEN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn avalidovval(&mut self) -> AVALIDOVVAL_W<5> {
        AVALIDOVVAL_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bvalidoven(&mut self) -> BVALIDOVEN_W<6> {
        BVALIDOVEN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bvalidovval(&mut self) -> BVALIDOVVAL_W<7> {
        BVALIDOVVAL_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HNPREQ_W<9> {
        HNPREQ_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn hstsethnpen(&mut self) -> HSTSETHNPEN_W<10> {
        HSTSETHNPEN_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn devhnpen(&mut self) -> DEVHNPEN_W<11> {
        DEVHNPEN_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ehen(&mut self) -> EHEN_W<12> {
        EHEN_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dbncefltrbypass(&mut self) -> DBNCEFLTRBYPASS_W<15> {
        DBNCEFLTRBYPASS_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<20> {
        OTGVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](index.html) module"]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gotgctl::R](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0"]
impl crate::Resettable for GOTGCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
