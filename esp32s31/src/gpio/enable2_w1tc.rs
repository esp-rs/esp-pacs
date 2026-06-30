#[doc = "Register `ENABLE2_W1TC` writer"]
pub type W = crate::W<ENABLE2_W1TC_SPEC>;
#[doc = "Field `ENABLE2_W1TC` writer - Configures whether or not to clear the output enable register GPIO_ENABLE2_REG of GPIO64 ~ GPIO66. \\\\ 0: Not clear\\\\ 1: The corresponding bit in GPIO_ENABLE2_REG will be cleared\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. \\\\ Recommended operation: use this register to clear GPIO_ENABLE2_REG.\\\\"]
pub type ENABLE2_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE2_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - Configures whether or not to clear the output enable register GPIO_ENABLE2_REG of GPIO64 ~ GPIO66. \\\\ 0: Not clear\\\\ 1: The corresponding bit in GPIO_ENABLE2_REG will be cleared\\\\ Bit64 ~ bit66 are corresponding to GPIO64 ~ GPIO66. Bitxx ~ bitxx is invalid. \\\\ Recommended operation: use this register to clear GPIO_ENABLE2_REG.\\\\"]
    #[inline(always)]
    pub fn enable2_w1tc(&mut self) -> ENABLE2_W1TC_W<'_, ENABLE2_W1TC_SPEC> {
        ENABLE2_W1TC_W::new(self, 0)
    }
}
#[doc = "GPIO output enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable2_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE2_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE2_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable2_w1tc::W`](W) writer structure"]
impl crate::Writable for ENABLE2_W1TC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE2_W1TC to value 0"]
impl crate::Resettable for ENABLE2_W1TC_SPEC {}
