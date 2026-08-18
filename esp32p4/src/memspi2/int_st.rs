#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SLV_ST_END` reader - "]
pub type SLV_ST_END_R = crate::BitReader;
#[doc = "Field `MST_ST_END` reader - "]
pub type MST_ST_END_R = crate::BitReader;
#[doc = "Field `ECC_ERR` reader - "]
pub type ECC_ERR_R = crate::BitReader;
#[doc = "Field `PMS_REJECT` reader - "]
pub type PMS_REJECT_R = crate::BitReader;
#[doc = "Field `AXI_RADDR_ERR` reader - "]
pub type AXI_RADDR_ERR_R = crate::BitReader;
#[doc = "Field `AXI_WR_FLASH_ERR` reader - "]
pub type AXI_WR_FLASH_ERR_R = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR` reader - "]
pub type AXI_WADDR_ERR_R = crate::BitReader;
#[doc = "Field `RX_TRANS_OVF` reader - "]
pub type RX_TRANS_OVF_R = crate::BitReader;
#[doc = "Field `TX_TRANS_UDF` reader - "]
pub type TX_TRANS_UDF_R = crate::BitReader;
#[doc = "Field `DQS_AFIFO_OVF(0-1)` reader - "]
pub type DQS_AFIFO_OVF_R = crate::BitReader;
#[doc = "Field `BUS_FIFO1_UDF` reader - "]
pub type BUS_FIFO1_UDF_R = crate::BitReader;
#[doc = "Field `BUS_FIFO0_UDF` reader - "]
pub type BUS_FIFO0_UDF_R = crate::BitReader;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_st_end(&self) -> SLV_ST_END_R {
        SLV_ST_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mst_st_end(&self) -> MST_ST_END_R {
        MST_ST_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ecc_err(&self) -> ECC_ERR_R {
        ECC_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pms_reject(&self) -> PMS_REJECT_R {
        PMS_REJECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn axi_raddr_err(&self) -> AXI_RADDR_ERR_R {
        AXI_RADDR_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn axi_wr_flash_err(&self) -> AXI_WR_FLASH_ERR_R {
        AXI_WR_FLASH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn axi_waddr_err(&self) -> AXI_WADDR_ERR_R {
        AXI_WADDR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_trans_ovf(&self) -> RX_TRANS_OVF_R {
        RX_TRANS_OVF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tx_trans_udf(&self) -> TX_TRANS_UDF_R {
        TX_TRANS_UDF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DQS0_AFIFO_OVF` field.</div>"]
    #[inline(always)]
    pub fn dqs_afifo_ovf(&self, n: u8) -> DQS_AFIFO_OVF_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DQS_AFIFO_OVF_R::new(((self.bits >> (n + 28)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn dqs_afifo_ovf_iter(&self) -> impl Iterator<Item = DQS_AFIFO_OVF_R> + '_ {
        (0..2).map(move |n| DQS_AFIFO_OVF_R::new(((self.bits >> (n + 28)) & 1) != 0))
    }
    #[doc = "Bit 28 - DQS0_AFIFO_OVF"]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&self) -> DQS_AFIFO_OVF_R {
        DQS_AFIFO_OVF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DQS1_AFIFO_OVF"]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&self) -> DQS_AFIFO_OVF_R {
        DQS_AFIFO_OVF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn bus_fifo1_udf(&self) -> BUS_FIFO1_UDF_R {
        BUS_FIFO1_UDF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn bus_fifo0_udf(&self) -> BUS_FIFO0_UDF_R {
        BUS_FIFO0_UDF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("slv_st_end", &self.slv_st_end())
            .field("mst_st_end", &self.mst_st_end())
            .field("ecc_err", &self.ecc_err())
            .field("pms_reject", &self.pms_reject())
            .field("axi_raddr_err", &self.axi_raddr_err())
            .field("axi_wr_flash_err", &self.axi_wr_flash_err())
            .field("axi_waddr_err", &self.axi_waddr_err())
            .field("rx_trans_ovf", &self.rx_trans_ovf())
            .field("tx_trans_udf", &self.tx_trans_udf())
            .field("dqs0_afifo_ovf", &self.dqs0_afifo_ovf())
            .field("dqs1_afifo_ovf", &self.dqs1_afifo_ovf())
            .field("bus_fifo1_udf", &self.bus_fifo1_udf())
            .field("bus_fifo0_udf", &self.bus_fifo0_udf())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
