#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `FDUMMY_OUT` reader - In the dummy phase the signal level of spi is output by the spi controller."]
pub type FDUMMY_OUT_R = crate::BitReader;
#[doc = "Field `FDUMMY_OUT` writer - In the dummy phase the signal level of spi is output by the spi controller."]
pub type FDUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_DUAL` reader - Apply 2 signals during command phase 1:enable 0: disable"]
pub type FCMD_DUAL_R = crate::BitReader;
#[doc = "Field `FCMD_DUAL` writer - Apply 2 signals during command phase 1:enable 0: disable"]
pub type FCMD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_QUAD` reader - Apply 4 signals during command phase 1:enable 0: disable"]
pub type FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `FCMD_QUAD` writer - Apply 4 signals during command phase 1:enable 0: disable"]
pub type FCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCS_CRC_EN` reader - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FCS_CRC_EN_R = crate::BitReader;
#[doc = "Field `FCS_CRC_EN` writer - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FCS_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CRC_EN` reader - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TX_CRC_EN_R = crate::BitReader;
#[doc = "Field `TX_CRC_EN` writer - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TX_CRC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FASTRD_MODE` reader - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub type FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `FASTRD_MODE` writer - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub type FASTRD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DUAL` reader - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `FREAD_DUAL` writer - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESANDRES` reader - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type RESANDRES_R = crate::BitReader;
#[doc = "Field `RESANDRES` writer - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type RESANDRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type Q_POL_R = crate::BitReader;
#[doc = "Field `Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type D_POL_R = crate::BitReader;
#[doc = "Field `D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type WP_R = crate::BitReader;
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
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
impl R {
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller."]
    #[inline(always)]
    pub fn fdummy_out(&self) -> FDUMMY_OUT_R {
        FDUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Apply 2 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    pub fn fcs_crc_en(&self) -> FCS_CRC_EN_R {
        FCS_CRC_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    pub fn tx_crc_en(&self) -> TX_CRC_EN_R {
        TX_CRC_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FASTRD_MODE_R {
        FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    pub fn resandres(&self) -> RESANDRES_R {
        RESANDRES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
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
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("fdummy_out", &format_args!("{}", self.fdummy_out().bit()))
            .field("fcmd_dual", &format_args!("{}", self.fcmd_dual().bit()))
            .field("fcmd_quad", &format_args!("{}", self.fcmd_quad().bit()))
            .field("fcs_crc_en", &format_args!("{}", self.fcs_crc_en().bit()))
            .field("tx_crc_en", &format_args!("{}", self.tx_crc_en().bit()))
            .field("fastrd_mode", &format_args!("{}", self.fastrd_mode().bit()))
            .field("fread_dual", &format_args!("{}", self.fread_dual().bit()))
            .field("resandres", &format_args!("{}", self.resandres().bit()))
            .field("q_pol", &format_args!("{}", self.q_pol().bit()))
            .field("d_pol", &format_args!("{}", self.d_pol().bit()))
            .field("fread_quad", &format_args!("{}", self.fread_quad().bit()))
            .field("wp", &format_args!("{}", self.wp().bit()))
            .field("wrsr_2b", &format_args!("{}", self.wrsr_2b().bit()))
            .field("fread_dio", &format_args!("{}", self.fread_dio().bit()))
            .field("fread_qio", &format_args!("{}", self.fread_qio().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller."]
    #[inline(always)]
    #[must_use]
    pub fn fdummy_out(&mut self) -> FDUMMY_OUT_W<CTRL_SPEC> {
        FDUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 7 - Apply 2 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W<CTRL_SPEC> {
        FCMD_DUAL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W<CTRL_SPEC> {
        FCMD_QUAD_W::new(self, 8)
    }
    #[doc = "Bit 10 - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    #[must_use]
    pub fn fcs_crc_en(&mut self) -> FCS_CRC_EN_W<CTRL_SPEC> {
        FCS_CRC_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_en(&mut self) -> TX_CRC_EN_W<CTRL_SPEC> {
        TX_CRC_EN_W::new(self, 11)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fastrd_mode(&mut self) -> FASTRD_MODE_W<CTRL_SPEC> {
        FASTRD_MODE_W::new(self, 13)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W<CTRL_SPEC> {
        FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn resandres(&mut self) -> RESANDRES_W<CTRL_SPEC> {
        RESANDRES_W::new(self, 15)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn q_pol(&mut self) -> Q_POL_W<CTRL_SPEC> {
        Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn d_pol(&mut self) -> D_POL_W<CTRL_SPEC> {
        D_POL_W::new(self, 19)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W<CTRL_SPEC> {
        FREAD_QUAD_W::new(self, 20)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
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
}
#[doc = "SPI1 control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x002c_a000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x002c_a000;
}
