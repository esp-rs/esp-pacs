#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SLV_ST_END_INT_ST` reader - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_ST_END_INT_ST` reader - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `ECC_ERR_INT_ST` reader - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `PMS_REJECT_INT_ST` reader - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_INT_ST_R = crate::BitReader;
#[doc = "Field `AXI_RADDR_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `AXI_WR_FLASH_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AXI_WR_FLASH_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AXI_WADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `DQS0_AFIFO_OVF_INT_ST` reader - The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
pub type DQS0_AFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DQS1_AFIFO_OVF_INT_ST` reader - The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
pub type DQS1_AFIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `BUS_FIFO1_UDF_INT_ST` reader - The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
pub type BUS_FIFO1_UDF_INT_ST_R = crate::BitReader;
#[doc = "Field `BUS_FIFO0_UDF_INT_ST` reader - The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
pub type BUS_FIFO0_UDF_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_st(&self) -> SLV_ST_END_INT_ST_R {
        SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_st(&self) -> MST_ST_END_INT_ST_R {
        MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_st(&self) -> ECC_ERR_INT_ST_R {
        ECC_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn pms_reject_int_st(&self) -> PMS_REJECT_INT_ST_R {
        PMS_REJECT_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_raddr_err_int_st(&self) -> AXI_RADDR_ERR_INT_ST_R {
        AXI_RADDR_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err_int_st(&self) -> AXI_WR_FLASH_ERR_INT_ST_R {
        AXI_WR_FLASH_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err_int_st(&self) -> AXI_WADDR_ERR_INT_ST_R {
        AXI_WADDR_ERR_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - The status bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf_int_st(&self) -> DQS0_AFIFO_OVF_INT_ST_R {
        DQS0_AFIFO_OVF_INT_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The status bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf_int_st(&self) -> DQS1_AFIFO_OVF_INT_ST_R {
        DQS1_AFIFO_OVF_INT_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The status bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo1_udf_int_st(&self) -> BUS_FIFO1_UDF_INT_ST_R {
        BUS_FIFO1_UDF_INT_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The status bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo0_udf_int_st(&self) -> BUS_FIFO0_UDF_INT_ST_R {
        BUS_FIFO0_UDF_INT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("slv_st_end_int_st", &self.slv_st_end_int_st())
            .field("mst_st_end_int_st", &self.mst_st_end_int_st())
            .field("ecc_err_int_st", &self.ecc_err_int_st())
            .field("pms_reject_int_st", &self.pms_reject_int_st())
            .field("axi_raddr_err_int_st", &self.axi_raddr_err_int_st())
            .field("axi_wr_flash_err_int_st", &self.axi_wr_flash_err_int_st())
            .field("axi_waddr_err_int_st", &self.axi_waddr_err_int_st())
            .field("dqs0_afifo_ovf_int_st", &self.dqs0_afifo_ovf_int_st())
            .field("dqs1_afifo_ovf_int_st", &self.dqs1_afifo_ovf_int_st())
            .field("bus_fifo1_udf_int_st", &self.bus_fifo1_udf_int_st())
            .field("bus_fifo0_udf_int_st", &self.bus_fifo0_udf_int_st())
            .finish()
    }
}
#[doc = "SPI0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
