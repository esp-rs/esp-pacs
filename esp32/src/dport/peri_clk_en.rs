#[doc = "Register `PERI_CLK_EN` reader"]
pub type R = crate::R<PERI_CLK_EN_SPEC>;
#[doc = "Register `PERI_CLK_EN` writer"]
pub type W = crate::W<PERI_CLK_EN_SPEC>;
#[doc = "Field `PERI_CLK_EN` reader - "]
pub type PERI_CLK_EN_R = crate::FieldReader<u32>;
#[doc = "Field `PERI_CLK_EN` writer - "]
pub type PERI_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peri_clk_en(&self) -> PERI_CLK_EN_R {
        PERI_CLK_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_EN")
            .field("peri_clk_en", &self.peri_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peri_clk_en(&mut self) -> PERI_CLK_EN_W<PERI_CLK_EN_SPEC> {
        PERI_CLK_EN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_EN_SPEC;
impl crate::RegisterSpec for PERI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_en::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_en::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_EN to value 0"]
impl crate::Resettable for PERI_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
