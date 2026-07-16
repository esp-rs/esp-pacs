#[doc = "Register `DDR` reader"]
pub type R = crate::R<DDR_SPEC>;
#[doc = "Register `DDR` writer"]
pub type W = crate::W<DDR_SPEC>;
#[doc = "Field `SPI_FMEM_S_DDR_EN` reader - "]
pub type SPI_FMEM_S_DDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_DDR_EN` writer - "]
pub type SPI_FMEM_S_DDR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_VAR_DUMMY` reader - "]
pub type SPI_FMEM_S_VAR_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_VAR_DUMMY` writer - "]
pub type SPI_FMEM_S_VAR_DUMMY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_DDR_RDAT_SWP` reader - "]
pub type SPI_FMEM_S_DDR_RDAT_SWP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_DDR_RDAT_SWP` writer - "]
pub type SPI_FMEM_S_DDR_RDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_DDR_WDAT_SWP` reader - "]
pub type SPI_FMEM_S_DDR_WDAT_SWP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_DDR_WDAT_SWP` writer - "]
pub type SPI_FMEM_S_DDR_WDAT_SWP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_DDR_CMD_DIS` reader - "]
pub type SPI_FMEM_S_DDR_CMD_DIS_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_DDR_CMD_DIS` writer - "]
pub type SPI_FMEM_S_DDR_CMD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_OUTMINBYTELEN` reader - "]
pub type SPI_FMEM_S_OUTMINBYTELEN_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_S_OUTMINBYTELEN` writer - "]
pub type SPI_FMEM_S_OUTMINBYTELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SPI_FMEM_S_TX_DDR_MSK_EN` reader - "]
pub type SPI_FMEM_S_TX_DDR_MSK_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_TX_DDR_MSK_EN` writer - "]
pub type SPI_FMEM_S_TX_DDR_MSK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_RX_DDR_MSK_EN` reader - "]
pub type SPI_FMEM_S_RX_DDR_MSK_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_RX_DDR_MSK_EN` writer - "]
pub type SPI_FMEM_S_RX_DDR_MSK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_USR_DDR_DQS_THD` reader - "]
pub type SPI_FMEM_S_USR_DDR_DQS_THD_R = crate::FieldReader;
#[doc = "Field `SPI_FMEM_S_USR_DDR_DQS_THD` writer - "]
pub type SPI_FMEM_S_USR_DDR_DQS_THD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SPI_FMEM_S_DDR_DQS_LOOP` reader - "]
pub type SPI_FMEM_S_DDR_DQS_LOOP_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_DDR_DQS_LOOP` writer - "]
pub type SPI_FMEM_S_DDR_DQS_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_CLK_DIFF_EN` reader - "]
pub type SPI_FMEM_S_CLK_DIFF_EN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_CLK_DIFF_EN` writer - "]
pub type SPI_FMEM_S_CLK_DIFF_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_DQS_CA_IN` reader - "]
pub type SPI_FMEM_S_DQS_CA_IN_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_DQS_CA_IN` writer - "]
pub type SPI_FMEM_S_DQS_CA_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_HYPERBUS_DUMMY_2X` reader - "]
pub type SPI_FMEM_S_HYPERBUS_DUMMY_2X_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_HYPERBUS_DUMMY_2X` writer - "]
pub type SPI_FMEM_S_HYPERBUS_DUMMY_2X_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_CLK_DIFF_INV` reader - "]
pub type SPI_FMEM_S_CLK_DIFF_INV_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_CLK_DIFF_INV` writer - "]
pub type SPI_FMEM_S_CLK_DIFF_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_OCTA_RAM_ADDR` reader - "]
pub type SPI_FMEM_S_OCTA_RAM_ADDR_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_OCTA_RAM_ADDR` writer - "]
pub type SPI_FMEM_S_OCTA_RAM_ADDR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_FMEM_S_HYPERBUS_CA` reader - "]
pub type SPI_FMEM_S_HYPERBUS_CA_R = crate::BitReader;
#[doc = "Field `SPI_FMEM_S_HYPERBUS_CA` writer - "]
pub type SPI_FMEM_S_HYPERBUS_CA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_en(&self) -> SPI_FMEM_S_DDR_EN_R {
        SPI_FMEM_S_DDR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_fmem_s_var_dummy(&self) -> SPI_FMEM_S_VAR_DUMMY_R {
        SPI_FMEM_S_VAR_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_rdat_swp(&self) -> SPI_FMEM_S_DDR_RDAT_SWP_R {
        SPI_FMEM_S_DDR_RDAT_SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_wdat_swp(&self) -> SPI_FMEM_S_DDR_WDAT_SWP_R {
        SPI_FMEM_S_DDR_WDAT_SWP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_cmd_dis(&self) -> SPI_FMEM_S_DDR_CMD_DIS_R {
        SPI_FMEM_S_DDR_CMD_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11"]
    #[inline(always)]
    pub fn spi_fmem_s_outminbytelen(&self) -> SPI_FMEM_S_OUTMINBYTELEN_R {
        SPI_FMEM_S_OUTMINBYTELEN_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_fmem_s_tx_ddr_msk_en(&self) -> SPI_FMEM_S_TX_DDR_MSK_EN_R {
        SPI_FMEM_S_TX_DDR_MSK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_fmem_s_rx_ddr_msk_en(&self) -> SPI_FMEM_S_RX_DDR_MSK_EN_R {
        SPI_FMEM_S_RX_DDR_MSK_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:20"]
    #[inline(always)]
    pub fn spi_fmem_s_usr_ddr_dqs_thd(&self) -> SPI_FMEM_S_USR_DDR_DQS_THD_R {
        SPI_FMEM_S_USR_DDR_DQS_THD_R::new(((self.bits >> 14) & 0x7f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_dqs_loop(&self) -> SPI_FMEM_S_DDR_DQS_LOOP_R {
        SPI_FMEM_S_DDR_DQS_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_fmem_s_clk_diff_en(&self) -> SPI_FMEM_S_CLK_DIFF_EN_R {
        SPI_FMEM_S_CLK_DIFF_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_fmem_s_dqs_ca_in(&self) -> SPI_FMEM_S_DQS_CA_IN_R {
        SPI_FMEM_S_DQS_CA_IN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_fmem_s_hyperbus_dummy_2x(&self) -> SPI_FMEM_S_HYPERBUS_DUMMY_2X_R {
        SPI_FMEM_S_HYPERBUS_DUMMY_2X_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_fmem_s_clk_diff_inv(&self) -> SPI_FMEM_S_CLK_DIFF_INV_R {
        SPI_FMEM_S_CLK_DIFF_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_fmem_s_octa_ram_addr(&self) -> SPI_FMEM_S_OCTA_RAM_ADDR_R {
        SPI_FMEM_S_OCTA_RAM_ADDR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_fmem_s_hyperbus_ca(&self) -> SPI_FMEM_S_HYPERBUS_CA_R {
        SPI_FMEM_S_HYPERBUS_CA_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDR")
            .field("spi_fmem_s_ddr_en", &self.spi_fmem_s_ddr_en())
            .field("spi_fmem_s_var_dummy", &self.spi_fmem_s_var_dummy())
            .field("spi_fmem_s_ddr_rdat_swp", &self.spi_fmem_s_ddr_rdat_swp())
            .field("spi_fmem_s_ddr_wdat_swp", &self.spi_fmem_s_ddr_wdat_swp())
            .field("spi_fmem_s_ddr_cmd_dis", &self.spi_fmem_s_ddr_cmd_dis())
            .field("spi_fmem_s_outminbytelen", &self.spi_fmem_s_outminbytelen())
            .field("spi_fmem_s_tx_ddr_msk_en", &self.spi_fmem_s_tx_ddr_msk_en())
            .field("spi_fmem_s_rx_ddr_msk_en", &self.spi_fmem_s_rx_ddr_msk_en())
            .field(
                "spi_fmem_s_usr_ddr_dqs_thd",
                &self.spi_fmem_s_usr_ddr_dqs_thd(),
            )
            .field("spi_fmem_s_ddr_dqs_loop", &self.spi_fmem_s_ddr_dqs_loop())
            .field("spi_fmem_s_clk_diff_en", &self.spi_fmem_s_clk_diff_en())
            .field("spi_fmem_s_dqs_ca_in", &self.spi_fmem_s_dqs_ca_in())
            .field(
                "spi_fmem_s_hyperbus_dummy_2x",
                &self.spi_fmem_s_hyperbus_dummy_2x(),
            )
            .field("spi_fmem_s_clk_diff_inv", &self.spi_fmem_s_clk_diff_inv())
            .field("spi_fmem_s_octa_ram_addr", &self.spi_fmem_s_octa_ram_addr())
            .field("spi_fmem_s_hyperbus_ca", &self.spi_fmem_s_hyperbus_ca())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_en(&mut self) -> SPI_FMEM_S_DDR_EN_W<'_, DDR_SPEC> {
        SPI_FMEM_S_DDR_EN_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spi_fmem_s_var_dummy(&mut self) -> SPI_FMEM_S_VAR_DUMMY_W<'_, DDR_SPEC> {
        SPI_FMEM_S_VAR_DUMMY_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_rdat_swp(&mut self) -> SPI_FMEM_S_DDR_RDAT_SWP_W<'_, DDR_SPEC> {
        SPI_FMEM_S_DDR_RDAT_SWP_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_wdat_swp(&mut self) -> SPI_FMEM_S_DDR_WDAT_SWP_W<'_, DDR_SPEC> {
        SPI_FMEM_S_DDR_WDAT_SWP_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_cmd_dis(&mut self) -> SPI_FMEM_S_DDR_CMD_DIS_W<'_, DDR_SPEC> {
        SPI_FMEM_S_DDR_CMD_DIS_W::new(self, 4)
    }
    #[doc = "Bits 5:11"]
    #[inline(always)]
    pub fn spi_fmem_s_outminbytelen(&mut self) -> SPI_FMEM_S_OUTMINBYTELEN_W<'_, DDR_SPEC> {
        SPI_FMEM_S_OUTMINBYTELEN_W::new(self, 5)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn spi_fmem_s_tx_ddr_msk_en(&mut self) -> SPI_FMEM_S_TX_DDR_MSK_EN_W<'_, DDR_SPEC> {
        SPI_FMEM_S_TX_DDR_MSK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn spi_fmem_s_rx_ddr_msk_en(&mut self) -> SPI_FMEM_S_RX_DDR_MSK_EN_W<'_, DDR_SPEC> {
        SPI_FMEM_S_RX_DDR_MSK_EN_W::new(self, 13)
    }
    #[doc = "Bits 14:20"]
    #[inline(always)]
    pub fn spi_fmem_s_usr_ddr_dqs_thd(&mut self) -> SPI_FMEM_S_USR_DDR_DQS_THD_W<'_, DDR_SPEC> {
        SPI_FMEM_S_USR_DDR_DQS_THD_W::new(self, 14)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn spi_fmem_s_ddr_dqs_loop(&mut self) -> SPI_FMEM_S_DDR_DQS_LOOP_W<'_, DDR_SPEC> {
        SPI_FMEM_S_DDR_DQS_LOOP_W::new(self, 21)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn spi_fmem_s_clk_diff_en(&mut self) -> SPI_FMEM_S_CLK_DIFF_EN_W<'_, DDR_SPEC> {
        SPI_FMEM_S_CLK_DIFF_EN_W::new(self, 24)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn spi_fmem_s_dqs_ca_in(&mut self) -> SPI_FMEM_S_DQS_CA_IN_W<'_, DDR_SPEC> {
        SPI_FMEM_S_DQS_CA_IN_W::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn spi_fmem_s_hyperbus_dummy_2x(&mut self) -> SPI_FMEM_S_HYPERBUS_DUMMY_2X_W<'_, DDR_SPEC> {
        SPI_FMEM_S_HYPERBUS_DUMMY_2X_W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn spi_fmem_s_clk_diff_inv(&mut self) -> SPI_FMEM_S_CLK_DIFF_INV_W<'_, DDR_SPEC> {
        SPI_FMEM_S_CLK_DIFF_INV_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn spi_fmem_s_octa_ram_addr(&mut self) -> SPI_FMEM_S_OCTA_RAM_ADDR_W<'_, DDR_SPEC> {
        SPI_FMEM_S_OCTA_RAM_ADDR_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn spi_fmem_s_hyperbus_ca(&mut self) -> SPI_FMEM_S_HYPERBUS_CA_W<'_, DDR_SPEC> {
        SPI_FMEM_S_HYPERBUS_CA_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDR_SPEC;
impl crate::RegisterSpec for DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr::R`](R) reader structure"]
impl crate::Readable for DDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ddr::W`](W) writer structure"]
impl crate::Writable for DDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DDR to value 0x3020"]
impl crate::Resettable for DDR_SPEC {
    const RESET_VALUE: u32 = 0x3020;
}
