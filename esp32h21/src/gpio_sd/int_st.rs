#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `GPIO_EXT_COMP_NEG_0` reader - analog comparator pos edge interrupt status"]
pub type GPIO_EXT_COMP_NEG_0_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_COMP_POS_0` reader - analog comparator neg edge interrupt status"]
pub type GPIO_EXT_COMP_POS_0_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_COMP_ALL_0` reader - analog comparator neg or pos edge interrupt status"]
pub type GPIO_EXT_COMP_ALL_0_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt status"]
    #[inline(always)]
    pub fn gpio_ext_comp_neg_0(&self) -> GPIO_EXT_COMP_NEG_0_R {
        GPIO_EXT_COMP_NEG_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt status"]
    #[inline(always)]
    pub fn gpio_ext_comp_pos_0(&self) -> GPIO_EXT_COMP_POS_0_R {
        GPIO_EXT_COMP_POS_0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt status"]
    #[inline(always)]
    pub fn gpio_ext_comp_all_0(&self) -> GPIO_EXT_COMP_ALL_0_R {
        GPIO_EXT_COMP_ALL_0_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("gpio_ext_comp_neg_0", &self.gpio_ext_comp_neg_0())
            .field("gpio_ext_comp_pos_0", &self.gpio_ext_comp_pos_0())
            .field("gpio_ext_comp_all_0", &self.gpio_ext_comp_all_0())
            .finish()
    }
}
#[doc = "GPIO_EXT interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
