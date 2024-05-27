///Register `SIGMADELTA%s` reader
pub type R = crate::R<SIGMADELTA_SPEC>;
///Register `SIGMADELTA%s` writer
pub type W = crate::W<SIGMADELTA_SPEC>;
///Field `IN` reader - This field is used to configure the duty cycle of sigma delta modulation output.
pub type IN_R = crate::FieldReader;
///Field `IN` writer - This field is used to configure the duty cycle of sigma delta modulation output.
pub type IN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PRESCALE` reader - This field is used to set a divider value to divide APB clock.
pub type PRESCALE_R = crate::FieldReader;
///Field `PRESCALE` writer - This field is used to set a divider value to divide APB clock.
pub type PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    pub fn in_(&self) -> IN_R {
        IN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA")
            .field("in_", &self.in_())
            .field("prescale", &self.prescale())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - This field is used to configure the duty cycle of sigma delta modulation output.
    #[inline(always)]
    #[must_use]
    pub fn in_(&mut self) -> IN_W<SIGMADELTA_SPEC> {
        IN_W::new(self, 0)
    }
    ///Bits 8:15 - This field is used to set a divider value to divide APB clock.
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<SIGMADELTA_SPEC> {
        PRESCALE_W::new(self, 8)
    }
}
/**Duty Cycle Configure Register of SDM%s

You can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SIGMADELTA_SPEC;
impl crate::RegisterSpec for SIGMADELTA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sigmadelta::R`](R) reader structure
impl crate::Readable for SIGMADELTA_SPEC {}
///`write(|w| ..)` method takes [`sigmadelta::W`](W) writer structure
impl crate::Writable for SIGMADELTA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SIGMADELTA%s to value 0xff00
impl crate::Resettable for SIGMADELTA_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
