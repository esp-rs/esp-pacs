///Register `STATUS1` reader
pub type R = crate::R<STATUS1_SPEC>;
///Register `STATUS1` writer
pub type W = crate::W<STATUS1_SPEC>;
///Field `INTERRUPT` reader - GPIO interrupt status register for GPIO32-56
pub type INTERRUPT_R = crate::FieldReader<u32>;
///Field `INTERRUPT` writer - GPIO interrupt status register for GPIO32-56
pub type INTERRUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - GPIO interrupt status register for GPIO32-56
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS1")
            .field("interrupt", &self.interrupt())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - GPIO interrupt status register for GPIO32-56
    #[inline(always)]
    #[must_use]
    pub fn interrupt(&mut self) -> INTERRUPT_W<STATUS1_SPEC> {
        INTERRUPT_W::new(self, 0)
    }
}
/**GPIO interrupt status register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS1_SPEC;
impl crate::RegisterSpec for STATUS1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status1::R`](R) reader structure
impl crate::Readable for STATUS1_SPEC {}
///`write(|w| ..)` method takes [`status1::W`](W) writer structure
impl crate::Writable for STATUS1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STATUS1 to value 0
impl crate::Resettable for STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
