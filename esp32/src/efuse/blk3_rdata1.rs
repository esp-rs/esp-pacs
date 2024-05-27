#[doc = "Register `BLK3_RDATA1` reader"]
pub type R = crate::R<BLK3_RDATA1_SPEC>;
#[doc = "Field `RD_CUSTOM_MAC_1` reader - "]
pub type RD_CUSTOM_MAC_1_R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_3_56` reader - "]
pub type RESERVED_3_56_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn rd_custom_mac_1(&self) -> RD_CUSTOM_MAC_1_R {
        RD_CUSTOM_MAC_1_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn reserved_3_56(&self) -> RESERVED_3_56_R {
        RESERVED_3_56_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA1")
            .field("rd_custom_mac_1", &self.rd_custom_mac_1())
            .field("reserved_3_56", &self.reserved_3_56())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_RDATA1_SPEC;
impl crate::RegisterSpec for BLK3_RDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_rdata1::R`](R) reader structure"]
impl crate::Readable for BLK3_RDATA1_SPEC {}
#[doc = "`reset()` method sets BLK3_RDATA1 to value 0"]
impl crate::Resettable for BLK3_RDATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
