#[doc = "Register `INT_ST1` reader"]
pub type R = crate::R<INT_ST1_SPEC>;
#[doc = "Field `TO_HS_TX` reader - NA"]
pub type TO_HS_TX_R = crate::BitReader;
#[doc = "Field `TO_LP_RX` reader - NA"]
pub type TO_LP_RX_R = crate::BitReader;
#[doc = "Field `ECC_SINGLE_ERR` reader - NA"]
pub type ECC_SINGLE_ERR_R = crate::BitReader;
#[doc = "Field `ECC_MILTI_ERR` reader - NA"]
pub type ECC_MILTI_ERR_R = crate::BitReader;
#[doc = "Field `CRC_ERR` reader - NA"]
pub type CRC_ERR_R = crate::BitReader;
#[doc = "Field `PKT_SIZE_ERR` reader - NA"]
pub type PKT_SIZE_ERR_R = crate::BitReader;
#[doc = "Field `EOPT_ERR` reader - NA"]
pub type EOPT_ERR_R = crate::BitReader;
#[doc = "Field `DPI_PLD_WR_ERR` reader - NA"]
pub type DPI_PLD_WR_ERR_R = crate::BitReader;
#[doc = "Field `GEN_CMD_WR_ERR` reader - NA"]
pub type GEN_CMD_WR_ERR_R = crate::BitReader;
#[doc = "Field `GEN_PLD_WR_ERR` reader - NA"]
pub type GEN_PLD_WR_ERR_R = crate::BitReader;
#[doc = "Field `GEN_PLD_SEND_ERR` reader - NA"]
pub type GEN_PLD_SEND_ERR_R = crate::BitReader;
#[doc = "Field `GEN_PLD_RD_ERR` reader - NA"]
pub type GEN_PLD_RD_ERR_R = crate::BitReader;
#[doc = "Field `GEN_PLD_RECEV_ERR` reader - NA"]
pub type GEN_PLD_RECEV_ERR_R = crate::BitReader;
#[doc = "Field `DPI_BUFF_PLD_UNDER` reader - NA"]
pub type DPI_BUFF_PLD_UNDER_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn to_hs_tx(&self) -> TO_HS_TX_R {
        TO_HS_TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn to_lp_rx(&self) -> TO_LP_RX_R {
        TO_LP_RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ecc_single_err(&self) -> ECC_SINGLE_ERR_R {
        ECC_SINGLE_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ecc_milti_err(&self) -> ECC_MILTI_ERR_R {
        ECC_MILTI_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn pkt_size_err(&self) -> PKT_SIZE_ERR_R {
        PKT_SIZE_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn eopt_err(&self) -> EOPT_ERR_R {
        EOPT_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn dpi_pld_wr_err(&self) -> DPI_PLD_WR_ERR_R {
        DPI_PLD_WR_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn gen_cmd_wr_err(&self) -> GEN_CMD_WR_ERR_R {
        GEN_CMD_WR_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn gen_pld_wr_err(&self) -> GEN_PLD_WR_ERR_R {
        GEN_PLD_WR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn gen_pld_send_err(&self) -> GEN_PLD_SEND_ERR_R {
        GEN_PLD_SEND_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn gen_pld_rd_err(&self) -> GEN_PLD_RD_ERR_R {
        GEN_PLD_RD_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn gen_pld_recev_err(&self) -> GEN_PLD_RECEV_ERR_R {
        GEN_PLD_RECEV_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn dpi_buff_pld_under(&self) -> DPI_BUFF_PLD_UNDER_R {
        DPI_BUFF_PLD_UNDER_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST1")
            .field("to_hs_tx", &self.to_hs_tx())
            .field("to_lp_rx", &self.to_lp_rx())
            .field("ecc_single_err", &self.ecc_single_err())
            .field("ecc_milti_err", &self.ecc_milti_err())
            .field("crc_err", &self.crc_err())
            .field("pkt_size_err", &self.pkt_size_err())
            .field("eopt_err", &self.eopt_err())
            .field("dpi_pld_wr_err", &self.dpi_pld_wr_err())
            .field("gen_cmd_wr_err", &self.gen_cmd_wr_err())
            .field("gen_pld_wr_err", &self.gen_pld_wr_err())
            .field("gen_pld_send_err", &self.gen_pld_send_err())
            .field("gen_pld_rd_err", &self.gen_pld_rd_err())
            .field("gen_pld_recev_err", &self.gen_pld_recev_err())
            .field("dpi_buff_pld_under", &self.dpi_buff_pld_under())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST1_SPEC;
impl crate::RegisterSpec for INT_ST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st1::R`](R) reader structure"]
impl crate::Readable for INT_ST1_SPEC {}
#[doc = "`reset()` method sets INT_ST1 to value 0"]
impl crate::Resettable for INT_ST1_SPEC {
    const RESET_VALUE: u32 = 0;
}
