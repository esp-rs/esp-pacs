#[doc = "Register `RX_CRC_DATA_EN_ADDR_CH` reader"]
pub type R = crate::R<RX_CRC_DATA_EN_ADDR_CH_SPEC>;
#[doc = "Register `RX_CRC_DATA_EN_ADDR_CH` writer"]
pub type W = crate::W<RX_CRC_DATA_EN_ADDR_CH_SPEC>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR_CH` reader - reserved"]
pub type RX_CRC_DATA_EN_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR_CH` writer - reserved"]
pub type RX_CRC_DATA_EN_ADDR_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_addr_ch(&self) -> RX_CRC_DATA_EN_ADDR_CH_R {
        RX_CRC_DATA_EN_ADDR_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CRC_DATA_EN_ADDR_CH")
            .field(
                "rx_crc_data_en_addr_ch",
                &format_args!("{}", self.rx_crc_data_en_addr_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CRC_DATA_EN_ADDR_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_en_addr_ch(
        &mut self,
    ) -> RX_CRC_DATA_EN_ADDR_CH_W<RX_CRC_DATA_EN_ADDR_CH_SPEC> {
        RX_CRC_DATA_EN_ADDR_CH_W::new(self, 0)
    }
}
#[doc = "This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_addr_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_addr_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CRC_DATA_EN_ADDR_CH_SPEC;
impl crate::RegisterSpec for RX_CRC_DATA_EN_ADDR_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_data_en_addr_ch::R`](R) reader structure"]
impl crate::Readable for RX_CRC_DATA_EN_ADDR_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_data_en_addr_ch::W`](W) writer structure"]
impl crate::Writable for RX_CRC_DATA_EN_ADDR_CH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_CRC_DATA_EN_ADDR_CH to value 0"]
impl crate::Resettable for RX_CRC_DATA_EN_ADDR_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
