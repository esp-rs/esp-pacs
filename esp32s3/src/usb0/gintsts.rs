#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GINTSTS_SPEC>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GINTSTS_SPEC>;
#[doc = "Field `CURMOD_INT` reader - "]
pub type CURMOD_INT_R = crate::BitReader;
#[doc = "Field `MODEMIS` reader - "]
pub type MODEMIS_R = crate::BitReader;
#[doc = "Field `MODEMIS` writer - "]
pub type MODEMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - "]
pub type OTGINT_R = crate::BitReader;
#[doc = "Field `SOF` reader - "]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - "]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVI` reader - "]
pub type RXFLVI_R = crate::BitReader;
#[doc = "Field `NPTXFEMP` reader - "]
pub type NPTXFEMP_R = crate::BitReader;
#[doc = "Field `GINNAKEFF` reader - "]
pub type GINNAKEFF_R = crate::BitReader;
#[doc = "Field `GOUTNAKEFF` reader - "]
pub type GOUTNAKEFF_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` reader - "]
pub type ERLYSUSP_R = crate::BitReader;
#[doc = "Field `ERLYSUSP` writer - "]
pub type ERLYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - "]
pub type USBSUSP_R = crate::BitReader;
#[doc = "Field `USBSUSP` writer - "]
pub type USBSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - "]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - "]
pub type USBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONE` reader - "]
pub type ENUMDONE_R = crate::BitReader;
#[doc = "Field `ENUMDONE` writer - "]
pub type ENUMDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROP` reader - "]
pub type ISOOUTDROP_R = crate::BitReader;
#[doc = "Field `ISOOUTDROP` writer - "]
pub type ISOOUTDROP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - "]
pub type EOPF_R = crate::BitReader;
#[doc = "Field `EOPF` writer - "]
pub type EOPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPMIS` reader - "]
pub type EPMIS_R = crate::BitReader;
#[doc = "Field `EPMIS` writer - "]
pub type EPMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPINT` reader - "]
pub type IEPINT_R = crate::BitReader;
#[doc = "Field `OEPINT` reader - "]
pub type OEPINT_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` reader - "]
pub type INCOMPISOIN_R = crate::BitReader;
#[doc = "Field `INCOMPISOIN` writer - "]
pub type INCOMPISOIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPIP` reader - "]
pub type INCOMPIP_R = crate::BitReader;
#[doc = "Field `INCOMPIP` writer - "]
pub type INCOMPIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FETSUSP` reader - "]
pub type FETSUSP_R = crate::BitReader;
#[doc = "Field `FETSUSP` writer - "]
pub type FETSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETDET` reader - "]
pub type RESETDET_R = crate::BitReader;
#[doc = "Field `RESETDET` writer - "]
pub type RESETDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLNT` reader - "]
pub type PRTLNT_R = crate::BitReader;
#[doc = "Field `HCHLNT` reader - "]
pub type HCHLNT_R = crate::BitReader;
#[doc = "Field `PTXFEMP` reader - "]
pub type PTXFEMP_R = crate::BitReader;
#[doc = "Field `CONIDSTSCHNG` reader - "]
pub type CONIDSTSCHNG_R = crate::BitReader;
#[doc = "Field `CONIDSTSCHNG` writer - "]
pub type CONIDSTSCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONNINT` reader - "]
pub type DISCONNINT_R = crate::BitReader;
#[doc = "Field `DISCONNINT` writer - "]
pub type DISCONNINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESSREQINT` reader - "]
pub type SESSREQINT_R = crate::BitReader;
#[doc = "Field `SESSREQINT` writer - "]
pub type SESSREQINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - "]
pub type WKUPINT_R = crate::BitReader;
#[doc = "Field `WKUPINT` writer - "]
pub type WKUPINT_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("curmod_int", &self.curmod_int())
            .field("modemis", &self.modemis())
            .field("otgint", &self.otgint())
            .field("sof", &self.sof())
            .field("rxflvi", &self.rxflvi())
            .field("nptxfemp", &self.nptxfemp())
            .field("ginnakeff", &self.ginnakeff())
            .field("goutnakeff", &self.goutnakeff())
            .field("erlysusp", &self.erlysusp())
            .field("usbsusp", &self.usbsusp())
            .field("usbrst", &self.usbrst())
            .field("enumdone", &self.enumdone())
            .field("isooutdrop", &self.isooutdrop())
            .field("eopf", &self.eopf())
            .field("epmis", &self.epmis())
            .field("iepint", &self.iepint())
            .field("oepint", &self.oepint())
            .field("incompisoin", &self.incompisoin())
            .field("incompip", &self.incompip())
            .field("fetsusp", &self.fetsusp())
            .field("resetdet", &self.resetdet())
            .field("prtlnt", &self.prtlnt())
            .field("hchlnt", &self.hchlnt())
            .field("ptxfemp", &self.ptxfemp())
            .field("conidstschng", &self.conidstschng())
            .field("disconnint", &self.disconnint())
            .field("sessreqint", &self.sessreqint())
            .field("wkupint", &self.wkupint())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modemis(&mut self) -> MODEMIS_W<GINTSTS_SPEC> {
        MODEMIS_W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<GINTSTS_SPEC> {
        SOF_W::new(self, 3)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn erlysusp(&mut self) -> ERLYSUSP_W<GINTSTS_SPEC> {
        ERLYSUSP_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W<GINTSTS_SPEC> {
        USBSUSP_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W<GINTSTS_SPEC> {
        USBRST_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enumdone(&mut self) -> ENUMDONE_W<GINTSTS_SPEC> {
        ENUMDONE_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn isooutdrop(&mut self) -> ISOOUTDROP_W<GINTSTS_SPEC> {
        ISOOUTDROP_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W<GINTSTS_SPEC> {
        EOPF_W::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn epmis(&mut self) -> EPMIS_W<GINTSTS_SPEC> {
        EPMIS_W::new(self, 17)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn incompisoin(&mut self) -> INCOMPISOIN_W<GINTSTS_SPEC> {
        INCOMPISOIN_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn incompip(&mut self) -> INCOMPIP_W<GINTSTS_SPEC> {
        INCOMPIP_W::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fetsusp(&mut self) -> FETSUSP_W<GINTSTS_SPEC> {
        FETSUSP_W::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn resetdet(&mut self) -> RESETDET_W<GINTSTS_SPEC> {
        RESETDET_W::new(self, 23)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn conidstschng(&mut self) -> CONIDSTSCHNG_W<GINTSTS_SPEC> {
        CONIDSTSCHNG_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn disconnint(&mut self) -> DISCONNINT_W<GINTSTS_SPEC> {
        DISCONNINT_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sessreqint(&mut self) -> SESSREQINT_W<GINTSTS_SPEC> {
        SESSREQINT_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W<GINTSTS_SPEC> {
        WKUPINT_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTSTS to value 0"]
impl crate::Resettable for GINTSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
