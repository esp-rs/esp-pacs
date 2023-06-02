#[doc = "Register `SPI_MEM_CTRL` reader"]
pub struct R(crate::R<SPI_MEM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CTRL` writer"]
pub struct W(crate::W<SPI_MEM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_FDUMMY_RIN` reader - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type SPI_MEM_FDUMMY_RIN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDUMMY_RIN` writer - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type SPI_MEM_FDUMMY_RIN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FDUMMY_WOUT` reader - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type SPI_MEM_FDUMMY_WOUT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDUMMY_WOUT` writer - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
pub type SPI_MEM_FDUMMY_WOUT_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FDOUT_OCT` reader - Apply 8 signals during write-data phase 1:enable 0: disable"]
pub type SPI_MEM_FDOUT_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDIN_OCT` reader - Apply 8 signals during read-data phase 1:enable 0: disable"]
pub type SPI_MEM_FDIN_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FADDR_OCT` reader - Apply 8 signals during address phase 1:enable 0: disable"]
pub type SPI_MEM_FADDR_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FCMD_QUAD` reader - Apply 4 signals during command phase 1:enable 0: disable"]
pub type SPI_MEM_FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FCMD_QUAD` writer - Apply 4 signals during command phase 1:enable 0: disable"]
pub type SPI_MEM_FCMD_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FCMD_OCT` reader - Apply 8 signals during command phase 1:enable 0: disable"]
pub type SPI_MEM_FCMD_OCT_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FCS_CRC_EN` reader - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
pub type SPI_MEM_FCS_CRC_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_TX_CRC_EN` reader - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type SPI_MEM_TX_CRC_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FASTRD_MODE` reader - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub type SPI_MEM_FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FASTRD_MODE` writer - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
pub type SPI_MEM_FASTRD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FREAD_DUAL` reader - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_DUAL` writer - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_RESANDRES` reader - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type SPI_MEM_RESANDRES_R = crate::BitReader;
#[doc = "Field `SPI_MEM_RESANDRES` writer - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type SPI_MEM_RESANDRES_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type SPI_MEM_Q_POL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type SPI_MEM_Q_POL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type SPI_MEM_D_POL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type SPI_MEM_D_POL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FREAD_QUAD` reader - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_QUAD` writer - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type SPI_MEM_WP_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type SPI_MEM_WP_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_WRSR_2B` reader - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type SPI_MEM_WRSR_2B_R = crate::BitReader;
#[doc = "Field `SPI_MEM_WRSR_2B` writer - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type SPI_MEM_WRSR_2B_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FREAD_DIO` reader - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_DIO` writer - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_DIO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FREAD_QIO` reader - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FREAD_QIO` writer - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
pub type SPI_MEM_FREAD_QIO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 2 - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_mem_fdummy_rin(&self) -> SPI_MEM_FDUMMY_RIN_R {
        SPI_MEM_FDUMMY_RIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    pub fn spi_mem_fdummy_wout(&self) -> SPI_MEM_FDUMMY_WOUT_R {
        SPI_MEM_FDUMMY_WOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Apply 8 signals during write-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fdout_oct(&self) -> SPI_MEM_FDOUT_OCT_R {
        SPI_MEM_FDOUT_OCT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Apply 8 signals during read-data phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fdin_oct(&self) -> SPI_MEM_FDIN_OCT_R {
        SPI_MEM_FDIN_OCT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Apply 8 signals during address phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_faddr_oct(&self) -> SPI_MEM_FADDR_OCT_R {
        SPI_MEM_FADDR_OCT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fcmd_quad(&self) -> SPI_MEM_FCMD_QUAD_R {
        SPI_MEM_FCMD_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Apply 8 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    pub fn spi_mem_fcmd_oct(&self) -> SPI_MEM_FCMD_OCT_R {
        SPI_MEM_FCMD_OCT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    pub fn spi_mem_fcs_crc_en(&self) -> SPI_MEM_FCS_CRC_EN_R {
        SPI_MEM_FCS_CRC_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    pub fn spi_mem_tx_crc_en(&self) -> SPI_MEM_TX_CRC_EN_R {
        SPI_MEM_TX_CRC_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fastrd_mode(&self) -> SPI_MEM_FASTRD_MODE_R {
        SPI_MEM_FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_dual(&self) -> SPI_MEM_FREAD_DUAL_R {
        SPI_MEM_FREAD_DUAL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_resandres(&self) -> SPI_MEM_RESANDRES_R {
        SPI_MEM_RESANDRES_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn spi_mem_q_pol(&self) -> SPI_MEM_Q_POL_R {
        SPI_MEM_Q_POL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    pub fn spi_mem_d_pol(&self) -> SPI_MEM_D_POL_R {
        SPI_MEM_D_POL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_quad(&self) -> SPI_MEM_FREAD_QUAD_R {
        SPI_MEM_FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn spi_mem_wp(&self) -> SPI_MEM_WP_R {
        SPI_MEM_WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_wrsr_2b(&self) -> SPI_MEM_WRSR_2B_R {
        SPI_MEM_WRSR_2B_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_dio(&self) -> SPI_MEM_FREAD_DIO_R {
        SPI_MEM_FREAD_DIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_fread_qio(&self) -> SPI_MEM_FREAD_QIO_R {
        SPI_MEM_FREAD_QIO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CTRL")
            .field(
                "spi_mem_fdummy_rin",
                &format_args!("{}", self.spi_mem_fdummy_rin().bit()),
            )
            .field(
                "spi_mem_fdummy_wout",
                &format_args!("{}", self.spi_mem_fdummy_wout().bit()),
            )
            .field(
                "spi_mem_fdout_oct",
                &format_args!("{}", self.spi_mem_fdout_oct().bit()),
            )
            .field(
                "spi_mem_fdin_oct",
                &format_args!("{}", self.spi_mem_fdin_oct().bit()),
            )
            .field(
                "spi_mem_faddr_oct",
                &format_args!("{}", self.spi_mem_faddr_oct().bit()),
            )
            .field(
                "spi_mem_fcmd_quad",
                &format_args!("{}", self.spi_mem_fcmd_quad().bit()),
            )
            .field(
                "spi_mem_fcmd_oct",
                &format_args!("{}", self.spi_mem_fcmd_oct().bit()),
            )
            .field(
                "spi_mem_fcs_crc_en",
                &format_args!("{}", self.spi_mem_fcs_crc_en().bit()),
            )
            .field(
                "spi_mem_tx_crc_en",
                &format_args!("{}", self.spi_mem_tx_crc_en().bit()),
            )
            .field(
                "spi_mem_fastrd_mode",
                &format_args!("{}", self.spi_mem_fastrd_mode().bit()),
            )
            .field(
                "spi_mem_fread_dual",
                &format_args!("{}", self.spi_mem_fread_dual().bit()),
            )
            .field(
                "spi_mem_resandres",
                &format_args!("{}", self.spi_mem_resandres().bit()),
            )
            .field(
                "spi_mem_q_pol",
                &format_args!("{}", self.spi_mem_q_pol().bit()),
            )
            .field(
                "spi_mem_d_pol",
                &format_args!("{}", self.spi_mem_d_pol().bit()),
            )
            .field(
                "spi_mem_fread_quad",
                &format_args!("{}", self.spi_mem_fread_quad().bit()),
            )
            .field("spi_mem_wp", &format_args!("{}", self.spi_mem_wp().bit()))
            .field(
                "spi_mem_wrsr_2b",
                &format_args!("{}", self.spi_mem_wrsr_2b().bit()),
            )
            .field(
                "spi_mem_fread_dio",
                &format_args!("{}", self.spi_mem_fread_dio().bit()),
            )
            .field(
                "spi_mem_fread_qio",
                &format_args!("{}", self.spi_mem_fread_qio().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - In the dummy phase of a MSPI read data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdummy_rin(&mut self) -> SPI_MEM_FDUMMY_RIN_W<2> {
        SPI_MEM_FDUMMY_RIN_W::new(self)
    }
    #[doc = "Bit 3 - In the dummy phase of a MSPI write data transfer when accesses to flash, the signal level of SPI bus is output by the MSPI controller."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdummy_wout(&mut self) -> SPI_MEM_FDUMMY_WOUT_W<3> {
        SPI_MEM_FDUMMY_WOUT_W::new(self)
    }
    #[doc = "Bit 8 - Apply 4 signals during command phase 1:enable 0: disable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fcmd_quad(&mut self) -> SPI_MEM_FCMD_QUAD_W<8> {
        SPI_MEM_FCMD_QUAD_W::new(self)
    }
    #[doc = "Bit 13 - This bit enable the bits: spi_mem_fread_qio, spi_mem_fread_dio, spi_mem_fread_qout and spi_mem_fread_dout. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fastrd_mode(&mut self) -> SPI_MEM_FASTRD_MODE_W<13> {
        SPI_MEM_FASTRD_MODE_W::new(self)
    }
    #[doc = "Bit 14 - In the read operations, read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_dual(&mut self) -> SPI_MEM_FREAD_DUAL_W<14> {
        SPI_MEM_FREAD_DUAL_W::new(self)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_resandres(&mut self) -> SPI_MEM_RESANDRES_W<15> {
        SPI_MEM_RESANDRES_W::new(self)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_q_pol(&mut self) -> SPI_MEM_Q_POL_W<18> {
        SPI_MEM_Q_POL_W::new(self)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_d_pol(&mut self) -> SPI_MEM_D_POL_W<19> {
        SPI_MEM_D_POL_W::new(self)
    }
    #[doc = "Bit 20 - In the read operations read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_quad(&mut self) -> SPI_MEM_FREAD_QUAD_W<20> {
        SPI_MEM_FREAD_QUAD_W::new(self)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wp(&mut self) -> SPI_MEM_WP_W<21> {
        SPI_MEM_WP_W::new(self)
    }
    #[doc = "Bit 22 - two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_wrsr_2b(&mut self) -> SPI_MEM_WRSR_2B_W<22> {
        SPI_MEM_WRSR_2B_W::new(self)
    }
    #[doc = "Bit 23 - In the read operations address phase and read-data phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_dio(&mut self) -> SPI_MEM_FREAD_DIO_W<23> {
        SPI_MEM_FREAD_DIO_W::new(self)
    }
    #[doc = "Bit 24 - In the read operations address phase and read-data phase apply 4 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fread_qio(&mut self) -> SPI_MEM_FREAD_QIO_W<24> {
        SPI_MEM_FREAD_QIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl](index.html) module"]
pub struct SPI_MEM_CTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL to value 0x002c_a00c"]
impl crate::Resettable for SPI_MEM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x002c_a00c;
}
