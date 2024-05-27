#[doc = "Register `BLK3_RDATA5` reader"]
pub type R = crate::R<BLK3_RDATA5_SPEC>;
#[doc = "Field `RESERVED_3_160` reader - "]
pub type RESERVED_3_160_R = crate::FieldReader<u32>;
#[doc = "Field `RD_MAC_VERSION` reader - "]
pub type RD_MAC_VERSION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn reserved_3_160(&self) -> RESERVED_3_160_R {
        RESERVED_3_160_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_mac_version(&self) -> RD_MAC_VERSION_R {
        RD_MAC_VERSION_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA5")
            .field("reserved_3_160", &self.reserved_3_160())
            .field("rd_mac_version", &self.rd_mac_version())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_RDATA5_SPEC;
impl crate::RegisterSpec for BLK3_RDATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_rdata5::R`](R) reader structure"]
impl crate::Readable for BLK3_RDATA5_SPEC {}
#[doc = "`reset()` method sets BLK3_RDATA5 to value 0"]
impl crate::Resettable for BLK3_RDATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
