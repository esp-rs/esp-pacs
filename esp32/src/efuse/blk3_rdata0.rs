#[doc = "Register `BLK3_RDATA0` reader"]
pub type R = crate::R<BLK3_RDATA0_SPEC>;
#[doc = "Field `RD_CUSTOM_MAC_CRC` reader - "]
pub type RD_CUSTOM_MAC_CRC_R = crate::FieldReader;
#[doc = "Field `RD_CUSTOM_MAC` reader - "]
pub type RD_CUSTOM_MAC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rd_custom_mac_crc(&self) -> RD_CUSTOM_MAC_CRC_R {
        RD_CUSTOM_MAC_CRC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rd_custom_mac(&self) -> RD_CUSTOM_MAC_R {
        RD_CUSTOM_MAC_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA0")
            .field("rd_custom_mac_crc", &self.rd_custom_mac_crc())
            .field("rd_custom_mac", &self.rd_custom_mac())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_RDATA0_SPEC;
impl crate::RegisterSpec for BLK3_RDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_rdata0::R`](R) reader structure"]
impl crate::Readable for BLK3_RDATA0_SPEC {}
#[doc = "`reset()` method sets BLK3_RDATA0 to value 0"]
impl crate::Resettable for BLK3_RDATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
