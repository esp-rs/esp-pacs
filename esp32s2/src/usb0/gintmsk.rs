///Register `GINTMSK` reader
pub type R = crate::R<GINTMSK_SPEC>;
///Register `GINTMSK` writer
pub type W = crate::W<GINTMSK_SPEC>;
///Field `MODEMISMSK` reader -
pub type MODEMISMSK_R = crate::BitReader;
///Field `MODEMISMSK` writer -
pub type MODEMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OTGINTMSK` reader -
pub type OTGINTMSK_R = crate::BitReader;
///Field `OTGINTMSK` writer -
pub type OTGINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFMSK` reader -
pub type SOFMSK_R = crate::BitReader;
///Field `SOFMSK` writer -
pub type SOFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXFLVIMSK` reader -
pub type RXFLVIMSK_R = crate::BitReader;
///Field `RXFLVIMSK` writer -
pub type RXFLVIMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NPTXFEMPMSK` reader -
pub type NPTXFEMPMSK_R = crate::BitReader;
///Field `NPTXFEMPMSK` writer -
pub type NPTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GINNAKEFFMSK` reader -
pub type GINNAKEFFMSK_R = crate::BitReader;
///Field `GINNAKEFFMSK` writer -
pub type GINNAKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GOUTNACKEFFMSK` reader -
pub type GOUTNACKEFFMSK_R = crate::BitReader;
///Field `GOUTNACKEFFMSK` writer -
pub type GOUTNACKEFFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERLYSUSPMSK` reader -
pub type ERLYSUSPMSK_R = crate::BitReader;
///Field `ERLYSUSPMSK` writer -
pub type ERLYSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBSUSPMSK` reader -
pub type USBSUSPMSK_R = crate::BitReader;
///Field `USBSUSPMSK` writer -
pub type USBSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBRSTMSK` reader -
pub type USBRSTMSK_R = crate::BitReader;
///Field `USBRSTMSK` writer -
pub type USBRSTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENUMDONEMSK` reader -
pub type ENUMDONEMSK_R = crate::BitReader;
///Field `ENUMDONEMSK` writer -
pub type ENUMDONEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ISOOUTDROPMSK` reader -
pub type ISOOUTDROPMSK_R = crate::BitReader;
///Field `ISOOUTDROPMSK` writer -
pub type ISOOUTDROPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EOPFMSK` reader -
pub type EOPFMSK_R = crate::BitReader;
///Field `EOPFMSK` writer -
pub type EOPFMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPMISMSK` reader -
pub type EPMISMSK_R = crate::BitReader;
///Field `EPMISMSK` writer -
pub type EPMISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEPINTMSK` reader -
pub type IEPINTMSK_R = crate::BitReader;
///Field `IEPINTMSK` writer -
pub type IEPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OEPINTMSK` reader -
pub type OEPINTMSK_R = crate::BitReader;
///Field `OEPINTMSK` writer -
pub type OEPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCOMPISOINMSK` reader -
pub type INCOMPISOINMSK_R = crate::BitReader;
///Field `INCOMPISOINMSK` writer -
pub type INCOMPISOINMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCOMPIPMSK` reader -
pub type INCOMPIPMSK_R = crate::BitReader;
///Field `INCOMPIPMSK` writer -
pub type INCOMPIPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FETSUSPMSK` reader -
pub type FETSUSPMSK_R = crate::BitReader;
///Field `FETSUSPMSK` writer -
pub type FETSUSPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RESETDETMSK` reader -
pub type RESETDETMSK_R = crate::BitReader;
///Field `RESETDETMSK` writer -
pub type RESETDETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRTLNTMSK` reader -
pub type PRTLNTMSK_R = crate::BitReader;
///Field `PRTLNTMSK` writer -
pub type PRTLNTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HCHINTMSK` reader -
pub type HCHINTMSK_R = crate::BitReader;
///Field `HCHINTMSK` writer -
pub type HCHINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTXFEMPMSK` reader -
pub type PTXFEMPMSK_R = crate::BitReader;
///Field `PTXFEMPMSK` writer -
pub type PTXFEMPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CONIDSTSCHNGMSK` reader -
pub type CONIDSTSCHNGMSK_R = crate::BitReader;
///Field `CONIDSTSCHNGMSK` writer -
pub type CONIDSTSCHNGMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DISCONNINTMSK` reader -
pub type DISCONNINTMSK_R = crate::BitReader;
///Field `DISCONNINTMSK` writer -
pub type DISCONNINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SESSREQINTMSK` reader -
pub type SESSREQINTMSK_R = crate::BitReader;
///Field `SESSREQINTMSK` writer -
pub type SESSREQINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUPINTMSK` reader -
pub type WKUPINTMSK_R = crate::BitReader;
///Field `WKUPINTMSK` writer -
pub type WKUPINTMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 1
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn otgintmsk(&self) -> OTGINTMSK_R {
        OTGINTMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn rxflvimsk(&self) -> RXFLVIMSK_R {
        RXFLVIMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NPTXFEMPMSK_R {
        NPTXFEMPMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn goutnackeffmsk(&self) -> GOUTNACKEFFMSK_R {
        GOUTNACKEFFMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn epmismsk(&self) -> EPMISMSK_R {
        EPMISMSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn iepintmsk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19
    #[inline(always)]
    pub fn oepintmsk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> INCOMPISOINMSK_R {
        INCOMPISOINMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn incompipmsk(&self) -> INCOMPIPMSK_R {
        INCOMPIPMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FETSUSPMSK_R {
        FETSUSPMSK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23
    #[inline(always)]
    pub fn resetdetmsk(&self) -> RESETDETMSK_R {
        RESETDETMSK_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24
    #[inline(always)]
    pub fn prtlntmsk(&self) -> PRTLNTMSK_R {
        PRTLNTMSK_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25
    #[inline(always)]
    pub fn hchintmsk(&self) -> HCHINTMSK_R {
        HCHINTMSK_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PTXFEMPMSK_R {
        PTXFEMPMSK_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 28
    #[inline(always)]
    pub fn conidstschngmsk(&self) -> CONIDSTSCHNGMSK_R {
        CONIDSTSCHNGMSK_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29
    #[inline(always)]
    pub fn disconnintmsk(&self) -> DISCONNINTMSK_R {
        DISCONNINTMSK_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30
    #[inline(always)]
    pub fn sessreqintmsk(&self) -> SESSREQINTMSK_R {
        SESSREQINTMSK_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK")
            .field("modemismsk", &self.modemismsk())
            .field("otgintmsk", &self.otgintmsk())
            .field("sofmsk", &self.sofmsk())
            .field("rxflvimsk", &self.rxflvimsk())
            .field("nptxfempmsk", &self.nptxfempmsk())
            .field("ginnakeffmsk", &self.ginnakeffmsk())
            .field("goutnackeffmsk", &self.goutnackeffmsk())
            .field("erlysuspmsk", &self.erlysuspmsk())
            .field("usbsuspmsk", &self.usbsuspmsk())
            .field("usbrstmsk", &self.usbrstmsk())
            .field("enumdonemsk", &self.enumdonemsk())
            .field("isooutdropmsk", &self.isooutdropmsk())
            .field("eopfmsk", &self.eopfmsk())
            .field("epmismsk", &self.epmismsk())
            .field("iepintmsk", &self.iepintmsk())
            .field("oepintmsk", &self.oepintmsk())
            .field("incompisoinmsk", &self.incompisoinmsk())
            .field("incompipmsk", &self.incompipmsk())
            .field("fetsuspmsk", &self.fetsuspmsk())
            .field("resetdetmsk", &self.resetdetmsk())
            .field("prtlntmsk", &self.prtlntmsk())
            .field("hchintmsk", &self.hchintmsk())
            .field("ptxfempmsk", &self.ptxfempmsk())
            .field("conidstschngmsk", &self.conidstschngmsk())
            .field("disconnintmsk", &self.disconnintmsk())
            .field("sessreqintmsk", &self.sessreqintmsk())
            .field("wkupintmsk", &self.wkupintmsk())
            .finish()
    }
}
impl W {
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W<GINTMSK_SPEC> {
        MODEMISMSK_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn otgintmsk(&mut self) -> OTGINTMSK_W<GINTMSK_SPEC> {
        OTGINTMSK_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn sofmsk(&mut self) -> SOFMSK_W<GINTMSK_SPEC> {
        SOFMSK_W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn rxflvimsk(&mut self) -> RXFLVIMSK_W<GINTMSK_SPEC> {
        RXFLVIMSK_W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    #[must_use]
    pub fn nptxfempmsk(&mut self) -> NPTXFEMPMSK_W<GINTMSK_SPEC> {
        NPTXFEMPMSK_W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    #[must_use]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W<GINTMSK_SPEC> {
        GINNAKEFFMSK_W::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    #[must_use]
    pub fn goutnackeffmsk(&mut self) -> GOUTNACKEFFMSK_W<GINTMSK_SPEC> {
        GOUTNACKEFFMSK_W::new(self, 7)
    }
    ///Bit 10
    #[inline(always)]
    #[must_use]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W<GINTMSK_SPEC> {
        ERLYSUSPMSK_W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    #[must_use]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W<GINTMSK_SPEC> {
        USBSUSPMSK_W::new(self, 11)
    }
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W<GINTMSK_SPEC> {
        USBRSTMSK_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W<GINTMSK_SPEC> {
        ENUMDONEMSK_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    #[must_use]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W<GINTMSK_SPEC> {
        ISOOUTDROPMSK_W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    #[must_use]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<GINTMSK_SPEC> {
        EOPFMSK_W::new(self, 15)
    }
    ///Bit 17
    #[inline(always)]
    #[must_use]
    pub fn epmismsk(&mut self) -> EPMISMSK_W<GINTMSK_SPEC> {
        EPMISMSK_W::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    #[must_use]
    pub fn iepintmsk(&mut self) -> IEPINTMSK_W<GINTMSK_SPEC> {
        IEPINTMSK_W::new(self, 18)
    }
    ///Bit 19
    #[inline(always)]
    #[must_use]
    pub fn oepintmsk(&mut self) -> OEPINTMSK_W<GINTMSK_SPEC> {
        OEPINTMSK_W::new(self, 19)
    }
    ///Bit 20
    #[inline(always)]
    #[must_use]
    pub fn incompisoinmsk(&mut self) -> INCOMPISOINMSK_W<GINTMSK_SPEC> {
        INCOMPISOINMSK_W::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    #[must_use]
    pub fn incompipmsk(&mut self) -> INCOMPIPMSK_W<GINTMSK_SPEC> {
        INCOMPIPMSK_W::new(self, 21)
    }
    ///Bit 22
    #[inline(always)]
    #[must_use]
    pub fn fetsuspmsk(&mut self) -> FETSUSPMSK_W<GINTMSK_SPEC> {
        FETSUSPMSK_W::new(self, 22)
    }
    ///Bit 23
    #[inline(always)]
    #[must_use]
    pub fn resetdetmsk(&mut self) -> RESETDETMSK_W<GINTMSK_SPEC> {
        RESETDETMSK_W::new(self, 23)
    }
    ///Bit 24
    #[inline(always)]
    #[must_use]
    pub fn prtlntmsk(&mut self) -> PRTLNTMSK_W<GINTMSK_SPEC> {
        PRTLNTMSK_W::new(self, 24)
    }
    ///Bit 25
    #[inline(always)]
    #[must_use]
    pub fn hchintmsk(&mut self) -> HCHINTMSK_W<GINTMSK_SPEC> {
        HCHINTMSK_W::new(self, 25)
    }
    ///Bit 26
    #[inline(always)]
    #[must_use]
    pub fn ptxfempmsk(&mut self) -> PTXFEMPMSK_W<GINTMSK_SPEC> {
        PTXFEMPMSK_W::new(self, 26)
    }
    ///Bit 28
    #[inline(always)]
    #[must_use]
    pub fn conidstschngmsk(&mut self) -> CONIDSTSCHNGMSK_W<GINTMSK_SPEC> {
        CONIDSTSCHNGMSK_W::new(self, 28)
    }
    ///Bit 29
    #[inline(always)]
    #[must_use]
    pub fn disconnintmsk(&mut self) -> DISCONNINTMSK_W<GINTMSK_SPEC> {
        DISCONNINTMSK_W::new(self, 29)
    }
    ///Bit 30
    #[inline(always)]
    #[must_use]
    pub fn sessreqintmsk(&mut self) -> SESSREQINTMSK_W<GINTMSK_SPEC> {
        SESSREQINTMSK_W::new(self, 30)
    }
    ///Bit 31
    #[inline(always)]
    #[must_use]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W<GINTMSK_SPEC> {
        WKUPINTMSK_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gintmsk::R`](R) reader structure
impl crate::Readable for GINTMSK_SPEC {}
///`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure
impl crate::Writable for GINTMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GINTMSK to value 0
impl crate::Resettable for GINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
