#[doc = "Register `INT_CLR` reader"]
pub type R = crate::R<INT_CLR_SPEC>;
#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `PER_END` reader - "]
pub type PER_END_R = crate::BitReader;
#[doc = "Field `PER_END` writer - "]
pub type PER_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PES_END` reader - "]
pub type PES_END_R = crate::BitReader;
#[doc = "Field `PES_END` writer - "]
pub type PES_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `WPE_END` reader - "]
pub type WPE_END_R = crate::BitReader;
#[doc = "Field `WPE_END` writer - "]
pub type WPE_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SLV_ST_END` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_ST_END` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ECC_ERR` writer - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PMS_REJECT` writer - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR` writer - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR` writer - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AXI_WR_FLASH_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_WADDR_ERR` writer - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AXI_WADDR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BROWN_OUT` reader - "]
pub type BROWN_OUT_R = crate::BitReader;
#[doc = "Field `BROWN_OUT` writer - "]
pub type BROWN_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `XTS_FAULT_DETECTED` writer - The clear bit for SPI_MEM_XTS_FAULT_DETECTED_INT interrupt."]
pub type XTS_FAULT_DETECTED_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_TRANS_OVF` writer - The clear bit for SPI_MEM_RX_TRANS_OVF_INT interrupt."]
pub type RX_TRANS_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_TRANS_UDF` writer - The clear bit for SPI_MEM_TX_TRANS_UDF_INT interrupt."]
pub type TX_TRANS_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DQS0_AFIFO_OVF` writer - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
pub type DQS0_AFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DQS1_AFIFO_OVF` writer - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
pub type DQS1_AFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUS_FIFO1_UDF` writer - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
pub type BUS_FIFO1_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUS_FIFO0_UDF` writer - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
pub type BUS_FIFO0_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
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
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn brown_out(&self) -> BROWN_OUT_R {
        BROWN_OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_CLR")
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
    pub fn per_end(&mut self) -> PER_END_W<'_, INT_CLR_SPEC> {
        PER_END_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pes_end(&mut self) -> PES_END_W<'_, INT_CLR_SPEC> {
        PES_END_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wpe_end(&mut self) -> WPE_END_W<'_, INT_CLR_SPEC> {
        WPE_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<'_, INT_CLR_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<'_, INT_CLR_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    #[doc = "Bit 5 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<'_, INT_CLR_SPEC> {
        ECC_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn pms_reject(&mut self) -> PMS_REJECT_W<'_, INT_CLR_SPEC> {
        PMS_REJECT_W::new(self, 6)
    }
    #[doc = "Bit 7 - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_raddr_err(&mut self) -> AXI_RADDR_ERR_W<'_, INT_CLR_SPEC> {
        AXI_RADDR_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err(&mut self) -> AXI_WR_FLASH_ERR_W<'_, INT_CLR_SPEC> {
        AXI_WR_FLASH_ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err(&mut self) -> AXI_WADDR_ERR_W<'_, INT_CLR_SPEC> {
        AXI_WADDR_ERR_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn brown_out(&mut self) -> BROWN_OUT_W<'_, INT_CLR_SPEC> {
        BROWN_OUT_W::new(self, 10)
    }
    #[doc = "Bit 13 - The clear bit for SPI_MEM_XTS_FAULT_DETECTED_INT interrupt."]
    #[inline(always)]
    pub fn xts_fault_detected(&mut self) -> XTS_FAULT_DETECTED_W<'_, INT_CLR_SPEC> {
        XTS_FAULT_DETECTED_W::new(self, 13)
    }
    #[doc = "Bit 26 - The clear bit for SPI_MEM_RX_TRANS_OVF_INT interrupt."]
    #[inline(always)]
    pub fn rx_trans_ovf(&mut self) -> RX_TRANS_OVF_W<'_, INT_CLR_SPEC> {
        RX_TRANS_OVF_W::new(self, 26)
    }
    #[doc = "Bit 27 - The clear bit for SPI_MEM_TX_TRANS_UDF_INT interrupt."]
    #[inline(always)]
    pub fn tx_trans_udf(&mut self) -> TX_TRANS_UDF_W<'_, INT_CLR_SPEC> {
        TX_TRANS_UDF_W::new(self, 27)
    }
    #[doc = "Bit 28 - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&mut self) -> DQS0_AFIFO_OVF_W<'_, INT_CLR_SPEC> {
        DQS0_AFIFO_OVF_W::new(self, 28)
    }
    #[doc = "Bit 29 - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&mut self) -> DQS1_AFIFO_OVF_W<'_, INT_CLR_SPEC> {
        DQS1_AFIFO_OVF_W::new(self, 29)
    }
    #[doc = "Bit 30 - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo1_udf(&mut self) -> BUS_FIFO1_UDF_W<'_, INT_CLR_SPEC> {
        BUS_FIFO1_UDF_W::new(self, 30)
    }
    #[doc = "Bit 31 - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo0_udf(&mut self) -> BUS_FIFO0_UDF_W<'_, INT_CLR_SPEC> {
        BUS_FIFO0_UDF_W::new(self, 31)
    }
}
#[doc = "SPI0 interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_clr::R`](R) reader structure"]
impl crate::Readable for INT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfc00_27ff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
