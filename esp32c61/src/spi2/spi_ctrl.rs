#[doc = "Register `SPI_CTRL` reader"]
pub type R = crate::R<SPI_CTRL_SPEC>;
#[doc = "Register `SPI_CTRL` writer"]
pub type W = crate::W<SPI_CTRL_SPEC>;
#[doc = "Field `SPI_DUMMY_OUT` reader - Configures whether or not to output the FSPI bus signals in DUMMY state. \\\\ 0: Not output \\\\ 1: Output \\\\ Can be configured in CONF state."]
pub type SPI_DUMMY_OUT_R = crate::BitReader;
#[doc = "Field `SPI_DUMMY_OUT` writer - Configures whether or not to output the FSPI bus signals in DUMMY state. \\\\ 0: Not output \\\\ 1: Output \\\\ Can be configured in CONF state."]
pub type SPI_DUMMY_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FADDR_DUAL` reader - Configures whether or not to enable 2-bit mode during address (ADDR) state.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FADDR_DUAL` writer - Configures whether or not to enable 2-bit mode during address (ADDR) state.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FADDR_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FADDR_QUAD` reader - Configures whether or not to enable 4-bit mode during address (ADDR) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FADDR_QUAD` writer - Configures whether or not to enable 4-bit mode during address (ADDR) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FADDR_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FADDR_OCT` reader - Configures whether or not to enable 8-bit mode during address (ADDR) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FADDR_OCT_R = crate::BitReader;
#[doc = "Field `SPI_FCMD_DUAL` reader - Configures whether or not to enable 2-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FCMD_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FCMD_DUAL` writer - Configures whether or not to enable 2-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FCMD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FCMD_QUAD` reader - Configures whether or not to enable 4-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FCMD_QUAD` writer - Configures whether or not to enable 4-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FCMD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FCMD_OCT` reader - Configures whether or not to enable 8-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FCMD_OCT_R = crate::BitReader;
#[doc = "Field `SPI_FREAD_DUAL` reader - Configures whether or not to enable the 2-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_FREAD_DUAL` writer - Configures whether or not to enable the 2-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FREAD_DUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FREAD_QUAD` reader - Configures whether or not to enable the 4-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_FREAD_QUAD` writer - Configures whether or not to enable the 4-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FREAD_QUAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FREAD_OCT` reader - Configures whether or not to enable the 8-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
pub type SPI_FREAD_OCT_R = crate::BitReader;
#[doc = "Field `SPI_Q_POL` reader - Configures MISO line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
pub type SPI_Q_POL_R = crate::BitReader;
#[doc = "Field `SPI_Q_POL` writer - Configures MISO line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
pub type SPI_Q_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_D_POL` reader - Configures MOSI line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
pub type SPI_D_POL_R = crate::BitReader;
#[doc = "Field `SPI_D_POL` writer - Configures MOSI line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
pub type SPI_D_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_HOLD_POL` reader - Configures SPI_HOLD output value when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
pub type SPI_HOLD_POL_R = crate::BitReader;
#[doc = "Field `SPI_HOLD_POL` writer - Configures SPI_HOLD output value when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
pub type SPI_HOLD_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_WP_POL` reader - Configures the output value of write-protect signal when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
pub type SPI_WP_POL_R = crate::BitReader;
#[doc = "Field `SPI_WP_POL` writer - Configures the output value of write-protect signal when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
pub type SPI_WP_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_RD_BIT_ORDER` reader - Configures the bit order in read-data (MISO) state. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
pub type SPI_RD_BIT_ORDER_R = crate::FieldReader;
#[doc = "Field `SPI_RD_BIT_ORDER` writer - Configures the bit order in read-data (MISO) state. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
pub type SPI_RD_BIT_ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPI_WR_BIT_ORDER` reader - Configures the bit order in command (CMD), address (ADDR), and write-data (MOSI) states. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
pub type SPI_WR_BIT_ORDER_R = crate::FieldReader;
#[doc = "Field `SPI_WR_BIT_ORDER` writer - Configures the bit order in command (CMD), address (ADDR), and write-data (MOSI) states. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
pub type SPI_WR_BIT_ORDER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 3 - Configures whether or not to output the FSPI bus signals in DUMMY state. \\\\ 0: Not output \\\\ 1: Output \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dummy_out(&self) -> SPI_DUMMY_OUT_R {
        SPI_DUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable 2-bit mode during address (ADDR) state.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_dual(&self) -> SPI_FADDR_DUAL_R {
        SPI_FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures whether or not to enable 4-bit mode during address (ADDR) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_quad(&self) -> SPI_FADDR_QUAD_R {
        SPI_FADDR_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures whether or not to enable 8-bit mode during address (ADDR) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_oct(&self) -> SPI_FADDR_OCT_R {
        SPI_FADDR_OCT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not to enable 2-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_dual(&self) -> SPI_FCMD_DUAL_R {
        SPI_FCMD_DUAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to enable 4-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_quad(&self) -> SPI_FCMD_QUAD_R {
        SPI_FCMD_QUAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Configures whether or not to enable 8-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_oct(&self) -> SPI_FCMD_OCT_R {
        SPI_FCMD_OCT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Configures whether or not to enable the 2-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_dual(&self) -> SPI_FREAD_DUAL_R {
        SPI_FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the 4-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_quad(&self) -> SPI_FREAD_QUAD_R {
        SPI_FREAD_QUAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Configures whether or not to enable the 8-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_oct(&self) -> SPI_FREAD_OCT_R {
        SPI_FREAD_OCT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Configures MISO line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_q_pol(&self) -> SPI_Q_POL_R {
        SPI_Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Configures MOSI line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_d_pol(&self) -> SPI_D_POL_R {
        SPI_D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configures SPI_HOLD output value when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_hold_pol(&self) -> SPI_HOLD_POL_R {
        SPI_HOLD_POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configures the output value of write-protect signal when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_wp_pol(&self) -> SPI_WP_POL_R {
        SPI_WP_POL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Configures the bit order in read-data (MISO) state. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_rd_bit_order(&self) -> SPI_RD_BIT_ORDER_R {
        SPI_RD_BIT_ORDER_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Configures the bit order in command (CMD), address (ADDR), and write-data (MOSI) states. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_wr_bit_order(&self) -> SPI_WR_BIT_ORDER_R {
        SPI_WR_BIT_ORDER_R::new(((self.bits >> 25) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CTRL")
            .field("spi_dummy_out", &self.spi_dummy_out())
            .field("spi_faddr_dual", &self.spi_faddr_dual())
            .field("spi_faddr_quad", &self.spi_faddr_quad())
            .field("spi_faddr_oct", &self.spi_faddr_oct())
            .field("spi_fcmd_dual", &self.spi_fcmd_dual())
            .field("spi_fcmd_quad", &self.spi_fcmd_quad())
            .field("spi_fcmd_oct", &self.spi_fcmd_oct())
            .field("spi_fread_dual", &self.spi_fread_dual())
            .field("spi_fread_quad", &self.spi_fread_quad())
            .field("spi_fread_oct", &self.spi_fread_oct())
            .field("spi_q_pol", &self.spi_q_pol())
            .field("spi_d_pol", &self.spi_d_pol())
            .field("spi_hold_pol", &self.spi_hold_pol())
            .field("spi_wp_pol", &self.spi_wp_pol())
            .field("spi_rd_bit_order", &self.spi_rd_bit_order())
            .field("spi_wr_bit_order", &self.spi_wr_bit_order())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - Configures whether or not to output the FSPI bus signals in DUMMY state. \\\\ 0: Not output \\\\ 1: Output \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_dummy_out(&mut self) -> SPI_DUMMY_OUT_W<SPI_CTRL_SPEC> {
        SPI_DUMMY_OUT_W::new(self, 3)
    }
    #[doc = "Bit 5 - Configures whether or not to enable 2-bit mode during address (ADDR) state.\\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_dual(&mut self) -> SPI_FADDR_DUAL_W<SPI_CTRL_SPEC> {
        SPI_FADDR_DUAL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable 4-bit mode during address (ADDR) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_quad(&mut self) -> SPI_FADDR_QUAD_W<SPI_CTRL_SPEC> {
        SPI_FADDR_QUAD_W::new(self, 6)
    }
    #[doc = "Bit 8 - Configures whether or not to enable 2-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_dual(&mut self) -> SPI_FCMD_DUAL_W<SPI_CTRL_SPEC> {
        SPI_FCMD_DUAL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable 4-bit mode during command (CMD) state. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_quad(&mut self) -> SPI_FCMD_QUAD_W<SPI_CTRL_SPEC> {
        SPI_FCMD_QUAD_W::new(self, 9)
    }
    #[doc = "Bit 14 - Configures whether or not to enable the 2-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_dual(&mut self) -> SPI_FREAD_DUAL_W<SPI_CTRL_SPEC> {
        SPI_FREAD_DUAL_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable the 4-bit mode of read-data (DIN) state in read operations. \\\\ 0: Disable \\\\ 1: Enable \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_quad(&mut self) -> SPI_FREAD_QUAD_W<SPI_CTRL_SPEC> {
        SPI_FREAD_QUAD_W::new(self, 15)
    }
    #[doc = "Bit 18 - Configures MISO line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_q_pol(&mut self) -> SPI_Q_POL_W<SPI_CTRL_SPEC> {
        SPI_Q_POL_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures MOSI line polarity. \\\\ 0: Low \\\\ 1: High \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_d_pol(&mut self) -> SPI_D_POL_W<SPI_CTRL_SPEC> {
        SPI_D_POL_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures SPI_HOLD output value when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_hold_pol(&mut self) -> SPI_HOLD_POL_W<SPI_CTRL_SPEC> {
        SPI_HOLD_POL_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures the output value of write-protect signal when SPI is in idle. \\\\ 0: Output low \\\\ 1: Output high \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_wp_pol(&mut self) -> SPI_WP_POL_W<SPI_CTRL_SPEC> {
        SPI_WP_POL_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Configures the bit order in read-data (MISO) state. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_rd_bit_order(&mut self) -> SPI_RD_BIT_ORDER_W<SPI_CTRL_SPEC> {
        SPI_RD_BIT_ORDER_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - Configures the bit order in command (CMD), address (ADDR), and write-data (MOSI) states. \\\\ 0: MSB first \\\\ 1: LSB first \\\\ Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_wr_bit_order(&mut self) -> SPI_WR_BIT_ORDER_W<SPI_CTRL_SPEC> {
        SPI_WR_BIT_ORDER_W::new(self, 25)
    }
}
#[doc = "SPI control register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CTRL_SPEC;
impl crate::RegisterSpec for SPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrl::W`](W) writer structure"]
impl crate::Writable for SPI_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_CTRL to value 0x003c_0000"]
impl crate::Resettable for SPI_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x003c_0000;
}
