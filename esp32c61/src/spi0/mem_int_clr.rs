#[doc = "Register `MEM_INT_CLR` writer"]
pub type W = crate::W<MEM_INT_CLR_SPEC>;
#[doc = "Field `MEM_SLV_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type MEM_SLV_ST_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_MST_ST_END_INT_CLR` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MEM_MST_ST_END_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_ECC_ERR_INT_CLR` writer - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type MEM_ECC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_PMS_REJECT_INT_CLR` writer - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type MEM_PMS_REJECT_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AXI_RADDR_ERR_INT_CLR` writer - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type MEM_AXI_RADDR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AXI_WR_FLASH_ERR_INT_CLR` writer - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type MEM_AXI_WR_FLASH_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AXI_WADDR_ERR_INT_CLR` writer - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type MEM_AXI_WADDR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DQS0_AFIFO_OVF_INT_CLR` writer - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
pub type MEM_DQS0_AFIFO_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DQS1_AFIFO_OVF_INT_CLR` writer - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
pub type MEM_DQS1_AFIFO_OVF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_BUS_FIFO1_UDF_INT_CLR` writer - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
pub type MEM_BUS_FIFO1_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_BUS_FIFO0_UDF_INT_CLR` writer - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
pub type MEM_BUS_FIFO0_UDF_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mem_slv_st_end_int_clr(&mut self) -> MEM_SLV_ST_END_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_SLV_ST_END_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mem_mst_st_end_int_clr(&mut self) -> MEM_MST_ST_END_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_MST_ST_END_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_ecc_err_int_clr(&mut self) -> MEM_ECC_ERR_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_ECC_ERR_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn mem_pms_reject_int_clr(&mut self) -> MEM_PMS_REJECT_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_PMS_REJECT_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_axi_raddr_err_int_clr(&mut self) -> MEM_AXI_RADDR_ERR_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_AXI_RADDR_ERR_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_axi_wr_flash_err_int_clr(
        &mut self,
    ) -> MEM_AXI_WR_FLASH_ERR_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_AXI_WR_FLASH_ERR_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mem_axi_waddr_err_int_clr(&mut self) -> MEM_AXI_WADDR_ERR_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_AXI_WADDR_ERR_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 28 - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn mem_dqs0_afifo_ovf_int_clr(&mut self) -> MEM_DQS0_AFIFO_OVF_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_DQS0_AFIFO_OVF_INT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn mem_dqs1_afifo_ovf_int_clr(&mut self) -> MEM_DQS1_AFIFO_OVF_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_DQS1_AFIFO_OVF_INT_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
    #[inline(always)]
    pub fn mem_bus_fifo1_udf_int_clr(&mut self) -> MEM_BUS_FIFO1_UDF_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_BUS_FIFO1_UDF_INT_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
    #[inline(always)]
    pub fn mem_bus_fifo0_udf_int_clr(&mut self) -> MEM_BUS_FIFO0_UDF_INT_CLR_W<MEM_INT_CLR_SPEC> {
        MEM_BUS_FIFO0_UDF_INT_CLR_W::new(self, 31)
    }
}
#[doc = "SPI0 interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_INT_CLR_SPEC;
impl crate::RegisterSpec for MEM_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mem_int_clr::W`](W) writer structure"]
impl crate::Writable for MEM_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_INT_CLR to value 0"]
impl crate::Resettable for MEM_INT_CLR_SPEC {}
