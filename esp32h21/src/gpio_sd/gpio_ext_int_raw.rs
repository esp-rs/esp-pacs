#[doc = "Register `GPIO_EXT_INT_RAW` reader"]
pub type R = crate::R<GPIO_EXT_INT_RAW_SPEC>;
#[doc = "Field `GPIO_EXT_COMP_NEG_0_INT_RAW` reader - analog comparator pos edge interrupt raw"]
pub type GPIO_EXT_COMP_NEG_0_INT_RAW_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_COMP_POS_0_INT_RAW` reader - analog comparator neg edge interrupt raw"]
pub type GPIO_EXT_COMP_POS_0_INT_RAW_R = crate::BitReader;
#[doc = "Field `GPIO_EXT_COMP_ALL_0_INT_RAW` reader - analog comparator neg or pos edge interrupt raw"]
pub type GPIO_EXT_COMP_ALL_0_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - analog comparator pos edge interrupt raw"]
    #[inline(always)]
    pub fn gpio_ext_comp_neg_0_int_raw(&self) -> GPIO_EXT_COMP_NEG_0_INT_RAW_R {
        GPIO_EXT_COMP_NEG_0_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - analog comparator neg edge interrupt raw"]
    #[inline(always)]
    pub fn gpio_ext_comp_pos_0_int_raw(&self) -> GPIO_EXT_COMP_POS_0_INT_RAW_R {
        GPIO_EXT_COMP_POS_0_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - analog comparator neg or pos edge interrupt raw"]
    #[inline(always)]
    pub fn gpio_ext_comp_all_0_int_raw(&self) -> GPIO_EXT_COMP_ALL_0_INT_RAW_R {
        GPIO_EXT_COMP_ALL_0_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_EXT_INT_RAW")
            .field(
                "gpio_ext_comp_neg_0_int_raw",
                &self.gpio_ext_comp_neg_0_int_raw(),
            )
            .field(
                "gpio_ext_comp_pos_0_int_raw",
                &self.gpio_ext_comp_pos_0_int_raw(),
            )
            .field(
                "gpio_ext_comp_all_0_int_raw",
                &self.gpio_ext_comp_all_0_int_raw(),
            )
            .finish()
    }
}
#[doc = "GPIO_EXT interrupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_ext_int_raw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_EXT_INT_RAW_SPEC;
impl crate::RegisterSpec for GPIO_EXT_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_ext_int_raw::R`](R) reader structure"]
impl crate::Readable for GPIO_EXT_INT_RAW_SPEC {}
#[doc = "`reset()` method sets GPIO_EXT_INT_RAW to value 0"]
impl crate::Resettable for GPIO_EXT_INT_RAW_SPEC {}
