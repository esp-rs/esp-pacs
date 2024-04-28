#[doc = "Register `INTR_RLS` reader"]
pub type R = crate::R<INTR_RLS_SPEC>;
#[doc = "Register `INTR_RLS` writer"]
pub type W = crate::W<INTR_RLS_SPEC>;
#[doc = "Field `GPIO(0-7)` reader - The enable bit for DEDIC_GPIO%s_INT_ST register."]
pub type GPIO_R = crate::BitReader;
#[doc = "Field `GPIO(0-7)` writer - The enable bit for DEDIC_GPIO%s_INT_ST register."]
pub type GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The enable bit for DEDIC_GPIO(0-7)_INT_ST register."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO0` field"]
    #[inline(always)]
    pub fn gpio(&self, n: u8) -> GPIO_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        GPIO_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The enable bit for DEDIC_GPIO(0-7)_INT_ST register."]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = GPIO_R> + '_ {
        (0..8).map(move |n| GPIO_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register."]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register."]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RLS")
            .field("gpio0", &format_args!("{}", self.gpio0().bit()))
            .field("gpio1", &format_args!("{}", self.gpio1().bit()))
            .field("gpio2", &format_args!("{}", self.gpio2().bit()))
            .field("gpio3", &format_args!("{}", self.gpio3().bit()))
            .field("gpio4", &format_args!("{}", self.gpio4().bit()))
            .field("gpio5", &format_args!("{}", self.gpio5().bit()))
            .field("gpio6", &format_args!("{}", self.gpio6().bit()))
            .field("gpio7", &format_args!("{}", self.gpio7().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_RLS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "The enable bit for DEDIC_GPIO(0-7)_INT_ST register."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO0` field"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self, n: u8) -> GPIO_W<INTR_RLS_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        GPIO_W::new(self, n)
    }
    #[doc = "Bit 0 - The enable bit for DEDIC_GPIO0_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 0)
    }
    #[doc = "Bit 1 - The enable bit for DEDIC_GPIO1_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 1)
    }
    #[doc = "Bit 2 - The enable bit for DEDIC_GPIO2_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio2(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 2)
    }
    #[doc = "Bit 3 - The enable bit for DEDIC_GPIO3_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio3(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 3)
    }
    #[doc = "Bit 4 - The enable bit for DEDIC_GPIO4_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio4(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 4)
    }
    #[doc = "Bit 5 - The enable bit for DEDIC_GPIO5_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio5(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 5)
    }
    #[doc = "Bit 6 - The enable bit for DEDIC_GPIO6_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio6(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 6)
    }
    #[doc = "Bit 7 - The enable bit for DEDIC_GPIO7_INT_ST register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio7(&mut self) -> GPIO_W<INTR_RLS_SPEC> {
        GPIO_W::new(self, 7)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_rls::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rls::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_RLS_SPEC;
impl crate::RegisterSpec for INTR_RLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rls::R`](R) reader structure"]
impl crate::Readable for INTR_RLS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_rls::W`](W) writer structure"]
impl crate::Writable for INTR_RLS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_RLS to value 0"]
impl crate::Resettable for INTR_RLS_SPEC {
    const RESET_VALUE: u32 = 0;
}
