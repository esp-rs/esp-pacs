///Register `UART1_INTR_MAP` reader
pub type R = crate::R<UART1_INTR_MAP_SPEC>;
///Register `UART1_INTR_MAP` writer
pub type W = crate::W<UART1_INTR_MAP_SPEC>;
///Field `UART1_INTR_MAP` reader - this register used to map uart1 interrupt to one of core1's external interrupt
pub type UART1_INTR_MAP_R = crate::FieldReader;
///Field `UART1_INTR_MAP` writer - this register used to map uart1 interrupt to one of core1's external interrupt
pub type UART1_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this register used to map uart1 interrupt to one of core1's external interrupt
    #[inline(always)]
    pub fn uart1_intr_map(&self) -> UART1_INTR_MAP_R {
        UART1_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1_INTR_MAP")
            .field("uart1_intr_map", &self.uart1_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this register used to map uart1 interrupt to one of core1's external interrupt
    #[inline(always)]
    #[must_use]
    pub fn uart1_intr_map(&mut self) -> UART1_INTR_MAP_W<UART1_INTR_MAP_SPEC> {
        UART1_INTR_MAP_W::new(self, 0)
    }
}
/**uart1 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart1_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UART1_INTR_MAP_SPEC;
impl crate::RegisterSpec for UART1_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`uart1_intr_map::R`](R) reader structure
impl crate::Readable for UART1_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`uart1_intr_map::W`](W) writer structure
impl crate::Writable for UART1_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets UART1_INTR_MAP to value 0x10
impl crate::Resettable for UART1_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
