#[doc = "Register `SIGMADELTA_CG` reader"]
pub type R = crate::R<SIGMADELTA_CG_SPEC>;
#[doc = "Register `SIGMADELTA_CG` writer"]
pub type W = crate::W<SIGMADELTA_CG_SPEC>;
#[doc = "Field `CLK_EN` reader - Clock enable bit of configuration registers for sigma delta modulation."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Clock enable bit of configuration registers for sigma delta modulation."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Clock enable bit of configuration registers for sigma delta modulation."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA_CG")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SIGMADELTA_CG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - Clock enable bit of configuration registers for sigma delta modulation."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<SIGMADELTA_CG_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
#[doc = "Clock Gating Configure Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_cg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_cg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA_CG_SPEC;
impl crate::RegisterSpec for SIGMADELTA_CG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta_cg::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA_CG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta_cg::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA_CG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGMADELTA_CG to value 0"]
impl crate::Resettable for SIGMADELTA_CG_SPEC {
    const RESET_VALUE: u32 = 0;
}
