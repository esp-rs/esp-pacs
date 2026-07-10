#[doc = "Register `RX_CRC_DATA_EN_WR_DATA_CH%s` reader"]
pub type R = crate::R<RX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "Register `RX_CRC_DATA_EN_WR_DATA_CH%s` writer"]
pub type W = crate::W<RX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "Field `RX_CRC_DATA_EN_WR_DATA_CH` reader - reserved"]
pub type RX_CRC_DATA_EN_WR_DATA_CH_R = crate::FieldReader;
#[doc = "Field `RX_CRC_DATA_EN_WR_DATA_CH` writer - reserved"]
pub type RX_CRC_DATA_EN_WR_DATA_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data_ch(&self) -> RX_CRC_DATA_EN_WR_DATA_CH_R {
        RX_CRC_DATA_EN_WR_DATA_CH_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CRC_DATA_EN_WR_DATA_CH")
            .field(
                "rx_crc_data_en_wr_data_ch",
                &self.rx_crc_data_en_wr_data_ch(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_wr_data_ch(
        &mut self,
    ) -> RX_CRC_DATA_EN_WR_DATA_CH_W<'_, RX_CRC_DATA_EN_WR_DATA_CH_SPEC> {
        RX_CRC_DATA_EN_WR_DATA_CH_W::new(self, 0)
    }
}
#[doc = "This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_data_en_wr_data_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_crc_data_en_wr_data_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CRC_DATA_EN_WR_DATA_CH_SPEC;
impl crate::RegisterSpec for RX_CRC_DATA_EN_WR_DATA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_data_en_wr_data_ch::R`](R) reader structure"]
impl crate::Readable for RX_CRC_DATA_EN_WR_DATA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_data_en_wr_data_ch::W`](W) writer structure"]
impl crate::Writable for RX_CRC_DATA_EN_WR_DATA_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_DATA_EN_WR_DATA_CH%s to value 0"]
impl crate::Resettable for RX_CRC_DATA_EN_WR_DATA_CH_SPEC {}
