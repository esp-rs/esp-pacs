#[doc = "Register `WDT_OP_ND` reader"]
pub type R = crate::R<WDT_OP_ND_SPEC>;
#[doc = "Register `WDT_OP_ND` writer"]
pub type W = crate::W<WDT_OP_ND_SPEC>;
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT_OP_ND")
            .field("register", &format_args!("{}", self.register().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDT_OP_ND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<WDT_OP_ND_SPEC> {
        REGISTER_W::new(self, 0)
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
#[doc = "Reload value for stage 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdt_op_nd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdt_op_nd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDT_OP_ND_SPEC;
impl crate::RegisterSpec for WDT_OP_ND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_op_nd::R`](R) reader structure"]
impl crate::Readable for WDT_OP_ND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdt_op_nd::W`](W) writer structure"]
impl crate::Writable for WDT_OP_ND_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_OP_ND to value 0"]
impl crate::Resettable for WDT_OP_ND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
