#[doc = "Register `PERI_CLK_CTRL01` reader"]
pub type R = crate::R<PERI_CLK_CTRL01_SPEC>;
#[doc = "Register `PERI_CLK_CTRL01` writer"]
pub type W = crate::W<PERI_CLK_CTRL01_SPEC>;
#[doc = "Field `EMAC_RX_CLK_DIV_NUM` reader - Reserved"]
pub type EMAC_RX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `EMAC_RX_CLK_DIV_NUM` writer - Reserved"]
pub type EMAC_RX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMAC_TX_CLK_SRC_SEL` reader - Reserved"]
pub type EMAC_TX_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `EMAC_TX_CLK_SRC_SEL` writer - Reserved"]
pub type EMAC_TX_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_TX_CLK_EN` reader - Reserved"]
pub type EMAC_TX_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_TX_CLK_EN` writer - Reserved"]
pub type EMAC_TX_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_TX_CLK_DIV_NUM` reader - Reserved"]
pub type EMAC_TX_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `EMAC_TX_CLK_DIV_NUM` writer - Reserved"]
pub type EMAC_TX_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EMAC_PTP_REF_CLK_SRC_SEL` reader - Reserved"]
pub type EMAC_PTP_REF_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `EMAC_PTP_REF_CLK_SRC_SEL` writer - Reserved"]
pub type EMAC_PTP_REF_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_PTP_REF_CLK_EN` reader - Reserved"]
pub type EMAC_PTP_REF_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_PTP_REF_CLK_EN` writer - Reserved"]
pub type EMAC_PTP_REF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_UNUSED0_CLK_EN` reader - Reserved"]
pub type EMAC_UNUSED0_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_UNUSED0_CLK_EN` writer - Reserved"]
pub type EMAC_UNUSED0_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_UNUSED1_CLK_EN` reader - Reserved"]
pub type EMAC_UNUSED1_CLK_EN_R = crate::BitReader;
#[doc = "Field `EMAC_UNUSED1_CLK_EN` writer - Reserved"]
pub type EMAC_UNUSED1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HS_MODE` reader - Reserved"]
pub type SDIO_HS_MODE_R = crate::BitReader;
#[doc = "Field `SDIO_HS_MODE` writer - Reserved"]
pub type SDIO_HS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_SRC_SEL` reader - Reserved"]
pub type SDIO_LS_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_LS_CLK_SRC_SEL` writer - Reserved"]
pub type SDIO_LS_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_EN` reader - Reserved"]
pub type SDIO_LS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_CLK_EN` writer - Reserved"]
pub type SDIO_LS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_div_num(&self) -> EMAC_RX_CLK_DIV_NUM_R {
        EMAC_RX_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_src_sel(&self) -> EMAC_TX_CLK_SRC_SEL_R {
        EMAC_TX_CLK_SRC_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_en(&self) -> EMAC_TX_CLK_EN_R {
        EMAC_TX_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_div_num(&self) -> EMAC_TX_CLK_DIV_NUM_R {
        EMAC_TX_CLK_DIV_NUM_R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_src_sel(&self) -> EMAC_PTP_REF_CLK_SRC_SEL_R {
        EMAC_PTP_REF_CLK_SRC_SEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_en(&self) -> EMAC_PTP_REF_CLK_EN_R {
        EMAC_PTP_REF_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn emac_unused0_clk_en(&self) -> EMAC_UNUSED0_CLK_EN_R {
        EMAC_UNUSED0_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn emac_unused1_clk_en(&self) -> EMAC_UNUSED1_CLK_EN_R {
        EMAC_UNUSED1_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn sdio_hs_mode(&self) -> SDIO_HS_MODE_R {
        SDIO_HS_MODE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_src_sel(&self) -> SDIO_LS_CLK_SRC_SEL_R {
        SDIO_LS_CLK_SRC_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reserved"]
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
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn emac_rx_clk_div_num(&mut self) -> EMAC_RX_CLK_DIV_NUM_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_RX_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_src_sel(&mut self) -> EMAC_TX_CLK_SRC_SEL_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_TX_CLK_SRC_SEL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_en(&mut self) -> EMAC_TX_CLK_EN_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_TX_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bits 10:17 - Reserved"]
    #[inline(always)]
    pub fn emac_tx_clk_div_num(&mut self) -> EMAC_TX_CLK_DIV_NUM_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_TX_CLK_DIV_NUM_W::new(self, 10)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_src_sel(
        &mut self,
    ) -> EMAC_PTP_REF_CLK_SRC_SEL_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_PTP_REF_CLK_SRC_SEL_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn emac_ptp_ref_clk_en(&mut self) -> EMAC_PTP_REF_CLK_EN_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_PTP_REF_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn emac_unused0_clk_en(&mut self) -> EMAC_UNUSED0_CLK_EN_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_UNUSED0_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn emac_unused1_clk_en(&mut self) -> EMAC_UNUSED1_CLK_EN_W<'_, PERI_CLK_CTRL01_SPEC> {
        EMAC_UNUSED1_CLK_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn sdio_hs_mode(&mut self) -> SDIO_HS_MODE_W<'_, PERI_CLK_CTRL01_SPEC> {
        SDIO_HS_MODE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_src_sel(&mut self) -> SDIO_LS_CLK_SRC_SEL_W<'_, PERI_CLK_CTRL01_SPEC> {
        SDIO_LS_CLK_SRC_SEL_W::new(self, 23)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_en(&mut self) -> SDIO_LS_CLK_EN_W<'_, PERI_CLK_CTRL01_SPEC> {
        SDIO_LS_CLK_EN_W::new(self, 24)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL01_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL01_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl01::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL01_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl01::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL01_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL01 to value 0x0401"]
impl crate::Resettable for PERI_CLK_CTRL01_SPEC {
    const RESET_VALUE: u32 = 0x0401;
}
