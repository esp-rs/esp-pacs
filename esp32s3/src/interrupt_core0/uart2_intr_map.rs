#[doc = "Register `UART2_INTR_MAP` reader"]
pub type R = crate::R<UART2_INTR_MAP_SPEC>;
#[doc = "Register `UART2_INTR_MAP` writer"]
pub type W = crate::W<UART2_INTR_MAP_SPEC>;
#[doc = "Field `UART2_INTR_MAP` reader - this register used to map uart2 interrupt to one of core0's external interrupt"]
pub type UART2_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `UART2_INTR_MAP` writer - this register used to map uart2 interrupt to one of core0's external interrupt"]
pub type UART2_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map uart2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn uart2_intr_map(&self) -> UART2_INTR_MAP_R {
        UART2_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART2_INTR_MAP")
            .field("uart2_intr_map", &self.uart2_intr_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map uart2 interrupt to one of core0's external interrupt"]
    #[inline(always)]
    pub fn uart2_intr_map(&mut self) -> UART2_INTR_MAP_W<UART2_INTR_MAP_SPEC> {
        UART2_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "uart2 interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART2_INTR_MAP_SPEC;
impl crate::RegisterSpec for UART2_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart2_intr_map::R`](R) reader structure"]
impl crate::Readable for UART2_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart2_intr_map::W`](W) writer structure"]
impl crate::Writable for UART2_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART2_INTR_MAP to value 0x10"]
impl crate::Resettable for UART2_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
