#[doc = "Register `TX_CRC_DATA_EN_WR_DATA_CH%s` reader"]
pub type R = crate::R<TX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "Register `TX_CRC_DATA_EN_WR_DATA_CH%s` writer"]
pub type W = crate::W<TX_CRC_DATA_EN_WR_DATA_CH_SPEC>;
#[doc = "Field `TX_CRC_DATA_EN_WR_DATA_CH` reader - reserved"]
pub type TX_CRC_DATA_EN_WR_DATA_CH_R = crate::FieldReader;
#[doc = "Field `TX_CRC_DATA_EN_WR_DATA_CH` writer - reserved"]
pub type TX_CRC_DATA_EN_WR_DATA_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data_ch(&self) -> TX_CRC_DATA_EN_WR_DATA_CH_R {
        TX_CRC_DATA_EN_WR_DATA_CH_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC_DATA_EN_WR_DATA_CH")
            .field(
                "tx_crc_data_en_wr_data_ch",
                &self.tx_crc_data_en_wr_data_ch(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data_ch(
        &mut self,
    ) -> TX_CRC_DATA_EN_WR_DATA_CH_W<'_, TX_CRC_DATA_EN_WR_DATA_CH_SPEC> {
        TX_CRC_DATA_EN_WR_DATA_CH_W::new(self, 0)
    }
}
#[doc = "This register is used to config crc data_8bit en\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_data_en_wr_data_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_DATA_EN_WR_DATA_CH_SPEC;
impl crate::RegisterSpec for TX_CRC_DATA_EN_WR_DATA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_data_en_wr_data_ch::R`](R) reader structure"]
impl crate::Readable for TX_CRC_DATA_EN_WR_DATA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_data_en_wr_data_ch::W`](W) writer structure"]
impl crate::Writable for TX_CRC_DATA_EN_WR_DATA_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_DATA_EN_WR_DATA_CH%s to value 0"]
impl crate::Resettable for TX_CRC_DATA_EN_WR_DATA_CH_SPEC {}
