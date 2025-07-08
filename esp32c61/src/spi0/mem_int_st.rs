#[doc = "Register `MEM_INT_ST` reader"]
pub type R = crate::R<MEM_INT_ST_SPEC>;
#[doc = "Field `MEM_SLV_ST_END_INT_ST` reader - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type MEM_SLV_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_MST_ST_END_INT_ST` reader - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MEM_MST_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_ECC_ERR_INT_ST` reader - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type MEM_ECC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_PMS_REJECT_INT_ST` reader - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type MEM_PMS_REJECT_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_AXI_RADDR_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type MEM_AXI_RADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_AXI_WR_FLASH_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type MEM_AXI_WR_FLASH_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_AXI_WADDR_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type MEM_AXI_WADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_DQS0_AFIFO_OVF_INT_ST` reader - The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
pub type MEM_DQS0_AFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_DQS1_AFIFO_OVF_INT_ST` reader - The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
pub type MEM_DQS1_AFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_BUS_FIFO1_UDF_INT_ST` reader - The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
pub type MEM_BUS_FIFO1_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `MEM_BUS_FIFO0_UDF_INT_ST` reader - The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
pub type MEM_BUS_FIFO0_UDF_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mem_slv_st_end_int_st(&self) -> MEM_SLV_ST_END_INT_ST_R {
        MEM_SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mem_mst_st_end_int_st(&self) -> MEM_MST_ST_END_INT_ST_R {
        MEM_MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_ecc_err_int_st(&self) -> MEM_ECC_ERR_INT_ST_R {
        MEM_ECC_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn mem_pms_reject_int_st(&self) -> MEM_PMS_REJECT_INT_ST_R {
        MEM_PMS_REJECT_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_axi_raddr_err_int_st(&self) -> MEM_AXI_RADDR_ERR_INT_ST_R {
        MEM_AXI_RADDR_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_axi_wr_flash_err_int_st(&self) -> MEM_AXI_WR_FLASH_ERR_INT_ST_R {
        MEM_AXI_WR_FLASH_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_axi_waddr_err_int_st(&self) -> MEM_AXI_WADDR_ERR_INT_ST_R {
        MEM_AXI_WADDR_ERR_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn mem_dqs0_afifo_ovf_int_st(&self) -> MEM_DQS0_AFIFO_OVF_INT_ST_R {
        MEM_DQS0_AFIFO_OVF_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn mem_dqs1_afifo_ovf_int_st(&self) -> MEM_DQS1_AFIFO_OVF_INT_ST_R {
        MEM_DQS1_AFIFO_OVF_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
    #[inline(always)]
    pub fn mem_bus_fifo1_udf_int_st(&self) -> MEM_BUS_FIFO1_UDF_INT_ST_R {
        MEM_BUS_FIFO1_UDF_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
    #[inline(always)]
    pub fn mem_bus_fifo0_udf_int_st(&self) -> MEM_BUS_FIFO0_UDF_INT_ST_R {
        MEM_BUS_FIFO0_UDF_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_INT_ST")
            .field("mem_slv_st_end_int_st", &self.mem_slv_st_end_int_st())
            .field("mem_mst_st_end_int_st", &self.mem_mst_st_end_int_st())
            .field("mem_ecc_err_int_st", &self.mem_ecc_err_int_st())
            .field("mem_pms_reject_int_st", &self.mem_pms_reject_int_st())
            .field("mem_axi_raddr_err_int_st", &self.mem_axi_raddr_err_int_st())
            .field(
                "mem_axi_wr_flash_err_int_st",
                &self.mem_axi_wr_flash_err_int_st(),
            )
            .field("mem_axi_waddr_err_int_st", &self.mem_axi_waddr_err_int_st())
            .field(
                "mem_dqs0_afifo_ovf_int_st",
                &self.mem_dqs0_afifo_ovf_int_st(),
            )
            .field(
                "mem_dqs1_afifo_ovf_int_st",
                &self.mem_dqs1_afifo_ovf_int_st(),
            )
            .field("mem_bus_fifo1_udf_int_st", &self.mem_bus_fifo1_udf_int_st())
            .field("mem_bus_fifo0_udf_int_st", &self.mem_bus_fifo0_udf_int_st())
            .finish()
    }
}
#[doc = "SPI0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_INT_ST_SPEC;
impl crate::RegisterSpec for MEM_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_int_st::R`](R) reader structure"]
impl crate::Readable for MEM_INT_ST_SPEC {}
#[doc = "`reset()` method sets MEM_INT_ST to value 0"]
impl crate::Resettable for MEM_INT_ST_SPEC {}
