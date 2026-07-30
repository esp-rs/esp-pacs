#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TARGET0_INT_CLR` writer - interupt0 clear"]
pub type TARGET0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET1_INT_CLR` writer - interupt1 clear"]
pub type TARGET1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARGET2_INT_CLR` writer - interupt2 clear"]
pub type TARGET2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - interupt0 clear"]
    #[inline(always)]
    pub fn target0_int_clr(&mut self) -> TARGET0_INT_CLR_W<'_, INT_CLR_SPEC> {
        TARGET0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - interupt1 clear"]
    #[inline(always)]
    pub fn target1_int_clr(&mut self) -> TARGET1_INT_CLR_W<'_, INT_CLR_SPEC> {
        TARGET1_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - interupt2 clear"]
    #[inline(always)]
    pub fn target2_int_clr(&mut self) -> TARGET2_INT_CLR_W<'_, INT_CLR_SPEC> {
        TARGET2_INT_CLR_W::new(self, 2)
    }
}
#[doc = "systimer interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
