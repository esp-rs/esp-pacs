#[doc = "Register `SPI_MEM_CTRL1` reader"]
pub struct R(crate::R<SPI_MEM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CTRL1` writer"]
pub struct W(crate::W<SPI_MEM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CTRL1_SPEC>;
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
impl From<crate::W<SPI_MEM_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CLK_MODE` reader - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_CLK_MODE` writer - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
pub type SPI_MEM_CLK_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_MEM_CTRL1_SPEC, 2, O>;
#[doc = "Field `SPI_AR_SIZE0_1_SUPPORT_EN` reader - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
pub type SPI_AR_SIZE0_1_SUPPORT_EN_R = crate::BitReader;
#[doc = "Field `SPI_AR_SIZE0_1_SUPPORT_EN` writer - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
pub type SPI_AR_SIZE0_1_SUPPORT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL1_SPEC, O>;
#[doc = "Field `SPI_AW_SIZE0_1_SUPPORT_EN` reader - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
pub type SPI_AW_SIZE0_1_SUPPORT_EN_R = crate::BitReader;
#[doc = "Field `SPI_AW_SIZE0_1_SUPPORT_EN` writer - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
pub type SPI_AW_SIZE0_1_SUPPORT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL1_SPEC, O>;
#[doc = "Field `SPI_AXI_RDATA_BACK_FAST` reader - 1: Reply AXI read data to AXI bus when one AXI read beat data is available. 0: Reply AXI read data to AXI bus when all the read data is available."]
pub type SPI_AXI_RDATA_BACK_FAST_R = crate::BitReader;
#[doc = "Field `SPI_MEM_RRESP_ECC_ERR_EN` reader - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
pub type SPI_MEM_RRESP_ECC_ERR_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_RRESP_ECC_ERR_EN` writer - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
pub type SPI_MEM_RRESP_ECC_ERR_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL1_SPEC, O>;
#[doc = "Field `SPI_MEM_AR_SPLICE_EN` reader - Set this bit to enable AXI Read Splice-transfer."]
pub type SPI_MEM_AR_SPLICE_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AW_SPLICE_EN` reader - Set this bit to enable AXI Write Splice-transfer."]
pub type SPI_MEM_AW_SPLICE_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_RAM0_EN` reader - When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
pub type SPI_MEM_RAM0_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DUAL_RAM_EN` reader - Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
pub type SPI_MEM_DUAL_RAM_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FAST_WRITE_EN` reader - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
pub type SPI_MEM_FAST_WRITE_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FAST_WRITE_EN` writer - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
pub type SPI_MEM_FAST_WRITE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL1_SPEC, O>;
#[doc = "Field `SPI_MEM_RXFIFO_RST` writer - The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO."]
pub type SPI_MEM_RXFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL1_SPEC, O>;
#[doc = "Field `SPI_MEM_TXFIFO_RST` writer - The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO."]
pub type SPI_MEM_TXFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CTRL1_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    pub fn spi_mem_clk_mode(&self) -> SPI_MEM_CLK_MODE_R {
        SPI_MEM_CLK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 21 - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    pub fn spi_ar_size0_1_support_en(&self) -> SPI_AR_SIZE0_1_SUPPORT_EN_R {
        SPI_AR_SIZE0_1_SUPPORT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    pub fn spi_aw_size0_1_support_en(&self) -> SPI_AW_SIZE0_1_SUPPORT_EN_R {
        SPI_AW_SIZE0_1_SUPPORT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: Reply AXI read data to AXI bus when one AXI read beat data is available. 0: Reply AXI read data to AXI bus when all the read data is available."]
    #[inline(always)]
    pub fn spi_axi_rdata_back_fast(&self) -> SPI_AXI_RDATA_BACK_FAST_R {
        SPI_AXI_RDATA_BACK_FAST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
    #[inline(always)]
    pub fn spi_mem_rresp_ecc_err_en(&self) -> SPI_MEM_RRESP_ECC_ERR_EN_R {
        SPI_MEM_RRESP_ECC_ERR_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable AXI Read Splice-transfer."]
    #[inline(always)]
    pub fn spi_mem_ar_splice_en(&self) -> SPI_MEM_AR_SPLICE_EN_R {
        SPI_MEM_AR_SPLICE_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable AXI Write Splice-transfer."]
    #[inline(always)]
    pub fn spi_mem_aw_splice_en(&self) -> SPI_MEM_AW_SPLICE_EN_R {
        SPI_MEM_AW_SPLICE_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 1, only EXT_RAM0 will be accessed. When SPI_MEM_DUAL_RAM_EN is 0 and SPI_MEM_RAM0_EN is 0, only EXT_RAM1 will be accessed. When SPI_MEM_DUAL_RAM_EN is 1, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
    #[inline(always)]
    pub fn spi_mem_ram0_en(&self) -> SPI_MEM_RAM0_EN_R {
        SPI_MEM_RAM0_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable DUAL-RAM mode, EXT_RAM0 and EXT_RAM1 will be accessed at the same time."]
    #[inline(always)]
    pub fn spi_mem_dual_ram_en(&self) -> SPI_MEM_DUAL_RAM_EN_R {
        SPI_MEM_DUAL_RAM_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
    #[inline(always)]
    pub fn spi_mem_fast_write_en(&self) -> SPI_MEM_FAST_WRITE_EN_R {
        SPI_MEM_FAST_WRITE_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CTRL1")
            .field(
                "spi_mem_clk_mode",
                &format_args!("{}", self.spi_mem_clk_mode().bits()),
            )
            .field(
                "spi_ar_size0_1_support_en",
                &format_args!("{}", self.spi_ar_size0_1_support_en().bit()),
            )
            .field(
                "spi_aw_size0_1_support_en",
                &format_args!("{}", self.spi_aw_size0_1_support_en().bit()),
            )
            .field(
                "spi_axi_rdata_back_fast",
                &format_args!("{}", self.spi_axi_rdata_back_fast().bit()),
            )
            .field(
                "spi_mem_rresp_ecc_err_en",
                &format_args!("{}", self.spi_mem_rresp_ecc_err_en().bit()),
            )
            .field(
                "spi_mem_ar_splice_en",
                &format_args!("{}", self.spi_mem_ar_splice_en().bit()),
            )
            .field(
                "spi_mem_aw_splice_en",
                &format_args!("{}", self.spi_mem_aw_splice_en().bit()),
            )
            .field(
                "spi_mem_ram0_en",
                &format_args!("{}", self.spi_mem_ram0_en().bit()),
            )
            .field(
                "spi_mem_dual_ram_en",
                &format_args!("{}", self.spi_mem_dual_ram_en().bit()),
            )
            .field(
                "spi_mem_fast_write_en",
                &format_args!("{}", self.spi_mem_fast_write_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI clock mode bits. 0: SPI clock is off when CS inactive 1: SPI clock is delayed one cycle after CS inactive 2: SPI clock is delayed two cycles after CS inactive 3: SPI clock is alwasy on."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_clk_mode(&mut self) -> SPI_MEM_CLK_MODE_W<0> {
        SPI_MEM_CLK_MODE_W::new(self)
    }
    #[doc = "Bit 21 - 1: MSPI supports ARSIZE 0~3. When ARSIZE =0~2, MSPI read address is 4*n and reply the real AXI read data back. 0: When ARSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ar_size0_1_support_en(&mut self) -> SPI_AR_SIZE0_1_SUPPORT_EN_W<21> {
        SPI_AR_SIZE0_1_SUPPORT_EN_W::new(self)
    }
    #[doc = "Bit 22 - 1: MSPI supports AWSIZE 0~3. 0: When AWSIZE 0~1, MSPI reply SLV_ERR."]
    #[inline(always)]
    #[must_use]
    pub fn spi_aw_size0_1_support_en(&mut self) -> SPI_AW_SIZE0_1_SUPPORT_EN_W<22> {
        SPI_AW_SIZE0_1_SUPPORT_EN_W::new(self)
    }
    #[doc = "Bit 24 - 1: RRESP is SLV_ERR when there is a ECC error in AXI read data. 0: RRESP is OKAY when there is a ECC error in AXI read data. The ECC error information is recorded in SPI_MEM_ECC_ERR_ADDR_REG."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_rresp_ecc_err_en(&mut self) -> SPI_MEM_RRESP_ECC_ERR_EN_W<24> {
        SPI_MEM_RRESP_ECC_ERR_EN_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to write data faster, do not wait write data has been stored in tx_bus_fifo_l2. It will wait 4*T_clk_ctrl to insure the write data has been stored in tx_bus_fifo_l2."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fast_write_en(&mut self) -> SPI_MEM_FAST_WRITE_EN_W<29> {
        SPI_MEM_FAST_WRITE_EN_W::new(self)
    }
    #[doc = "Bit 30 - The synchronous reset signal for SPI0 RX AFIFO and all the AES_MSPI SYNC FIFO to receive signals from AXI. Set this bit to reset these FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_rxfifo_rst(&mut self) -> SPI_MEM_RXFIFO_RST_W<30> {
        SPI_MEM_RXFIFO_RST_W::new(self)
    }
    #[doc = "Bit 31 - The synchronous reset signal for SPI0 TX AFIFO and all the AES_MSPI SYNC FIFO to send signals to AXI. Set this bit to reset these FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_txfifo_rst(&mut self) -> SPI_MEM_TXFIFO_RST_W<31> {
        SPI_MEM_TXFIFO_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 control1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_ctrl1](index.html) module"]
pub struct SPI_MEM_CTRL1_SPEC;
impl crate::RegisterSpec for SPI_MEM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_ctrl1::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_ctrl1::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CTRL1 to value 0x28e0_0000"]
impl crate::Resettable for SPI_MEM_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x28e0_0000;
}
