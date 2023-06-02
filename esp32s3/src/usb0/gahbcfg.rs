#[doc = "Register `GAHBCFG` reader"]
pub struct R(crate::R<GAHBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAHBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAHBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAHBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAHBCFG` writer"]
pub struct W(crate::W<GAHBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAHBCFG_SPEC>;
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
impl From<crate::W<GAHBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAHBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLBLLNTRMSK` reader - "]
pub type GLBLLNTRMSK_R = crate::BitReader;
#[doc = "Field `GLBLLNTRMSK` writer - "]
pub type GLBLLNTRMSK_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
#[doc = "Field `HBSTLEN` reader - "]
pub type HBSTLEN_R = crate::FieldReader;
#[doc = "Field `HBSTLEN` writer - "]
pub type HBSTLEN_W<'a, const O: u8> = crate::FieldWriter<'a, GAHBCFG_SPEC, 4, O>;
#[doc = "Field `DMAEN` reader - "]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - "]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
#[doc = "Field `NPTXFEMPLVL` reader - "]
pub type NPTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `NPTXFEMPLVL` writer - "]
pub type NPTXFEMPLVL_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
#[doc = "Field `PTXFEMPLVL` reader - "]
pub type PTXFEMPLVL_R = crate::BitReader;
#[doc = "Field `PTXFEMPLVL` writer - "]
pub type PTXFEMPLVL_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
#[doc = "Field `REMMEMSUPP` reader - "]
pub type REMMEMSUPP_R = crate::BitReader;
#[doc = "Field `REMMEMSUPP` writer - "]
pub type REMMEMSUPP_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
#[doc = "Field `NOTIALLDMAWRIT` reader - "]
pub type NOTIALLDMAWRIT_R = crate::BitReader;
#[doc = "Field `NOTIALLDMAWRIT` writer - "]
pub type NOTIALLDMAWRIT_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
#[doc = "Field `AHBSINGLE` reader - "]
pub type AHBSINGLE_R = crate::BitReader;
#[doc = "Field `AHBSINGLE` writer - "]
pub type AHBSINGLE_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
#[doc = "Field `INVDESCENDIANESS` reader - "]
pub type INVDESCENDIANESS_R = crate::BitReader;
#[doc = "Field `INVDESCENDIANESS` writer - "]
pub type INVDESCENDIANESS_W<'a, const O: u8> = crate::BitWriter<'a, GAHBCFG_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn glbllntrmsk(&self) -> GLBLLNTRMSK_R {
        GLBLLNTRMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn nptxfemplvl(&self) -> NPTXFEMPLVL_R {
        NPTXFEMPLVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ptxfemplvl(&self) -> PTXFEMPLVL_R {
        PTXFEMPLVL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn remmemsupp(&self) -> REMMEMSUPP_R {
        REMMEMSUPP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn notialldmawrit(&self) -> NOTIALLDMAWRIT_R {
        NOTIALLDMAWRIT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ahbsingle(&self) -> AHBSINGLE_R {
        AHBSINGLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn invdescendianess(&self) -> INVDESCENDIANESS_R {
        INVDESCENDIANESS_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAHBCFG")
            .field("glbllntrmsk", &format_args!("{}", self.glbllntrmsk().bit()))
            .field("hbstlen", &format_args!("{}", self.hbstlen().bits()))
            .field("dmaen", &format_args!("{}", self.dmaen().bit()))
            .field("nptxfemplvl", &format_args!("{}", self.nptxfemplvl().bit()))
            .field("ptxfemplvl", &format_args!("{}", self.ptxfemplvl().bit()))
            .field("remmemsupp", &format_args!("{}", self.remmemsupp().bit()))
            .field(
                "notialldmawrit",
                &format_args!("{}", self.notialldmawrit().bit()),
            )
            .field("ahbsingle", &format_args!("{}", self.ahbsingle().bit()))
            .field(
                "invdescendianess",
                &format_args!("{}", self.invdescendianess().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GAHBCFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn glbllntrmsk(&mut self) -> GLBLLNTRMSK_W<0> {
        GLBLLNTRMSK_W::new(self)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    #[must_use]
    pub fn hbstlen(&mut self) -> HBSTLEN_W<1> {
        HBSTLEN_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<5> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfemplvl(&mut self) -> NPTXFEMPLVL_W<7> {
        NPTXFEMPLVL_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfemplvl(&mut self) -> PTXFEMPLVL_W<8> {
        PTXFEMPLVL_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn remmemsupp(&mut self) -> REMMEMSUPP_W<21> {
        REMMEMSUPP_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn notialldmawrit(&mut self) -> NOTIALLDMAWRIT_W<22> {
        NOTIALLDMAWRIT_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn ahbsingle(&mut self) -> AHBSINGLE_W<23> {
        AHBSINGLE_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn invdescendianess(&mut self) -> INVDESCENDIANESS_W<24> {
        INVDESCENDIANESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](index.html) module"]
pub struct GAHBCFG_SPEC;
impl crate::RegisterSpec for GAHBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gahbcfg::R](R) reader structure"]
impl crate::Readable for GAHBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](W) writer structure"]
impl crate::Writable for GAHBCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAHBCFG to value 0"]
impl crate::Resettable for GAHBCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
