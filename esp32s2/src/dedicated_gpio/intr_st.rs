#[doc = "Register `INTR_ST` reader"]
pub type R = crate::R<INTR_ST_SPEC>;
#[doc = "Field `GPIO(0-7)` reader - This is the status bit for DEDIC_GPIO%s_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
pub type GPIO_R = crate::BitReader;
impl R {
    #[doc = "This is the status bit for DEDIC_GPIO(0-7)_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO0` field"]
    #[inline(always)]
    pub fn gpio(&self, n: u8) -> GPIO_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        GPIO_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "This is the status bit for DEDIC_GPIO(0-7)_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = GPIO_R> + '_ {
        (0..8).map(move |n| GPIO_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - This is the status bit for DEDIC_GPIO0_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the status bit for DEDIC_GPIO1_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the status bit for DEDIC_GPIO2_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the status bit for DEDIC_GPIO3_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the status bit for DEDIC_GPIO4_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the status bit for DEDIC_GPIO5_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the status bit for DEDIC_GPIO6_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the status bit for DEDIC_GPIO7_INT_RAW when DEDIC_GPIO7_INT_ENA is set to 1."]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_ST")
            .field("gpio0", &self.gpio0().bit())
            .field("gpio1", &self.gpio1().bit())
            .field("gpio2", &self.gpio2().bit())
            .field("gpio3", &self.gpio3().bit())
            .field("gpio4", &self.gpio4().bit())
            .field("gpio5", &self.gpio5().bit())
            .field("gpio6", &self.gpio6().bit())
            .field("gpio7", &self.gpio7().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_ST_SPEC;
impl crate::RegisterSpec for INTR_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_st::R`](R) reader structure"]
impl crate::Readable for INTR_ST_SPEC {}
#[doc = "`reset()` method sets INTR_ST to value 0"]
impl crate::Resettable for INTR_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
