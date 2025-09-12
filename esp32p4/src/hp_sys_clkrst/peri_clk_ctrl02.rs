#[doc = "Register `PERI_CLK_CTRL02` reader"]
pub type R = crate::R<PERI_CLK_CTRL02_SPEC>;
#[doc = "Register `PERI_CLK_CTRL02` writer"]
pub type W = crate::W<PERI_CLK_CTRL02_SPEC>;
#[doc = "Field `SDIO_LS_CLK_DIV_NUM` reader - Reserved"]
pub type SDIO_LS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_DIV_NUM` writer - Reserved"]
pub type SDIO_LS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDIO_LS_CLK_EDGE_CFG_UPDATE` writer - Reserved"]
pub type SDIO_LS_CLK_EDGE_CFG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_EDGE_L` reader - Reserved"]
pub type SDIO_LS_CLK_EDGE_L_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_L` writer - Reserved"]
pub type SDIO_LS_CLK_EDGE_L_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_CLK_EDGE_H` reader - Reserved"]
pub type SDIO_LS_CLK_EDGE_H_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_H` writer - Reserved"]
pub type SDIO_LS_CLK_EDGE_H_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_CLK_EDGE_N` reader - Reserved"]
pub type SDIO_LS_CLK_EDGE_N_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_N` writer - Reserved"]
pub type SDIO_LS_CLK_EDGE_N_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_SLF_CLK_EDGE_SEL` reader - Reserved"]
pub type SDIO_LS_SLF_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_SLF_CLK_EDGE_SEL` writer - Reserved"]
pub type SDIO_LS_SLF_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO_LS_DRV_CLK_EDGE_SEL` reader - Reserved"]
pub type SDIO_LS_DRV_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_DRV_CLK_EDGE_SEL` writer - Reserved"]
pub type SDIO_LS_DRV_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO_LS_SAM_CLK_EDGE_SEL` reader - Reserved"]
pub type SDIO_LS_SAM_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_SAM_CLK_EDGE_SEL` writer - Reserved"]
pub type SDIO_LS_SAM_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDIO_LS_SLF_CLK_EN` reader - Reserved"]
pub type SDIO_LS_SLF_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_SLF_CLK_EN` writer - Reserved"]
pub type SDIO_LS_SLF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_DRV_CLK_EN` reader - Reserved"]
pub type SDIO_LS_DRV_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_DRV_CLK_EN` writer - Reserved"]
pub type SDIO_LS_DRV_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_SAM_CLK_EN` reader - Reserved"]
pub type SDIO_LS_SAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_SAM_CLK_EN` writer - Reserved"]
pub type SDIO_LS_SAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIPI_DSI_DPHY_CLK_SRC_SEL` reader - Reserved"]
pub type MIPI_DSI_DPHY_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `MIPI_DSI_DPHY_CLK_SRC_SEL` writer - Reserved"]
pub type MIPI_DSI_DPHY_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_div_num(&self) -> SDIO_LS_CLK_DIV_NUM_R {
        SDIO_LS_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:12 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_l(&self) -> SDIO_LS_CLK_EDGE_L_R {
        SDIO_LS_CLK_EDGE_L_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_h(&self) -> SDIO_LS_CLK_EDGE_H_R {
        SDIO_LS_CLK_EDGE_H_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_n(&self) -> SDIO_LS_CLK_EDGE_N_R {
        SDIO_LS_CLK_EDGE_N_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_edge_sel(&self) -> SDIO_LS_SLF_CLK_EDGE_SEL_R {
        SDIO_LS_SLF_CLK_EDGE_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_edge_sel(&self) -> SDIO_LS_DRV_CLK_EDGE_SEL_R {
        SDIO_LS_DRV_CLK_EDGE_SEL_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_edge_sel(&self) -> SDIO_LS_SAM_CLK_EDGE_SEL_R {
        SDIO_LS_SAM_CLK_EDGE_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_en(&self) -> SDIO_LS_SLF_CLK_EN_R {
        SDIO_LS_SLF_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_en(&self) -> SDIO_LS_DRV_CLK_EN_R {
        SDIO_LS_DRV_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_en(&self) -> SDIO_LS_SAM_CLK_EN_R {
        SDIO_LS_SAM_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_clk_src_sel(&self) -> MIPI_DSI_DPHY_CLK_SRC_SEL_R {
        MIPI_DSI_DPHY_CLK_SRC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL02")
            .field("sdio_ls_clk_div_num", &self.sdio_ls_clk_div_num())
            .field("sdio_ls_clk_edge_l", &self.sdio_ls_clk_edge_l())
            .field("sdio_ls_clk_edge_h", &self.sdio_ls_clk_edge_h())
            .field("sdio_ls_clk_edge_n", &self.sdio_ls_clk_edge_n())
            .field("sdio_ls_slf_clk_edge_sel", &self.sdio_ls_slf_clk_edge_sel())
            .field("sdio_ls_drv_clk_edge_sel", &self.sdio_ls_drv_clk_edge_sel())
            .field("sdio_ls_sam_clk_edge_sel", &self.sdio_ls_sam_clk_edge_sel())
            .field("sdio_ls_slf_clk_en", &self.sdio_ls_slf_clk_en())
            .field("sdio_ls_drv_clk_en", &self.sdio_ls_drv_clk_en())
            .field("sdio_ls_sam_clk_en", &self.sdio_ls_sam_clk_en())
            .field(
                "mipi_dsi_dphy_clk_src_sel",
                &self.mipi_dsi_dphy_clk_src_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_div_num(&mut self) -> SDIO_LS_CLK_DIV_NUM_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_cfg_update(
        &mut self,
    ) -> SDIO_LS_CLK_EDGE_CFG_UPDATE_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_CLK_EDGE_CFG_UPDATE_W::new(self, 8)
    }
    #[doc = "Bits 9:12 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_l(&mut self) -> SDIO_LS_CLK_EDGE_L_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_CLK_EDGE_L_W::new(self, 9)
    }
    #[doc = "Bits 13:16 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_h(&mut self) -> SDIO_LS_CLK_EDGE_H_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_CLK_EDGE_H_W::new(self, 13)
    }
    #[doc = "Bits 17:20 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_n(&mut self) -> SDIO_LS_CLK_EDGE_N_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_CLK_EDGE_N_W::new(self, 17)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_edge_sel(
        &mut self,
    ) -> SDIO_LS_SLF_CLK_EDGE_SEL_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_SLF_CLK_EDGE_SEL_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_edge_sel(
        &mut self,
    ) -> SDIO_LS_DRV_CLK_EDGE_SEL_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_DRV_CLK_EDGE_SEL_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_edge_sel(
        &mut self,
    ) -> SDIO_LS_SAM_CLK_EDGE_SEL_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_SAM_CLK_EDGE_SEL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_en(&mut self) -> SDIO_LS_SLF_CLK_EN_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_SLF_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_en(&mut self) -> SDIO_LS_DRV_CLK_EN_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_DRV_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_en(&mut self) -> SDIO_LS_SAM_CLK_EN_W<'_, PERI_CLK_CTRL02_SPEC> {
        SDIO_LS_SAM_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn mipi_dsi_dphy_clk_src_sel(
        &mut self,
    ) -> MIPI_DSI_DPHY_CLK_SRC_SEL_W<'_, PERI_CLK_CTRL02_SPEC> {
        MIPI_DSI_DPHY_CLK_SRC_SEL_W::new(self, 30)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl02::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl02::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL02_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL02_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl02::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL02_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl02::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL02_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL02 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL02_SPEC {}
