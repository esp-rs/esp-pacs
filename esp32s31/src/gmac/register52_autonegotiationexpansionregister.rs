#[doc = "Register `REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER` reader"]
pub type R = crate::R<REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER_SPEC>;
#[doc = "Field `NPR` reader - New Page Received When set, this bit indicates that the MAC has received a new page This bit is cleared when read"]
pub type NPR_R = crate::BitReader;
#[doc = "Field `NPA` reader - Next Page Ability This bit is always low because the MAC does not support the next page function"]
pub type NPA_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - New Page Received When set, this bit indicates that the MAC has received a new page This bit is cleared when read"]
    #[inline(always)]
    pub fn npr(&self) -> NPR_R {
        NPR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Next Page Ability This bit is always low because the MAC does not support the next page function"]
    #[inline(always)]
    pub fn npa(&self) -> NPA_R {
        NPA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER")
            .field("npr", &self.npr())
            .field("npa", &self.npa())
            .finish()
    }
}
#[doc = "Indicates whether a new base page has been received from the link partner This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register52_autonegotiationexpansionregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register52_autonegotiationexpansionregister::R`](R) reader structure"]
impl crate::Readable for REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER to value 0"]
impl crate::Resettable for REGISTER52_AUTONEGOTIATIONEXPANSIONREGISTER_SPEC {}
