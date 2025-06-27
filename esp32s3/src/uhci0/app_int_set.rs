#[doc = "Register `APP_INT_SET` writer"]
pub type W = crate::W<APP_INT_SET_SPEC>;
#[doc = "Field `APP_CTRL0_INT_SET` writer - This bit is software interrupt trigger source of UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1_INT_SET` writer - This bit is software interrupt trigger source of UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_INT_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - This bit is software interrupt trigger source of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_set(&mut self) -> APP_CTRL0_INT_SET_W<APP_INT_SET_SPEC> {
        APP_CTRL0_INT_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is software interrupt trigger source of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_set(&mut self) -> APP_CTRL1_INT_SET_W<APP_INT_SET_SPEC> {
        APP_CTRL1_INT_SET_W::new(self, 1)
    }
}
#[doc = "Software interrupt trigger source\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_int_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APP_INT_SET_SPEC;
impl crate::RegisterSpec for APP_INT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`app_int_set::W`](W) writer structure"]
impl crate::Writable for APP_INT_SET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APP_INT_SET to value 0"]
impl crate::Resettable for APP_INT_SET_SPEC {}
