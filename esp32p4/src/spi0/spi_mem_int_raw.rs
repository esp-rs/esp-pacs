#[doc = "Register `SPI_MEM_INT_RAW` reader"]
pub type R = crate::R<SPI_MEM_INT_RAW_SPEC>;
#[doc = "Register `SPI_MEM_INT_RAW` writer"]
pub type W = crate::W<SPI_MEM_INT_RAW_SPEC>;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SPI_MEM_SLV_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_SLV_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SPI_MEM_SLV_ST_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type SPI_MEM_MST_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_MST_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type SPI_MEM_MST_ST_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_ECC_ERR_INT_RAW` reader - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type SPI_MEM_ECC_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_ECC_ERR_INT_RAW` writer - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type SPI_MEM_ECC_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_PMS_REJECT_INT_RAW` reader - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type SPI_MEM_PMS_REJECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_PMS_REJECT_INT_RAW` writer - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type SPI_MEM_PMS_REJECT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AXI_RADDR_ERR_INT_RAW` reader - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type SPI_MEM_AXI_RADDR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_RADDR_ERR_INT_RAW` writer - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type SPI_MEM_AXI_RADDR_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW` reader - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW` writer - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_AXI_WADDR_ERR_INT_RAW` reader - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type SPI_MEM_AXI_WADDR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_WADDR_ERR_INT_RAW` writer - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type SPI_MEM_AXI_WADDR_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DQS0_AFIFO_OVF_INT_RAW` reader - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DQS0_AFIFO_OVF_INT_RAW` writer - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_DQS1_AFIFO_OVF_INT_RAW` reader - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_DQS1_AFIFO_OVF_INT_RAW` writer - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_BUS_FIFO1_UDF_INT_RAW` reader - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type SPI_MEM_BUS_FIFO1_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_BUS_FIFO1_UDF_INT_RAW` writer - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type SPI_MEM_BUS_FIFO1_UDF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MEM_BUS_FIFO0_UDF_INT_RAW` reader - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type SPI_MEM_BUS_FIFO0_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SPI_MEM_BUS_FIFO0_UDF_INT_RAW` writer - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type SPI_MEM_BUS_FIFO0_UDF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn spi_mem_slv_st_end_int_raw(&self) -> SPI_MEM_SLV_ST_END_INT_RAW_R {
        SPI_MEM_SLV_ST_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_mst_st_end_int_raw(&self) -> SPI_MEM_MST_ST_END_INT_RAW_R {
        SPI_MEM_MST_ST_END_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn spi_mem_ecc_err_int_raw(&self) -> SPI_MEM_ECC_ERR_INT_RAW_R {
        SPI_MEM_ECC_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_pms_reject_int_raw(&self) -> SPI_MEM_PMS_REJECT_INT_RAW_R {
        SPI_MEM_PMS_REJECT_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_axi_raddr_err_int_raw(&self) -> SPI_MEM_AXI_RADDR_ERR_INT_RAW_R {
        SPI_MEM_AXI_RADDR_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_axi_wr_flash_err_int_raw(&self) -> SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_R {
        SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_axi_waddr_err_int_raw(&self) -> SPI_MEM_AXI_WADDR_ERR_INT_RAW_R {
        SPI_MEM_AXI_WADDR_ERR_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    pub fn spi_mem_dqs0_afifo_ovf_int_raw(&self) -> SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_R {
        SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    pub fn spi_mem_dqs1_afifo_ovf_int_raw(&self) -> SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_R {
        SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    pub fn spi_mem_bus_fifo1_udf_int_raw(&self) -> SPI_MEM_BUS_FIFO1_UDF_INT_RAW_R {
        SPI_MEM_BUS_FIFO1_UDF_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
    #[inline(always)]
    pub fn spi_mem_bus_fifo0_udf_int_raw(&self) -> SPI_MEM_BUS_FIFO0_UDF_INT_RAW_R {
        SPI_MEM_BUS_FIFO0_UDF_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_INT_RAW")
            .field(
                "spi_mem_slv_st_end_int_raw",
                &format_args!("{}", self.spi_mem_slv_st_end_int_raw().bit()),
            )
            .field(
                "spi_mem_mst_st_end_int_raw",
                &format_args!("{}", self.spi_mem_mst_st_end_int_raw().bit()),
            )
            .field(
                "spi_mem_ecc_err_int_raw",
                &format_args!("{}", self.spi_mem_ecc_err_int_raw().bit()),
            )
            .field(
                "spi_mem_pms_reject_int_raw",
                &format_args!("{}", self.spi_mem_pms_reject_int_raw().bit()),
            )
            .field(
                "spi_mem_axi_raddr_err_int_raw",
                &format_args!("{}", self.spi_mem_axi_raddr_err_int_raw().bit()),
            )
            .field(
                "spi_mem_axi_wr_flash_err_int_raw",
                &format_args!("{}", self.spi_mem_axi_wr_flash_err_int_raw().bit()),
            )
            .field(
                "spi_mem_axi_waddr_err_int_raw",
                &format_args!("{}", self.spi_mem_axi_waddr_err_int_raw().bit()),
            )
            .field(
                "spi_mem_dqs0_afifo_ovf_int_raw",
                &format_args!("{}", self.spi_mem_dqs0_afifo_ovf_int_raw().bit()),
            )
            .field(
                "spi_mem_dqs1_afifo_ovf_int_raw",
                &format_args!("{}", self.spi_mem_dqs1_afifo_ovf_int_raw().bit()),
            )
            .field(
                "spi_mem_bus_fifo1_udf_int_raw",
                &format_args!("{}", self.spi_mem_bus_fifo1_udf_int_raw().bit()),
            )
            .field(
                "spi_mem_bus_fifo0_udf_int_raw",
                &format_args!("{}", self.spi_mem_bus_fifo0_udf_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_slv_st_end_int_raw(
        &mut self,
    ) -> SPI_MEM_SLV_ST_END_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_SLV_ST_END_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_mst_st_end_int_raw(
        &mut self,
    ) -> SPI_MEM_MST_ST_END_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_MST_ST_END_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ecc_err_int_raw(&mut self) -> SPI_MEM_ECC_ERR_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_ECC_ERR_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_pms_reject_int_raw(
        &mut self,
    ) -> SPI_MEM_PMS_REJECT_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_PMS_REJECT_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_axi_raddr_err_int_raw(
        &mut self,
    ) -> SPI_MEM_AXI_RADDR_ERR_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_AXI_RADDR_ERR_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_axi_wr_flash_err_int_raw(
        &mut self,
    ) -> SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_AXI_WR_FLASH_ERR_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_axi_waddr_err_int_raw(
        &mut self,
    ) -> SPI_MEM_AXI_WADDR_ERR_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_AXI_WADDR_ERR_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dqs0_afifo_ovf_int_raw(
        &mut self,
    ) -> SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_DQS0_AFIFO_OVF_INT_RAW_W::new(self, 28)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_dqs1_afifo_ovf_int_raw(
        &mut self,
    ) -> SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_DQS1_AFIFO_OVF_INT_RAW_W::new(self, 29)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_bus_fifo1_udf_int_raw(
        &mut self,
    ) -> SPI_MEM_BUS_FIFO1_UDF_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_BUS_FIFO1_UDF_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_bus_fifo0_udf_int_raw(
        &mut self,
    ) -> SPI_MEM_BUS_FIFO0_UDF_INT_RAW_W<SPI_MEM_INT_RAW_SPEC> {
        SPI_MEM_BUS_FIFO0_UDF_INT_RAW_W::new(self, 31)
    }
}
#[doc = "SPI0 interrupt raw register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_mem_int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_INT_RAW_SPEC;
impl crate::RegisterSpec for SPI_MEM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_int_raw::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_mem_int_raw::W`](W) writer structure"]
impl crate::Writable for SPI_MEM_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_MEM_INT_RAW to value 0"]
impl crate::Resettable for SPI_MEM_INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
