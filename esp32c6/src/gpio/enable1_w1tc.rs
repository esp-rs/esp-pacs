#[doc = "Register `ENABLE1_W1TC` writer"]
pub type W = crate::W<ENABLE1_W1TC_SPEC>;
#[doc = "Field `ENABLE1_W1TC` writer - GPIO output enable clear register for GPIO32-34"]
pub type ENABLE1_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENABLE1_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO output enable clear register for GPIO32-34"]
    #[inline(always)]
    #[must_use]
    pub fn enable1_w1tc(&mut self) -> ENABLE1_W1TC_W<ENABLE1_W1TC_SPEC> {
        ENABLE1_W1TC_W::new(self, 0)
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
#[doc = "GPIO output enable clear register for GPIO32-34\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE1_W1TC_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enable1_w1tc::W`](W) writer structure"]
impl crate::Writable for ENABLE1_W1TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENABLE1_W1TC to value 0"]
impl crate::Resettable for ENABLE1_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
