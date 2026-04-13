#[doc = "Register `ICM_CLK_EN` reader"]
pub type R = crate::R<ICM_CLK_EN_SPEC>;
#[doc = "Register `ICM_CLK_EN` writer"]
pub type W = crate::W<ICM_CLK_EN_SPEC>;
#[doc = "Field `ICM_REG_CLK_EN` reader - "]
pub type ICM_REG_CLK_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_CLK_EN` writer - "]
pub type ICM_REG_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icm_reg_clk_en(&self) -> ICM_REG_CLK_EN_R {
        ICM_REG_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_CLK_EN")
            .field("icm_reg_clk_en", &self.icm_reg_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn icm_reg_clk_en(&mut self) -> ICM_REG_CLK_EN_W<'_, ICM_CLK_EN_SPEC> {
        ICM_REG_CLK_EN_W::new(self, 0)
    }
}
#[doc = "ICM clock enable\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_CLK_EN_SPEC;
impl crate::RegisterSpec for ICM_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_clk_en::R`](R) reader structure"]
impl crate::Readable for ICM_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_clk_en::W`](W) writer structure"]
impl crate::Writable for ICM_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_CLK_EN to value 0"]
impl crate::Resettable for ICM_CLK_EN_SPEC {}
