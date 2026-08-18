#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `SLV_ST_END` writer - "]
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MST_ST_END` writer - "]
pub type MST_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ECC_ERR` writer - "]
pub type ECC_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PMS_REJECT` writer - "]
pub type PMS_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_RADDR_ERR` writer - "]
pub type AXI_RADDR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_WR_FLASH_ERR` writer - "]
pub type AXI_WR_FLASH_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AXI_WADDR_ERR` writer - "]
pub type AXI_WADDR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_TRANS_OVF` writer - "]
pub type RX_TRANS_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_TRANS_UDF` writer - "]
pub type TX_TRANS_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DQS_AFIFO_OVF(0-1)` writer - "]
pub type DQS_AFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUS_FIFO1_UDF` writer - "]
pub type BUS_FIFO1_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BUS_FIFO0_UDF` writer - "]
pub type BUS_FIFO0_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<'_, INT_CLR_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<'_, INT_CLR_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<'_, INT_CLR_SPEC> {
        ECC_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pms_reject(&mut self) -> PMS_REJECT_W<'_, INT_CLR_SPEC> {
        PMS_REJECT_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn axi_raddr_err(&mut self) -> AXI_RADDR_ERR_W<'_, INT_CLR_SPEC> {
        AXI_RADDR_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn axi_wr_flash_err(&mut self) -> AXI_WR_FLASH_ERR_W<'_, INT_CLR_SPEC> {
        AXI_WR_FLASH_ERR_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn axi_waddr_err(&mut self) -> AXI_WADDR_ERR_W<'_, INT_CLR_SPEC> {
        AXI_WADDR_ERR_W::new(self, 9)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rx_trans_ovf(&mut self) -> RX_TRANS_OVF_W<'_, INT_CLR_SPEC> {
        RX_TRANS_OVF_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tx_trans_udf(&mut self) -> TX_TRANS_UDF_W<'_, INT_CLR_SPEC> {
        TX_TRANS_UDF_W::new(self, 27)
    }
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DQS0_AFIFO_OVF` field.</div>"]
    #[inline(always)]
    pub fn dqs_afifo_ovf(&mut self, n: u8) -> DQS_AFIFO_OVF_W<'_, INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        DQS_AFIFO_OVF_W::new(self, n + 28)
    }
    #[doc = "Bit 28 - DQS0_AFIFO_OVF"]
    #[inline(always)]
    pub fn dqs0_afifo_ovf(&mut self) -> DQS_AFIFO_OVF_W<'_, INT_CLR_SPEC> {
        DQS_AFIFO_OVF_W::new(self, 28)
    }
    #[doc = "Bit 29 - DQS1_AFIFO_OVF"]
    #[inline(always)]
    pub fn dqs1_afifo_ovf(&mut self) -> DQS_AFIFO_OVF_W<'_, INT_CLR_SPEC> {
        DQS_AFIFO_OVF_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn bus_fifo1_udf(&mut self) -> BUS_FIFO1_UDF_W<'_, INT_CLR_SPEC> {
        BUS_FIFO1_UDF_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn bus_fifo0_udf(&mut self) -> BUS_FIFO0_UDF_W<'_, INT_CLR_SPEC> {
        BUS_FIFO0_UDF_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfc00_03f8;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
