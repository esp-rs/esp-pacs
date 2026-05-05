#[doc = "Register `IN` reader"]
pub type R = crate::R<IN_SPEC>;
#[doc = "Field `REG_GPIO_IN_DATA_NEXT` reader - Reserved"]
pub type REG_GPIO_IN_DATA_NEXT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_in_data_next(&self) -> REG_GPIO_IN_DATA_NEXT_R {
        REG_GPIO_IN_DATA_NEXT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN")
            .field("reg_gpio_in_data_next", &self.reg_gpio_in_data_next())
            .finish()
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_SPEC;
impl crate::RegisterSpec for IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for IN_SPEC {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for IN_SPEC {}
