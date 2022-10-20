#[doc = "Register `GINTMSK` reader"]
pub struct R(crate::R<GINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTMSK` writer"]
pub struct W(crate::W<GINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_SPEC>;
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
impl From<crate::W<GINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEMISMSK` reader - "]
pub type MODEMISMSK_R = crate::BitReader<bool>;
#[doc = "Field `MODEMISMSK` writer - "]
pub type MODEMISMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `OTGINTMSK` reader - "]
pub type OTGINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `OTGINTMSK` writer - "]
pub type OTGINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `SOFMSK` reader - "]
pub type SOFMSK_R = crate::BitReader<bool>;
#[doc = "Field `SOFMSK` writer - "]
pub type SOFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `RXFLVIMSK` reader - "]
pub type RXFLVIMSK_R = crate::BitReader<bool>;
#[doc = "Field `RXFLVIMSK` writer - "]
pub type RXFLVIMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `NPTXFEMPMSK` reader - "]
pub type NPTXFEMPMSK_R = crate::BitReader<bool>;
#[doc = "Field `NPTXFEMPMSK` writer - "]
pub type NPTXFEMPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `GINNAKEFFMSK` reader - "]
pub type GINNAKEFFMSK_R = crate::BitReader<bool>;
#[doc = "Field `GINNAKEFFMSK` writer - "]
pub type GINNAKEFFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `GOUTNACKEFFMSK` reader - "]
pub type GOUTNACKEFFMSK_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNACKEFFMSK` writer - "]
pub type GOUTNACKEFFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ERLYSUSPMSK` reader - "]
pub type ERLYSUSPMSK_R = crate::BitReader<bool>;
#[doc = "Field `ERLYSUSPMSK` writer - "]
pub type ERLYSUSPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `USBSUSPMSK` reader - "]
pub type USBSUSPMSK_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSPMSK` writer - "]
pub type USBSUSPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `USBRSTMSK` reader - "]
pub type USBRSTMSK_R = crate::BitReader<bool>;
#[doc = "Field `USBRSTMSK` writer - "]
pub type USBRSTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ENUMDONEMSK` reader - "]
pub type ENUMDONEMSK_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDONEMSK` writer - "]
pub type ENUMDONEMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ISOOUTDROPMSK` reader - "]
pub type ISOOUTDROPMSK_R = crate::BitReader<bool>;
#[doc = "Field `ISOOUTDROPMSK` writer - "]
pub type ISOOUTDROPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `EOPFMSK` reader - "]
pub type EOPFMSK_R = crate::BitReader<bool>;
#[doc = "Field `EOPFMSK` writer - "]
pub type EOPFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `EPMISMSK` reader - "]
pub type EPMISMSK_R = crate::BitReader<bool>;
#[doc = "Field `EPMISMSK` writer - "]
pub type EPMISMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `IEPINTMSK` reader - "]
pub type IEPINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `IEPINTMSK` writer - "]
pub type IEPINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `OEPINTMSK` reader - "]
pub type OEPINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `OEPINTMSK` writer - "]
pub type OEPINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `INCOMPISOINMSK` reader - "]
pub type INCOMPISOINMSK_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPISOINMSK` writer - "]
pub type INCOMPISOINMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `INCOMPIPMSK` reader - "]
pub type INCOMPIPMSK_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPIPMSK` writer - "]
pub type INCOMPIPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `FETSUSPMSK` reader - "]
pub type FETSUSPMSK_R = crate::BitReader<bool>;
#[doc = "Field `FETSUSPMSK` writer - "]
pub type FETSUSPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `RESETDETMSK` reader - "]
pub type RESETDETMSK_R = crate::BitReader<bool>;
#[doc = "Field `RESETDETMSK` writer - "]
pub type RESETDETMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `PRTLNTMSK` reader - "]
pub type PRTLNTMSK_R = crate::BitReader<bool>;
#[doc = "Field `PRTLNTMSK` writer - "]
pub type PRTLNTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `HCHINTMSK` reader - "]
pub type HCHINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `HCHINTMSK` writer - "]
pub type HCHINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `PTXFEMPMSK` reader - "]
pub type PTXFEMPMSK_R = crate::BitReader<bool>;
#[doc = "Field `PTXFEMPMSK` writer - "]
pub type PTXFEMPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `CONIDSTSCHNGMSK` reader - "]
pub type CONIDSTSCHNGMSK_R = crate::BitReader<bool>;
#[doc = "Field `CONIDSTSCHNGMSK` writer - "]
pub type CONIDSTSCHNGMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `DISCONNINTMSK` reader - "]
pub type DISCONNINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `DISCONNINTMSK` writer - "]
pub type DISCONNINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `SESSREQINTMSK` reader - "]
pub type SESSREQINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `SESSREQINTMSK` writer - "]
pub type SESSREQINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `WKUPINTMSK` reader - "]
pub type WKUPINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `WKUPINTMSK` writer - "]
pub type WKUPINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn otgintmsk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxflvimsk(&self) -> RXFLVIMSK_R {
        RXFLVIMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NPTXFEMPMSK_R {
        NPTXFEMPMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn goutnackeffmsk(&self) -> GOUTNACKEFFMSK_R {
        GOUTNACKEFFMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn epmismsk(&self) -> EPMISMSK_R {
        EPMISMSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> INCOMPISOINMSK_R {
        INCOMPISOINMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn incompipmsk(&self) -> INCOMPIPMSK_R {
        INCOMPIPMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FETSUSPMSK_R {
        FETSUSPMSK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> RESETDETMSK_R {
        RESETDETMSK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn prtlntmsk(&self) -> PRTLNTMSK_R {
        PRTLNTMSK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hchintmsk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn conidstschngmsk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn disconnintmsk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sessreqintmsk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W<1> {
        MODEMISMSK_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn otgintmsk(&mut self) -> OTGINTMSK_W<2> {
        OTGINTMSK_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofmsk(&mut self) -> SOFMSK_W<3> {
        SOFMSK_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxflvimsk(&mut self) -> RXFLVIMSK_W<4> {
        RXFLVIMSK_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn nptxfempmsk(&mut self) -> NPTXFEMPMSK_W<5> {
        NPTXFEMPMSK_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W<6> {
        GINNAKEFFMSK_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn goutnackeffmsk(&mut self) -> GOUTNACKEFFMSK_W<7> {
        GOUTNACKEFFMSK_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W<10> {
        ERLYSUSPMSK_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W<11> {
        USBSUSPMSK_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W<12> {
        USBRSTMSK_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W<13> {
        ENUMDONEMSK_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W<14> {
        ISOOUTDROPMSK_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<15> {
        EOPFMSK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn epmismsk(&mut self) -> EPMISMSK_W<17> {
        EPMISMSK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn iepintmsk(&mut self) -> IEPINTMSK_W<18> {
        IEPINTMSK_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn oepintmsk(&mut self) -> OEPINTMSK_W<19> {
        OEPINTMSK_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn incompisoinmsk(&mut self) -> INCOMPISOINMSK_W<20> {
        INCOMPISOINMSK_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn incompipmsk(&mut self) -> INCOMPIPMSK_W<21> {
        INCOMPIPMSK_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn fetsuspmsk(&mut self) -> FETSUSPMSK_W<22> {
        FETSUSPMSK_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn resetdetmsk(&mut self) -> RESETDETMSK_W<23> {
        RESETDETMSK_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn prtlntmsk(&mut self) -> PRTLNTMSK_W<24> {
        PRTLNTMSK_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn hchintmsk(&mut self) -> HCHINTMSK_W<25> {
        HCHINTMSK_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ptxfempmsk(&mut self) -> PTXFEMPMSK_W<26> {
        PTXFEMPMSK_W::new(self)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn conidstschngmsk(&mut self) -> CONIDSTSCHNGMSK_W<28> {
        CONIDSTSCHNGMSK_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn disconnintmsk(&mut self) -> DISCONNINTMSK_W<29> {
        DISCONNINTMSK_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sessreqintmsk(&mut self) -> SESSREQINTMSK_W<30> {
        SESSREQINTMSK_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W<31> {
        WKUPINTMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk](index.html) module"]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintmsk::R](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
