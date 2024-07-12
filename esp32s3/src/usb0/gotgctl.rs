#[doc = "Register `GOTGCTL` reader"]
pub type R = crate::R<GOTGCTL_SPEC>;
#[doc = "Register `GOTGCTL` writer"]
pub type W = crate::W<GOTGCTL_SPEC>;
#[doc = "Field `SESREQSCS` reader - "]
pub type SESREQSCS_R = crate::BitReader;
#[doc = "Field `SESREQ` reader - "]
pub type SESREQ_R = crate::BitReader;
#[doc = "Field `SESREQ` writer - "]
pub type SESREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALIDOVEN` reader - "]
pub type VBVALIDOVEN_R = crate::BitReader;
#[doc = "Field `VBVALIDOVEN` writer - "]
pub type VBVALIDOVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBVALIDOVVAL` reader - "]
pub type VBVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `VBVALIDOVVAL` writer - "]
pub type VBVALIDOVVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALIDOVEN` reader - "]
pub type AVALIDOVEN_R = crate::BitReader;
#[doc = "Field `AVALIDOVEN` writer - "]
pub type AVALIDOVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVALIDOVVAL` reader - "]
pub type AVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `AVALIDOVVAL` writer - "]
pub type AVALIDOVVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALIDOVEN` reader - "]
pub type BVALIDOVEN_R = crate::BitReader;
#[doc = "Field `BVALIDOVEN` writer - "]
pub type BVALIDOVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVALIDOVVAL` reader - "]
pub type BVALIDOVVAL_R = crate::BitReader;
#[doc = "Field `BVALIDOVVAL` writer - "]
pub type BVALIDOVVAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGSCS` reader - "]
pub type HSTNEGSCS_R = crate::BitReader;
#[doc = "Field `HNPREQ` reader - "]
pub type HNPREQ_R = crate::BitReader;
#[doc = "Field `HNPREQ` writer - "]
pub type HNPREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTSETHNPEN` reader - "]
pub type HSTSETHNPEN_R = crate::BitReader;
#[doc = "Field `HSTSETHNPEN` writer - "]
pub type HSTSETHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEVHNPEN` reader - "]
pub type DEVHNPEN_R = crate::BitReader;
#[doc = "Field `DEVHNPEN` writer - "]
pub type DEVHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHEN` reader - "]
pub type EHEN_R = crate::BitReader;
#[doc = "Field `EHEN` writer - "]
pub type EHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCEFLTRBYPASS` reader - "]
pub type DBNCEFLTRBYPASS_R = crate::BitReader;
#[doc = "Field `DBNCEFLTRBYPASS` writer - "]
pub type DBNCEFLTRBYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
pub type OTGVER_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("sesreqscs", &self.sesreqscs())
            .field("sesreq", &self.sesreq())
            .field("vbvalidoven", &self.vbvalidoven())
            .field("vbvalidovval", &self.vbvalidovval())
            .field("avalidoven", &self.avalidoven())
            .field("avalidovval", &self.avalidovval())
            .field("bvalidoven", &self.bvalidoven())
            .field("bvalidovval", &self.bvalidovval())
            .field("hstnegscs", &self.hstnegscs())
            .field("hnpreq", &self.hnpreq())
            .field("hstsethnpen", &self.hstsethnpen())
            .field("devhnpen", &self.devhnpen())
            .field("ehen", &self.ehen())
            .field("dbncefltrbypass", &self.dbncefltrbypass())
            .field("conidsts", &self.conidsts())
            .field("dbnctime", &self.dbnctime())
            .field("asesvld", &self.asesvld())
            .field("bsesvld", &self.bsesvld())
            .field("otgver", &self.otgver())
            .field("curmod", &self.curmod())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sesreq(&mut self) -> SESREQ_W<GOTGCTL_SPEC> {
        SESREQ_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalidoven(&mut self) -> VBVALIDOVEN_W<GOTGCTL_SPEC> {
        VBVALIDOVEN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn vbvalidovval(&mut self) -> VBVALIDOVVAL_W<GOTGCTL_SPEC> {
        VBVALIDOVVAL_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn avalidoven(&mut self) -> AVALIDOVEN_W<GOTGCTL_SPEC> {
        AVALIDOVEN_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn avalidovval(&mut self) -> AVALIDOVVAL_W<GOTGCTL_SPEC> {
        AVALIDOVVAL_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn bvalidoven(&mut self) -> BVALIDOVEN_W<GOTGCTL_SPEC> {
        BVALIDOVEN_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn bvalidovval(&mut self) -> BVALIDOVVAL_W<GOTGCTL_SPEC> {
        BVALIDOVVAL_W::new(self, 7)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hnpreq(&mut self) -> HNPREQ_W<GOTGCTL_SPEC> {
        HNPREQ_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn hstsethnpen(&mut self) -> HSTSETHNPEN_W<GOTGCTL_SPEC> {
        HSTSETHNPEN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn devhnpen(&mut self) -> DEVHNPEN_W<GOTGCTL_SPEC> {
        DEVHNPEN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn ehen(&mut self) -> EHEN_W<GOTGCTL_SPEC> {
        EHEN_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dbncefltrbypass(&mut self) -> DBNCEFLTRBYPASS_W<GOTGCTL_SPEC> {
        DBNCEFLTRBYPASS_W::new(self, 15)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn otgver(&mut self) -> OTGVER_W<GOTGCTL_SPEC> {
        OTGVER_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGCTL_SPEC;
impl crate::RegisterSpec for GOTGCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgctl::R`](R) reader structure"]
impl crate::Readable for GOTGCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgctl::W`](W) writer structure"]
impl crate::Writable for GOTGCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGCTL to value 0"]
impl crate::Resettable for GOTGCTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
