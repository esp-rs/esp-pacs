#[doc = "Register `SDIO_HOST_CTRL0` reader"]
pub type R = crate::R<SDIO_HOST_CTRL0_SPEC>;
#[doc = "Register `SDIO_HOST_CTRL0` writer"]
pub type W = crate::W<SDIO_HOST_CTRL0_SPEC>;
#[doc = "Field `SDMMC_SYS_CLK_EN` reader - need_des"]
pub type SDMMC_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDMMC_SYS_CLK_EN` writer - need_des"]
pub type SDMMC_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HS_MODE` reader - reserved"]
pub type SDIO_HS_MODE_R = crate::BitReader;
#[doc = "Field `SDIO_HS_MODE` writer - reserved"]
pub type SDIO_HS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_SRC_SEL` reader - reserved"]
pub type SDIO_LS_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `SDIO_LS_CLK_SRC_SEL` writer - reserved"]
pub type SDIO_LS_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_EN` reader - need_des"]
pub type SDIO_LS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_LS_CLK_EN` writer - need_des"]
pub type SDIO_LS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_LS_CLK_DIV_NUM` reader - need_des"]
pub type SDIO_LS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SDIO_LS_CLK_DIV_NUM` writer - need_des"]
pub type SDIO_LS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sdmmc_sys_clk_en(&self) -> SDMMC_SYS_CLK_EN_R {
        SDMMC_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn sdio_hs_mode(&self) -> SDIO_HS_MODE_R {
        SDIO_HS_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_src_sel(&self) -> SDIO_LS_CLK_SRC_SEL_R {
        SDIO_LS_CLK_SRC_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_en(&self) -> SDIO_LS_CLK_EN_R {
        SDIO_LS_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_div_num(&self) -> SDIO_LS_CLK_DIV_NUM_R {
        SDIO_LS_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_HOST_CTRL0")
            .field("sdmmc_sys_clk_en", &self.sdmmc_sys_clk_en())
            .field("sdio_hs_mode", &self.sdio_hs_mode())
            .field("sdio_ls_clk_src_sel", &self.sdio_ls_clk_src_sel())
            .field("sdio_ls_clk_en", &self.sdio_ls_clk_en())
            .field("sdio_ls_clk_div_num", &self.sdio_ls_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sdmmc_sys_clk_en(&mut self) -> SDMMC_SYS_CLK_EN_W<'_, SDIO_HOST_CTRL0_SPEC> {
        SDMMC_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn sdio_hs_mode(&mut self) -> SDIO_HS_MODE_W<'_, SDIO_HOST_CTRL0_SPEC> {
        SDIO_HS_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn sdio_ls_clk_src_sel(&mut self) -> SDIO_LS_CLK_SRC_SEL_W<'_, SDIO_HOST_CTRL0_SPEC> {
        SDIO_LS_CLK_SRC_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_en(&mut self) -> SDIO_LS_CLK_EN_W<'_, SDIO_HOST_CTRL0_SPEC> {
        SDIO_LS_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - need_des"]
    #[inline(always)]
    pub fn sdio_ls_clk_div_num(&mut self) -> SDIO_LS_CLK_DIV_NUM_W<'_, SDIO_HOST_CTRL0_SPEC> {
        SDIO_LS_CLK_DIV_NUM_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_host_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_host_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_HOST_CTRL0_SPEC;
impl crate::RegisterSpec for SDIO_HOST_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_host_ctrl0::R`](R) reader structure"]
impl crate::Readable for SDIO_HOST_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_host_ctrl0::W`](W) writer structure"]
impl crate::Writable for SDIO_HOST_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_HOST_CTRL0 to value 0"]
impl crate::Resettable for SDIO_HOST_CTRL0_SPEC {}
