///Register `WAKEUP_SEL` reader
pub type R = crate::R<WAKEUP_SEL_SPEC>;
///Register `WAKEUP_SEL` writer
pub type W = crate::W<WAKEUP_SEL_SPEC>;
///Field `SAR_WAKEUP_SEL` reader - 0: ADC1. 1: ADC2.
pub type SAR_WAKEUP_SEL_R = crate::BitReader;
///Field `SAR_WAKEUP_SEL` writer - 0: ADC1. 1: ADC2.
pub type SAR_WAKEUP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 0: ADC1. 1: ADC2.
    #[inline(always)]
    pub fn sar_wakeup_sel(&self) -> SAR_WAKEUP_SEL_R {
        SAR_WAKEUP_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WAKEUP_SEL")
            .field("sar_wakeup_sel", &self.sar_wakeup_sel())
            .finish()
    }
}
impl W {
    ///Bit 0 - 0: ADC1. 1: ADC2.
    #[inline(always)]
    #[must_use]
    pub fn sar_wakeup_sel(&mut self) -> SAR_WAKEUP_SEL_W<WAKEUP_SEL_SPEC> {
        SAR_WAKEUP_SEL_W::new(self, 0)
    }
}
/**Wakeup source select register.

You can [`read`](crate::generic::Reg::read) this register and get [`wakeup_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wakeup_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WAKEUP_SEL_SPEC;
impl crate::RegisterSpec for WAKEUP_SEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`wakeup_sel::R`](R) reader structure
impl crate::Readable for WAKEUP_SEL_SPEC {}
///`write(|w| ..)` method takes [`wakeup_sel::W`](W) writer structure
impl crate::Writable for WAKEUP_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WAKEUP_SEL to value 0
impl crate::Resettable for WAKEUP_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
