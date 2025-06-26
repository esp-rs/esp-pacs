#[doc = "Register `BLK0_WDATA1` reader"]
pub type R = crate::R<BLK0_WDATA1_SPEC>;
#[doc = "Register `BLK0_WDATA1` writer"]
pub type W = crate::W<BLK0_WDATA1_SPEC>;
#[doc = "Field `WIFI_MAC_CRC_LOW` reader - "]
pub type WIFI_MAC_CRC_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_MAC_CRC_LOW` writer - "]
pub type WIFI_MAC_CRC_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wifi_mac_crc_low(&self) -> WIFI_MAC_CRC_LOW_R {
        WIFI_MAC_CRC_LOW_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_WDATA1")
            .field("wifi_mac_crc_low", &self.wifi_mac_crc_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wifi_mac_crc_low(&mut self) -> WIFI_MAC_CRC_LOW_W<BLK0_WDATA1_SPEC> {
        WIFI_MAC_CRC_LOW_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`blk0_wdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk0_wdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_WDATA1_SPEC;
impl crate::RegisterSpec for BLK0_WDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_wdata1::R`](R) reader structure"]
impl crate::Readable for BLK0_WDATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_wdata1::W`](W) writer structure"]
impl crate::Writable for BLK0_WDATA1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK0_WDATA1 to value 0"]
impl crate::Resettable for BLK0_WDATA1_SPEC {}
