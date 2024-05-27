///Register `INTR_CLR` writer
pub type W = crate::W<INTR_CLR_SPEC>;
///Field `GPIO(0-7)` writer - Set this bit to clear the DEDIC_GPIO%s_INT_RAW interrupt.
pub type GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Set this bit to clear the DEDIC_GPIO(0-7)_INT_RAW interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO0` field
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self, n: u8) -> GPIO_W<INTR_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        GPIO_W::new(self, n)
    }
    ///Bit 0 - Set this bit to clear the DEDIC_GPIO0_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio0(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear the DEDIC_GPIO1_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio1(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear the DEDIC_GPIO2_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio2(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear the DEDIC_GPIO3_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio3(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to clear the DEDIC_GPIO4_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio4(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to clear the DEDIC_GPIO5_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio5(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 5)
    }
    ///Bit 6 - Set this bit to clear the DEDIC_GPIO6_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio6(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to clear the DEDIC_GPIO7_INT_RAW interrupt.
    #[inline(always)]
    #[must_use]
    pub fn gpio7(&mut self) -> GPIO_W<INTR_CLR_SPEC> {
        GPIO_W::new(self, 7)
    }
}
/**Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTR_CLR_SPEC;
impl crate::RegisterSpec for INTR_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`intr_clr::W`](W) writer structure
impl crate::Writable for INTR_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTR_CLR to value 0
impl crate::Resettable for INTR_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
