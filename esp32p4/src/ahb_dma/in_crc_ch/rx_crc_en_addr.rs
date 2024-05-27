///Register `RX_CRC_EN_ADDR` reader
pub type R = crate::R<RX_CRC_EN_ADDR_SPEC>;
///Register `RX_CRC_EN_ADDR` writer
pub type W = crate::W<RX_CRC_EN_ADDR_SPEC>;
///Field `RX_CRC_EN_ADDR` reader - reserved
pub type RX_CRC_EN_ADDR_R = crate::FieldReader<u32>;
///Field `RX_CRC_EN_ADDR` writer - reserved
pub type RX_CRC_EN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - reserved
    #[inline(always)]
    pub fn rx_crc_en_addr(&self) -> RX_CRC_EN_ADDR_R {
        RX_CRC_EN_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CRC_EN_ADDR")
            .field("rx_crc_en_addr", &self.rx_crc_en_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - reserved
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_en_addr(&mut self) -> RX_CRC_EN_ADDR_W<RX_CRC_EN_ADDR_SPEC> {
        RX_CRC_EN_ADDR_W::new(self, 0)
    }
}
/**This register is used to config ch0 crc en addr

You can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_en_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_en_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_CRC_EN_ADDR_SPEC;
impl crate::RegisterSpec for RX_CRC_EN_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_crc_en_addr::R`](R) reader structure
impl crate::Readable for RX_CRC_EN_ADDR_SPEC {}
///`write(|w| ..)` method takes [`rx_crc_en_addr::W`](W) writer structure
impl crate::Writable for RX_CRC_EN_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_CRC_EN_ADDR to value 0
impl crate::Resettable for RX_CRC_EN_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
