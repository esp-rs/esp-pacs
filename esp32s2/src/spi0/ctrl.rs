#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `EXT_HOLD_EN` reader - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
pub type EXT_HOLD_EN_R = crate::BitReader;
#[doc = "Field `EXT_HOLD_EN` writer - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
pub type EXT_HOLD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMMY_OUT` reader - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
pub type DUMMY_OUT_R = crate::BitReader;
#[doc = "Field `DUMMY_OUT` writer - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
pub type DUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_DUAL` reader - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `FADDR_DUAL` writer - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_QUAD` reader - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `FADDR_QUAD` writer - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FADDR_OCT` reader - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_OCT_R = crate::BitReader;
#[doc = "Field `FADDR_OCT` writer - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FADDR_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_DUAL` reader - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_DUAL_R = crate::BitReader;
#[doc = "Field `FCMD_DUAL` writer - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_QUAD` reader - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `FCMD_QUAD` writer - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMD_OCT` reader - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_OCT_R = crate::BitReader;
#[doc = "Field `FCMD_OCT` writer - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type FCMD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_DUAL` reader - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `FREAD_DUAL` writer - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_QUAD` reader - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `FREAD_QUAD` writer - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREAD_OCT` reader - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_OCT_R = crate::BitReader;
#[doc = "Field `FREAD_OCT` writer - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
pub type FREAD_OCT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type Q_POL_R = crate::BitReader;
#[doc = "Field `Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type D_POL_R = crate::BitReader;
#[doc = "Field `D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type WP_R = crate::BitReader;
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type WP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_BIT_ORDER` reader - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type RD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `RD_BIT_ORDER` writer - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type RD_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_BIT_ORDER` reader - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type WR_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `WR_BIT_ORDER` writer - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type WR_BIT_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ext_hold_en(&self) -> EXT_HOLD_EN_R {
        EXT_HOLD_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dummy_out(&self) -> DUMMY_OUT_R {
        DUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_dual(&self) -> FADDR_DUAL_R {
        FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_quad(&self) -> FADDR_QUAD_R {
        FADDR_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn faddr_oct(&self) -> FADDR_OCT_R {
        FADDR_OCT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fcmd_oct(&self) -> FCMD_OCT_R {
        FCMD_OCT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_dual(&self) -> FREAD_DUAL_R {
        FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn fread_oct(&self) -> FREAD_OCT_R {
        FREAD_OCT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn q_pol(&self) -> Q_POL_R {
        Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn d_pol(&self) -> D_POL_R {
        D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn rd_bit_order(&self) -> RD_BIT_ORDER_R {
        RD_BIT_ORDER_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn wr_bit_order(&self) -> WR_BIT_ORDER_R {
        WR_BIT_ORDER_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("ext_hold_en", &self.ext_hold_en())
            .field("dummy_out", &self.dummy_out())
            .field("faddr_dual", &self.faddr_dual())
            .field("faddr_quad", &self.faddr_quad())
            .field("faddr_oct", &self.faddr_oct())
            .field("fcmd_dual", &self.fcmd_dual())
            .field("fcmd_quad", &self.fcmd_quad())
            .field("fcmd_oct", &self.fcmd_oct())
            .field("fread_dual", &self.fread_dual())
            .field("fread_quad", &self.fread_quad())
            .field("fread_oct", &self.fread_oct())
            .field("q_pol", &self.q_pol())
            .field("d_pol", &self.d_pol())
            .field("wp", &self.wp())
            .field("rd_bit_order", &self.rd_bit_order())
            .field("wr_bit_order", &self.wr_bit_order())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Set the bit to hold spi. The bit is combined with SPI_USR_PREP_HOLD,SPI_USR_CMD_HOLD,SPI_USR_ADDR_HOLD,SPI_USR_DUMMY_HOLD,SPI_USR_DIN_HOLD,SPI_USR_DOUT_HOLD and SPI_USR_HOLD_POL. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn ext_hold_en(&mut self) -> EXT_HOLD_EN_W<CTRL_SPEC> {
        EXT_HOLD_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - In the dummy phase the signal level of spi is output by the spi controller. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn dummy_out(&mut self) -> DUMMY_OUT_W<CTRL_SPEC> {
        DUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 5 - Apply 2-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_dual(&mut self) -> FADDR_DUAL_W<CTRL_SPEC> {
        FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Apply 4-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_quad(&mut self) -> FADDR_QUAD_W<CTRL_SPEC> {
        FADDR_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 7 - Apply 8-bit mode during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_oct(&mut self) -> FADDR_OCT_W<CTRL_SPEC> {
        FADDR_OCT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Apply 2-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W<CTRL_SPEC> {
        FCMD_DUAL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Apply 4-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W<CTRL_SPEC> {
        FCMD_QUAD_W::new(self, 9)
    }
    #[doc = "Bit 10 - Apply 8-bit mode during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_oct(&mut self) -> FCMD_OCT_W<CTRL_SPEC> {
        FCMD_OCT_W::new(self, 10)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase is in 2-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W<CTRL_SPEC> {
        FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 15 - In the read operations read-data phase is in 4-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W<CTRL_SPEC> {
        FREAD_QUAD_W::new(self, 15)
    }
    #[doc = "Bit 16 - In the read operations read-data phase is in 8-bit mode. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn fread_oct(&mut self) -> FREAD_OCT_W<CTRL_SPEC> {
        FREAD_OCT_W::new(self, 16)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn q_pol(&mut self) -> Q_POL_W<CTRL_SPEC> {
        Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn d_pol(&mut self) -> D_POL_W<CTRL_SPEC> {
        D_POL_W::new(self, 19)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<CTRL_SPEC> {
        WP_W::new(self, 21)
    }
    #[doc = "Bit 25 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn rd_bit_order(&mut self) -> RD_BIT_ORDER_W<CTRL_SPEC> {
        RD_BIT_ORDER_W::new(self, 25)
    }
    #[doc = "Bit 26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn wr_bit_order(&mut self) -> WR_BIT_ORDER_W<CTRL_SPEC> {
        WR_BIT_ORDER_W::new(self, 26)
    }
}
#[doc = "SPI control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL to value 0x002c_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x002c_0000;
}
