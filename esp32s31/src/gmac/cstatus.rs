#[doc = "Register `CSTATUS` reader"]
pub type R = crate::R<CSTATUS_SPEC>;
#[doc = "Field `LNKMOD` reader - Link Mode This bit indicates the current mode of operation of the link: 1’b0: Halfduplex mode 1’b1: Fullduplex mode"]
pub type LNKMOD_R = crate::BitReader;
#[doc = "Field `LNKSPEED` reader - Link Speed"]
pub type LNKSPEED_R = crate::FieldReader;
#[doc = "Field `LNKSTS` reader - Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link"]
pub type LNKSTS_R = crate::BitReader;
#[doc = "Field `JABTO` reader - Jabber Timeout This bit indicates whether there is jabber timeout error _1'b1_ in the received frame This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface"]
pub type JABTO_R = crate::BitReader;
#[doc = "Field `FALSCARDET` reader - False Carrier Detected This bit indicates whether the SMII PHY detected false carrier _1'b1_ This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface"]
pub type FALSCARDET_R = crate::BitReader;
#[doc = "Field `SMIDRXS` reader - Delay SMII RX Data Sampling with respect to the SMII SYNC Signal When set, the first bit of the SMII RX data is sampled one cycle after the SMII SYNC signal When reset, the first bit of the SMII RX data is sampled along with the SMII SYNC signal If the SMII PHY Interface with source synchronous mode is selected during core configuration, this bit is reserved _RO with default value_"]
pub type SMIDRXS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Link Mode This bit indicates the current mode of operation of the link: 1’b0: Halfduplex mode 1’b1: Fullduplex mode"]
    #[inline(always)]
    pub fn lnkmod(&self) -> LNKMOD_R {
        LNKMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Link Speed"]
    #[inline(always)]
    pub fn lnkspeed(&self) -> LNKSPEED_R {
        LNKSPEED_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Link Status This bit indicates whether the link between the local PHY and the remote PHY is up or down It gives the status of the link between the SGMII of MAC and the SGMII of the local PHY The status bits are received from the local PHY during ANEG betweent he MAC and PHY on the SGMII link"]
    #[inline(always)]
    pub fn lnksts(&self) -> LNKSTS_R {
        LNKSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Jabber Timeout This bit indicates whether there is jabber timeout error _1'b1_ in the received frame This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface"]
    #[inline(always)]
    pub fn jabto(&self) -> JABTO_R {
        JABTO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - False Carrier Detected This bit indicates whether the SMII PHY detected false carrier _1'b1_ This bit is reserved when the MAC is configured for the SGMII or RGMII PHY interface"]
    #[inline(always)]
    pub fn falscardet(&self) -> FALSCARDET_R {
        FALSCARDET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - Delay SMII RX Data Sampling with respect to the SMII SYNC Signal When set, the first bit of the SMII RX data is sampled one cycle after the SMII SYNC signal When reset, the first bit of the SMII RX data is sampled along with the SMII SYNC signal If the SMII PHY Interface with source synchronous mode is selected during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn smidrxs(&self) -> SMIDRXS_R {
        SMIDRXS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSTATUS")
            .field("lnkmod", &self.lnkmod())
            .field("lnkspeed", &self.lnkspeed())
            .field("lnksts", &self.lnksts())
            .field("jabto", &self.jabto())
            .field("falscardet", &self.falscardet())
            .field("smidrxs", &self.smidrxs())
            .finish()
    }
}
#[doc = "Link communication status\n\nYou can [`read`](crate::Reg::read) this register and get [`cstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSTATUS_SPEC;
impl crate::RegisterSpec for CSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cstatus::R`](R) reader structure"]
impl crate::Readable for CSTATUS_SPEC {}
#[doc = "`reset()` method sets CSTATUS to value 0x04"]
impl crate::Resettable for CSTATUS_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
