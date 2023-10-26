#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `TARGET0_INT_CLR` writer - interupt0 clear"]
pub type TARGET0_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET1_INT_CLR` writer - interupt1 clear"]
pub type TARGET1_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TARGET2_INT_CLR` writer - interupt2 clear"]
pub type TARGET2_INT_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - interupt0 clear"]
    #[inline(always)]
    #[must_use]
    pub fn target0_int_clr(&mut self) -> TARGET0_INT_CLR_W<INT_CLR_SPEC, 0> {
        TARGET0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - interupt1 clear"]
    #[inline(always)]
    #[must_use]
    pub fn target1_int_clr(&mut self) -> TARGET1_INT_CLR_W<INT_CLR_SPEC, 1> {
        TARGET1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - interupt2 clear"]
    #[inline(always)]
    #[must_use]
    pub fn target2_int_clr(&mut self) -> TARGET2_INT_CLR_W<INT_CLR_SPEC, 2> {
        TARGET2_INT_CLR_W::new(self)
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
#[doc = "systimer interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
