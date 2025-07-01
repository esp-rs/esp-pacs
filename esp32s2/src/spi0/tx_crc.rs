#[doc = "Register `TX_CRC` reader"]
pub type R = crate::R<TX_CRC_SPEC>;
#[doc = "Register `TX_CRC` writer"]
pub type W = crate::W<TX_CRC_SPEC>;
#[doc = "Field `TX_CRC_DATA` reader - "]
pub type TX_CRC_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `TX_CRC_DATA` writer - "]
pub type TX_CRC_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_crc_data(&self) -> TX_CRC_DATA_R {
        TX_CRC_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC")
            .field("tx_crc_data", &self.tx_crc_data())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_crc_data(&mut self) -> TX_CRC_DATA_W<TX_CRC_SPEC> {
        TX_CRC_DATA_W::new(self, 0)
    }
}
#[doc = "SPI Memory Transmit CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_SPEC;
impl crate::RegisterSpec for TX_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc::R`](R) reader structure"]
impl crate::Readable for TX_CRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc::W`](W) writer structure"]
impl crate::Writable for TX_CRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC to value 0"]
impl crate::Resettable for TX_CRC_SPEC {}
