#[doc = "Register `CLK_FORCE_ON` reader"]
pub type R = crate::R<CLK_FORCE_ON_SPEC>;
#[doc = "Register `CLK_FORCE_ON` writer"]
pub type W = crate::W<CLK_FORCE_ON_SPEC>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - 0: RSA mem do not force on; 1:RSA mem force on."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - 0: RSA mem do not force on; 1:RSA mem force on."]
pub type MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: RSA mem do not force on; 1:RSA mem force on."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_FORCE_ON")
            .field("mem_clk_force_on", &self.mem_clk_force_on())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 0: RSA mem do not force on; 1:RSA mem force on."]
    #[inline(always)]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<'_, CLK_FORCE_ON_SPEC> {
        MEM_CLK_FORCE_ON_W::new(self, 0)
    }
}
#[doc = "RSA mem clk force register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_force_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_force_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_FORCE_ON_SPEC;
impl crate::RegisterSpec for CLK_FORCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_force_on::R`](R) reader structure"]
impl crate::Readable for CLK_FORCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_force_on::W`](W) writer structure"]
impl crate::Writable for CLK_FORCE_ON_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_FORCE_ON to value 0"]
impl crate::Resettable for CLK_FORCE_ON_SPEC {}
