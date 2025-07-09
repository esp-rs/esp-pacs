#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SLV_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_ST_END_INT_ENA` reader - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `MST_ST_END_INT_ENA` writer - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_ERR_INT_ENA` reader - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `ECC_ERR_INT_ENA` writer - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMS_REJECT_INT_ENA` reader - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `PMS_REJECT_INT_ENA` writer - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR_INT_ENA` reader - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `AXI_RADDR_ERR_INT_ENA` writer - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR_INT_ENA` reader - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AXI_WR_FLASH_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `AXI_WR_FLASH_ERR_INT_ENA` writer - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AXI_WR_FLASH_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI_WADDR_ERR_INT__ENA` reader - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AXI_WADDR_ERR_INT__ENA_R = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR_INT__ENA` writer - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AXI_WADDR_ERR_INT__ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS0_AFIFO_OVF_INT_ENA` reader - The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
pub type DQS0_AFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `DQS0_AFIFO_OVF_INT_ENA` writer - The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
pub type DQS0_AFIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS1_AFIFO_OVF_INT_ENA` reader - The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
pub type DQS1_AFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `DQS1_AFIFO_OVF_INT_ENA` writer - The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
pub type DQS1_AFIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_FIFO1_UDF_INT_ENA` reader - The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
pub type BUS_FIFO1_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `BUS_FIFO1_UDF_INT_ENA` writer - The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
pub type BUS_FIFO1_UDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_FIFO0_UDF_INT_ENA` reader - The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
pub type BUS_FIFO0_UDF_INT_ENA_R = crate::BitReader;
#[doc = "Field `BUS_FIFO0_UDF_INT_ENA` writer - The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
pub type BUS_FIFO0_UDF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_ena(&self) -> SLV_ST_END_INT_ENA_R {
        SLV_ST_END_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_ena(&self) -> MST_ST_END_INT_ENA_R {
        MST_ST_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_ena(&self) -> ECC_ERR_INT_ENA_R {
        ECC_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn pms_reject_int_ena(&self) -> PMS_REJECT_INT_ENA_R {
        PMS_REJECT_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_raddr_err_int_ena(&self) -> AXI_RADDR_ERR_INT_ENA_R {
        AXI_RADDR_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err_int_ena(&self) -> AXI_WR_FLASH_ERR_INT_ENA_R {
        AXI_WR_FLASH_ERR_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err_int__ena(&self) -> AXI_WADDR_ERR_INT__ENA_R {
        AXI_WADDR_ERR_INT__ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf_int_ena(&self) -> DQS0_AFIFO_OVF_INT_ENA_R {
        DQS0_AFIFO_OVF_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf_int_ena(&self) -> DQS1_AFIFO_OVF_INT_ENA_R {
        DQS1_AFIFO_OVF_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo1_udf_int_ena(&self) -> BUS_FIFO1_UDF_INT_ENA_R {
        BUS_FIFO1_UDF_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo0_udf_int_ena(&self) -> BUS_FIFO0_UDF_INT_ENA_R {
        BUS_FIFO0_UDF_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("slv_st_end_int_ena", &self.slv_st_end_int_ena())
            .field("mst_st_end_int_ena", &self.mst_st_end_int_ena())
            .field("ecc_err_int_ena", &self.ecc_err_int_ena())
            .field("pms_reject_int_ena", &self.pms_reject_int_ena())
            .field("axi_raddr_err_int_ena", &self.axi_raddr_err_int_ena())
            .field("axi_wr_flash_err_int_ena", &self.axi_wr_flash_err_int_ena())
            .field("axi_waddr_err_int__ena", &self.axi_waddr_err_int__ena())
            .field("dqs0_afifo_ovf_int_ena", &self.dqs0_afifo_ovf_int_ena())
            .field("dqs1_afifo_ovf_int_ena", &self.dqs1_afifo_ovf_int_ena())
            .field("bus_fifo1_udf_int_ena", &self.bus_fifo1_udf_int_ena())
            .field("bus_fifo0_udf_int_ena", &self.bus_fifo0_udf_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 3 - The enable bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_ena(&mut self) -> SLV_ST_END_INT_ENA_W<INT_ENA_SPEC> {
        SLV_ST_END_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_ena(&mut self) -> MST_ST_END_INT_ENA_W<INT_ENA_SPEC> {
        MST_ST_END_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The enable bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_ena(&mut self) -> ECC_ERR_INT_ENA_W<INT_ENA_SPEC> {
        ECC_ERR_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The enable bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn pms_reject_int_ena(&mut self) -> PMS_REJECT_INT_ENA_W<INT_ENA_SPEC> {
        PMS_REJECT_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_raddr_err_int_ena(&mut self) -> AXI_RADDR_ERR_INT_ENA_W<INT_ENA_SPEC> {
        AXI_RADDR_ERR_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err_int_ena(&mut self) -> AXI_WR_FLASH_ERR_INT_ENA_W<INT_ENA_SPEC> {
        AXI_WR_FLASH_ERR_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err_int__ena(&mut self) -> AXI_WADDR_ERR_INT__ENA_W<INT_ENA_SPEC> {
        AXI_WADDR_ERR_INT__ENA_W::new(self, 9)
    }
    #[doc = "Bit 28 - The enable bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs0_afifo_ovf_int_ena(&mut self) -> DQS0_AFIFO_OVF_INT_ENA_W<INT_ENA_SPEC> {
        DQS0_AFIFO_OVF_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - The enable bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt."]
    #[inline(always)]
    pub fn dqs1_afifo_ovf_int_ena(&mut self) -> DQS1_AFIFO_OVF_INT_ENA_W<INT_ENA_SPEC> {
        DQS1_AFIFO_OVF_INT_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - The enable bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo1_udf_int_ena(&mut self) -> BUS_FIFO1_UDF_INT_ENA_W<INT_ENA_SPEC> {
        BUS_FIFO1_UDF_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - The enable bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt."]
    #[inline(always)]
    pub fn bus_fifo0_udf_int_ena(&mut self) -> BUS_FIFO0_UDF_INT_ENA_W<INT_ENA_SPEC> {
        BUS_FIFO0_UDF_INT_ENA_W::new(self, 31)
    }
}
#[doc = "SPI0 interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
