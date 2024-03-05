#[doc = "Register `RWBT_IRQ_MAP` reader"]
pub type R = crate::R<RWBT_IRQ_MAP_SPEC>;
#[doc = "Register `RWBT_IRQ_MAP` writer"]
pub type W = crate::W<RWBT_IRQ_MAP_SPEC>;
#[doc = "Field `RWBT_IRQ_MAP` reader - reg_core0_rwbt_irq_map"]
pub type RWBT_IRQ_MAP_R = crate::FieldReader;
#[doc = "Field `RWBT_IRQ_MAP` writer - reg_core0_rwbt_irq_map"]
pub type RWBT_IRQ_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_rwbt_irq_map"]
    #[inline(always)]
    pub fn rwbt_irq_map(&self) -> RWBT_IRQ_MAP_R {
        RWBT_IRQ_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RWBT_IRQ_MAP")
            .field(
                "rwbt_irq_map",
                &format_args!("{}", self.rwbt_irq_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RWBT_IRQ_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_rwbt_irq_map"]
    #[inline(always)]
    #[must_use]
    pub fn rwbt_irq_map(&mut self) -> RWBT_IRQ_MAP_W<RWBT_IRQ_MAP_SPEC> {
        RWBT_IRQ_MAP_W::new(self, 0)
    }
}
#[doc = "rwbt intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwbt_irq_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwbt_irq_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWBT_IRQ_MAP_SPEC;
impl crate::RegisterSpec for RWBT_IRQ_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwbt_irq_map::R`](R) reader structure"]
impl crate::Readable for RWBT_IRQ_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rwbt_irq_map::W`](W) writer structure"]
impl crate::Writable for RWBT_IRQ_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RWBT_IRQ_MAP to value 0"]
impl crate::Resettable for RWBT_IRQ_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
