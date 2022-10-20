#[doc = "Register `GINTSTS` reader"]
pub struct R(crate::R<GINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTSTS` writer"]
pub struct W(crate::W<GINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTSTS_SPEC>;
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
impl From<crate::W<GINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURMOD_INT` reader - "]
pub type CURMOD_INT_R = crate::BitReader<bool>;
#[doc = "Field `MODEMIS` reader - "]
pub type MODEMIS_R = crate::BitReader<bool>;
#[doc = "Field `MODEMIS` writer - "]
pub type MODEMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `OTGINT` reader - "]
pub type OTGINT_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - "]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - "]
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `RXFLVI` reader - "]
pub type RXFLVI_R = crate::BitReader<bool>;
#[doc = "Field `NPTXFEMP` reader - "]
pub type NPTXFEMP_R = crate::BitReader<bool>;
#[doc = "Field `GINNAKEFF` reader - "]
pub type GINNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNAKEFF` reader - "]
pub type GOUTNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `ERLYSUSP` reader - "]
pub type ERLYSUSP_R = crate::BitReader<bool>;
#[doc = "Field `ERLYSUSP` writer - "]
pub type ERLYSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `USBSUSP` reader - "]
pub type USBSUSP_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSP` writer - "]
pub type USBSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `USBRST` reader - "]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - "]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `ENUMDONE` reader - "]
pub type ENUMDONE_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDONE` writer - "]
pub type ENUMDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `ISOOUTDROP` reader - "]
pub type ISOOUTDROP_R = crate::BitReader<bool>;
#[doc = "Field `ISOOUTDROP` writer - "]
pub type ISOOUTDROP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `EOPF` reader - "]
pub type EOPF_R = crate::BitReader<bool>;
#[doc = "Field `EOPF` writer - "]
pub type EOPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `EPMIS` reader - "]
pub type EPMIS_R = crate::BitReader<bool>;
#[doc = "Field `EPMIS` writer - "]
pub type EPMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `IEPINT` reader - "]
pub type IEPINT_R = crate::BitReader<bool>;
#[doc = "Field `OEPINT` reader - "]
pub type OEPINT_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPISOIN` reader - "]
pub type INCOMPISOIN_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPISOIN` writer - "]
pub type INCOMPISOIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `INCOMPIP` reader - "]
pub type INCOMPIP_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPIP` writer - "]
pub type INCOMPIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `FETSUSP` reader - "]
pub type FETSUSP_R = crate::BitReader<bool>;
#[doc = "Field `FETSUSP` writer - "]
pub type FETSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `RESETDET` reader - "]
pub type RESETDET_R = crate::BitReader<bool>;
#[doc = "Field `RESETDET` writer - "]
pub type RESETDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `PRTLNT` reader - "]
pub type PRTLNT_R = crate::BitReader<bool>;
#[doc = "Field `HCHLNT` reader - "]
pub type HCHLNT_R = crate::BitReader<bool>;
#[doc = "Field `PTXFEMP` reader - "]
pub type PTXFEMP_R = crate::BitReader<bool>;
#[doc = "Field `CONIDSTSCHNG` reader - "]
pub type CONIDSTSCHNG_R = crate::BitReader<bool>;
#[doc = "Field `CONIDSTSCHNG` writer - "]
pub type CONIDSTSCHNG_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `DISCONNINT` reader - "]
pub type DISCONNINT_R = crate::BitReader<bool>;
#[doc = "Field `DISCONNINT` writer - "]
pub type DISCONNINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `SESSREQINT` reader - "]
pub type SESSREQINT_R = crate::BitReader<bool>;
#[doc = "Field `SESSREQINT` writer - "]
pub type SESSREQINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
#[doc = "Field `WKUPINT` reader - "]
pub type WKUPINT_R = crate::BitReader<bool>;
#[doc = "Field `WKUPINT` writer - "]
pub type WKUPINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn curmod_int(&self) -> CURMOD_INT_R {
        CURMOD_INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modemis(&self) -> MODEMIS_R {
        MODEMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn otgint(&self) -> OTGINT_R {
        OTGINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxflvi(&self) -> RXFLVI_R {
        RXFLVI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nptxfemp(&self) -> NPTXFEMP_R {
        NPTXFEMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GINNAKEFF_R {
        GINNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn erlysusp(&self) -> ERLYSUSP_R {
        ERLYSUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enumdone(&self) -> ENUMDONE_R {
        ENUMDONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn isooutdrop(&self) -> ISOOUTDROP_R {
        ISOOUTDROP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn epmis(&self) -> EPMIS_R {
        EPMIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn incompisoin(&self) -> INCOMPISOIN_R {
        INCOMPISOIN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn incompip(&self) -> INCOMPIP_R {
        INCOMPIP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fetsusp(&self) -> FETSUSP_R {
        FETSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn resetdet(&self) -> RESETDET_R {
        RESETDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn prtlnt(&self) -> PRTLNT_R {
        PRTLNT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hchlnt(&self) -> HCHLNT_R {
        HCHLNT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ptxfemp(&self) -> PTXFEMP_R {
        PTXFEMP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn conidstschng(&self) -> CONIDSTSCHNG_R {
        CONIDSTSCHNG_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn disconnint(&self) -> DISCONNINT_R {
        DISCONNINT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sessreqint(&self) -> SESSREQINT_R {
        SESSREQINT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modemis(&mut self) -> MODEMIS_W<1> {
        MODEMIS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<3> {
        SOF_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn erlysusp(&mut self) -> ERLYSUSP_W<10> {
        ERLYSUSP_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W<11> {
        USBSUSP_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<12> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enumdone(&mut self) -> ENUMDONE_W<13> {
        ENUMDONE_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn isooutdrop(&mut self) -> ISOOUTDROP_W<14> {
        ISOOUTDROP_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<15> {
        EOPF_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn epmis(&mut self) -> EPMIS_W<17> {
        EPMIS_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn incompisoin(&mut self) -> INCOMPISOIN_W<20> {
        INCOMPISOIN_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn incompip(&mut self) -> INCOMPIP_W<21> {
        INCOMPIP_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fetsusp(&mut self) -> FETSUSP_W<22> {
        FETSUSP_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn resetdet(&mut self) -> RESETDET_W<23> {
        RESETDET_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn conidstschng(&mut self) -> CONIDSTSCHNG_W<28> {
        CONIDSTSCHNG_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn disconnint(&mut self) -> DISCONNINT_W<29> {
        DISCONNINT_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sessreqint(&mut self) -> SESSREQINT_W<30> {
        SESSREQINT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W<31> {
        WKUPINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts](index.html) module"]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintsts::R](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintsts::W](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTSTS to value 0"]
impl crate::Resettable for GINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
