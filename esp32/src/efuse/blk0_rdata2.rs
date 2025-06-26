#[doc = "Register `BLK0_RDATA2` reader"]
pub type R = crate::R<BLK0_RDATA2_SPEC>;
#[doc = "Register `BLK0_RDATA2` writer"]
pub type W = crate::W<BLK0_RDATA2_SPEC>;
#[doc = "Field `RD_MAC_1` reader - "]
pub type RD_MAC_1_R = crate::FieldReader<u16>;
#[doc = "Field `RD_MAC_CRC` reader - "]
pub type RD_MAC_CRC_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_88` reader - "]
pub type RD_RESERVE_0_88_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_88` writer - "]
pub type RD_RESERVE_0_88_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rd_mac_1(&self) -> RD_MAC_1_R {
        RD_MAC_1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rd_mac_crc(&self) -> RD_MAC_CRC_R {
        RD_MAC_CRC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_reserve_0_88(&self) -> RD_RESERVE_0_88_R {
        RD_RESERVE_0_88_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA2")
            .field("rd_mac_1", &self.rd_mac_1())
            .field("rd_mac_crc", &self.rd_mac_crc())
            .field("rd_reserve_0_88", &self.rd_reserve_0_88())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rd_reserve_0_88(&mut self) -> RD_RESERVE_0_88_W<BLK0_RDATA2_SPEC> {
        RD_RESERVE_0_88_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`blk0_rdata2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blk0_rdata2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_RDATA2_SPEC;
impl crate::RegisterSpec for BLK0_RDATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_rdata2::R`](R) reader structure"]
impl crate::Readable for BLK0_RDATA2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk0_rdata2::W`](W) writer structure"]
impl crate::Writable for BLK0_RDATA2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLK0_RDATA2 to value 0"]
impl crate::Resettable for BLK0_RDATA2_SPEC {}
