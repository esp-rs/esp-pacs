#[doc = "Register `SDIO_HOST_FUNC_CTRL0` reader"]
pub type R = crate::R<SDIO_HOST_FUNC_CTRL0_SPEC>;
#[doc = "Register `SDIO_HOST_FUNC_CTRL0` writer"]
pub type W = crate::W<SDIO_HOST_FUNC_CTRL0_SPEC>;
#[doc = "Field `SDIO_LS_CLK_EDGE_CFG_UPDATE` writer - need_des"]
pub type SDIO_LS_CLK_EDGE_CFG_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_EDGE_L` reader - need_des"]
pub type SDIO_LS_CLK_EDGE_L_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_L` writer - need_des"]
pub type SDIO_LS_CLK_EDGE_L_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_CLK_EDGE_H` reader - need_des"]
pub type SDIO_LS_CLK_EDGE_H_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_H` writer - need_des"]
pub type SDIO_LS_CLK_EDGE_H_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_CLK_EDGE_N` reader - need_des"]
pub type SDIO_LS_CLK_EDGE_N_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_EDGE_N` writer - need_des"]
pub type SDIO_LS_CLK_EDGE_N_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_LS_SLF_CLK_EDGE_SEL` reader - need_des"]
pub type SDIO_LS_SLF_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_SLF_CLK_EDGE_SEL` writer - need_des"]
pub type SDIO_LS_SLF_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDIO_LS_DRV_CLK_EDGE_SEL` reader - need_des"]
pub type SDIO_LS_DRV_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_DRV_CLK_EDGE_SEL` writer - need_des"]
pub type SDIO_LS_DRV_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDIO_LS_SAM_CLK_EDGE_SEL` reader - need_des"]
pub type SDIO_LS_SAM_CLK_EDGE_SEL_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_SAM_CLK_EDGE_SEL` writer - need_des"]
pub type SDIO_LS_SAM_CLK_EDGE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDIO_LS_SLF_CLK_EN` reader - need_des"]
pub type SDIO_LS_SLF_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_SLF_CLK_EN` writer - need_des"]
pub type SDIO_LS_SLF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_DRV_CLK_EN` reader - need_des"]
pub type SDIO_LS_DRV_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_DRV_CLK_EN` writer - need_des"]
pub type SDIO_LS_DRV_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_SAM_CLK_EN` reader - need_des"]
pub type SDIO_LS_SAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_SAM_CLK_EN` writer - need_des"]
pub type SDIO_LS_SAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:4 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_l(&self) -> SDIO_LS_CLK_EDGE_L_R {
        SDIO_LS_CLK_EDGE_L_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_h(&self) -> SDIO_LS_CLK_EDGE_H_R {
        SDIO_LS_CLK_EDGE_H_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_n(&self) -> SDIO_LS_CLK_EDGE_N_R {
        SDIO_LS_CLK_EDGE_N_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_edge_sel(&self) -> SDIO_LS_SLF_CLK_EDGE_SEL_R {
        SDIO_LS_SLF_CLK_EDGE_SEL_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_edge_sel(&self) -> SDIO_LS_DRV_CLK_EDGE_SEL_R {
        SDIO_LS_DRV_CLK_EDGE_SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_edge_sel(&self) -> SDIO_LS_SAM_CLK_EDGE_SEL_R {
        SDIO_LS_SAM_CLK_EDGE_SEL_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_en(&self) -> SDIO_LS_SLF_CLK_EN_R {
        SDIO_LS_SLF_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_en(&self) -> SDIO_LS_DRV_CLK_EN_R {
        SDIO_LS_DRV_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_en(&self) -> SDIO_LS_SAM_CLK_EN_R {
        SDIO_LS_SAM_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_HOST_FUNC_CTRL0")
            .field("sdio_ls_clk_edge_l", &self.sdio_ls_clk_edge_l())
            .field("sdio_ls_clk_edge_h", &self.sdio_ls_clk_edge_h())
            .field("sdio_ls_clk_edge_n", &self.sdio_ls_clk_edge_n())
            .field("sdio_ls_slf_clk_edge_sel", &self.sdio_ls_slf_clk_edge_sel())
            .field("sdio_ls_drv_clk_edge_sel", &self.sdio_ls_drv_clk_edge_sel())
            .field("sdio_ls_sam_clk_edge_sel", &self.sdio_ls_sam_clk_edge_sel())
            .field("sdio_ls_slf_clk_en", &self.sdio_ls_slf_clk_en())
            .field("sdio_ls_drv_clk_en", &self.sdio_ls_drv_clk_en())
            .field("sdio_ls_sam_clk_en", &self.sdio_ls_sam_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_cfg_update(
        &mut self,
    ) -> SDIO_LS_CLK_EDGE_CFG_UPDATE_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_CLK_EDGE_CFG_UPDATE_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_l(&mut self) -> SDIO_LS_CLK_EDGE_L_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_CLK_EDGE_L_W::new(self, 1)
    }
    #[doc = "Bits 5:8 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_h(&mut self) -> SDIO_LS_CLK_EDGE_H_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_CLK_EDGE_H_W::new(self, 5)
    }
    #[doc = "Bits 9:12 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_edge_n(&mut self) -> SDIO_LS_CLK_EDGE_N_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_CLK_EDGE_N_W::new(self, 9)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_edge_sel(
        &mut self,
    ) -> SDIO_LS_SLF_CLK_EDGE_SEL_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_SLF_CLK_EDGE_SEL_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_edge_sel(
        &mut self,
    ) -> SDIO_LS_DRV_CLK_EDGE_SEL_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_DRV_CLK_EDGE_SEL_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_edge_sel(
        &mut self,
    ) -> SDIO_LS_SAM_CLK_EDGE_SEL_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_SAM_CLK_EDGE_SEL_W::new(self, 19)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_slf_clk_en(&mut self) -> SDIO_LS_SLF_CLK_EN_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_SLF_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_drv_clk_en(&mut self) -> SDIO_LS_DRV_CLK_EN_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_DRV_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_sam_clk_en(&mut self) -> SDIO_LS_SAM_CLK_EN_W<'_, SDIO_HOST_FUNC_CTRL0_SPEC> {
        SDIO_LS_SAM_CLK_EN_W::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_host_func_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_host_func_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_HOST_FUNC_CTRL0_SPEC;
impl crate::RegisterSpec for SDIO_HOST_FUNC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_host_func_ctrl0::R`](R) reader structure"]
impl crate::Readable for SDIO_HOST_FUNC_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_host_func_ctrl0::W`](W) writer structure"]
impl crate::Writable for SDIO_HOST_FUNC_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_HOST_FUNC_CTRL0 to value 0"]
impl crate::Resettable for SDIO_HOST_FUNC_CTRL0_SPEC {}
