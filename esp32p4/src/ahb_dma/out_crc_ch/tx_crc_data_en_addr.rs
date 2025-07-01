#[doc = "Register `TX_CRC_DATA_EN_ADDR` reader"]
pub type R = crate::R<TX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "Register `TX_CRC_DATA_EN_ADDR` writer"]
pub type W = crate::W<TX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "Field `TX_CRC_DATA_EN_ADDR` reader - reserved"]
pub type TX_CRC_DATA_EN_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `TX_CRC_DATA_EN_ADDR` writer - reserved"]
pub type TX_CRC_DATA_EN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_addr(&self) -> TX_CRC_DATA_EN_ADDR_R {
        TX_CRC_DATA_EN_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC_DATA_EN_ADDR")
            .field("tx_crc_data_en_addr", &self.tx_crc_data_en_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_addr(&mut self) -> TX_CRC_DATA_EN_ADDR_W<TX_CRC_DATA_EN_ADDR_SPEC> {
        TX_CRC_DATA_EN_ADDR_W::new(self, 0)
    }
}
#[doc = "This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_DATA_EN_ADDR_SPEC;
impl crate::RegisterSpec for TX_CRC_DATA_EN_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_data_en_addr::R`](R) reader structure"]
impl crate::Readable for TX_CRC_DATA_EN_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_data_en_addr::W`](W) writer structure"]
impl crate::Writable for TX_CRC_DATA_EN_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_DATA_EN_ADDR to value 0"]
impl crate::Resettable for TX_CRC_DATA_EN_ADDR_SPEC {}
