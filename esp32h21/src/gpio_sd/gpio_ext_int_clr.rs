#[doc = "Register `GPIO_EXT_INT_CLR` writer"]
pub type W = crate::W<GPIO_EXT_INT_CLR_SPEC>;
#[doc = "Field `GPIO_EXT_COMP_NEG_0_INT_CLR` writer - analog comparator pos edge interrupt clear"]
pub type GPIO_EXT_COMP_NEG_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_COMP_POS_0_INT_CLR` writer - analog comparator neg edge interrupt clear"]
pub type GPIO_EXT_COMP_POS_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_EXT_COMP_ALL_0_INT_CLR` writer - analog comparator neg or pos edge interrupt clear"]
pub type GPIO_EXT_COMP_ALL_0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_EXT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    pub fn gpio_ext_comp_neg_0_int_clr(
        &mut self,
    ) -> GPIO_EXT_COMP_NEG_0_INT_CLR_W<'_, GPIO_EXT_INT_CLR_SPEC> {
        GPIO_EXT_COMP_NEG_0_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    pub fn gpio_ext_comp_pos_0_int_clr(
        &mut self,
    ) -> GPIO_EXT_COMP_POS_0_INT_CLR_W<'_, GPIO_EXT_INT_CLR_SPEC> {
        GPIO_EXT_COMP_POS_0_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    pub fn gpio_ext_comp_all_0_int_clr(
        &mut self,
    ) -> GPIO_EXT_COMP_ALL_0_INT_CLR_W<'_, GPIO_EXT_INT_CLR_SPEC> {
        GPIO_EXT_COMP_ALL_0_INT_CLR_W::new(self, 2)
    }
}
#[doc = "GPIO_EXT interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_ext_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_INT_CLR_SPEC;
impl crate::RegisterSpec for GPIO_EXT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_ext_int_clr::W`](W) writer structure"]
impl crate::Writable for GPIO_EXT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_EXT_INT_CLR to value 0"]
impl crate::Resettable for GPIO_EXT_INT_CLR_SPEC {}
