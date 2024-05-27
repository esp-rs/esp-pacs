///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Register `INT_RAW` writer
pub type W = crate::W<INT_RAW_SPEC>;
///Field `SLV_ST_END` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
pub type SLV_ST_END_R = crate::BitReader;
///Field `SLV_ST_END` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MST_ST_END` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
pub type MST_ST_END_R = crate::BitReader;
///Field `MST_ST_END` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
pub type MST_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECC_ERR` reader - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
pub type ECC_ERR_R = crate::BitReader;
///Field `ECC_ERR` writer - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
pub type ECC_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PMS_REJECT` reader - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
pub type PMS_REJECT_R = crate::BitReader;
///Field `PMS_REJECT` writer - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
pub type PMS_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_RADDR_ERR` reader - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
pub type AXI_RADDR_ERR_R = crate::BitReader;
///Field `AXI_RADDR_ERR` writer - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
pub type AXI_RADDR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_WR_FLASH_ERR` reader - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
pub type AXI_WR_FLASH_ERR_R = crate::BitReader;
///Field `AXI_WR_FLASH_ERR` writer - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
pub type AXI_WR_FLASH_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AXI_WADDR_ERR` reader - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
pub type AXI_WADDR_ERR_R = crate::BitReader;
///Field `AXI_WADDR_ERR` writer - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
pub type AXI_WADDR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQS0_AFIFO_OVF` reader - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
pub type DQS0_AFIFO_OVF_R = crate::BitReader;
///Field `DQS0_AFIFO_OVF` writer - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
pub type DQS0_AFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQS1_AFIFO_OVF` reader - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
pub type DQS1_AFIFO_OVF_R = crate::BitReader;
///Field `DQS1_AFIFO_OVF` writer - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
pub type DQS1_AFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUS_FIFO1_UDF` reader - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
pub type BUS_FIFO1_UDF_R = crate::BitReader;
///Field `BUS_FIFO1_UDF` writer - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
pub type BUS_FIFO1_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BUS_FIFO0_UDF` reader - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
pub type BUS_FIFO0_UDF_R = crate::BitReader;
///Field `BUS_FIFO0_UDF` writer - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
pub type BUS_FIFO0_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    #[inline(always)]
    pub fn slv_st_end(&self) -> SLV_ST_END_R {
        SLV_ST_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
    #[inline(always)]
    pub fn mst_st_end(&self) -> MST_ST_END_R {
        MST_ST_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
    #[inline(always)]
    pub fn pms_reject(&self) -> PMS_REJECT_R {
        PMS_REJECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub fn axi_raddr_err(&self) -> AXI_RADDR_ERR_R {
        AXI_RADDR_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
    #[inline(always)]
    pub fn axi_wr_flash_err(&self) -> AXI_WR_FLASH_ERR_R {
        AXI_WR_FLASH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    pub fn axi_waddr_err(&self) -> AXI_WADDR_ERR_R {
        AXI_WADDR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&self) -> DQS0_AFIFO_OVF_R {
        DQS0_AFIFO_OVF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&self) -> DQS1_AFIFO_OVF_R {
        DQS1_AFIFO_OVF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
    #[inline(always)]
    pub fn bus_fifo1_udf(&self) -> BUS_FIFO1_UDF_R {
        BUS_FIFO1_UDF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
    #[inline(always)]
    pub fn bus_fifo0_udf(&self) -> BUS_FIFO0_UDF_R {
        BUS_FIFO0_UDF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("slv_st_end", &self.slv_st_end())
            .field("mst_st_end", &self.mst_st_end())
            .field("ecc_err", &self.ecc_err())
            .field("pms_reject", &self.pms_reject())
            .field("axi_raddr_err", &self.axi_raddr_err())
            .field("axi_wr_flash_err", &self.axi_wr_flash_err())
            .field("axi_waddr_err", &self.axi_waddr_err())
            .field("dqs0_afifo_ovf", &self.dqs0_afifo_ovf())
            .field("dqs1_afifo_ovf", &self.dqs1_afifo_ovf())
            .field("bus_fifo1_udf", &self.bus_fifo1_udf())
            .field("bus_fifo0_udf", &self.bus_fifo0_udf())
            .finish()
    }
}
impl W {
    ///Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others
    #[inline(always)]
    #[must_use]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<INT_RAW_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    ///Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others.
    #[inline(always)]
    #[must_use]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<INT_RAW_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    ///Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered.
    #[inline(always)]
    #[must_use]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<INT_RAW_SPEC> {
        ECC_ERR_W::new(self, 5)
    }
    ///Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others.
    #[inline(always)]
    #[must_use]
    pub fn pms_reject(&mut self) -> PMS_REJECT_W<INT_RAW_SPEC> {
        PMS_REJECT_W::new(self, 6)
    }
    ///Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    #[must_use]
    pub fn axi_raddr_err(&mut self) -> AXI_RADDR_ERR_W<INT_RAW_SPEC> {
        AXI_RADDR_ERR_W::new(self, 7)
    }
    ///Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others.
    #[inline(always)]
    #[must_use]
    pub fn axi_wr_flash_err(&mut self) -> AXI_WR_FLASH_ERR_W<INT_RAW_SPEC> {
        AXI_WR_FLASH_ERR_W::new(self, 8)
    }
    ///Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others.
    #[inline(always)]
    #[must_use]
    pub fn axi_waddr_err(&mut self) -> AXI_WADDR_ERR_W<INT_RAW_SPEC> {
        AXI_WADDR_ERR_W::new(self, 9)
    }
    ///Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow.
    #[inline(always)]
    #[must_use]
    pub fn dqs0_afifo_ovf(&mut self) -> DQS0_AFIFO_OVF_W<INT_RAW_SPEC> {
        DQS0_AFIFO_OVF_W::new(self, 28)
    }
    ///Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow.
    #[inline(always)]
    #[must_use]
    pub fn dqs1_afifo_ovf(&mut self) -> DQS1_AFIFO_OVF_W<INT_RAW_SPEC> {
        DQS1_AFIFO_OVF_W::new(self, 29)
    }
    ///Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow.
    #[inline(always)]
    #[must_use]
    pub fn bus_fifo1_udf(&mut self) -> BUS_FIFO1_UDF_W<INT_RAW_SPEC> {
        BUS_FIFO1_UDF_W::new(self, 30)
    }
    ///Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow.
    #[inline(always)]
    #[must_use]
    pub fn bus_fifo0_udf(&mut self) -> BUS_FIFO0_UDF_W<INT_RAW_SPEC> {
        BUS_FIFO0_UDF_W::new(self, 31)
    }
}
/**SPI0 interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`write(|w| ..)` method takes [`int_raw::W`](W) writer structure
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
