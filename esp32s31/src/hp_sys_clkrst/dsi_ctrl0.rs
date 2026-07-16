#[doc = "Register `DSI_CTRL0` reader"]
pub type R = crate::R<DSI_CTRL0_SPEC>;
#[doc = "Register `DSI_CTRL0` writer"]
pub type W = crate::W<DSI_CTRL0_SPEC>;
#[doc = "Field `DSI_SYS_CLK_EN` reader - need_des"]
pub type DSI_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `DSI_SYS_CLK_EN` writer - need_des"]
pub type DSI_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn dsi_sys_clk_en(&self) -> DSI_SYS_CLK_EN_R {
        DSI_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_CTRL0")
            .field("dsi_sys_clk_en", &self.dsi_sys_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn dsi_sys_clk_en(&mut self) -> DSI_SYS_CLK_EN_W<'_, DSI_CTRL0_SPEC> {
        DSI_SYS_CLK_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_CTRL0_SPEC;
impl crate::RegisterSpec for DSI_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ctrl0::R`](R) reader structure"]
impl crate::Readable for DSI_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dsi_ctrl0::W`](W) writer structure"]
impl crate::Writable for DSI_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_CTRL0 to value 0"]
impl crate::Resettable for DSI_CTRL0_SPEC {}
