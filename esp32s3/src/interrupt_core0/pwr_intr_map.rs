///Register `PWR_INTR_MAP` reader
pub type R = crate::R<PWR_INTR_MAP_SPEC>;
///Register `PWR_INTR_MAP` writer
pub type W = crate::W<PWR_INTR_MAP_SPEC>;
///Field `PWR_INTR_MAP` reader - this register used to map pwr interrupt to one of core0's external interrupt
pub type PWR_INTR_MAP_R = crate::FieldReader;
///Field `PWR_INTR_MAP` writer - this register used to map pwr interrupt to one of core0's external interrupt
pub type PWR_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this register used to map pwr interrupt to one of core0's external interrupt
    #[inline(always)]
    pub fn pwr_intr_map(&self) -> PWR_INTR_MAP_R {
        PWR_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_INTR_MAP")
            .field("pwr_intr_map", &self.pwr_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this register used to map pwr interrupt to one of core0's external interrupt
    #[inline(always)]
    #[must_use]
    pub fn pwr_intr_map(&mut self) -> PWR_INTR_MAP_W<PWR_INTR_MAP_SPEC> {
        PWR_INTR_MAP_W::new(self, 0)
    }
}
/**pwr interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwr_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwr_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PWR_INTR_MAP_SPEC;
impl crate::RegisterSpec for PWR_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pwr_intr_map::R`](R) reader structure
impl crate::Readable for PWR_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`pwr_intr_map::W`](W) writer structure
impl crate::Writable for PWR_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_INTR_MAP to value 0x10
impl crate::Resettable for PWR_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
