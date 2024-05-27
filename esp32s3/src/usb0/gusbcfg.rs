///Register `GUSBCFG` reader
pub type R = crate::R<GUSBCFG_SPEC>;
///Register `GUSBCFG` writer
pub type W = crate::W<GUSBCFG_SPEC>;
///Field `TOUTCAL` reader -
pub type TOUTCAL_R = crate::FieldReader;
///Field `TOUTCAL` writer -
pub type TOUTCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PHYIF` reader -
pub type PHYIF_R = crate::BitReader;
///Field `PHYIF` writer -
pub type PHYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULPI_UTMI_SEL` reader -
pub type ULPI_UTMI_SEL_R = crate::BitReader;
///Field `FSINTF` reader -
pub type FSINTF_R = crate::BitReader;
///Field `FSINTF` writer -
pub type FSINTF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PHYSEL` reader -
pub type PHYSEL_R = crate::BitReader;
///Field `SRPCAP` reader -
pub type SRPCAP_R = crate::BitReader;
///Field `SRPCAP` writer -
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HNPCAP` reader -
pub type HNPCAP_R = crate::BitReader;
///Field `HNPCAP` writer -
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBTRDTIM` reader -
pub type USBTRDTIM_R = crate::FieldReader;
///Field `USBTRDTIM` writer -
pub type USBTRDTIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TERMSELDLPULSE` reader -
pub type TERMSELDLPULSE_R = crate::BitReader;
///Field `TERMSELDLPULSE` writer -
pub type TERMSELDLPULSE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXENDDELAY` reader -
pub type TXENDDELAY_R = crate::BitReader;
///Field `TXENDDELAY` writer -
pub type TXENDDELAY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCEHSTMODE` reader -
pub type FORCEHSTMODE_R = crate::BitReader;
///Field `FORCEHSTMODE` writer -
pub type FORCEHSTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FORCEDEVMODE` reader -
pub type FORCEDEVMODE_R = crate::BitReader;
///Field `FORCEDEVMODE` writer -
pub type FORCEDEVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CORRUPTTXPKT` reader -
pub type CORRUPTTXPKT_R = crate::BitReader;
///Field `CORRUPTTXPKT` writer -
pub type CORRUPTTXPKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3
    #[inline(always)]
    pub fn phyif(&self) -> PHYIF_R {
        PHYIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn ulpi_utmi_sel(&self) -> ULPI_UTMI_SEL_R {
        ULPI_UTMI_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn fsintf(&self) -> FSINTF_R {
        FSINTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:13
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    ///Bit 22
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 28
    #[inline(always)]
    pub fn txenddelay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29
    #[inline(always)]
    pub fn forcehstmode(&self) -> FORCEHSTMODE_R {
        FORCEHSTMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30
    #[inline(always)]
    pub fn forcedevmode(&self) -> FORCEDEVMODE_R {
        FORCEDEVMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn corrupttxpkt(&self) -> CORRUPTTXPKT_R {
        CORRUPTTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("toutcal", &self.toutcal())
            .field("phyif", &self.phyif())
            .field("ulpi_utmi_sel", &self.ulpi_utmi_sel())
            .field("fsintf", &self.fsintf())
            .field("physel", &self.physel())
            .field("srpcap", &self.srpcap())
            .field("hnpcap", &self.hnpcap())
            .field("usbtrdtim", &self.usbtrdtim())
            .field("termseldlpulse", &self.termseldlpulse())
            .field("txenddelay", &self.txenddelay())
            .field("forcehstmode", &self.forcehstmode())
            .field("forcedevmode", &self.forcedevmode())
            .field("corrupttxpkt", &self.corrupttxpkt())
            .finish()
    }
}
impl W {
    ///Bits 0:2
    #[inline(always)]
    #[must_use]
    pub fn toutcal(&mut self) -> TOUTCAL_W<GUSBCFG_SPEC> {
        TOUTCAL_W::new(self, 0)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn phyif(&mut self) -> PHYIF_W<GUSBCFG_SPEC> {
        PHYIF_W::new(self, 3)
    }
    ///Bit 5
    #[inline(always)]
    #[must_use]
    pub fn fsintf(&mut self) -> FSINTF_W<GUSBCFG_SPEC> {
        FSINTF_W::new(self, 5)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn srpcap(&mut self) -> SRPCAP_W<GUSBCFG_SPEC> {
        SRPCAP_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    #[must_use]
    pub fn hnpcap(&mut self) -> HNPCAP_W<GUSBCFG_SPEC> {
        HNPCAP_W::new(self, 9)
    }
    ///Bits 10:13
    #[inline(always)]
    #[must_use]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W<GUSBCFG_SPEC> {
        USBTRDTIM_W::new(self, 10)
    }
    ///Bit 22
    #[inline(always)]
    #[must_use]
    pub fn termseldlpulse(&mut self) -> TERMSELDLPULSE_W<GUSBCFG_SPEC> {
        TERMSELDLPULSE_W::new(self, 22)
    }
    ///Bit 28
    #[inline(always)]
    #[must_use]
    pub fn txenddelay(&mut self) -> TXENDDELAY_W<GUSBCFG_SPEC> {
        TXENDDELAY_W::new(self, 28)
    }
    ///Bit 29
    #[inline(always)]
    #[must_use]
    pub fn forcehstmode(&mut self) -> FORCEHSTMODE_W<GUSBCFG_SPEC> {
        FORCEHSTMODE_W::new(self, 29)
    }
    ///Bit 30
    #[inline(always)]
    #[must_use]
    pub fn forcedevmode(&mut self) -> FORCEDEVMODE_W<GUSBCFG_SPEC> {
        FORCEDEVMODE_W::new(self, 30)
    }
    ///Bit 31
    #[inline(always)]
    #[must_use]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W<GUSBCFG_SPEC> {
        CORRUPTTXPKT_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gusbcfg::R`](R) reader structure
impl crate::Readable for GUSBCFG_SPEC {}
///`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure
impl crate::Writable for GUSBCFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GUSBCFG to value 0x1440
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: u32 = 0x1440;
}
