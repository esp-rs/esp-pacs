#[doc = "Register `DSI_DPI_CTRL0` reader"]
pub type R = crate::R<DSI_DPI_CTRL0_SPEC>;
#[doc = "Register `DSI_DPI_CTRL0` writer"]
pub type W = crate::W<DSI_DPI_CTRL0_SPEC>;
#[doc = "Field `MIPI_DSI_DPICLK_SRC_SEL` reader - need_des"]
pub type MIPI_DSI_DPICLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `MIPI_DSI_DPICLK_SRC_SEL` writer - need_des"]
pub type MIPI_DSI_DPICLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MIPI_DSI_DPICLK_EN` reader - need_des"]
pub type MIPI_DSI_DPICLK_EN_R = crate::BitReader;
#[doc = "Field `MIPI_DSI_DPICLK_EN` writer - need_des"]
pub type MIPI_DSI_DPICLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIPI_DSI_DPICLK_DIV_NUM` reader - need_des"]
pub type MIPI_DSI_DPICLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MIPI_DSI_DPICLK_DIV_NUM` writer - need_des"]
pub type MIPI_DSI_DPICLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_src_sel(&self) -> MIPI_DSI_DPICLK_SRC_SEL_R {
        MIPI_DSI_DPICLK_SRC_SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_en(&self) -> MIPI_DSI_DPICLK_EN_R {
        MIPI_DSI_DPICLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_div_num(&self) -> MIPI_DSI_DPICLK_DIV_NUM_R {
        MIPI_DSI_DPICLK_DIV_NUM_R::new(((self.bits >> 3) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_DPI_CTRL0")
            .field("mipi_dsi_dpiclk_src_sel", &self.mipi_dsi_dpiclk_src_sel())
            .field("mipi_dsi_dpiclk_en", &self.mipi_dsi_dpiclk_en())
            .field("mipi_dsi_dpiclk_div_num", &self.mipi_dsi_dpiclk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - need_des"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_src_sel(&mut self) -> MIPI_DSI_DPICLK_SRC_SEL_W<'_, DSI_DPI_CTRL0_SPEC> {
        MIPI_DSI_DPICLK_SRC_SEL_W::new(self, 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_en(&mut self) -> MIPI_DSI_DPICLK_EN_W<'_, DSI_DPI_CTRL0_SPEC> {
        MIPI_DSI_DPICLK_EN_W::new(self, 2)
    }
    #[doc = "Bits 3:10 - need_des"]
    #[inline(always)]
    pub fn mipi_dsi_dpiclk_div_num(&mut self) -> MIPI_DSI_DPICLK_DIV_NUM_W<'_, DSI_DPI_CTRL0_SPEC> {
        MIPI_DSI_DPICLK_DIV_NUM_W::new(self, 3)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_dpi_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_dpi_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_DPI_CTRL0_SPEC;
impl crate::RegisterSpec for DSI_DPI_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_dpi_ctrl0::R`](R) reader structure"]
impl crate::Readable for DSI_DPI_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_dpi_ctrl0::W`](W) writer structure"]
impl crate::Writable for DSI_DPI_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_DPI_CTRL0 to value 0"]
impl crate::Resettable for DSI_DPI_CTRL0_SPEC {}
