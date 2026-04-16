#[doc = "Register `TX_CRC_WIDTH_CH%s` reader"]
pub type R = crate::R<TX_CRC_WIDTH_CH_SPEC>;
#[doc = "Register `TX_CRC_WIDTH_CH%s` writer"]
pub type W = crate::W<TX_CRC_WIDTH_CH_SPEC>;
#[doc = "Field `TX_CRC_WIDTH_CH` reader - reserved"]
pub type TX_CRC_WIDTH_CH_R = crate::FieldReader;
#[doc = "Field `TX_CRC_WIDTH_CH` writer - reserved"]
pub type TX_CRC_WIDTH_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_CRC_LAUTCH_FLGA_CH` reader - reserved"]
pub type TX_CRC_LAUTCH_FLGA_CH_R = crate::BitReader;
#[doc = "Field `TX_CRC_LAUTCH_FLGA_CH` writer - reserved"]
pub type TX_CRC_LAUTCH_FLGA_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - reserved"]
    #[inline(always)]
    pub fn tx_crc_width_ch(&self) -> TX_CRC_WIDTH_CH_R {
        TX_CRC_WIDTH_CH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn tx_crc_lautch_flga_ch(&self) -> TX_CRC_LAUTCH_FLGA_CH_R {
        TX_CRC_LAUTCH_FLGA_CH_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC_WIDTH_CH")
            .field("tx_crc_width_ch", &self.tx_crc_width_ch())
            .field("tx_crc_lautch_flga_ch", &self.tx_crc_lautch_flga_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - reserved"]
    #[inline(always)]
    pub fn tx_crc_width_ch(&mut self) -> TX_CRC_WIDTH_CH_W<'_, TX_CRC_WIDTH_CH_SPEC> {
        TX_CRC_WIDTH_CH_W::new(self, 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn tx_crc_lautch_flga_ch(&mut self) -> TX_CRC_LAUTCH_FLGA_CH_W<'_, TX_CRC_WIDTH_CH_SPEC> {
        TX_CRC_LAUTCH_FLGA_CH_W::new(self, 2)
    }
}
#[doc = "This register is used to confiig tx ch%s crc result width,2'b00 mean crc_width <=8bit,2'b01 8<crc_width<=16 ,2'b10 mean 16<crc_width <=24,2'b11 mean 24<crc_width<=32\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_crc_width_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_crc_width_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_WIDTH_CH_SPEC;
impl crate::RegisterSpec for TX_CRC_WIDTH_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_width_ch::R`](R) reader structure"]
impl crate::Readable for TX_CRC_WIDTH_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_width_ch::W`](W) writer structure"]
impl crate::Writable for TX_CRC_WIDTH_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_WIDTH_CH%s to value 0"]
impl crate::Resettable for TX_CRC_WIDTH_CH_SPEC {}
