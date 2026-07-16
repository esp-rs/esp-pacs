#[doc = "Register `GDMA_CTRL0` reader"]
pub type R = crate::R<GDMA_CTRL0_SPEC>;
#[doc = "Register `GDMA_CTRL0` writer"]
pub type W = crate::W<GDMA_CTRL0_SPEC>;
#[doc = "Field `GDMA_SYS_CLK_EN` reader - need_des"]
pub type GDMA_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `GDMA_SYS_CLK_EN` writer - need_des"]
pub type GDMA_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn gdma_sys_clk_en(&self) -> GDMA_SYS_CLK_EN_R {
        GDMA_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GDMA_CTRL0")
            .field("gdma_sys_clk_en", &self.gdma_sys_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn gdma_sys_clk_en(&mut self) -> GDMA_SYS_CLK_EN_W<'_, GDMA_CTRL0_SPEC> {
        GDMA_SYS_CLK_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GDMA_CTRL0_SPEC;
impl crate::RegisterSpec for GDMA_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gdma_ctrl0::R`](R) reader structure"]
impl crate::Readable for GDMA_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gdma_ctrl0::W`](W) writer structure"]
impl crate::Writable for GDMA_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GDMA_CTRL0 to value 0"]
impl crate::Resettable for GDMA_CTRL0_SPEC {}
