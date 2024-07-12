#[doc = "Register `CLK` reader"]
pub type R = crate::R<CLK_SPEC>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<CLK_SPEC>;
#[doc = "Field `GATE_FORCE_ON` reader - Write 1 to force on register clock gate."]
pub type GATE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `GATE_FORCE_ON` writer - Write 1 to force on register clock gate."]
pub type GATE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to force on register clock gate."]
    #[inline(always)]
    pub fn gate_force_on(&self) -> GATE_FORCE_ON_R {
        GATE_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK")
            .field("gate_force_on", &self.gate_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to force on register clock gate."]
    #[inline(always)]
    #[must_use]
    pub fn gate_force_on(&mut self) -> GATE_FORCE_ON_W<CLK_SPEC> {
        GATE_FORCE_ON_W::new(self, 0)
    }
}
#[doc = "ECDSA clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: u32 = 0;
}
