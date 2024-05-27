#[doc = "Register `TX_CRC` reader"]
pub type R = crate::R<TX_CRC_SPEC>;
#[doc = "Register `TX_CRC` writer"]
pub type W = crate::W<TX_CRC_SPEC>;
#[doc = "Field `DATA` reader - For SPI1 the value of crc32 for 256 bits data."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - For SPI1 the value of crc32 for 256 bits data."]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - For SPI1 the value of crc32 for 256 bits data."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<TX_CRC_SPEC> {
        DATA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_SPEC;
impl crate::RegisterSpec for TX_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc::R`](R) reader structure"]
impl crate::Readable for TX_CRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc::W`](W) writer structure"]
impl crate::Writable for TX_CRC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_CRC to value 0"]
impl crate::Resettable for TX_CRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
