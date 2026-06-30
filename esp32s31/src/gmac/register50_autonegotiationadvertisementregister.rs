#[doc = "Register `REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER` reader"]
pub type R = crate::R<REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC>;
#[doc = "Register `REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER` writer"]
pub type W = crate::W<REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC>;
#[doc = "Field `FD` reader - FullDuplex When set high, this bit indicates that the MAC supports the fullduplex mode"]
pub type FD_R = crate::BitReader;
#[doc = "Field `FD` writer - FullDuplex When set high, this bit indicates that the MAC supports the fullduplex mode"]
pub type FD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HD` reader - HalfDuplex When set high, this bit indicates that the MAC supports the halfduplex mode This bit is always low _and RO_ when the MAC is configured for the fullduplexonly mode"]
pub type HD_R = crate::BitReader;
#[doc = "Field `HD` writer - HalfDuplex When set high, this bit indicates that the MAC supports the halfduplex mode This bit is always low _and RO_ when the MAC is configured for the fullduplexonly mode"]
pub type HD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSE` reader - Pause Encoding These bits provide an encoding for the Pause bits, indicating that the MAC is capable of configuring the Pause function as defined in IEEE 8023x The encoding of these bits is defined in IEEE 8023z, Section 37214"]
pub type PSE_R = crate::FieldReader;
#[doc = "Field `PSE` writer - Pause Encoding These bits provide an encoding for the Pause bits, indicating that the MAC is capable of configuring the Pause function as defined in IEEE 8023x The encoding of these bits is defined in IEEE 8023z, Section 37214"]
pub type PSE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RFE` reader - Remote Fault Encoding These bits provide a remote fault encoding, indicating to a link partner that a fault or error condition has occurred The encoding of these bits is defined in IEEE 8023z, Section 37215"]
pub type RFE_R = crate::FieldReader;
#[doc = "Field `RFE` writer - Remote Fault Encoding These bits provide a remote fault encoding, indicating to a link partner that a fault or error condition has occurred The encoding of these bits is defined in IEEE 8023z, Section 37215"]
pub type RFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NP` reader - Next Page Support This bit is always low because the MAC does not support the next page"]
pub type NP_R = crate::BitReader;
impl R {
    #[doc = "Bit 5 - FullDuplex When set high, this bit indicates that the MAC supports the fullduplex mode"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HalfDuplex When set high, this bit indicates that the MAC supports the halfduplex mode This bit is always low _and RO_ when the MAC is configured for the fullduplexonly mode"]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Pause Encoding These bits provide an encoding for the Pause bits, indicating that the MAC is capable of configuring the Pause function as defined in IEEE 8023x The encoding of these bits is defined in IEEE 8023z, Section 37214"]
    #[inline(always)]
    pub fn pse(&self) -> PSE_R {
        PSE_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Remote Fault Encoding These bits provide a remote fault encoding, indicating to a link partner that a fault or error condition has occurred The encoding of these bits is defined in IEEE 8023z, Section 37215"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Next Page Support This bit is always low because the MAC does not support the next page"]
    #[inline(always)]
    pub fn np(&self) -> NP_R {
        NP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER")
            .field("fd", &self.fd())
            .field("hd", &self.hd())
            .field("pse", &self.pse())
            .field("rfe", &self.rfe())
            .field("np", &self.np())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - FullDuplex When set high, this bit indicates that the MAC supports the fullduplex mode"]
    #[inline(always)]
    pub fn fd(&mut self) -> FD_W<'_, REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC> {
        FD_W::new(self, 5)
    }
    #[doc = "Bit 6 - HalfDuplex When set high, this bit indicates that the MAC supports the halfduplex mode This bit is always low _and RO_ when the MAC is configured for the fullduplexonly mode"]
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W<'_, REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC> {
        HD_W::new(self, 6)
    }
    #[doc = "Bits 7:8 - Pause Encoding These bits provide an encoding for the Pause bits, indicating that the MAC is capable of configuring the Pause function as defined in IEEE 8023x The encoding of these bits is defined in IEEE 8023z, Section 37214"]
    #[inline(always)]
    pub fn pse(&mut self) -> PSE_W<'_, REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC> {
        PSE_W::new(self, 7)
    }
    #[doc = "Bits 12:13 - Remote Fault Encoding These bits provide a remote fault encoding, indicating to a link partner that a fault or error condition has occurred The encoding of these bits is defined in IEEE 8023z, Section 37215"]
    #[inline(always)]
    pub fn rfe(&mut self) -> RFE_W<'_, REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC> {
        RFE_W::new(self, 12)
    }
}
#[doc = "This register is configured before autonegotiation begins It contains the advertised ability of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register50_autonegotiationadvertisementregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register50_autonegotiationadvertisementregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register50_autonegotiationadvertisementregister::R`](R) reader structure"]
impl crate::Readable for REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register50_autonegotiationadvertisementregister::W`](W) writer structure"]
impl crate::Writable for REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER to value 0x01e0"]
impl crate::Resettable for REGISTER50_AUTONEGOTIATIONADVERTISEMENTREGISTER_SPEC {
    const RESET_VALUE: u32 = 0x01e0;
}
