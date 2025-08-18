#[doc = "Register `INTR_CLR` writer"]
pub type W = crate::W<INTR_CLR_SPEC>;
#[doc = "Field `GPIO(0-7)` writer - Set this bit to clear the DEDIC_GPIO%s_INT_RAW interrupt."]
pub type GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set this bit to clear the DEDIC_GPIO(0-7)_INT_RAW interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO0` field.</div>"]
    #[inline(always)]
    pub fn gpio(&mut self, n: u8) -> GPIO_W<'_, INTR_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        GPIO_W::new(self, n)
    }
    #[doc = "Bit 0 - Set this bit to clear the DEDIC_GPIO0_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the DEDIC_GPIO1_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the DEDIC_GPIO2_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio2(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the DEDIC_GPIO3_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio3(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the DEDIC_GPIO4_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio4(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the DEDIC_GPIO5_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio5(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the DEDIC_GPIO6_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio6(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the DEDIC_GPIO7_INT_RAW interrupt."]
    #[inline(always)]
    pub fn gpio7(&mut self) -> GPIO_W<'_, INTR_CLR_SPEC> {
        GPIO_W::new(self, 7)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_CLR_SPEC;
impl crate::RegisterSpec for INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intr_clr::W`](W) writer structure"]
impl crate::Writable for INTR_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_CLR to value 0"]
impl crate::Resettable for INTR_CLR_SPEC {}
