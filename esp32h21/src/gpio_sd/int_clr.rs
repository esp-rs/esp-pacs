#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `GPIO_EXT_COMP_NEG_0` writer - analog comparator pos edge interrupt clear"]
pub type GPIO_EXT_COMP_NEG_0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GPIO_EXT_COMP_POS_0` writer - analog comparator neg edge interrupt clear"]
pub type GPIO_EXT_COMP_POS_0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `GPIO_EXT_COMP_ALL_0` writer - analog comparator neg or pos edge interrupt clear"]
pub type GPIO_EXT_COMP_ALL_0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - analog comparator pos edge interrupt clear"]
    #[inline(always)]
    pub fn gpio_ext_comp_neg_0(&mut self) -> GPIO_EXT_COMP_NEG_0_W<'_, INT_CLR_SPEC> {
        GPIO_EXT_COMP_NEG_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt clear"]
    #[inline(always)]
    pub fn gpio_ext_comp_pos_0(&mut self) -> GPIO_EXT_COMP_POS_0_W<'_, INT_CLR_SPEC> {
        GPIO_EXT_COMP_POS_0_W::new(self, 1)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt clear"]
    #[inline(always)]
    pub fn gpio_ext_comp_all_0(&mut self) -> GPIO_EXT_COMP_ALL_0_W<'_, INT_CLR_SPEC> {
        GPIO_EXT_COMP_ALL_0_W::new(self, 2)
    }
}
#[doc = "GPIO_EXT interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
