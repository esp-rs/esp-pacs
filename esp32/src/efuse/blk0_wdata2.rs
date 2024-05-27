#[doc = "Register `BLK0_WDATA2` reader"]
pub type R = crate::R<BLK0_WDATA2_SPEC>;
#[doc = "Register `BLK0_WDATA2` writer"]
pub type W = crate::W<BLK0_WDATA2_SPEC>;
#[doc = "Field `WIFI_MAC_CRC_HIGH` reader - "]
pub type WIFI_MAC_CRC_HIGH_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_MAC_CRC_HIGH` writer - "]
pub type WIFI_MAC_CRC_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn wifi_mac_crc_high(&self) -> WIFI_MAC_CRC_HIGH_R {
        WIFI_MAC_CRC_HIGH_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA2")
            .field("wifi_mac_crc_high", &self.wifi_mac_crc_high())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_mac_crc_high(&mut self) -> WIFI_MAC_CRC_HIGH_W<BLK0_WDATA2_SPEC> {
        WIFI_MAC_CRC_HIGH_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_wdata2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk0_wdata2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_WDATA2_SPEC;
impl crate::RegisterSpec for BLK0_WDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_wdata2::R`](R) reader structure"]
impl crate::Readable for BLK0_WDATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_wdata2::W`](W) writer structure"]
impl crate::Writable for BLK0_WDATA2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK0_WDATA2 to value 0"]
impl crate::Resettable for BLK0_WDATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
