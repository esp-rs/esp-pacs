#[doc = "Register `BLK0_RDATA2` reader"]
pub struct R(crate::R<BLK0_RDATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_WIFI_MAC_CRC_HIGH` reader - read for high 24bit WIFI_MAC_Address"]
pub type RD_WIFI_MAC_CRC_HIGH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - read for high 24bit WIFI_MAC_Address"]
    #[inline(always)]
    pub fn rd_wifi_mac_crc_high(&self) -> RD_WIFI_MAC_CRC_HIGH_R {
        RD_WIFI_MAC_CRC_HIGH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA2")
            .field(
                "rd_wifi_mac_crc_high",
                &format_args!("{}", self.rd_wifi_mac_crc_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata2](index.html) module"]
pub struct BLK0_RDATA2_SPEC;
impl crate::RegisterSpec for BLK0_RDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata2::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_RDATA2 to value 0"]
impl crate::Resettable for BLK0_RDATA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
