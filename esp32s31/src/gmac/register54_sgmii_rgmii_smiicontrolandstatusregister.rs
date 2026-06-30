#[doc = "Register `REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC>;
#[doc = "Register `REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER` writer"]
pub type W = crate::W<REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC>;
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
#[doc = "Field `SMIDRXS` writer - Delay SMII RX Data Sampling with respect to the SMII SYNC Signal When set, the first bit of the SMII RX data is sampled one cycle after the SMII SYNC signal When reset, the first bit of the SMII RX data is sampled along with the SMII SYNC signal If the SMII PHY Interface with source synchronous mode is selected during core configuration, this bit is reserved _RO with default value_"]
pub type SMIDRXS_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        f.debug_struct("REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER")
            .field("lnkmod", &self.lnkmod())
            .field("lnkspeed", &self.lnkspeed())
            .field("lnksts", &self.lnksts())
            .field("jabto", &self.jabto())
            .field("falscardet", &self.falscardet())
            .field("smidrxs", &self.smidrxs())
            .finish()
    }
}
impl W {
    #[doc = "Bit 16 - Delay SMII RX Data Sampling with respect to the SMII SYNC Signal When set, the first bit of the SMII RX data is sampled one cycle after the SMII SYNC signal When reset, the first bit of the SMII RX data is sampled along with the SMII SYNC signal If the SMII PHY Interface with source synchronous mode is selected during core configuration, this bit is reserved _RO with default value_"]
    #[inline(always)]
    pub fn smidrxs(
        &mut self,
    ) -> SMIDRXS_W<'_, REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC> {
        SMIDRXS_W::new(self, 16)
    }
}
#[doc = "Indicates the status signals received from the PHY through the SGMII, RGMII, or SMII interface This register is present only when you select the SGMII, RGMII, or SMII interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register54_sgmii_rgmii_smiicontrolandstatusregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register54_sgmii_rgmii_smiicontrolandstatusregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register54_sgmii_rgmii_smiicontrolandstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register54_sgmii_rgmii_smiicontrolandstatusregister::W`](W) writer structure"]
impl crate::Writable for REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER to value 0x04"]
impl crate::Resettable for REGISTER54_SGMII_RGMII_SMIICONTROLANDSTATUSREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
