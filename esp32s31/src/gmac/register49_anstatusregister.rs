#[doc = "Register `REGISTER49_ANSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER49_ANSTATUSREGISTER_SPEC>;
#[doc = "Field `LS` reader - Link Status This bit indicates whether the data channel _link_ is up or down For the TBI, RTBI or SGMII interfaces, if ANEG is going on, data cannot be transferred across the link and hence the link is given as down"]
pub type LS_R = crate::BitReader;
#[doc = "Field `ANA` reader - AutoNegotiation Ability This bit is always high because the MAC supports auto negotiation"]
pub type ANA_R = crate::BitReader;
#[doc = "Field `ANC` reader - AutoNegotiation Complete When set, this bit indicates that the autonegotiation process is complete This bit is cleared when autonegotiation is reinitiated"]
pub type ANC_R = crate::BitReader;
#[doc = "Field `ES` reader - Extended Status This bit is tied to high if the TBI or RTBI interface is selected during core configuration indicating that the MAC supports extended status information in Register 53 _TBI Extended Status Register_ This bit is tied to low if the SGMII interface is selected and the TBI or RTBI interface is not selected during core configuration indicating that Register 53 is not present"]
pub type ES_R = crate::BitReader;
impl R {
    #[doc = "Bit 2 - Link Status This bit indicates whether the data channel _link_ is up or down For the TBI, RTBI or SGMII interfaces, if ANEG is going on, data cannot be transferred across the link and hence the link is given as down"]
    #[inline(always)]
    pub fn ls(&self) -> LS_R {
        LS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AutoNegotiation Ability This bit is always high because the MAC supports auto negotiation"]
    #[inline(always)]
    pub fn ana(&self) -> ANA_R {
        ANA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - AutoNegotiation Complete When set, this bit indicates that the autonegotiation process is complete This bit is cleared when autonegotiation is reinitiated"]
    #[inline(always)]
    pub fn anc(&self) -> ANC_R {
        ANC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Extended Status This bit is tied to high if the TBI or RTBI interface is selected during core configuration indicating that the MAC supports extended status information in Register 53 _TBI Extended Status Register_ This bit is tied to low if the SGMII interface is selected and the TBI or RTBI interface is not selected during core configuration indicating that Register 53 is not present"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER49_ANSTATUSREGISTER")
            .field("ls", &self.ls())
            .field("ana", &self.ana())
            .field("anc", &self.anc())
            .field("es", &self.es())
            .finish()
    }
}
#[doc = "Indicates the link and autonegotiation status This register is present only when you select the TBI, RTBI, or SGMII interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register49_anstatusregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER49_ANSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER49_ANSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register49_anstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER49_ANSTATUSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER49_ANSTATUSREGISTER to value 0x0108"]
impl crate::Resettable for REGISTER49_ANSTATUSREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x0108;
}
