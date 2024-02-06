#[doc = "Register `SPI_CTRL` reader"]
pub type R = crate::R<SPI_CTRL_SPEC>;
#[doc = "Register `SPI_CTRL` writer"]
pub type W = crate::W<SPI_CTRL_SPEC>;
#[doc = "Field `SPI_DUMMY_OUT` reader - 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state."]
pub type SPI_DUMMY_OUT_R = crate::BitReader;
#[doc = "Field `SPI_DUMMY_OUT` writer - 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state."]
pub type SPI_DUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FADDR_DUAL` reader - Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FADDR_DUAL` writer - Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FADDR_QUAD` reader - Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FADDR_QUAD` writer - Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FCMD_DUAL` reader - Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FCMD_DUAL` writer - Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FCMD_QUAD` reader - Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FCMD_QUAD` writer - Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FREAD_DUAL` reader - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FREAD_DUAL` writer - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_Q_POL_R = crate::BitReader;
#[doc = "Field `SPI_Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_D_POL_R = crate::BitReader;
#[doc = "Field `SPI_D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_HOLD_POL` reader - SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_HOLD_POL_R = crate::BitReader;
#[doc = "Field `SPI_HOLD_POL` writer - SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_HOLD_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_WP_POL` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_WP_POL_R = crate::BitReader;
#[doc = "Field `SPI_WP_POL` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_WP_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RD_BIT_ORDER` reader - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type SPI_RD_BIT_ORDER_R = crate::FieldReader;
#[doc = "Field `SPI_RD_BIT_ORDER` writer - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type SPI_RD_BIT_ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_WR_BIT_ORDER` reader - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type SPI_WR_BIT_ORDER_R = crate::FieldReader;
#[doc = "Field `SPI_WR_BIT_ORDER` writer - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type SPI_WR_BIT_ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 3 - 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dummy_out(&self) -> SPI_DUMMY_OUT_R {
        SPI_DUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_dual(&self) -> SPI_FADDR_DUAL_R {
        SPI_FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_quad(&self) -> SPI_FADDR_QUAD_R {
        SPI_FADDR_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_dual(&self) -> SPI_FCMD_DUAL_R {
        SPI_FCMD_DUAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_quad(&self) -> SPI_FCMD_QUAD_R {
        SPI_FCMD_QUAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_dual(&self) -> SPI_FREAD_DUAL_R {
        SPI_FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_quad(&self) -> SPI_FREAD_QUAD_R {
        SPI_FREAD_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_q_pol(&self) -> SPI_Q_POL_R {
        SPI_Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_d_pol(&self) -> SPI_D_POL_R {
        SPI_D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_hold_pol(&self) -> SPI_HOLD_POL_R {
        SPI_HOLD_POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_wp_pol(&self) -> SPI_WP_POL_R {
        SPI_WP_POL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 23:24 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_rd_bit_order(&self) -> SPI_RD_BIT_ORDER_R {
        SPI_RD_BIT_ORDER_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_wr_bit_order(&self) -> SPI_WR_BIT_ORDER_R {
        SPI_WR_BIT_ORDER_R::new(((self.bits >> 25) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CTRL")
            .field(
                "spi_dummy_out",
                &format_args!("{}", self.spi_dummy_out().bit()),
            )
            .field(
                "spi_faddr_dual",
                &format_args!("{}", self.spi_faddr_dual().bit()),
            )
            .field(
                "spi_faddr_quad",
                &format_args!("{}", self.spi_faddr_quad().bit()),
            )
            .field(
                "spi_fcmd_dual",
                &format_args!("{}", self.spi_fcmd_dual().bit()),
            )
            .field(
                "spi_fcmd_quad",
                &format_args!("{}", self.spi_fcmd_quad().bit()),
            )
            .field(
                "spi_fread_dual",
                &format_args!("{}", self.spi_fread_dual().bit()),
            )
            .field(
                "spi_fread_quad",
                &format_args!("{}", self.spi_fread_quad().bit()),
            )
            .field("spi_q_pol", &format_args!("{}", self.spi_q_pol().bit()))
            .field("spi_d_pol", &format_args!("{}", self.spi_d_pol().bit()))
            .field(
                "spi_hold_pol",
                &format_args!("{}", self.spi_hold_pol().bit()),
            )
            .field("spi_wp_pol", &format_args!("{}", self.spi_wp_pol().bit()))
            .field(
                "spi_rd_bit_order",
                &format_args!("{}", self.spi_rd_bit_order().bits()),
            )
            .field(
                "spi_wr_bit_order",
                &format_args!("{}", self.spi_wr_bit_order().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dummy_out(&mut self) -> SPI_DUMMY_OUT_W<SPI_CTRL_SPEC> {
        SPI_DUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 5 - Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_faddr_dual(&mut self) -> SPI_FADDR_DUAL_W<SPI_CTRL_SPEC> {
        SPI_FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_faddr_quad(&mut self) -> SPI_FADDR_QUAD_W<SPI_CTRL_SPEC> {
        SPI_FADDR_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 8 - Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fcmd_dual(&mut self) -> SPI_FCMD_DUAL_W<SPI_CTRL_SPEC> {
        SPI_FCMD_DUAL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fcmd_quad(&mut self) -> SPI_FCMD_QUAD_W<SPI_CTRL_SPEC> {
        SPI_FCMD_QUAD_W::new(self, 9)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fread_dual(&mut self) -> SPI_FREAD_DUAL_W<SPI_CTRL_SPEC> {
        SPI_FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 15 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fread_quad(&mut self) -> SPI_FREAD_QUAD_W<SPI_CTRL_SPEC> {
        SPI_FREAD_QUAD_W::new(self, 15)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_q_pol(&mut self) -> SPI_Q_POL_W<SPI_CTRL_SPEC> {
        SPI_Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_d_pol(&mut self) -> SPI_D_POL_W<SPI_CTRL_SPEC> {
        SPI_D_POL_W::new(self, 19)
    }
    #[doc = "Bit 20 - SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_hold_pol(&mut self) -> SPI_HOLD_POL_W<SPI_CTRL_SPEC> {
        SPI_HOLD_POL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_wp_pol(&mut self) -> SPI_WP_POL_W<SPI_CTRL_SPEC> {
        SPI_WP_POL_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rd_bit_order(&mut self) -> SPI_RD_BIT_ORDER_W<SPI_CTRL_SPEC> {
        SPI_RD_BIT_ORDER_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_wr_bit_order(&mut self) -> SPI_WR_BIT_ORDER_W<SPI_CTRL_SPEC> {
        SPI_WR_BIT_ORDER_W::new(self, 25)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CTRL_SPEC;
impl crate::RegisterSpec for SPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CTRL to value 0x003c_0000"]
impl crate::Resettable for SPI_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x003c_0000;
}
