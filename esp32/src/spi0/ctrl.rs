#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `FCS_CRC_EN` reader - For SPI1 initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FCS_CRC_EN_R = crate::BitReader;
#[doc = "Field `FCS_CRC_EN` writer - For SPI1 initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FCS_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CRC_EN` reader - For SPI1 enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TX_CRC_EN_R = crate::BitReader;
#[doc = "Field `TX_CRC_EN` writer - For SPI1 enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TX_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAIT_FLASH_IDLE_EN` reader - wait flash idle when program flash or erase flash. 1: enable 0: disable."]
pub type WAIT_FLASH_IDLE_EN_R = crate::BitReader;
#[doc = "Field `WAIT_FLASH_IDLE_EN` writer - wait flash idle when program flash or erase flash. 1: enable 0: disable."]
pub type WAIT_FLASH_IDLE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTRD_MODE` reader - This bit enable the bits: spi_fread_qio spi_fread_dio spi_fread_qout and spi_fread_dout. 1: enable 0: disable."]
pub type FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `FASTRD_MODE` writer - This bit enable the bits: spi_fread_qio spi_fread_dio spi_fread_qout and spi_fread_dout. 1: enable 0: disable."]
pub type FASTRD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DUAL` reader - In the read operations read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `FREAD_DUAL` writer - In the read operations read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESANDRES` reader - The Device ID is read out to SPI_RD_STATUS register, this bit combine with spi_flash_res bit. 1: enable 0: disable."]
pub type RESANDRES_R = crate::BitReader;
#[doc = "Field `RESANDRES` writer - The Device ID is read out to SPI_RD_STATUS register, this bit combine with spi_flash_res bit. 1: enable 0: disable."]
pub type RESANDRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high 0: output low."]
pub type WP_R = crate::BitReader;
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high 0: output low."]
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRSR_2B` reader - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type WRSR_2B_R = crate::BitReader;
#[doc = "Field `WRSR_2B` writer - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type WRSR_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DIO` reader - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DIO_R = crate::BitReader;
#[doc = "Field `FREAD_DIO` writer - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QIO` reader - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FREAD_QIO_R = crate::BitReader;
#[doc = "Field `FREAD_QIO` writer - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FREAD_QIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_BIT_ORDER` reader - In read-data (MISO) phase 1: LSB first 0: MSB first"]
pub type RD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `RD_BIT_ORDER` writer - In read-data (MISO) phase 1: LSB first 0: MSB first"]
pub type RD_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_BIT_ORDER` reader - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first"]
pub type WR_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `WR_BIT_ORDER` writer - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first"]
pub type WR_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - For SPI1 initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    pub fn fcs_crc_en(&self) -> FCS_CRC_EN_R {
        FCS_CRC_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI1 enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    pub fn tx_crc_en(&self) -> TX_CRC_EN_R {
        TX_CRC_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - wait flash idle when program flash or erase flash. 1: enable 0: disable."]
    #[inline(always)]
    pub fn wait_flash_idle_en(&self) -> WAIT_FLASH_IDLE_EN_R {
        WAIT_FLASH_IDLE_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_fread_qio spi_fread_dio spi_fread_qout and spi_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FASTRD_MODE_R {
        FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_RD_STATUS register, this bit combine with spi_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    pub fn resandres(&self) -> RESANDRES_R {
        RESANDRES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high 0: output low."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    pub fn wrsr_2b(&self) -> WRSR_2B_R {
        WRSR_2B_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&self) -> FREAD_DIO_R {
        FREAD_DIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_qio(&self) -> FREAD_QIO_R {
        FREAD_QIO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first"]
    #[inline(always)]
    pub fn rd_bit_order(&self) -> RD_BIT_ORDER_R {
        RD_BIT_ORDER_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first"]
    #[inline(always)]
    pub fn wr_bit_order(&self) -> WR_BIT_ORDER_R {
        WR_BIT_ORDER_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("fcs_crc_en", &self.fcs_crc_en())
            .field("tx_crc_en", &self.tx_crc_en())
            .field("wait_flash_idle_en", &self.wait_flash_idle_en())
            .field("fastrd_mode", &self.fastrd_mode())
            .field("fread_dual", &self.fread_dual())
            .field("resandres", &self.resandres())
            .field("fread_quad", &self.fread_quad())
            .field("wp", &self.wp())
            .field("wrsr_2b", &self.wrsr_2b())
            .field("fread_dio", &self.fread_dio())
            .field("fread_qio", &self.fread_qio())
            .field("rd_bit_order", &self.rd_bit_order())
            .field("wr_bit_order", &self.wr_bit_order())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - For SPI1 initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    #[must_use]
    pub fn fcs_crc_en(&mut self) -> FCS_CRC_EN_W<CTRL_SPEC> {
        FCS_CRC_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - For SPI1 enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_en(&mut self) -> TX_CRC_EN_W<CTRL_SPEC> {
        TX_CRC_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - wait flash idle when program flash or erase flash. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn wait_flash_idle_en(&mut self) -> WAIT_FLASH_IDLE_EN_W<CTRL_SPEC> {
        WAIT_FLASH_IDLE_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_fread_qio spi_fread_dio spi_fread_qout and spi_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fastrd_mode(&mut self) -> FASTRD_MODE_W<CTRL_SPEC> {
        FASTRD_MODE_W::new(self, 13)
    }
    #[doc = "Bit 14 - In the read operations read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W<CTRL_SPEC> {
        FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_RD_STATUS register, this bit combine with spi_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn resandres(&mut self) -> RESANDRES_W<CTRL_SPEC> {
        RESANDRES_W::new(self, 15)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W<CTRL_SPEC> {
        FREAD_QUAD_W::new(self, 20)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high 0: output low."]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<CTRL_SPEC> {
        WP_W::new(self, 21)
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn wrsr_2b(&mut self) -> WRSR_2B_W<CTRL_SPEC> {
        WRSR_2B_W::new(self, 22)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_dio(&mut self) -> FREAD_DIO_W<CTRL_SPEC> {
        FREAD_DIO_W::new(self, 23)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_qio(&mut self) -> FREAD_QIO_W<CTRL_SPEC> {
        FREAD_QIO_W::new(self, 24)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first"]
    #[inline(always)]
    #[must_use]
    pub fn rd_bit_order(&mut self) -> RD_BIT_ORDER_W<CTRL_SPEC> {
        RD_BIT_ORDER_W::new(self, 25)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first"]
    #[inline(always)]
    #[must_use]
    pub fn wr_bit_order(&mut self) -> WR_BIT_ORDER_W<CTRL_SPEC> {
        WR_BIT_ORDER_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0020_a400"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0020_a400;
}
