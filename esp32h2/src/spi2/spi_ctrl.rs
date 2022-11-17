#[doc = "Register `SPI_CTRL` reader"]
pub struct R(crate::R<SPI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CTRL` writer"]
pub struct W(crate::W<SPI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_DUMMY_OUT` reader - 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state."]
pub type SPI_DUMMY_OUT_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DUMMY_OUT` writer - 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state."]
pub type SPI_DUMMY_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_FADDR_DUAL` reader - Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FADDR_DUAL` writer - Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_FADDR_QUAD` reader - Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FADDR_QUAD` writer - Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_FADDR_OCT` reader - Apply 8 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FADDR_OCT_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FCMD_DUAL` reader - Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FCMD_DUAL` writer - Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_FCMD_QUAD` reader - Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FCMD_QUAD` writer - Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_FCMD_OCT` reader - Apply 8 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
pub type SPI_FCMD_OCT_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FREAD_DUAL` reader - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_DUAL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FREAD_DUAL` writer - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_QUAD_R = crate::BitReader<bool>;
#[doc = "Field `SPI_FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_FREAD_OCT` reader - In the read operations read-data phase apply 8 signals. 1: enable 0: disable. Can be configured in CONF state."]
pub type SPI_FREAD_OCT_R = crate::BitReader<bool>;
#[doc = "Field `SPI_Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_Q_POL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_Q_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_D_POL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
pub type SPI_D_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_HOLD_POL` reader - SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_HOLD_POL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_HOLD_POL` writer - SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_HOLD_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_WP_POL` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_WP_POL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_WP_POL` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
pub type SPI_WP_POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_CTRL_SPEC, bool, O>;
#[doc = "Field `SPI_RD_BIT_ORDER` reader - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type SPI_RD_BIT_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_RD_BIT_ORDER` writer - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
pub type SPI_RD_BIT_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPI_WR_BIT_ORDER` reader - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type SPI_WR_BIT_ORDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_WR_BIT_ORDER` writer - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
pub type SPI_WR_BIT_ORDER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_CTRL_SPEC, u8, u8, 2, O>;
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
    #[doc = "Bit 7 - Apply 8 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_faddr_oct(&self) -> SPI_FADDR_OCT_R {
        SPI_FADDR_OCT_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 10 - Apply 8 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fcmd_oct(&self) -> SPI_FCMD_OCT_R {
        SPI_FCMD_OCT_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 16 - In the read operations read-data phase apply 8 signals. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn spi_fread_oct(&self) -> SPI_FREAD_OCT_R {
        SPI_FREAD_OCT_R::new(((self.bits >> 16) & 1) != 0)
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
impl W {
    #[doc = "Bit 3 - 0: In the dummy phase, the FSPI bus signals are not output. 1: In the dummy phase, the FSPI bus signals are output. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dummy_out(&mut self) -> SPI_DUMMY_OUT_W<3> {
        SPI_DUMMY_OUT_W::new(self)
    }
    #[doc = "Bit 5 - Apply 2 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_faddr_dual(&mut self) -> SPI_FADDR_DUAL_W<5> {
        SPI_FADDR_DUAL_W::new(self)
    }
    #[doc = "Bit 6 - Apply 4 signals during addr phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_faddr_quad(&mut self) -> SPI_FADDR_QUAD_W<6> {
        SPI_FADDR_QUAD_W::new(self)
    }
    #[doc = "Bit 8 - Apply 2 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fcmd_dual(&mut self) -> SPI_FCMD_DUAL_W<8> {
        SPI_FCMD_DUAL_W::new(self)
    }
    #[doc = "Bit 9 - Apply 4 signals during command phase 1:enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fcmd_quad(&mut self) -> SPI_FCMD_QUAD_W<9> {
        SPI_FCMD_QUAD_W::new(self)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fread_dual(&mut self) -> SPI_FREAD_DUAL_W<14> {
        SPI_FREAD_DUAL_W::new(self)
    }
    #[doc = "Bit 15 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_fread_quad(&mut self) -> SPI_FREAD_QUAD_W<15> {
        SPI_FREAD_QUAD_W::new(self)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_q_pol(&mut self) -> SPI_Q_POL_W<18> {
        SPI_Q_POL_W::new(self)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_d_pol(&mut self) -> SPI_D_POL_W<19> {
        SPI_D_POL_W::new(self)
    }
    #[doc = "Bit 20 - SPI_HOLD output value when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_hold_pol(&mut self) -> SPI_HOLD_POL_W<20> {
        SPI_HOLD_POL_W::new(self)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_wp_pol(&mut self) -> SPI_WP_POL_W<21> {
        SPI_WP_POL_W::new(self)
    }
    #[doc = "Bits 23:24 - In read-data (MISO) phase 1: LSB first 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_rd_bit_order(&mut self) -> SPI_RD_BIT_ORDER_W<23> {
        SPI_RD_BIT_ORDER_W::new(self)
    }
    #[doc = "Bits 25:26 - In command address write-data (MOSI) phases 1: LSB firs 0: MSB first. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn spi_wr_bit_order(&mut self) -> SPI_WR_BIT_ORDER_W<25> {
        SPI_WR_BIT_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl](index.html) module"]
pub struct SPI_CTRL_SPEC;
impl crate::RegisterSpec for SPI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CTRL to value 0x003c_0000"]
impl crate::Resettable for SPI_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x003c_0000;
}
