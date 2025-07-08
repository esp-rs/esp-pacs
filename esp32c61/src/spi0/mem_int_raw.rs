#[doc = "Register `MEM_INT_RAW` reader"]
pub type R = crate::R<MEM_INT_RAW_SPEC>;
#[doc = "Register `MEM_INT_RAW` writer"]
pub type W = crate::W<MEM_INT_RAW_SPEC>;
#[doc = "Field `MEM_SLV_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type MEM_SLV_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_SLV_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type MEM_SLV_ST_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_MST_ST_END_INT_RAW` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MEM_MST_ST_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_MST_ST_END_INT_RAW` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MEM_MST_ST_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_ECC_ERR_INT_RAW` reader - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type MEM_ECC_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_ECC_ERR_INT_RAW` writer - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type MEM_ECC_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_PMS_REJECT_INT_RAW` reader - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type MEM_PMS_REJECT_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_PMS_REJECT_INT_RAW` writer - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type MEM_PMS_REJECT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AXI_RADDR_ERR_INT_RAW` reader - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type MEM_AXI_RADDR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_AXI_RADDR_ERR_INT_RAW` writer - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type MEM_AXI_RADDR_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AXI_WR_FLASH_ERR_INT_RAW` reader - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type MEM_AXI_WR_FLASH_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_AXI_WR_FLASH_ERR_INT_RAW` writer - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type MEM_AXI_WR_FLASH_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_AXI_WADDR_ERR_INT_RAW` reader - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type MEM_AXI_WADDR_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_AXI_WADDR_ERR_INT_RAW` writer - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type MEM_AXI_WADDR_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DQS0_AFIFO_OVF_INT_RAW` reader - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type MEM_DQS0_AFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_DQS0_AFIFO_OVF_INT_RAW` writer - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type MEM_DQS0_AFIFO_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_DQS1_AFIFO_OVF_INT_RAW` reader - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type MEM_DQS1_AFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_DQS1_AFIFO_OVF_INT_RAW` writer - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type MEM_DQS1_AFIFO_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_BUS_FIFO1_UDF_INT_RAW` reader - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type MEM_BUS_FIFO1_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_BUS_FIFO1_UDF_INT_RAW` writer - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type MEM_BUS_FIFO1_UDF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_BUS_FIFO0_UDF_INT_RAW` reader - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type MEM_BUS_FIFO0_UDF_INT_RAW_R = crate::BitReader;
#[doc = "Field `MEM_BUS_FIFO0_UDF_INT_RAW` writer - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type MEM_BUS_FIFO0_UDF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn mem_slv_st_end_int_raw(&self) -> MEM_SLV_ST_END_INT_RAW_R {
        MEM_SLV_ST_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mem_mst_st_end_int_raw(&self) -> MEM_MST_ST_END_INT_RAW_R {
        MEM_MST_ST_END_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn mem_ecc_err_int_raw(&self) -> MEM_ECC_ERR_INT_RAW_R {
        MEM_ECC_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    pub fn mem_pms_reject_int_raw(&self) -> MEM_PMS_REJECT_INT_RAW_R {
        MEM_PMS_REJECT_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn mem_axi_raddr_err_int_raw(&self) -> MEM_AXI_RADDR_ERR_INT_RAW_R {
        MEM_AXI_RADDR_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    pub fn mem_axi_wr_flash_err_int_raw(&self) -> MEM_AXI_WR_FLASH_ERR_INT_RAW_R {
        MEM_AXI_WR_FLASH_ERR_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn mem_axi_waddr_err_int_raw(&self) -> MEM_AXI_WADDR_ERR_INT_RAW_R {
        MEM_AXI_WADDR_ERR_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    pub fn mem_dqs0_afifo_ovf_int_raw(&self) -> MEM_DQS0_AFIFO_OVF_INT_RAW_R {
        MEM_DQS0_AFIFO_OVF_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    pub fn mem_dqs1_afifo_ovf_int_raw(&self) -> MEM_DQS1_AFIFO_OVF_INT_RAW_R {
        MEM_DQS1_AFIFO_OVF_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    pub fn mem_bus_fifo1_udf_int_raw(&self) -> MEM_BUS_FIFO1_UDF_INT_RAW_R {
        MEM_BUS_FIFO1_UDF_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
    #[inline(always)]
    pub fn mem_bus_fifo0_udf_int_raw(&self) -> MEM_BUS_FIFO0_UDF_INT_RAW_R {
        MEM_BUS_FIFO0_UDF_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_INT_RAW")
            .field("mem_slv_st_end_int_raw", &self.mem_slv_st_end_int_raw())
            .field("mem_mst_st_end_int_raw", &self.mem_mst_st_end_int_raw())
            .field("mem_ecc_err_int_raw", &self.mem_ecc_err_int_raw())
            .field("mem_pms_reject_int_raw", &self.mem_pms_reject_int_raw())
            .field(
                "mem_axi_raddr_err_int_raw",
                &self.mem_axi_raddr_err_int_raw(),
            )
            .field(
                "mem_axi_wr_flash_err_int_raw",
                &self.mem_axi_wr_flash_err_int_raw(),
            )
            .field(
                "mem_axi_waddr_err_int_raw",
                &self.mem_axi_waddr_err_int_raw(),
            )
            .field(
                "mem_dqs0_afifo_ovf_int_raw",
                &self.mem_dqs0_afifo_ovf_int_raw(),
            )
            .field(
                "mem_dqs1_afifo_ovf_int_raw",
                &self.mem_dqs1_afifo_ovf_int_raw(),
            )
            .field(
                "mem_bus_fifo1_udf_int_raw",
                &self.mem_bus_fifo1_udf_int_raw(),
            )
            .field(
                "mem_bus_fifo0_udf_int_raw",
                &self.mem_bus_fifo0_udf_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn mem_slv_st_end_int_raw(&mut self) -> MEM_SLV_ST_END_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_SLV_ST_END_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mem_mst_st_end_int_raw(&mut self) -> MEM_MST_ST_END_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_MST_ST_END_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn mem_ecc_err_int_raw(&mut self) -> MEM_ECC_ERR_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_ECC_ERR_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    pub fn mem_pms_reject_int_raw(&mut self) -> MEM_PMS_REJECT_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_PMS_REJECT_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn mem_axi_raddr_err_int_raw(&mut self) -> MEM_AXI_RADDR_ERR_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_AXI_RADDR_ERR_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    pub fn mem_axi_wr_flash_err_int_raw(
        &mut self,
    ) -> MEM_AXI_WR_FLASH_ERR_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_AXI_WR_FLASH_ERR_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn mem_axi_waddr_err_int_raw(&mut self) -> MEM_AXI_WADDR_ERR_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_AXI_WADDR_ERR_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    pub fn mem_dqs0_afifo_ovf_int_raw(&mut self) -> MEM_DQS0_AFIFO_OVF_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_DQS0_AFIFO_OVF_INT_RAW_W::new(self, 28)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    pub fn mem_dqs1_afifo_ovf_int_raw(&mut self) -> MEM_DQS1_AFIFO_OVF_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_DQS1_AFIFO_OVF_INT_RAW_W::new(self, 29)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    pub fn mem_bus_fifo1_udf_int_raw(&mut self) -> MEM_BUS_FIFO1_UDF_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_BUS_FIFO1_UDF_INT_RAW_W::new(self, 30)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
    #[inline(always)]
    pub fn mem_bus_fifo0_udf_int_raw(&mut self) -> MEM_BUS_FIFO0_UDF_INT_RAW_W<MEM_INT_RAW_SPEC> {
        MEM_BUS_FIFO0_UDF_INT_RAW_W::new(self, 31)
    }
}
#[doc = "SPI0 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_INT_RAW_SPEC;
impl crate::RegisterSpec for MEM_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_int_raw::R`](R) reader structure"]
impl crate::Readable for MEM_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_int_raw::W`](W) writer structure"]
impl crate::Writable for MEM_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_INT_RAW to value 0"]
impl crate::Resettable for MEM_INT_RAW_SPEC {}
