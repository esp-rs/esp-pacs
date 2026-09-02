#[doc = "Register `LPI_CRS` reader"]
pub type R = crate::R<LPI_CRS_SPEC>;
#[doc = "Field `TLPIEN` reader - Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit This bit is cleared by a read into this register"]
pub type TLPIEN_R = crate::BitReader;
#[doc = "Field `TLPIEX` reader - Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired This bit is cleared by a read into this register"]
pub type TLPIEX_R = crate::BitReader;
#[doc = "Field `RLPIEN` reader - Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state This bit is cleared by a read into this register Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock"]
pub type RLPIEN_R = crate::BitReader;
#[doc = "Field `RLPIEX` reader - Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception This bit is cleared by a read into this register Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock"]
pub type RLPIEX_R = crate::BitReader;
#[doc = "Field `TLPIST` reader - Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface"]
pub type TLPIST_R = crate::BitReader;
#[doc = "Field `RLPIST` reader - Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface"]
pub type RLPIST_R = crate::BitReader;
#[doc = "Field `LPIEN` reader - LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission"]
pub type LPIEN_R = crate::BitReader;
#[doc = "Field `PLS` reader - PHY Link Status This bit indicates the link status of the PHY The MAC Transmitter asserts the LPI pattern only when the link status is up _okay_ at least for the time indicated by the LPI LS TIMER When set, the link is considered to be okay _up_ and when reset, the link is considered to be down"]
pub type PLS_R = crate::BitReader;
#[doc = "Field `PLSEN` reader - PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER When set, the MAC uses the linkstatus bits of Register 54 _SGMII/RGMII/SMII Control and Status Register_ and Bit 17 _PLS_ for the LPI LS Timer trigger When cleared, the MAC ignores the linkstatus bits of Register 54 and takes only the PLS bit This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface"]
pub type PLSEN_R = crate::BitReader;
#[doc = "Field `LPITXA` reader - LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side This bit is not functional in the GMAC CORE configuration in which the Tx clock gating is done during the LPI mode If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames _in the core_ and pending frames _in the application interface_ have been transmitted The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state If TX FIFO Flush is set in Bit 20 of Register 6 _Operation Mode Register_, when the MAC is in the LPI mode, the MAC exits the LPI mode When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode"]
pub type LPITXA_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry When set, this bit indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit This bit is cleared by a read into this register"]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit When set, this bit indicates that the MAC transmitter has exited the LPI state after the user has cleared the LPIEN bit and the LPI TW Timer has expired This bit is cleared by a read into this register"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry When set, this bit indicates that the MAC Receiver has received an LPI pattern and entered the LPI state This bit is cleared by a read into this register Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock"]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit When set, this bit indicates that the MAC Receiver has stopped receiving the LPI pattern on the GMII or MII interface, exited the LPI state, and resumed the normal reception This bit is cleared by a read into this register Note: This bit may not get set if the MAC stops receiving the LPI pattern for a very short duration, such as, less than 3 clock cycles of CSR clock"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State When set, this bit indicates that the MAC is transmitting the LPI pattern on the GMII or MII interface"]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State When set, this bit indicates that the MAC is receiving the LPI pattern on the GMII or MII interface"]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPI Enable When set, this bit instructs the MAC Transmitter to enter the LPI state When reset, this bit instructs the MAC to exit the LPI state and resume normal transmission This bit is cleared when the LPITXA bit is set and the MAC exits the LPI state because of the arrival of a new packet for transmission"]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY The MAC Transmitter asserts the LPI pattern only when the link status is up _okay_ at least for the time indicated by the LPI LS TIMER When set, the link is considered to be okay _up_ and when reset, the link is considered to be down"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PHY Link Status Enable This bit enables the link status received on the RGMII, SGMII, or SMII receive paths to be used for activating the LPI LS TIMER When set, the MAC uses the linkstatus bits of Register 54 _SGMII/RGMII/SMII Control and Status Register_ and Bit 17 _PLS_ for the LPI LS Timer trigger When cleared, the MAC ignores the linkstatus bits of Register 54 and takes only the PLS bit This bit is RO and reserved if you have not selected the RGMII, SGMII, or SMII PHY interface"]
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPI TX Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the transmit side This bit is not functional in the GMAC CORE configuration in which the Tx clock gating is done during the LPI mode If the LPITXA and LPIEN bits are set to 1, the MAC enters the LPI mode only after all outstanding frames _in the core_ and pending frames _in the application interface_ have been transmitted The MAC comes out of the LPI mode when the application sends any frame for transmission or the application issues a TX FIFO Flush command In addition, the MAC automatically clears the LPIEN bit when it exits the LPI state If TX FIFO Flush is set in Bit 20 of Register 6 _Operation Mode Register_, when the MAC is in the LPI mode, the MAC exits the LPI mode When this bit is 0, the LPIEN bit directly controls behavior of the MAC when it is entering or coming out of the LPI mode"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPI_CRS")
            .field("tlpien", &self.tlpien())
            .field("tlpiex", &self.tlpiex())
            .field("rlpien", &self.rlpien())
            .field("rlpiex", &self.rlpiex())
            .field("tlpist", &self.tlpist())
            .field("rlpist", &self.rlpist())
            .field("lpien", &self.lpien())
            .field("pls", &self.pls())
            .field("plsen", &self.plsen())
            .field("lpitxa", &self.lpitxa())
            .finish()
    }
}
#[doc = "LPI Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`lpi_crs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPI_CRS_SPEC;
impl crate::RegisterSpec for LPI_CRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpi_crs::R`](R) reader structure"]
impl crate::Readable for LPI_CRS_SPEC {}
#[doc = "`reset()` method sets LPI_CRS to value 0"]
impl crate::Resettable for LPI_CRS_SPEC {}
