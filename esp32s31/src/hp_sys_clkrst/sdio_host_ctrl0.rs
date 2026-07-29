#[doc = "Register `SDIO_HOST_CTRL0` reader"]
pub type R = crate::R<SDIO_HOST_CTRL0_SPEC>;
#[doc = "Register `SDIO_HOST_CTRL0` writer"]
pub type W = crate::W<SDIO_HOST_CTRL0_SPEC>;
#[doc = "Field `SYS_CLK_EN` reader - need_des"]
pub type SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_CLK_EN` writer - need_des"]
pub type SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HS_MODE` reader - reserved"]
pub type HS_MODE_R = crate::BitReader;
#[doc = "Field `HS_MODE` writer - reserved"]
pub type HS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS_CLK_SRC_SEL` reader - reserved"]
pub type LS_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `LS_CLK_SRC_SEL` writer - reserved"]
pub type LS_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS_CLK_EN` reader - need_des"]
pub type LS_CLK_EN_R = crate::BitReader;
#[doc = "Field `LS_CLK_EN` writer - need_des"]
pub type LS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LS_CLK_DIV_NUM` reader - need_des"]
pub type LS_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LS_CLK_DIV_NUM` writer - need_des"]
pub type LS_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&self) -> SYS_CLK_EN_R {
        SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn hs_mode(&self) -> HS_MODE_R {
        HS_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn ls_clk_src_sel(&self) -> LS_CLK_SRC_SEL_R {
        LS_CLK_SRC_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn ls_clk_en(&self) -> LS_CLK_EN_R {
        LS_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - need_des"]
    #[inline(always)]
    pub fn ls_clk_div_num(&self) -> LS_CLK_DIV_NUM_R {
        LS_CLK_DIV_NUM_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_HOST_CTRL0")
            .field("sys_clk_en", &self.sys_clk_en())
            .field("hs_mode", &self.hs_mode())
            .field("ls_clk_src_sel", &self.ls_clk_src_sel())
            .field("ls_clk_en", &self.ls_clk_en())
            .field("ls_clk_div_num", &self.ls_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sys_clk_en(&mut self) -> SYS_CLK_EN_W<'_, SDIO_HOST_CTRL0_SPEC> {
        SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn hs_mode(&mut self) -> HS_MODE_W<'_, SDIO_HOST_CTRL0_SPEC> {
        HS_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn ls_clk_src_sel(&mut self) -> LS_CLK_SRC_SEL_W<'_, SDIO_HOST_CTRL0_SPEC> {
        LS_CLK_SRC_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn ls_clk_en(&mut self) -> LS_CLK_EN_W<'_, SDIO_HOST_CTRL0_SPEC> {
        LS_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:11 - need_des"]
    #[inline(always)]
    pub fn ls_clk_div_num(&mut self) -> LS_CLK_DIV_NUM_W<'_, SDIO_HOST_CTRL0_SPEC> {
        LS_CLK_DIV_NUM_W::new(self, 4)
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
