#[doc = "Register `TX_CRC_EN_WR_DATA_CH%s` reader"]
pub type R = crate::R<TX_CRC_EN_WR_DATA_CH_SPEC>;
#[doc = "Register `TX_CRC_EN_WR_DATA_CH%s` writer"]
pub type W = crate::W<TX_CRC_EN_WR_DATA_CH_SPEC>;
#[doc = "Field `TX_CRC_EN_WR_DATA_CH` reader - This register is used to enable tx ch0 crc 32bit on/off"]
pub type TX_CRC_EN_WR_DATA_CH_R = crate::FieldReader<u32>;
#[doc = "Field `TX_CRC_EN_WR_DATA_CH` writer - This register is used to enable tx ch0 crc 32bit on/off"]
pub type TX_CRC_EN_WR_DATA_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to enable tx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn tx_crc_en_wr_data_ch(&self) -> TX_CRC_EN_WR_DATA_CH_R {
        TX_CRC_EN_WR_DATA_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC_EN_WR_DATA_CH")
            .field("tx_crc_en_wr_data_ch", &self.tx_crc_en_wr_data_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to enable tx ch0 crc 32bit on/off"]
    #[inline(always)]
    pub fn tx_crc_en_wr_data_ch(
        &mut self,
    ) -> TX_CRC_EN_WR_DATA_CH_W<'_, TX_CRC_EN_WR_DATA_CH_SPEC> {
        TX_CRC_EN_WR_DATA_CH_W::new(self, 0)
    }
}
#[doc = "This resister is used to config ch%s crc en for every bit\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_en_wr_data_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_en_wr_data_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_EN_WR_DATA_CH_SPEC;
impl crate::RegisterSpec for TX_CRC_EN_WR_DATA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_en_wr_data_ch::R`](R) reader structure"]
impl crate::Readable for TX_CRC_EN_WR_DATA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_en_wr_data_ch::W`](W) writer structure"]
impl crate::Writable for TX_CRC_EN_WR_DATA_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_EN_WR_DATA_CH%s to value 0"]
impl crate::Resettable for TX_CRC_EN_WR_DATA_CH_SPEC {}
