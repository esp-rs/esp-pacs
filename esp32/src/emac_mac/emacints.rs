#[doc = "Register `EMACINTS` reader"]
pub type R = crate::R<EMACINTS_SPEC>;
#[doc = "Field `PMTINTS` reader - This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bit\\[5\\] and Bit\\[6\\] in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\] are cleared because of a read operation to the PMT Control and Status register. This bit is valid only when you select the optional PMT module during core configuration."]
pub type PMTINTS_R = crate::BitReader;
#[doc = "Field `LPIIS` reader - When the Energy Efficient Ethernet feature is enabled this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit\\[0\\] of Register (LPI Control and Status Register)."]
pub type LPIIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - This bit is set when a magic packet or remote wake-up frame is received in the power-down mode (see Bit\\[5\\] and Bit\\[6\\] in the PMT Control and Status Register). This bit is cleared when both Bits\\[6:5\\] are cleared because of a read operation to the PMT Control and Status register. This bit is valid only when you select the optional PMT module during core configuration."]
    #[inline(always)]
    pub fn pmtints(&self) -> PMTINTS_R {
        PMTINTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - When the Energy Efficient Ethernet feature is enabled this bit is set for any LPI state entry or exit in the MAC Transmitter or Receiver. This bit is cleared on reading Bit\\[0\\] of Register (LPI Control and Status Register)."]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EMACINTS")
            .field("pmtints", &format_args!("{}", self.pmtints().bit()))
            .field("lpiis", &format_args!("{}", self.lpiis().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACINTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacints::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACINTS_SPEC;
impl crate::RegisterSpec for EMACINTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacints::R`](R) reader structure"]
impl crate::Readable for EMACINTS_SPEC {}
#[doc = "`reset()` method sets EMACINTS to value 0"]
impl crate::Resettable for EMACINTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
