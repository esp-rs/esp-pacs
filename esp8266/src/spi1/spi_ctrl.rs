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
#[doc = "Field `spi_fastrd_mode` reader - this bit enable the bits: spi_qio_mode, spi_dio_mode, spi_qout_mode and spi_dout_mode"]
pub type SPI_FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `spi_fastrd_mode` writer - this bit enable the bits: spi_qio_mode, spi_dio_mode, spi_qout_mode and spi_dout_mode"]
pub type SPI_FASTRD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `spi_dout_mode` reader - In the read operations, \"read-data\" phase apply 2 signals"]
pub type SPI_DOUT_MODE_R = crate::BitReader;
#[doc = "Field `spi_dout_mode` writer - In the read operations, \"read-data\" phase apply 2 signals"]
pub type SPI_DOUT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `res_and_res` reader - 'Res and res'?"]
pub type RES_AND_RES_R = crate::BitReader;
#[doc = "Field `res_and_res` writer - 'Res and res'?"]
pub type RES_AND_RES_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `sst_aai` reader - SST_AAI?"]
pub type SST_AAI_R = crate::BitReader;
#[doc = "Field `sst_aai` writer - SST_AAI?"]
pub type SST_AAI_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `enable_ahb` reader - Enable AHB"]
pub type ENABLE_AHB_R = crate::BitReader;
#[doc = "Field `enable_ahb` writer - Enable AHB"]
pub type ENABLE_AHB_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `hold_mode` reader - Hold mode"]
pub type HOLD_MODE_R = crate::BitReader;
#[doc = "Field `hold_mode` writer - Hold mode"]
pub type HOLD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `share_but` reader - Share bus"]
pub type SHARE_BUT_R = crate::BitReader;
#[doc = "Field `share_but` writer - Share bus"]
pub type SHARE_BUT_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `spi_qout_mode` reader - In the read operations, \"read-data\" phase apply 4 signals"]
pub type SPI_QOUT_MODE_R = crate::BitReader;
#[doc = "Field `spi_qout_mode` writer - In the read operations, \"read-data\" phase apply 4 signals"]
pub type SPI_QOUT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `wp_reg` reader - Write protect?"]
pub type WP_REG_R = crate::BitReader;
#[doc = "Field `wp_reg` writer - Write protect?"]
pub type WP_REG_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `two_byte_status` reader - Enable two byte status"]
pub type TWO_BYTE_STATUS_R = crate::BitReader;
#[doc = "Field `two_byte_status` writer - Enable two byte status"]
pub type TWO_BYTE_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `spi_dio_mode` reader - In the read operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
pub type SPI_DIO_MODE_R = crate::BitReader;
#[doc = "Field `spi_dio_mode` writer - In the read operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
pub type SPI_DIO_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `spi_qio_mode` reader - In the read operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
pub type SPI_QIO_MODE_R = crate::BitReader;
#[doc = "Field `spi_qio_mode` writer - In the read operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
pub type SPI_QIO_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `spi_rd_bit_order` reader - In \"read-data\" (MISO) phase, 1: LSB first; 0: MSB first"]
pub type SPI_RD_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `spi_rd_bit_order` writer - In \"read-data\" (MISO) phase, 1: LSB first; 0: MSB first"]
pub type SPI_RD_BIT_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
#[doc = "Field `spi_wr_bit_order` reader - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: LSB first; 0: MSB first"]
pub type SPI_WR_BIT_ORDER_R = crate::BitReader;
#[doc = "Field `spi_wr_bit_order` writer - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: LSB first; 0: MSB first"]
pub type SPI_WR_BIT_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 13 - this bit enable the bits: spi_qio_mode, spi_dio_mode, spi_qout_mode and spi_dout_mode"]
    #[inline(always)]
    pub fn spi_fastrd_mode(&self) -> SPI_FASTRD_MODE_R {
        SPI_FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    pub fn spi_dout_mode(&self) -> SPI_DOUT_MODE_R {
        SPI_DOUT_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 'Res and res'?"]
    #[inline(always)]
    pub fn res_and_res(&self) -> RES_AND_RES_R {
        RES_AND_RES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SST_AAI?"]
    #[inline(always)]
    pub fn sst_aai(&self) -> SST_AAI_R {
        SST_AAI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable AHB"]
    #[inline(always)]
    pub fn enable_ahb(&self) -> ENABLE_AHB_R {
        ENABLE_AHB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Hold mode"]
    #[inline(always)]
    pub fn hold_mode(&self) -> HOLD_MODE_R {
        HOLD_MODE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Share bus"]
    #[inline(always)]
    pub fn share_but(&self) -> SHARE_BUT_R {
        SHARE_BUT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In the read operations, \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    pub fn spi_qout_mode(&self) -> SPI_QOUT_MODE_R {
        SPI_QOUT_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect?"]
    #[inline(always)]
    pub fn wp_reg(&self) -> WP_REG_R {
        WP_REG_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable two byte status"]
    #[inline(always)]
    pub fn two_byte_status(&self) -> TWO_BYTE_STATUS_R {
        TWO_BYTE_STATUS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the read operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    pub fn spi_dio_mode(&self) -> SPI_DIO_MODE_R {
        SPI_DIO_MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the read operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    pub fn spi_qio_mode(&self) -> SPI_QIO_MODE_R {
        SPI_QIO_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - In \"read-data\" (MISO) phase, 1: LSB first; 0: MSB first"]
    #[inline(always)]
    pub fn spi_rd_bit_order(&self) -> SPI_RD_BIT_ORDER_R {
        SPI_RD_BIT_ORDER_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: LSB first; 0: MSB first"]
    #[inline(always)]
    pub fn spi_wr_bit_order(&self) -> SPI_WR_BIT_ORDER_R {
        SPI_WR_BIT_ORDER_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CTRL")
            .field(
                "spi_wr_bit_order",
                &format_args!("{}", self.spi_wr_bit_order().bit()),
            )
            .field(
                "spi_rd_bit_order",
                &format_args!("{}", self.spi_rd_bit_order().bit()),
            )
            .field(
                "spi_qio_mode",
                &format_args!("{}", self.spi_qio_mode().bit()),
            )
            .field(
                "spi_dio_mode",
                &format_args!("{}", self.spi_dio_mode().bit()),
            )
            .field(
                "spi_qout_mode",
                &format_args!("{}", self.spi_qout_mode().bit()),
            )
            .field(
                "spi_dout_mode",
                &format_args!("{}", self.spi_dout_mode().bit()),
            )
            .field(
                "spi_fastrd_mode",
                &format_args!("{}", self.spi_fastrd_mode().bit()),
            )
            .field(
                "two_byte_status",
                &format_args!("{}", self.two_byte_status().bit()),
            )
            .field("wp_reg", &format_args!("{}", self.wp_reg().bit()))
            .field("share_but", &format_args!("{}", self.share_but().bit()))
            .field("hold_mode", &format_args!("{}", self.hold_mode().bit()))
            .field("enable_ahb", &format_args!("{}", self.enable_ahb().bit()))
            .field("sst_aai", &format_args!("{}", self.sst_aai().bit()))
            .field("res_and_res", &format_args!("{}", self.res_and_res().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 13 - this bit enable the bits: spi_qio_mode, spi_dio_mode, spi_qout_mode and spi_dout_mode"]
    #[inline(always)]
    #[must_use]
    pub fn spi_fastrd_mode(&mut self) -> SPI_FASTRD_MODE_W<13> {
        SPI_FASTRD_MODE_W::new(self)
    }
    #[doc = "Bit 14 - In the read operations, \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_dout_mode(&mut self) -> SPI_DOUT_MODE_W<14> {
        SPI_DOUT_MODE_W::new(self)
    }
    #[doc = "Bit 15 - 'Res and res'?"]
    #[inline(always)]
    #[must_use]
    pub fn res_and_res(&mut self) -> RES_AND_RES_W<15> {
        RES_AND_RES_W::new(self)
    }
    #[doc = "Bit 16 - SST_AAI?"]
    #[inline(always)]
    #[must_use]
    pub fn sst_aai(&mut self) -> SST_AAI_W<16> {
        SST_AAI_W::new(self)
    }
    #[doc = "Bit 17 - Enable AHB"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ahb(&mut self) -> ENABLE_AHB_W<17> {
        ENABLE_AHB_W::new(self)
    }
    #[doc = "Bit 18 - Hold mode"]
    #[inline(always)]
    #[must_use]
    pub fn hold_mode(&mut self) -> HOLD_MODE_W<18> {
        HOLD_MODE_W::new(self)
    }
    #[doc = "Bit 19 - Share bus"]
    #[inline(always)]
    #[must_use]
    pub fn share_but(&mut self) -> SHARE_BUT_W<19> {
        SHARE_BUT_W::new(self)
    }
    #[doc = "Bit 20 - In the read operations, \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_qout_mode(&mut self) -> SPI_QOUT_MODE_W<20> {
        SPI_QOUT_MODE_W::new(self)
    }
    #[doc = "Bit 21 - Write protect?"]
    #[inline(always)]
    #[must_use]
    pub fn wp_reg(&mut self) -> WP_REG_W<21> {
        WP_REG_W::new(self)
    }
    #[doc = "Bit 22 - Enable two byte status"]
    #[inline(always)]
    #[must_use]
    pub fn two_byte_status(&mut self) -> TWO_BYTE_STATUS_W<22> {
        TWO_BYTE_STATUS_W::new(self)
    }
    #[doc = "Bit 23 - In the read operations, \"address\" phase and \"read-data\" phase apply 2 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_dio_mode(&mut self) -> SPI_DIO_MODE_W<23> {
        SPI_DIO_MODE_W::new(self)
    }
    #[doc = "Bit 24 - In the read operations, \"address\" phase and \"read-data\" phase apply 4 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_qio_mode(&mut self) -> SPI_QIO_MODE_W<24> {
        SPI_QIO_MODE_W::new(self)
    }
    #[doc = "Bit 25 - In \"read-data\" (MISO) phase, 1: LSB first; 0: MSB first"]
    #[inline(always)]
    #[must_use]
    pub fn spi_rd_bit_order(&mut self) -> SPI_RD_BIT_ORDER_W<25> {
        SPI_RD_BIT_ORDER_W::new(self)
    }
    #[doc = "Bit 26 - In \"command\", \"address\", \"write-data\" (MOSI) phases, 1: LSB first; 0: MSB first"]
    #[inline(always)]
    #[must_use]
    pub fn spi_wr_bit_order(&mut self) -> SPI_WR_BIT_ORDER_W<26> {
        SPI_WR_BIT_ORDER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ctrl](index.html) module"]
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
#[doc = "`reset()` method sets SPI_CTRL to value 0"]
impl crate::Resettable for SPI_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
