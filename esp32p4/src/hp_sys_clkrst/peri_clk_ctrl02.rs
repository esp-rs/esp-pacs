#[doc = "Register `PERI_CLK_CTRL02` reader"]
pub type R = crate::R<PERI_CLK_CTRL02_SPEC>;
#[doc = "Register `PERI_CLK_CTRL02` writer"]
pub type W = crate::W<PERI_CLK_CTRL02_SPEC>;
#[doc = "Field `REG_SDIO_LS_CLK_DIV_NUM` reader - Reserved"]
pub type REG_SDIO_LS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REG_SDIO_LS_CLK_DIV_NUM` writer - Reserved"]
pub type REG_SDIO_LS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_SDIO_LS_CLK_EDGE_CFG_UPDATE` writer - Reserved"]
pub type REG_SDIO_LS_CLK_EDGE_CFG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDIO_LS_CLK_EDGE_L` reader - Reserved"]
pub type REG_SDIO_LS_CLK_EDGE_L_R = crate::FieldReader;
#[doc = "Field `REG_SDIO_LS_CLK_EDGE_L` writer - Reserved"]
pub type REG_SDIO_LS_CLK_EDGE_L_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_SDIO_LS_CLK_EDGE_H` reader - Reserved"]
pub type REG_SDIO_LS_CLK_EDGE_H_R = crate::FieldReader;
#[doc = "Field `REG_SDIO_LS_CLK_EDGE_H` writer - Reserved"]
pub type REG_SDIO_LS_CLK_EDGE_H_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_SDIO_LS_CLK_EDGE_N` reader - Reserved"]
pub type REG_SDIO_LS_CLK_EDGE_N_R = crate::FieldReader;
#[doc = "Field `REG_SDIO_LS_CLK_EDGE_N` writer - Reserved"]
pub type REG_SDIO_LS_CLK_EDGE_N_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_SDIO_LS_SLF_CLK_EDGE_SEL` reader - Reserved"]
pub type REG_SDIO_LS_SLF_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `REG_SDIO_LS_SLF_CLK_EDGE_SEL` writer - Reserved"]
pub type REG_SDIO_LS_SLF_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SDIO_LS_DRV_CLK_EDGE_SEL` reader - Reserved"]
pub type REG_SDIO_LS_DRV_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `REG_SDIO_LS_DRV_CLK_EDGE_SEL` writer - Reserved"]
pub type REG_SDIO_LS_DRV_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SDIO_LS_SAM_CLK_EDGE_SEL` reader - Reserved"]
pub type REG_SDIO_LS_SAM_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `REG_SDIO_LS_SAM_CLK_EDGE_SEL` writer - Reserved"]
pub type REG_SDIO_LS_SAM_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_SDIO_LS_SLF_CLK_EN` reader - Reserved"]
pub type REG_SDIO_LS_SLF_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_SDIO_LS_SLF_CLK_EN` writer - Reserved"]
pub type REG_SDIO_LS_SLF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDIO_LS_DRV_CLK_EN` reader - Reserved"]
pub type REG_SDIO_LS_DRV_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_SDIO_LS_DRV_CLK_EN` writer - Reserved"]
pub type REG_SDIO_LS_DRV_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_SDIO_LS_SAM_CLK_EN` reader - Reserved"]
pub type REG_SDIO_LS_SAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_SDIO_LS_SAM_CLK_EN` writer - Reserved"]
pub type REG_SDIO_LS_SAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_MIPI_DSI_DPHY_CLK_SRC_SEL` reader - Reserved"]
pub type REG_MIPI_DSI_DPHY_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `REG_MIPI_DSI_DPHY_CLK_SRC_SEL` writer - Reserved"]
pub type REG_MIPI_DSI_DPHY_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_clk_div_num(&self) -> REG_SDIO_LS_CLK_DIV_NUM_R {
        REG_SDIO_LS_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 9:12 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_clk_edge_l(&self) -> REG_SDIO_LS_CLK_EDGE_L_R {
        REG_SDIO_LS_CLK_EDGE_L_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_clk_edge_h(&self) -> REG_SDIO_LS_CLK_EDGE_H_R {
        REG_SDIO_LS_CLK_EDGE_H_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_clk_edge_n(&self) -> REG_SDIO_LS_CLK_EDGE_N_R {
        REG_SDIO_LS_CLK_EDGE_N_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_slf_clk_edge_sel(&self) -> REG_SDIO_LS_SLF_CLK_EDGE_SEL_R {
        REG_SDIO_LS_SLF_CLK_EDGE_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_drv_clk_edge_sel(&self) -> REG_SDIO_LS_DRV_CLK_EDGE_SEL_R {
        REG_SDIO_LS_DRV_CLK_EDGE_SEL_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_sam_clk_edge_sel(&self) -> REG_SDIO_LS_SAM_CLK_EDGE_SEL_R {
        REG_SDIO_LS_SAM_CLK_EDGE_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_slf_clk_en(&self) -> REG_SDIO_LS_SLF_CLK_EN_R {
        REG_SDIO_LS_SLF_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_drv_clk_en(&self) -> REG_SDIO_LS_DRV_CLK_EN_R {
        REG_SDIO_LS_DRV_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn reg_sdio_ls_sam_clk_en(&self) -> REG_SDIO_LS_SAM_CLK_EN_R {
        REG_SDIO_LS_SAM_CLK_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    pub fn reg_mipi_dsi_dphy_clk_src_sel(&self) -> REG_MIPI_DSI_DPHY_CLK_SRC_SEL_R {
        REG_MIPI_DSI_DPHY_CLK_SRC_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL02")
            .field(
                "reg_sdio_ls_clk_div_num",
                &format_args!("{}", self.reg_sdio_ls_clk_div_num().bits()),
            )
            .field(
                "reg_sdio_ls_clk_edge_l",
                &format_args!("{}", self.reg_sdio_ls_clk_edge_l().bits()),
            )
            .field(
                "reg_sdio_ls_clk_edge_h",
                &format_args!("{}", self.reg_sdio_ls_clk_edge_h().bits()),
            )
            .field(
                "reg_sdio_ls_clk_edge_n",
                &format_args!("{}", self.reg_sdio_ls_clk_edge_n().bits()),
            )
            .field(
                "reg_sdio_ls_slf_clk_edge_sel",
                &format_args!("{}", self.reg_sdio_ls_slf_clk_edge_sel().bits()),
            )
            .field(
                "reg_sdio_ls_drv_clk_edge_sel",
                &format_args!("{}", self.reg_sdio_ls_drv_clk_edge_sel().bits()),
            )
            .field(
                "reg_sdio_ls_sam_clk_edge_sel",
                &format_args!("{}", self.reg_sdio_ls_sam_clk_edge_sel().bits()),
            )
            .field(
                "reg_sdio_ls_slf_clk_en",
                &format_args!("{}", self.reg_sdio_ls_slf_clk_en().bit()),
            )
            .field(
                "reg_sdio_ls_drv_clk_en",
                &format_args!("{}", self.reg_sdio_ls_drv_clk_en().bit()),
            )
            .field(
                "reg_sdio_ls_sam_clk_en",
                &format_args!("{}", self.reg_sdio_ls_sam_clk_en().bit()),
            )
            .field(
                "reg_mipi_dsi_dphy_clk_src_sel",
                &format_args!("{}", self.reg_mipi_dsi_dphy_clk_src_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_CTRL02_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_clk_div_num(&mut self) -> REG_SDIO_LS_CLK_DIV_NUM_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_clk_edge_cfg_update(
        &mut self,
    ) -> REG_SDIO_LS_CLK_EDGE_CFG_UPDATE_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_CLK_EDGE_CFG_UPDATE_W::new(self, 8)
    }
    #[doc = "Bits 9:12 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_clk_edge_l(&mut self) -> REG_SDIO_LS_CLK_EDGE_L_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_CLK_EDGE_L_W::new(self, 9)
    }
    #[doc = "Bits 13:16 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_clk_edge_h(&mut self) -> REG_SDIO_LS_CLK_EDGE_H_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_CLK_EDGE_H_W::new(self, 13)
    }
    #[doc = "Bits 17:20 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_clk_edge_n(&mut self) -> REG_SDIO_LS_CLK_EDGE_N_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_CLK_EDGE_N_W::new(self, 17)
    }
    #[doc = "Bits 21:22 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_slf_clk_edge_sel(
        &mut self,
    ) -> REG_SDIO_LS_SLF_CLK_EDGE_SEL_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_SLF_CLK_EDGE_SEL_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_drv_clk_edge_sel(
        &mut self,
    ) -> REG_SDIO_LS_DRV_CLK_EDGE_SEL_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_DRV_CLK_EDGE_SEL_W::new(self, 23)
    }
    #[doc = "Bits 25:26 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_sam_clk_edge_sel(
        &mut self,
    ) -> REG_SDIO_LS_SAM_CLK_EDGE_SEL_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_SAM_CLK_EDGE_SEL_W::new(self, 25)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_slf_clk_en(&mut self) -> REG_SDIO_LS_SLF_CLK_EN_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_SLF_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_drv_clk_en(&mut self) -> REG_SDIO_LS_DRV_CLK_EN_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_DRV_CLK_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sdio_ls_sam_clk_en(&mut self) -> REG_SDIO_LS_SAM_CLK_EN_W<PERI_CLK_CTRL02_SPEC> {
        REG_SDIO_LS_SAM_CLK_EN_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mipi_dsi_dphy_clk_src_sel(
        &mut self,
    ) -> REG_MIPI_DSI_DPHY_CLK_SRC_SEL_W<PERI_CLK_CTRL02_SPEC> {
        REG_MIPI_DSI_DPHY_CLK_SRC_SEL_W::new(self, 30)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_ctrl02::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_ctrl02::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL02_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL02_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl02::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL02_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl02::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL02_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL02 to value 0"]
impl crate::Resettable for PERI_CLK_CTRL02_SPEC {
    const RESET_VALUE: u32 = 0;
}
