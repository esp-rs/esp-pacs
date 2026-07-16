#[doc = "Register `REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER` reader"]
pub type R = crate::R<REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER_SPEC>;
#[doc = "Field `FD_ABILITY` reader - FullDuplex When set, this bit indicates that the link partner has the ability to operate in the full duplex mode When cleared, this bit indicates that the link partner does not have the ability to operate in the fullduplex mode"]
pub type FD_ABILITY_R = crate::BitReader;
#[doc = "Field `HD_ABILITY` reader - HalfDuplex When set, this bit indicates that the link partner has the ability to operate in the halfduplex mode When cleared, this bit indicates that the link partner does not have the ability to operate in the halfduplex mode"]
pub type HD_ABILITY_R = crate::BitReader;
#[doc = "Field `PSE_ABILITY` reader - Pause Encoding These bits provide an encoding for the Pause bits, indicating that the link partner's capability of configuring the Pause function as defined in the IEEE 8023x specification The encoding of these bits is defined in IEEE 8023z, Section 37214"]
pub type PSE_ABILITY_R = crate::FieldReader;
#[doc = "Field `RFE_ABILITY` reader - Remote Fault Encoding These bits provide a remote fault encoding, indicating a fault or error condition of the link partner The encoding of these bits is defined in IEEE 8023z, Section 37215"]
pub type RFE_ABILITY_R = crate::FieldReader;
#[doc = "Field `ACK` reader - Acknowledge When set, the autonegotiation function uses this bit to indicate that the link partner has successfully received the base page of the MAC When cleared, it indicates that the link partner did not successfully receive the base page of the MAC"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `NO` reader - Next Page Support When set, this bit indicates that more next page information is available When cleared, this bit indicates that next page exchange is not desired"]
pub type NO_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - FullDuplex When set, this bit indicates that the link partner has the ability to operate in the full duplex mode When cleared, this bit indicates that the link partner does not have the ability to operate in the fullduplex mode"]
    #[inline(always)]
    pub fn fd_ability(&self) -> FD_ABILITY_R {
        FD_ABILITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HalfDuplex When set, this bit indicates that the link partner has the ability to operate in the halfduplex mode When cleared, this bit indicates that the link partner does not have the ability to operate in the halfduplex mode"]
    #[inline(always)]
    pub fn hd_ability(&self) -> HD_ABILITY_R {
        HD_ABILITY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Pause Encoding These bits provide an encoding for the Pause bits, indicating that the link partner's capability of configuring the Pause function as defined in the IEEE 8023x specification The encoding of these bits is defined in IEEE 8023z, Section 37214"]
    #[inline(always)]
    pub fn pse_ability(&self) -> PSE_ABILITY_R {
        PSE_ABILITY_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Remote Fault Encoding These bits provide a remote fault encoding, indicating a fault or error condition of the link partner The encoding of these bits is defined in IEEE 8023z, Section 37215"]
    #[inline(always)]
    pub fn rfe_ability(&self) -> RFE_ABILITY_R {
        RFE_ABILITY_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Acknowledge When set, the autonegotiation function uses this bit to indicate that the link partner has successfully received the base page of the MAC When cleared, it indicates that the link partner did not successfully receive the base page of the MAC"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Next Page Support When set, this bit indicates that more next page information is available When cleared, this bit indicates that next page exchange is not desired"]
    #[inline(always)]
    pub fn no(&self) -> NO_R {
        NO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER")
            .field("fd_ability", &self.fd_ability())
            .field("hd_ability", &self.hd_ability())
            .field("pse_ability", &self.pse_ability())
            .field("rfe_ability", &self.rfe_ability())
            .field("ack", &self.ack())
            .field("no", &self.no())
            .finish()
    }
}
#[doc = "Contains the advertised ability of the link partner Its value is valid after successful completion of autonegotiation or when a new base page has been received _indicated in the AutoNegotiation Expansion Register_ This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register51_autonegotiationlinkpartnerabilityregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register51_autonegotiationlinkpartnerabilityregister::R`](R) reader structure"]
impl crate::Readable for REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER to value 0"]
impl crate::Resettable for REGISTER51_AUTONEGOTIATIONLINKPARTNERABILITYREGISTER_SPEC {}
