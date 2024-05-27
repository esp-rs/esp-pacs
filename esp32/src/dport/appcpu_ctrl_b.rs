#[doc = "Register `APPCPU_CTRL_B` reader"]
pub type R = crate::R<APPCPU_CTRL_B_SPEC>;
#[doc = "Register `APPCPU_CTRL_B` writer"]
pub type W = crate::W<APPCPU_CTRL_B_SPEC>;
#[doc = "Field `APPCPU_CLKGATE_EN` reader - "]
pub type APPCPU_CLKGATE_EN_R = crate::BitReader;
#[doc = "Field `APPCPU_CLKGATE_EN` writer - "]
pub type APPCPU_CLKGATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn appcpu_clkgate_en(&self) -> APPCPU_CLKGATE_EN_R {
        APPCPU_CLKGATE_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPCPU_CTRL_B")
            .field("appcpu_clkgate_en", &self.appcpu_clkgate_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn appcpu_clkgate_en(&mut self) -> APPCPU_CLKGATE_EN_W<APPCPU_CTRL_B_SPEC> {
        APPCPU_CLKGATE_EN_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`appcpu_ctrl_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`appcpu_ctrl_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPCPU_CTRL_B_SPEC;
impl crate::RegisterSpec for APPCPU_CTRL_B_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appcpu_ctrl_b::R`](R) reader structure"]
impl crate::Readable for APPCPU_CTRL_B_SPEC {}
#[doc = "`write(|w| ..)` method takes [`appcpu_ctrl_b::W`](W) writer structure"]
impl crate::Writable for APPCPU_CTRL_B_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPCPU_CTRL_B to value 0"]
impl crate::Resettable for APPCPU_CTRL_B_SPEC {
    const RESET_VALUE: u32 = 0;
}
