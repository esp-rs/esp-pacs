#[doc = "Register `RD_MAC_SYS1` reader"]
pub type R = crate::R<RD_MAC_SYS1_SPEC>;
#[doc = "Field `MAC_1` reader - Represents MAC address. High 16-bit."]
pub type MAC_1_R = crate::FieldReader<u16>;
#[doc = "Field `MAC_EXT` reader - Represents the extended bits of MAC address."]
pub type MAC_EXT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Represents MAC address. High 16-bit."]
    #[inline(always)]
    pub fn mac_1(&self) -> MAC_1_R {
        MAC_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Represents the extended bits of MAC address."]
    #[inline(always)]
    pub fn mac_ext(&self) -> MAC_EXT_R {
        MAC_EXT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS1")
            .field("mac_1", &self.mac_1())
            .field("mac_ext", &self.mac_ext())
            .finish()
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS1_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys1::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS1_SPEC {}
#[doc = "`reset()` method sets RD_MAC_SYS1 to value 0"]
impl crate::Resettable for RD_MAC_SYS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
