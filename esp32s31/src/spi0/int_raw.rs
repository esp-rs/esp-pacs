#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `PER_END` reader - "]
pub type PER_END_R = crate::BitReader;
#[doc = "Field `PER_END` writer - "]
pub type PER_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PES_END` reader - "]
pub type PES_END_R = crate::BitReader;
#[doc = "Field `PES_END` writer - "]
pub type PES_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPE_END` reader - "]
pub type WPE_END_R = crate::BitReader;
#[doc = "Field `WPE_END` writer - "]
pub type WPE_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_ST_END` reader - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SLV_ST_END_R = crate::BitReader;
#[doc = "Field `SLV_ST_END` writer - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_ST_END` reader - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MST_ST_END_R = crate::BitReader;
#[doc = "Field `MST_ST_END` writer - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
pub type MST_ST_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR` reader - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type ECC_ERR_R = crate::BitReader;
#[doc = "Field `ECC_ERR` writer - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
pub type ECC_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_REJECT` reader - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type PMS_REJECT_R = crate::BitReader;
#[doc = "Field `PMS_REJECT` writer - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
pub type PMS_REJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR` reader - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type AXI_RADDR_ERR_R = crate::BitReader;
#[doc = "Field `AXI_RADDR_ERR` writer - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
pub type AXI_RADDR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR` reader - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type AXI_WR_FLASH_ERR_R = crate::BitReader;
#[doc = "Field `AXI_WR_FLASH_ERR` writer - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
pub type AXI_WR_FLASH_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_WADDR_ERR` reader - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type AXI_WADDR_ERR_R = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR` writer - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
pub type AXI_WADDR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - "]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - "]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTS_FAULT_DETECTED` reader - The raw bit for SPI_MEM_XTS_FAULT_DETECTED_INT interrupt. 1: Triggered when xts aes core detected fault injection attack. 0: Others."]
pub type XTS_FAULT_DETECTED_R = crate::BitReader;
#[doc = "Field `XTS_FAULT_DETECTED` writer - The raw bit for SPI_MEM_XTS_FAULT_DETECTED_INT interrupt. 1: Triggered when xts aes core detected fault injection attack. 0: Others."]
pub type XTS_FAULT_DETECTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRANS_OVF` reader - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
pub type RX_TRANS_OVF_R = crate::BitReader;
#[doc = "Field `RX_TRANS_OVF` writer - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
pub type RX_TRANS_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_TRANS_UDF` reader - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
pub type TX_TRANS_UDF_R = crate::BitReader;
#[doc = "Field `TX_TRANS_UDF` writer - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
pub type TX_TRANS_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS0_AFIFO_OVF` reader - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type DQS0_AFIFO_OVF_R = crate::BitReader;
#[doc = "Field `DQS0_AFIFO_OVF` writer - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
pub type DQS0_AFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS1_AFIFO_OVF` reader - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type DQS1_AFIFO_OVF_R = crate::BitReader;
#[doc = "Field `DQS1_AFIFO_OVF` writer - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
pub type DQS1_AFIFO_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_FIFO1_UDF` reader - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type BUS_FIFO1_UDF_R = crate::BitReader;
#[doc = "Field `BUS_FIFO1_UDF` writer - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
pub type BUS_FIFO1_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_FIFO0_UDF` reader - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type BUS_FIFO0_UDF_R = crate::BitReader;
#[doc = "Field `BUS_FIFO0_UDF` writer - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
pub type BUS_FIFO0_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn per_end(&self) -> PER_END_R {
        PER_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pes_end(&self) -> PES_END_R {
        PES_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wpe_end(&self) -> WPE_END_R {
        WPE_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn slv_st_end(&self) -> SLV_ST_END_R {
        SLV_ST_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mst_st_end(&self) -> MST_ST_END_R {
        MST_ST_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    pub fn pms_reject(&self) -> PMS_REJECT_R {
        PMS_REJECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_raddr_err(&self) -> AXI_RADDR_ERR_R {
        AXI_RADDR_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&self) -> AXI_WR_FLASH_ERR_R {
        AXI_WR_FLASH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_waddr_err(&self) -> AXI_WADDR_ERR_R {
        AXI_WADDR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw bit for SPI_MEM_XTS_FAULT_DETECTED_INT interrupt. 1: Triggered when xts aes core detected fault injection attack. 0: Others."]
    #[inline(always)]
    pub fn xts_fault_detected(&self) -> XTS_FAULT_DETECTED_R {
        XTS_FAULT_DETECTED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 26 - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
    #[inline(always)]
    pub fn rx_trans_ovf(&self) -> RX_TRANS_OVF_R {
        RX_TRANS_OVF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
    #[inline(always)]
    pub fn tx_trans_udf(&self) -> TX_TRANS_UDF_R {
        TX_TRANS_UDF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&self) -> DQS0_AFIFO_OVF_R {
        DQS0_AFIFO_OVF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&self) -> DQS1_AFIFO_OVF_R {
        DQS1_AFIFO_OVF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    pub fn bus_fifo1_udf(&self) -> BUS_FIFO1_UDF_R {
        BUS_FIFO1_UDF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
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
            .field("xts_fault_detected", &self.xts_fault_detected())
            .field("rx_trans_ovf", &self.rx_trans_ovf())
            .field("tx_trans_udf", &self.tx_trans_udf())
            .field("dqs0_afifo_ovf", &self.dqs0_afifo_ovf())
            .field("dqs1_afifo_ovf", &self.dqs1_afifo_ovf())
            .field("bus_fifo1_udf", &self.bus_fifo1_udf())
            .field("bus_fifo0_udf", &self.bus_fifo0_udf())
            .field("per_end", &self.per_end())
            .field("pes_end", &self.pes_end())
            .field("wpe_end", &self.wpe_end())
            .field("brown_out", &self.brown_out())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn per_end(&mut self) -> PER_END_W<'_, INT_RAW_SPEC> {
        PER_END_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pes_end(&mut self) -> PES_END_W<'_, INT_RAW_SPEC> {
        PES_END_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wpe_end(&mut self) -> WPE_END_W<'_, INT_RAW_SPEC> {
        WPE_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw bit for SPI_MEM_SLV_ST_END_INT interrupt. 1: Triggered when spi0_slv_st is changed from non idle state to idle state. It means that SPI_CS raises high. 0: Others"]
    #[inline(always)]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<'_, INT_RAW_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw bit for SPI_MEM_MST_ST_END_INT interrupt. 1: Triggered when spi0_mst_st is changed from non idle state to idle state. 0: Others."]
    #[inline(always)]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<'_, INT_RAW_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw bit for SPI_MEM_ECC_ERR_INT interrupt. When SPI_FMEM_ECC_ERR_INT_EN is set and SPI_SMEM_ECC_ERR_INT_EN is cleared, this bit is triggered when the error times of SPI0/1 ECC read flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN is cleared and SPI_SMEM_ECC_ERR_INT_EN is set, this bit is triggered when the error times of SPI0/1 ECC read external RAM are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are set, this bit is triggered when the total error times of SPI0/1 ECC read external RAM and flash are equal or bigger than SPI_MEM_ECC_ERR_INT_NUM. When SPI_FMEM_ECC_ERR_INT_EN and SPI_SMEM_ECC_ERR_INT_EN are cleared, this bit will not be triggered."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<'_, INT_RAW_SPEC> {
        ECC_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw bit for SPI_MEM_PMS_REJECT_INT interrupt. 1: Triggered when SPI1 access is rejected. 0: Others."]
    #[inline(always)]
    pub fn pms_reject(&mut self) -> PMS_REJECT_W<'_, INT_RAW_SPEC> {
        PMS_REJECT_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt. 1: Triggered when AXI read address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_raddr_err(&mut self) -> AXI_RADDR_ERR_W<'_, INT_RAW_SPEC> {
        AXI_RADDR_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt. 1: Triggered when AXI write flash request is received. 0: Others."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&mut self) -> AXI_WR_FLASH_ERR_W<'_, INT_RAW_SPEC> {
        AXI_WR_FLASH_ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt. 1: Triggered when AXI write address is invalid by compared to MMU configuration. 0: Others."]
    #[inline(always)]
    pub fn axi_waddr_err(&mut self) -> AXI_WADDR_ERR_W<'_, INT_RAW_SPEC> {
        AXI_WADDR_ERR_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<'_, INT_RAW_SPEC> {
        BROWN_OUT_W::new(self, 10)
    }
    #[doc = "Bit 13 - The raw bit for SPI_MEM_XTS_FAULT_DETECTED_INT interrupt. 1: Triggered when xts aes core detected fault injection attack. 0: Others."]
    #[inline(always)]
    pub fn xts_fault_detected(&mut self) -> XTS_FAULT_DETECTED_W<'_, INT_RAW_SPEC> {
        XTS_FAULT_DETECTED_W::new(self, 13)
    }
    #[doc = "Bit 26 - The raw bit for SPI_MEM_RX_TRANS_OVF_INT interrupt. 1: Triggered when the rx fifo to spi bus is overrflow."]
    #[inline(always)]
    pub fn rx_trans_ovf(&mut self) -> RX_TRANS_OVF_W<'_, INT_RAW_SPEC> {
        RX_TRANS_OVF_W::new(self, 26)
    }
    #[doc = "Bit 27 - The raw bit for SPI_MEM_TX_TRANS_UDF_INT interrupt. 1: Triggered when the tx fifo to spi bus is underflow."]
    #[inline(always)]
    pub fn tx_trans_udf(&mut self) -> TX_TRANS_UDF_W<'_, INT_RAW_SPEC> {
        TX_TRANS_UDF_W::new(self, 27)
    }
    #[doc = "Bit 28 - The raw bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS1 is overflow."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&mut self) -> DQS0_AFIFO_OVF_W<'_, INT_RAW_SPEC> {
        DQS0_AFIFO_OVF_W::new(self, 28)
    }
    #[doc = "Bit 29 - The raw bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt. 1: Triggered when the AFIFO connected to SPI_DQS is overflow."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&mut self) -> DQS1_AFIFO_OVF_W<'_, INT_RAW_SPEC> {
        DQS1_AFIFO_OVF_W::new(self, 29)
    }
    #[doc = "Bit 30 - The raw bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt. 1: Triggered when BUS1 FIFO is underflow."]
    #[inline(always)]
    pub fn bus_fifo1_udf(&mut self) -> BUS_FIFO1_UDF_W<'_, INT_RAW_SPEC> {
        BUS_FIFO1_UDF_W::new(self, 30)
    }
    #[doc = "Bit 31 - The raw bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt. 1: Triggered when BUS0 FIFO is underflow."]
    #[inline(always)]
    pub fn bus_fifo0_udf(&mut self) -> BUS_FIFO0_UDF_W<'_, INT_RAW_SPEC> {
        BUS_FIFO0_UDF_W::new(self, 31)
    }
}
#[doc = "SPI0 interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
