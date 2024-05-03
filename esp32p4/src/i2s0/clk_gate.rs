#[doc = "Register `CLK_GATE` reader"]
pub type R = crate::R<CLK_GATE_SPEC>;
#[doc = "Register `CLK_GATE` writer"]
pub type W = crate::W<CLK_GATE_SPEC>;
#[doc = "Field `CLK_EN` reader - set this bit to enable clock gate"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - set this bit to enable clock gate"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - set this bit to enable clock gate"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_GATE")
            .field("clk_en", &self.clk_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_GATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - set this bit to enable clock gate"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLK_GATE_SPEC> {
        CLK_EN_W::new(self, 0)
    }
}
#[doc = "Clock gate register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_GATE_SPEC;
impl crate::RegisterSpec for CLK_GATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_gate::R`](R) reader structure"]
impl crate::Readable for CLK_GATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_gate::W`](W) writer structure"]
impl crate::Writable for CLK_GATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_GATE to value 0"]
impl crate::Resettable for CLK_GATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
