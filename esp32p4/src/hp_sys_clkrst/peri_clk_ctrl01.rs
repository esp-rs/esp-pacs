///Register `PERI_CLK_CTRL01` reader
pub type R = crate::R<PERI_CLK_CTRL01_SPEC>;
///Register `PERI_CLK_CTRL01` writer
pub type W = crate::W<PERI_CLK_CTRL01_SPEC>;
///Field `EMAC_RX_CLK_DIV_NUM` reader - Reserved
pub type EMAC_RX_CLK_DIV_NUM_R = crate::FieldReader;
///Field `EMAC_RX_CLK_DIV_NUM` writer - Reserved
pub type EMAC_RX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EMAC_TX_CLK_SRC_SEL` reader - Reserved
pub type EMAC_TX_CLK_SRC_SEL_R = crate::BitReader;
///Field `EMAC_TX_CLK_SRC_SEL` writer - Reserved
pub type EMAC_TX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_TX_CLK_EN` reader - Reserved
pub type EMAC_TX_CLK_EN_R = crate::BitReader;
///Field `EMAC_TX_CLK_EN` writer - Reserved
pub type EMAC_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_TX_CLK_DIV_NUM` reader - Reserved
pub type EMAC_TX_CLK_DIV_NUM_R = crate::FieldReader;
///Field `EMAC_TX_CLK_DIV_NUM` writer - Reserved
pub type EMAC_TX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EMAC_PTP_REF_CLK_SRC_SEL` reader - Reserved
pub type EMAC_PTP_REF_CLK_SRC_SEL_R = crate::BitReader;
///Field `EMAC_PTP_REF_CLK_SRC_SEL` writer - Reserved
pub type EMAC_PTP_REF_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_PTP_REF_CLK_EN` reader - Reserved
pub type EMAC_PTP_REF_CLK_EN_R = crate::BitReader;
///Field `EMAC_PTP_REF_CLK_EN` writer - Reserved
pub type EMAC_PTP_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_UNUSED0_CLK_EN` reader - Reserved
pub type EMAC_UNUSED0_CLK_EN_R = crate::BitReader;
///Field `EMAC_UNUSED0_CLK_EN` writer - Reserved
pub type EMAC_UNUSED0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EMAC_UNUSED1_CLK_EN` reader - Reserved
pub type EMAC_UNUSED1_CLK_EN_R = crate::BitReader;
///Field `EMAC_UNUSED1_CLK_EN` writer - Reserved
pub type EMAC_UNUSED1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_HS_MODE` reader - Reserved
pub type SDIO_HS_MODE_R = crate::BitReader;
///Field `SDIO_HS_MODE` writer - Reserved
pub type SDIO_HS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_LS_CLK_SRC_SEL` reader - Reserved
pub type SDIO_LS_CLK_SRC_SEL_R = crate::BitReader;
///Field `SDIO_LS_CLK_SRC_SEL` writer - Reserved
pub type SDIO_LS_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_LS_CLK_EN` reader - Reserved
pub type SDIO_LS_CLK_EN_R = crate::BitReader;
///Field `SDIO_LS_CLK_EN` writer - Reserved
pub type SDIO_LS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    pub fn emac_rx_clk_div_num(&self) -> EMAC_RX_CLK_DIV_NUM_R {
        EMAC_RX_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Reserved
    #[inline(always)]
    pub fn emac_tx_clk_src_sel(&self) -> EMAC_TX_CLK_SRC_SEL_R {
        EMAC_TX_CLK_SRC_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Reserved
    #[inline(always)]
    pub fn emac_tx_clk_en(&self) -> EMAC_TX_CLK_EN_R {
        EMAC_TX_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:17 - Reserved
    #[inline(always)]
    pub fn emac_tx_clk_div_num(&self) -> EMAC_TX_CLK_DIV_NUM_R {
        EMAC_TX_CLK_DIV_NUM_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    ///Bit 18 - Reserved
    #[inline(always)]
    pub fn emac_ptp_ref_clk_src_sel(&self) -> EMAC_PTP_REF_CLK_SRC_SEL_R {
        EMAC_PTP_REF_CLK_SRC_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Reserved
    #[inline(always)]
    pub fn emac_ptp_ref_clk_en(&self) -> EMAC_PTP_REF_CLK_EN_R {
        EMAC_PTP_REF_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Reserved
    #[inline(always)]
    pub fn emac_unused0_clk_en(&self) -> EMAC_UNUSED0_CLK_EN_R {
        EMAC_UNUSED0_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Reserved
    #[inline(always)]
    pub fn emac_unused1_clk_en(&self) -> EMAC_UNUSED1_CLK_EN_R {
        EMAC_UNUSED1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Reserved
    #[inline(always)]
    pub fn sdio_hs_mode(&self) -> SDIO_HS_MODE_R {
        SDIO_HS_MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Reserved
    #[inline(always)]
    pub fn sdio_ls_clk_src_sel(&self) -> SDIO_LS_CLK_SRC_SEL_R {
        SDIO_LS_CLK_SRC_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Reserved
    #[inline(always)]
    pub fn sdio_ls_clk_en(&self) -> SDIO_LS_CLK_EN_R {
        SDIO_LS_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL01")
            .field("emac_rx_clk_div_num", &self.emac_rx_clk_div_num())
            .field("emac_tx_clk_src_sel", &self.emac_tx_clk_src_sel())
            .field("emac_tx_clk_en", &self.emac_tx_clk_en())
            .field("emac_tx_clk_div_num", &self.emac_tx_clk_div_num())
            .field("emac_ptp_ref_clk_src_sel", &self.emac_ptp_ref_clk_src_sel())
            .field("emac_ptp_ref_clk_en", &self.emac_ptp_ref_clk_en())
            .field("emac_unused0_clk_en", &self.emac_unused0_clk_en())
            .field("emac_unused1_clk_en", &self.emac_unused1_clk_en())
            .field("sdio_hs_mode", &self.sdio_hs_mode())
            .field("sdio_ls_clk_src_sel", &self.sdio_ls_clk_src_sel())
            .field("sdio_ls_clk_en", &self.sdio_ls_clk_en())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_rx_clk_div_num(&mut self) -> EMAC_RX_CLK_DIV_NUM_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_RX_CLK_DIV_NUM_W::new(self, 0)
    }
    ///Bit 8 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_tx_clk_src_sel(&mut self) -> EMAC_TX_CLK_SRC_SEL_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_TX_CLK_SRC_SEL_W::new(self, 8)
    }
    ///Bit 9 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_tx_clk_en(&mut self) -> EMAC_TX_CLK_EN_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_TX_CLK_EN_W::new(self, 9)
    }
    ///Bits 10:17 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_tx_clk_div_num(&mut self) -> EMAC_TX_CLK_DIV_NUM_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_TX_CLK_DIV_NUM_W::new(self, 10)
    }
    ///Bit 18 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_ptp_ref_clk_src_sel(&mut self) -> EMAC_PTP_REF_CLK_SRC_SEL_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_PTP_REF_CLK_SRC_SEL_W::new(self, 18)
    }
    ///Bit 19 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_ptp_ref_clk_en(&mut self) -> EMAC_PTP_REF_CLK_EN_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_PTP_REF_CLK_EN_W::new(self, 19)
    }
    ///Bit 20 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_unused0_clk_en(&mut self) -> EMAC_UNUSED0_CLK_EN_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_UNUSED0_CLK_EN_W::new(self, 20)
    }
    ///Bit 21 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn emac_unused1_clk_en(&mut self) -> EMAC_UNUSED1_CLK_EN_W<PERI_CLK_CTRL01_SPEC> {
        EMAC_UNUSED1_CLK_EN_W::new(self, 21)
    }
    ///Bit 22 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn sdio_hs_mode(&mut self) -> SDIO_HS_MODE_W<PERI_CLK_CTRL01_SPEC> {
        SDIO_HS_MODE_W::new(self, 22)
    }
    ///Bit 23 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn sdio_ls_clk_src_sel(&mut self) -> SDIO_LS_CLK_SRC_SEL_W<PERI_CLK_CTRL01_SPEC> {
        SDIO_LS_CLK_SRC_SEL_W::new(self, 23)
    }
    ///Bit 24 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn sdio_ls_clk_en(&mut self) -> SDIO_LS_CLK_EN_W<PERI_CLK_CTRL01_SPEC> {
        SDIO_LS_CLK_EN_W::new(self, 24)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERI_CLK_CTRL01_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL01_SPEC {
    type Ux = u32;
}
///`read()` method returns [`peri_clk_ctrl01::R`](R) reader structure
impl crate::Readable for PERI_CLK_CTRL01_SPEC {}
///`write(|w| ..)` method takes [`peri_clk_ctrl01::W`](W) writer structure
impl crate::Writable for PERI_CLK_CTRL01_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERI_CLK_CTRL01 to value 0x0401
impl crate::Resettable for PERI_CLK_CTRL01_SPEC {
    const RESET_VALUE: u32 = 0x0401;
}
