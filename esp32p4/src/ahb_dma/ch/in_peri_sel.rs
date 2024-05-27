///Register `IN_PERI_SEL` reader
pub type R = crate::R<IN_PERI_SEL_SPEC>;
///Register `IN_PERI_SEL` writer
pub type W = crate::W<IN_PERI_SEL_SPEC>;
///Field `PERI_IN_SEL` reader - This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
pub type PERI_IN_SEL_R = crate::FieldReader;
///Field `PERI_IN_SEL` writer - This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
pub type PERI_IN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    pub fn peri_in_sel(&self) -> PERI_IN_SEL_R {
        PERI_IN_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_PERI_SEL")
            .field("peri_in_sel", &self.peri_in_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - This register is used to select peripheral for Rx channel 0. I3C. 1: Dummy. 2: UHCI0. 3: I2S0. 4: I2S1. 5: I2S2. 6: Dummy. 7: Dummy. 8: ADC_DAC. 9: Dummy. 10: RMT,11~15: Dummy
    #[inline(always)]
    #[must_use]
    pub fn peri_in_sel(&mut self) -> PERI_IN_SEL_W<IN_PERI_SEL_SPEC> {
        PERI_IN_SEL_W::new(self, 0)
    }
}
/**Peripheral selection of Rx channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`in_peri_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_peri_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IN_PERI_SEL_SPEC;
impl crate::RegisterSpec for IN_PERI_SEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`in_peri_sel::R`](R) reader structure
impl crate::Readable for IN_PERI_SEL_SPEC {}
///`write(|w| ..)` method takes [`in_peri_sel::W`](W) writer structure
impl crate::Writable for IN_PERI_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IN_PERI_SEL to value 0x3f
impl crate::Resettable for IN_PERI_SEL_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
