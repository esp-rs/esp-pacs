#[doc = "Register `DOEPINT5` reader"]
pub struct R(crate::R<DOEPINT5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPINT5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPINT5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPINT5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPINT5` writer"]
pub struct W(crate::W<DOEPINT5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPINT5_SPEC>;
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
impl From<crate::W<DOEPINT5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPINT5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPL5` reader - "]
pub type XFERCOMPL5_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPL5` writer - "]
pub type XFERCOMPL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `EPDISBLD5` reader - "]
pub type EPDISBLD5_R = crate::BitReader<bool>;
#[doc = "Field `EPDISBLD5` writer - "]
pub type EPDISBLD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `AHBERR5` reader - "]
pub type AHBERR5_R = crate::BitReader<bool>;
#[doc = "Field `AHBERR5` writer - "]
pub type AHBERR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `SETUP5` reader - "]
pub type SETUP5_R = crate::BitReader<bool>;
#[doc = "Field `SETUP5` writer - "]
pub type SETUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `OUTTKNEPDIS5` reader - "]
pub type OUTTKNEPDIS5_R = crate::BitReader<bool>;
#[doc = "Field `OUTTKNEPDIS5` writer - "]
pub type OUTTKNEPDIS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `STSPHSERCVD5` reader - "]
pub type STSPHSERCVD5_R = crate::BitReader<bool>;
#[doc = "Field `STSPHSERCVD5` writer - "]
pub type STSPHSERCVD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `BACK2BACKSETUP5` reader - "]
pub type BACK2BACKSETUP5_R = crate::BitReader<bool>;
#[doc = "Field `BACK2BACKSETUP5` writer - "]
pub type BACK2BACKSETUP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `OUTPKTERR5` reader - "]
pub type OUTPKTERR5_R = crate::BitReader<bool>;
#[doc = "Field `OUTPKTERR5` writer - "]
pub type OUTPKTERR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `BNAINTR5` reader - "]
pub type BNAINTR5_R = crate::BitReader<bool>;
#[doc = "Field `BNAINTR5` writer - "]
pub type BNAINTR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `PKTDRPSTS5` reader - "]
pub type PKTDRPSTS5_R = crate::BitReader<bool>;
#[doc = "Field `PKTDRPSTS5` writer - "]
pub type PKTDRPSTS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `BBLEERR5` reader - "]
pub type BBLEERR5_R = crate::BitReader<bool>;
#[doc = "Field `BBLEERR5` writer - "]
pub type BBLEERR5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `NAKINTRPT5` reader - "]
pub type NAKINTRPT5_R = crate::BitReader<bool>;
#[doc = "Field `NAKINTRPT5` writer - "]
pub type NAKINTRPT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `NYEPINTRPT5` reader - "]
pub type NYEPINTRPT5_R = crate::BitReader<bool>;
#[doc = "Field `NYEPINTRPT5` writer - "]
pub type NYEPINTRPT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
#[doc = "Field `STUPPKTRCVD5` reader - "]
pub type STUPPKTRCVD5_R = crate::BitReader<bool>;
#[doc = "Field `STUPPKTRCVD5` writer - "]
pub type STUPPKTRCVD5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPINT5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl5(&self) -> XFERCOMPL5_R {
        XFERCOMPL5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld5(&self) -> EPDISBLD5_R {
        EPDISBLD5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr5(&self) -> AHBERR5_R {
        AHBERR5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup5(&self) -> SETUP5_R {
        SETUP5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis5(&self) -> OUTTKNEPDIS5_R {
        OUTTKNEPDIS5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd5(&self) -> STSPHSERCVD5_R {
        STSPHSERCVD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup5(&self) -> BACK2BACKSETUP5_R {
        BACK2BACKSETUP5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr5(&self) -> OUTPKTERR5_R {
        OUTPKTERR5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr5(&self) -> BNAINTR5_R {
        BNAINTR5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts5(&self) -> PKTDRPSTS5_R {
        PKTDRPSTS5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr5(&self) -> BBLEERR5_R {
        BBLEERR5_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt5(&self) -> NAKINTRPT5_R {
        NAKINTRPT5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt5(&self) -> NYEPINTRPT5_R {
        NYEPINTRPT5_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd5(&self) -> STUPPKTRCVD5_R {
        STUPPKTRCVD5_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercompl5(&mut self) -> XFERCOMPL5_W<0> {
        XFERCOMPL5_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbld5(&mut self) -> EPDISBLD5_W<1> {
        EPDISBLD5_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberr5(&mut self) -> AHBERR5_W<2> {
        AHBERR5_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setup5(&mut self) -> SETUP5_W<3> {
        SETUP5_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdis5(&mut self) -> OUTTKNEPDIS5_W<4> {
        OUTTKNEPDIS5_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvd5(&mut self) -> STSPHSERCVD5_W<5> {
        STSPHSERCVD5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup5(&mut self) -> BACK2BACKSETUP5_W<6> {
        BACK2BACKSETUP5_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterr5(&mut self) -> OUTPKTERR5_W<8> {
        OUTPKTERR5_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaintr5(&mut self) -> BNAINTR5_W<9> {
        BNAINTR5_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pktdrpsts5(&mut self) -> PKTDRPSTS5_W<11> {
        PKTDRPSTS5_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerr5(&mut self) -> BBLEERR5_W<12> {
        BBLEERR5_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakintrpt5(&mut self) -> NAKINTRPT5_W<13> {
        NAKINTRPT5_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyepintrpt5(&mut self) -> NYEPINTRPT5_W<14> {
        NYEPINTRPT5_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn stuppktrcvd5(&mut self) -> STUPPKTRCVD5_W<15> {
        STUPPKTRCVD5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepint5](index.html) module"]
pub struct DOEPINT5_SPEC;
impl crate::RegisterSpec for DOEPINT5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepint5::R](R) reader structure"]
impl crate::Readable for DOEPINT5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepint5::W](W) writer structure"]
impl crate::Writable for DOEPINT5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPINT5 to value 0"]
impl crate::Resettable for DOEPINT5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
