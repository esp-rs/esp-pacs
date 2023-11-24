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
            .field(
                "peri_clk_en",
                &format_args!("{}", self.peri_clk_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn peri_clk_en(&mut self) -> PERI_CLK_EN_W<PERI_CLK_EN_SPEC> {
        PERI_CLK_EN_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_EN_SPEC;
impl crate::RegisterSpec for PERI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_en::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_en::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_CLK_EN to value 0"]
impl crate::Resettable for PERI_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
