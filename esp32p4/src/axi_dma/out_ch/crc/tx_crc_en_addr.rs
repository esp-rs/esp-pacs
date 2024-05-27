///Register `TX_CRC_EN_ADDR` reader
pub type R = crate::R<TX_CRC_EN_ADDR_SPEC>;
///Register `TX_CRC_EN_ADDR` writer
pub type W = crate::W<TX_CRC_EN_ADDR_SPEC>;
///Field `TX_CRC_EN_ADDR` reader - reserved
pub type TX_CRC_EN_ADDR_R = crate::FieldReader<u32>;
///Field `TX_CRC_EN_ADDR` writer - reserved
pub type TX_CRC_EN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reserved
    #[inline(always)]
    pub fn tx_crc_en_addr(&self) -> TX_CRC_EN_ADDR_R {
        TX_CRC_EN_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC_EN_ADDR")
            .field("tx_crc_en_addr", &self.tx_crc_en_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - reserved
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_en_addr(&mut self) -> TX_CRC_EN_ADDR_W<TX_CRC_EN_ADDR_SPEC> {
        TX_CRC_EN_ADDR_W::new(self, 0)
    }
}
/**This register is used to config ch0 crc en addr

You can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_en_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_en_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_CRC_EN_ADDR_SPEC;
impl crate::RegisterSpec for TX_CRC_EN_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_crc_en_addr::R`](R) reader structure
impl crate::Readable for TX_CRC_EN_ADDR_SPEC {}
///`write(|w| ..)` method takes [`tx_crc_en_addr::W`](W) writer structure
impl crate::Writable for TX_CRC_EN_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_CRC_EN_ADDR to value 0
impl crate::Resettable for TX_CRC_EN_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
