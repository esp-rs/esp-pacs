///Register `CH_ENA_AD1_SET` writer
pub type W = crate::W<CH_ENA_AD1_SET_SPEC>;
///Field `CH_SET(32-49)` writer - Configures whether or not to enable ch%s.\\0: Invalid, No effect\\1: Enable
pub type CH_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Configures whether or not to enable ch(32-49).\\0: Invalid, No effect\\1: Enable
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH_SET32` field
    #[inline(always)]
    #[must_use]
    pub fn ch_set(&mut self, n: u8) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        CH_SET_W::new(self, n)
    }
    ///Bit 0 - Configures whether or not to enable ch32.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set32(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 0)
    }
    ///Bit 1 - Configures whether or not to enable ch33.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set33(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 1)
    }
    ///Bit 2 - Configures whether or not to enable ch34.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set34(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 2)
    }
    ///Bit 3 - Configures whether or not to enable ch35.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set35(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 3)
    }
    ///Bit 4 - Configures whether or not to enable ch36.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set36(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 4)
    }
    ///Bit 5 - Configures whether or not to enable ch37.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set37(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 5)
    }
    ///Bit 6 - Configures whether or not to enable ch38.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set38(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 6)
    }
    ///Bit 7 - Configures whether or not to enable ch39.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set39(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 7)
    }
    ///Bit 8 - Configures whether or not to enable ch40.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set40(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 8)
    }
    ///Bit 9 - Configures whether or not to enable ch41.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set41(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 9)
    }
    ///Bit 10 - Configures whether or not to enable ch42.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set42(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 10)
    }
    ///Bit 11 - Configures whether or not to enable ch43.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set43(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 11)
    }
    ///Bit 12 - Configures whether or not to enable ch44.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set44(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 12)
    }
    ///Bit 13 - Configures whether or not to enable ch45.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set45(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 13)
    }
    ///Bit 14 - Configures whether or not to enable ch46.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set46(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 14)
    }
    ///Bit 15 - Configures whether or not to enable ch47.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set47(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 15)
    }
    ///Bit 16 - Configures whether or not to enable ch48.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set48(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 16)
    }
    ///Bit 17 - Configures whether or not to enable ch49.\\0: Invalid, No effect\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn ch_set49(&mut self) -> CH_SET_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET_W::new(self, 17)
    }
}
/**Channel enable set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_ENA_AD1_SET_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_SET_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ch_ena_ad1_set::W`](W) writer structure
impl crate::Writable for CH_ENA_AD1_SET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CH_ENA_AD1_SET to value 0
impl crate::Resettable for CH_ENA_AD1_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
