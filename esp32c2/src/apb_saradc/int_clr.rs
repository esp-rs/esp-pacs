///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `THRES1_LOW` writer - Need add description
pub type THRES1_LOW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `THRES0_LOW` writer - Need add description
pub type THRES0_LOW_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `THRES1_HIGH` writer - Need add description
pub type THRES1_HIGH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `THRES0_HIGH` writer - Need add description
pub type THRES0_HIGH_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ADC2_DONE` writer - Need add description
pub type ADC2_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ADC1_DONE` writer - Need add description
pub type ADC1_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 26 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn thres1_low(&mut self) -> THRES1_LOW_W<INT_CLR_SPEC> {
        THRES1_LOW_W::new(self, 26)
    }
    ///Bit 27 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn thres0_low(&mut self) -> THRES0_LOW_W<INT_CLR_SPEC> {
        THRES0_LOW_W::new(self, 27)
    }
    ///Bit 28 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn thres1_high(&mut self) -> THRES1_HIGH_W<INT_CLR_SPEC> {
        THRES1_HIGH_W::new(self, 28)
    }
    ///Bit 29 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn thres0_high(&mut self) -> THRES0_HIGH_W<INT_CLR_SPEC> {
        THRES0_HIGH_W::new(self, 29)
    }
    ///Bit 30 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn adc2_done(&mut self) -> ADC2_DONE_W<INT_CLR_SPEC> {
        ADC2_DONE_W::new(self, 30)
    }
    ///Bit 31 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn adc1_done(&mut self) -> ADC1_DONE_W<INT_CLR_SPEC> {
        ADC1_DONE_W::new(self, 31)
    }
}
/**register description

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xfc00_0000;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
