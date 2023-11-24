#[doc = "Register `CG` reader"]
pub type R = crate::R<CG_SPEC>;
#[doc = "Register `CG` writer"]
pub type W = crate::W<CG_SPEC>;
#[doc = "Field `SD_CLK_EN` reader - "]
pub type SD_CLK_EN_R = crate::BitReader;
#[doc = "Field `SD_CLK_EN` writer - "]
pub type SD_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sd_clk_en(&self) -> SD_CLK_EN_R {
        SD_CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CG")
            .field("sd_clk_en", &format_args!("{}", self.sd_clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn sd_clk_en(&mut self) -> SD_CLK_EN_W<CG_SPEC> {
        SD_CLK_EN_W::new(self, 31)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CG_SPEC;
impl crate::RegisterSpec for CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cg::R`](R) reader structure"]
impl crate::Readable for CG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cg::W`](W) writer structure"]
impl crate::Writable for CG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CG to value 0"]
impl crate::Resettable for CG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
