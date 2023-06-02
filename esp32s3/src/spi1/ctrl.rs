#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDUMMY_OUT` reader - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub type FDUMMY_OUT_R = crate::BitReader;
#[doc = "Field `FDUMMY_OUT` writer - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
pub type FDUMMY_OUT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FDOUT_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
pub type FDOUT_OCT_R = crate::BitReader;
#[doc = "Field `FDOUT_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
pub type FDOUT_OCT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FDIN_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
pub type FDIN_OCT_R = crate::BitReader;
#[doc = "Field `FDIN_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
pub type FDIN_OCT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FADDR_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
pub type FADDR_OCT_R = crate::BitReader;
#[doc = "Field `FADDR_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
pub type FADDR_OCT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FCMD_DUAL` reader - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
pub type FCMD_DUAL_R = crate::BitReader;
#[doc = "Field `FCMD_DUAL` writer - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
pub type FCMD_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FCMD_QUAD` reader - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
pub type FCMD_QUAD_R = crate::BitReader;
#[doc = "Field `FCMD_QUAD` writer - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
pub type FCMD_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FCMD_OCT` reader - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
pub type FCMD_OCT_R = crate::BitReader;
#[doc = "Field `FCMD_OCT` writer - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
pub type FCMD_OCT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FCS_CRC_EN` reader - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FCS_CRC_EN_R = crate::BitReader;
#[doc = "Field `FCS_CRC_EN` writer - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
pub type FCS_CRC_EN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `TX_CRC_EN` reader - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TX_CRC_EN_R = crate::BitReader;
#[doc = "Field `TX_CRC_EN` writer - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
pub type TX_CRC_EN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FASTRD_MODE` reader - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
pub type FASTRD_MODE_R = crate::BitReader;
#[doc = "Field `FASTRD_MODE` writer - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
pub type FASTRD_MODE_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FREAD_DUAL` reader - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_R = crate::BitReader;
#[doc = "Field `FREAD_DUAL` writer - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
pub type FREAD_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `RESANDRES` reader - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type RESANDRES_R = crate::BitReader;
#[doc = "Field `RESANDRES` writer - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
pub type RESANDRES_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `Q_POL` reader - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type Q_POL_R = crate::BitReader;
#[doc = "Field `Q_POL` writer - The bit is used to set MISO line polarity, 1: high 0, low"]
pub type Q_POL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `D_POL` reader - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type D_POL_R = crate::BitReader;
#[doc = "Field `D_POL` writer - The bit is used to set MOSI line polarity, 1: high 0, low"]
pub type D_POL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FREAD_QUAD` reader - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QUAD_R = crate::BitReader;
#[doc = "Field `FREAD_QUAD` writer - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `WP` reader - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type WP_R = crate::BitReader;
#[doc = "Field `WP` writer - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
pub type WP_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `WRSR_2B` reader - Two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type WRSR_2B_R = crate::BitReader;
#[doc = "Field `WRSR_2B` writer - Two bytes data will be written to status register when it is set. 1: enable 0: disable."]
pub type WRSR_2B_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FREAD_DIO` reader - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
pub type FREAD_DIO_R = crate::BitReader;
#[doc = "Field `FREAD_DIO` writer - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
pub type FREAD_DIO_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
#[doc = "Field `FREAD_QIO` reader - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QIO_R = crate::BitReader;
#[doc = "Field `FREAD_QIO` writer - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
pub type FREAD_QIO_W<'a, const O: u8> = crate::BitWriter<'a, CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 3 - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    pub fn fdummy_out(&self) -> FDUMMY_OUT_R {
        FDUMMY_OUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
    #[inline(always)]
    pub fn fdout_oct(&self) -> FDOUT_OCT_R {
        FDOUT_OCT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
    #[inline(always)]
    pub fn fdin_oct(&self) -> FDIN_OCT_R {
        FDIN_OCT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
    #[inline(always)]
    pub fn faddr_oct(&self) -> FADDR_OCT_R {
        FADDR_OCT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_dual(&self) -> FCMD_DUAL_R {
        FCMD_DUAL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_quad(&self) -> FCMD_QUAD_R {
        FCMD_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
    #[inline(always)]
    pub fn fcmd_oct(&self) -> FCMD_OCT_R {
        FCMD_OCT_R::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 13 - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
    #[inline(always)]
    pub fn fastrd_mode(&self) -> FASTRD_MODE_R {
        FASTRD_MODE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
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
    #[doc = "Bit 20 - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_quad(&self) -> FREAD_QUAD_R {
        FREAD_QUAD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    pub fn wp(&self) -> WP_R {
        WP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    pub fn wrsr_2b(&self) -> WRSR_2B_R {
        WRSR_2B_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    pub fn fread_dio(&self) -> FREAD_DIO_R {
        FREAD_DIO_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
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
            .field("fdout_oct", &format_args!("{}", self.fdout_oct().bit()))
            .field("fdin_oct", &format_args!("{}", self.fdin_oct().bit()))
            .field("faddr_oct", &format_args!("{}", self.faddr_oct().bit()))
            .field("fcmd_dual", &format_args!("{}", self.fcmd_dual().bit()))
            .field("fcmd_quad", &format_args!("{}", self.fcmd_quad().bit()))
            .field("fcmd_oct", &format_args!("{}", self.fcmd_oct().bit()))
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
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 3 - In the DUMMY phase the signal level of SPI bus is output by the SPI0 controller."]
    #[inline(always)]
    #[must_use]
    pub fn fdummy_out(&mut self) -> FDUMMY_OUT_W<3> {
        FDUMMY_OUT_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to enable 8-bit-mode(8-bm) in DOUT phase."]
    #[inline(always)]
    #[must_use]
    pub fn fdout_oct(&mut self) -> FDOUT_OCT_W<4> {
        FDOUT_OCT_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to enable 8-bit-mode(8-bm) in DIN phase."]
    #[inline(always)]
    #[must_use]
    pub fn fdin_oct(&mut self) -> FDIN_OCT_W<5> {
        FDIN_OCT_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to enable 8-bit-mode(8-bm) in ADDR phase."]
    #[inline(always)]
    #[must_use]
    pub fn faddr_oct(&mut self) -> FADDR_OCT_W<6> {
        FADDR_OCT_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to enable 2-bit-mode(2-bm) in CMD phase."]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_dual(&mut self) -> FCMD_DUAL_W<7> {
        FCMD_DUAL_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to enable 4-bit-mode(4-bm) in CMD phase."]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_quad(&mut self) -> FCMD_QUAD_W<8> {
        FCMD_QUAD_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to enable 8-bit-mode(8-bm) in CMD phase."]
    #[inline(always)]
    #[must_use]
    pub fn fcmd_oct(&mut self) -> FCMD_OCT_W<9> {
        FCMD_OCT_W::new(self)
    }
    #[doc = "Bit 10 - For SPI1, initialize crc32 module before writing encrypted data to flash. Active low."]
    #[inline(always)]
    #[must_use]
    pub fn fcs_crc_en(&mut self) -> FCS_CRC_EN_W<10> {
        FCS_CRC_EN_W::new(self)
    }
    #[doc = "Bit 11 - For SPI1, enable crc32 when writing encrypted data to flash. 1: enable 0:disable"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_en(&mut self) -> TX_CRC_EN_W<11> {
        TX_CRC_EN_W::new(self)
    }
    #[doc = "Bit 13 - This bit should be set when SPI_MEM_FREAD_QIO, SPI_MEM_FREAD_DIO, SPI_MEM_FREAD_QUAD or SPI_MEM_FREAD_DUAL is set."]
    #[inline(always)]
    #[must_use]
    pub fn fastrd_mode(&mut self) -> FASTRD_MODE_W<13> {
        FASTRD_MODE_W::new(self)
    }
    #[doc = "Bit 14 - In hardware 0x3B read operation, DIN phase apply 2 signals. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_dual(&mut self) -> FREAD_DUAL_W<14> {
        FREAD_DUAL_W::new(self)
    }
    #[doc = "Bit 15 - The Device ID is read out to SPI_MEM_RD_STATUS register, this bit combine with spi_mem_flash_res bit. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn resandres(&mut self) -> RESANDRES_W<15> {
        RESANDRES_W::new(self)
    }
    #[doc = "Bit 18 - The bit is used to set MISO line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn q_pol(&mut self) -> Q_POL_W<18> {
        Q_POL_W::new(self)
    }
    #[doc = "Bit 19 - The bit is used to set MOSI line polarity, 1: high 0, low"]
    #[inline(always)]
    #[must_use]
    pub fn d_pol(&mut self) -> D_POL_W<19> {
        D_POL_W::new(self)
    }
    #[doc = "Bit 20 - In hardware 0x6B read operation, DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_quad(&mut self) -> FREAD_QUAD_W<20> {
        FREAD_QUAD_W::new(self)
    }
    #[doc = "Bit 21 - Write protect signal output when SPI is idle. 1: output high, 0: output low."]
    #[inline(always)]
    #[must_use]
    pub fn wp(&mut self) -> WP_W<21> {
        WP_W::new(self)
    }
    #[doc = "Bit 22 - Two bytes data will be written to status register when it is set. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn wrsr_2b(&mut self) -> WRSR_2B_W<22> {
        WRSR_2B_W::new(self)
    }
    #[doc = "Bit 23 - In hardware 0xBB read operation, ADDR phase and DIN phase apply 2 signals(2-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_dio(&mut self) -> FREAD_DIO_W<23> {
        FREAD_DIO_W::new(self)
    }
    #[doc = "Bit 24 - In hardware 0xEB read operation, ADDR phase and DIN phase apply 4 signals(4-bit-mode). 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn fread_qio(&mut self) -> FREAD_QIO_W<24> {
        FREAD_QIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x002c_a000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x002c_a000;
}
