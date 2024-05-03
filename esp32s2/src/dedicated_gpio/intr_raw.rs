#[doc = "Register `INTR_RAW` reader"]
pub type R = crate::R<INTR_RAW_SPEC>;
#[doc = "Field `GPIO(0-7)` reader - This interrupt raw bit turns to high level when dedicated GPIO%s has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
pub type GPIO_R = crate::BitReader;
impl R {
    #[doc = "This interrupt raw bit turns to high level when dedicated GPIO(0-7) has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO0` field"]
    #[inline(always)]
    pub fn gpio(&self, n: u8) -> GPIO_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        GPIO_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "This interrupt raw bit turns to high level when dedicated GPIO(0-7) has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio_iter(&self) -> impl Iterator<Item = GPIO_R> + '_ {
        (0..8).map(move |n| GPIO_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - This interrupt raw bit turns to high level when dedicated GPIO0 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO_R {
        GPIO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This interrupt raw bit turns to high level when dedicated GPIO1 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This interrupt raw bit turns to high level when dedicated GPIO2 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This interrupt raw bit turns to high level when dedicated GPIO3 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This interrupt raw bit turns to high level when dedicated GPIO4 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt raw bit turns to high level when dedicated GPIO5 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This interrupt raw bit turns to high level when dedicated GPIO6 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This interrupt raw bit turns to high level when dedicated GPIO7 has level/edge change configured by DEDIC_GPIO_INTR_RCGN_REG."]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RAW")
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
impl core::fmt::Debug for crate::generic::Reg<INTR_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_RAW_SPEC;
impl crate::RegisterSpec for INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_raw::R`](R) reader structure"]
impl crate::Readable for INTR_RAW_SPEC {}
#[doc = "`reset()` method sets INTR_RAW to value 0"]
impl crate::Resettable for INTR_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
