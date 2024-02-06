#[doc = "Register `INT_FORCE1` reader"]
pub type R = crate::R<INT_FORCE1_SPEC>;
#[doc = "Register `INT_FORCE1` writer"]
pub type W = crate::W<INT_FORCE1_SPEC>;
#[doc = "Field `FORCE_TO_HS_TX` reader - NA"]
pub type FORCE_TO_HS_TX_R = crate::BitReader;
#[doc = "Field `FORCE_TO_HS_TX` writer - NA"]
pub type FORCE_TO_HS_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TO_LP_RX` reader - NA"]
pub type FORCE_TO_LP_RX_R = crate::BitReader;
#[doc = "Field `FORCE_TO_LP_RX` writer - NA"]
pub type FORCE_TO_LP_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ECC_SINGLE_ERR` reader - NA"]
pub type FORCE_ECC_SINGLE_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_ECC_SINGLE_ERR` writer - NA"]
pub type FORCE_ECC_SINGLE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ECC_MILTI_ERR` reader - NA"]
pub type FORCE_ECC_MILTI_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_ECC_MILTI_ERR` writer - NA"]
pub type FORCE_ECC_MILTI_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CRC_ERR` reader - NA"]
pub type FORCE_CRC_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_CRC_ERR` writer - NA"]
pub type FORCE_CRC_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_PKT_SIZE_ERR` reader - NA"]
pub type FORCE_PKT_SIZE_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_PKT_SIZE_ERR` writer - NA"]
pub type FORCE_PKT_SIZE_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_EOPT_ERR` reader - NA"]
pub type FORCE_EOPT_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_EOPT_ERR` writer - NA"]
pub type FORCE_EOPT_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DPI_PLD_WR_ERR` reader - NA"]
pub type FORCE_DPI_PLD_WR_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_DPI_PLD_WR_ERR` writer - NA"]
pub type FORCE_DPI_PLD_WR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_CMD_WR_ERR` reader - NA"]
pub type FORCE_GEN_CMD_WR_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_GEN_CMD_WR_ERR` writer - NA"]
pub type FORCE_GEN_CMD_WR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_WR_ERR` reader - NA"]
pub type FORCE_GEN_PLD_WR_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_WR_ERR` writer - NA"]
pub type FORCE_GEN_PLD_WR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_SEND_ERR` reader - NA"]
pub type FORCE_GEN_PLD_SEND_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_SEND_ERR` writer - NA"]
pub type FORCE_GEN_PLD_SEND_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_RD_ERR` reader - NA"]
pub type FORCE_GEN_PLD_RD_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_RD_ERR` writer - NA"]
pub type FORCE_GEN_PLD_RD_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_GEN_PLD_RECEV_ERR` reader - NA"]
pub type FORCE_GEN_PLD_RECEV_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_GEN_PLD_RECEV_ERR` writer - NA"]
pub type FORCE_GEN_PLD_RECEV_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DPI_BUFF_PLD_UNDER` reader - NA"]
pub type FORCE_DPI_BUFF_PLD_UNDER_R = crate::BitReader;
#[doc = "Field `FORCE_DPI_BUFF_PLD_UNDER` writer - NA"]
pub type FORCE_DPI_BUFF_PLD_UNDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn force_to_hs_tx(&self) -> FORCE_TO_HS_TX_R {
        FORCE_TO_HS_TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn force_to_lp_rx(&self) -> FORCE_TO_LP_RX_R {
        FORCE_TO_LP_RX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn force_ecc_single_err(&self) -> FORCE_ECC_SINGLE_ERR_R {
        FORCE_ECC_SINGLE_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn force_ecc_milti_err(&self) -> FORCE_ECC_MILTI_ERR_R {
        FORCE_ECC_MILTI_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn force_crc_err(&self) -> FORCE_CRC_ERR_R {
        FORCE_CRC_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn force_pkt_size_err(&self) -> FORCE_PKT_SIZE_ERR_R {
        FORCE_PKT_SIZE_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn force_eopt_err(&self) -> FORCE_EOPT_ERR_R {
        FORCE_EOPT_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn force_dpi_pld_wr_err(&self) -> FORCE_DPI_PLD_WR_ERR_R {
        FORCE_DPI_PLD_WR_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn force_gen_cmd_wr_err(&self) -> FORCE_GEN_CMD_WR_ERR_R {
        FORCE_GEN_CMD_WR_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_wr_err(&self) -> FORCE_GEN_PLD_WR_ERR_R {
        FORCE_GEN_PLD_WR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_send_err(&self) -> FORCE_GEN_PLD_SEND_ERR_R {
        FORCE_GEN_PLD_SEND_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_rd_err(&self) -> FORCE_GEN_PLD_RD_ERR_R {
        FORCE_GEN_PLD_RD_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    pub fn force_gen_pld_recev_err(&self) -> FORCE_GEN_PLD_RECEV_ERR_R {
        FORCE_GEN_PLD_RECEV_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn force_dpi_buff_pld_under(&self) -> FORCE_DPI_BUFF_PLD_UNDER_R {
        FORCE_DPI_BUFF_PLD_UNDER_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_FORCE1")
            .field(
                "force_to_hs_tx",
                &format_args!("{}", self.force_to_hs_tx().bit()),
            )
            .field(
                "force_to_lp_rx",
                &format_args!("{}", self.force_to_lp_rx().bit()),
            )
            .field(
                "force_ecc_single_err",
                &format_args!("{}", self.force_ecc_single_err().bit()),
            )
            .field(
                "force_ecc_milti_err",
                &format_args!("{}", self.force_ecc_milti_err().bit()),
            )
            .field(
                "force_crc_err",
                &format_args!("{}", self.force_crc_err().bit()),
            )
            .field(
                "force_pkt_size_err",
                &format_args!("{}", self.force_pkt_size_err().bit()),
            )
            .field(
                "force_eopt_err",
                &format_args!("{}", self.force_eopt_err().bit()),
            )
            .field(
                "force_dpi_pld_wr_err",
                &format_args!("{}", self.force_dpi_pld_wr_err().bit()),
            )
            .field(
                "force_gen_cmd_wr_err",
                &format_args!("{}", self.force_gen_cmd_wr_err().bit()),
            )
            .field(
                "force_gen_pld_wr_err",
                &format_args!("{}", self.force_gen_pld_wr_err().bit()),
            )
            .field(
                "force_gen_pld_send_err",
                &format_args!("{}", self.force_gen_pld_send_err().bit()),
            )
            .field(
                "force_gen_pld_rd_err",
                &format_args!("{}", self.force_gen_pld_rd_err().bit()),
            )
            .field(
                "force_gen_pld_recev_err",
                &format_args!("{}", self.force_gen_pld_recev_err().bit()),
            )
            .field(
                "force_dpi_buff_pld_under",
                &format_args!("{}", self.force_dpi_buff_pld_under().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_FORCE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_to_hs_tx(&mut self) -> FORCE_TO_HS_TX_W<INT_FORCE1_SPEC> {
        FORCE_TO_HS_TX_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_to_lp_rx(&mut self) -> FORCE_TO_LP_RX_W<INT_FORCE1_SPEC> {
        FORCE_TO_LP_RX_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_ecc_single_err(&mut self) -> FORCE_ECC_SINGLE_ERR_W<INT_FORCE1_SPEC> {
        FORCE_ECC_SINGLE_ERR_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_ecc_milti_err(&mut self) -> FORCE_ECC_MILTI_ERR_W<INT_FORCE1_SPEC> {
        FORCE_ECC_MILTI_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_crc_err(&mut self) -> FORCE_CRC_ERR_W<INT_FORCE1_SPEC> {
        FORCE_CRC_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_pkt_size_err(&mut self) -> FORCE_PKT_SIZE_ERR_W<INT_FORCE1_SPEC> {
        FORCE_PKT_SIZE_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_eopt_err(&mut self) -> FORCE_EOPT_ERR_W<INT_FORCE1_SPEC> {
        FORCE_EOPT_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_dpi_pld_wr_err(&mut self) -> FORCE_DPI_PLD_WR_ERR_W<INT_FORCE1_SPEC> {
        FORCE_DPI_PLD_WR_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_gen_cmd_wr_err(&mut self) -> FORCE_GEN_CMD_WR_ERR_W<INT_FORCE1_SPEC> {
        FORCE_GEN_CMD_WR_ERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_gen_pld_wr_err(&mut self) -> FORCE_GEN_PLD_WR_ERR_W<INT_FORCE1_SPEC> {
        FORCE_GEN_PLD_WR_ERR_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_gen_pld_send_err(&mut self) -> FORCE_GEN_PLD_SEND_ERR_W<INT_FORCE1_SPEC> {
        FORCE_GEN_PLD_SEND_ERR_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_gen_pld_rd_err(&mut self) -> FORCE_GEN_PLD_RD_ERR_W<INT_FORCE1_SPEC> {
        FORCE_GEN_PLD_RD_ERR_W::new(self, 11)
    }
    #[doc = "Bit 12 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_gen_pld_recev_err(&mut self) -> FORCE_GEN_PLD_RECEV_ERR_W<INT_FORCE1_SPEC> {
        FORCE_GEN_PLD_RECEV_ERR_W::new(self, 12)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn force_dpi_buff_pld_under(&mut self) -> FORCE_DPI_BUFF_PLD_UNDER_W<INT_FORCE1_SPEC> {
        FORCE_DPI_BUFF_PLD_UNDER_W::new(self, 19)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_force1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_FORCE1_SPEC;
impl crate::RegisterSpec for INT_FORCE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_force1::R`](R) reader structure"]
impl crate::Readable for INT_FORCE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_force1::W`](W) writer structure"]
impl crate::Writable for INT_FORCE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FORCE1 to value 0"]
impl crate::Resettable for INT_FORCE1_SPEC {
    const RESET_VALUE: u32 = 0;
}
