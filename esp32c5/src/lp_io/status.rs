#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `INTERRUPT` reader - Configures the interrupt status of GPIO0 ~ GPIO7.\\\\ 0: No interrupt\\\\ 1: Interrupt is triggered\\\\ Bit0 is corresponding to GPIO0, bit1 is corresponding to GPIO1, and etc. This register is used together LP_IO_PIN0_INT_TYPE in register LP_IO_PIN0_REG.\\\\"]
pub type INTERRUPT_R = crate::FieldReader;
#[doc = "Field `INTERRUPT` writer - Configures the interrupt status of GPIO0 ~ GPIO7.\\\\ 0: No interrupt\\\\ 1: Interrupt is triggered\\\\ Bit0 is corresponding to GPIO0, bit1 is corresponding to GPIO1, and etc. This register is used together LP_IO_PIN0_INT_TYPE in register LP_IO_PIN0_REG.\\\\"]
pub type INTERRUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the interrupt status of GPIO0 ~ GPIO7.\\\\ 0: No interrupt\\\\ 1: Interrupt is triggered\\\\ Bit0 is corresponding to GPIO0, bit1 is corresponding to GPIO1, and etc. This register is used together LP_IO_PIN0_INT_TYPE in register LP_IO_PIN0_REG.\\\\"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("interrupt", &self.interrupt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the interrupt status of GPIO0 ~ GPIO7.\\\\ 0: No interrupt\\\\ 1: Interrupt is triggered\\\\ Bit0 is corresponding to GPIO0, bit1 is corresponding to GPIO1, and etc. This register is used together LP_IO_PIN0_INT_TYPE in register LP_IO_PIN0_REG.\\\\"]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W<'_, STATUS_SPEC> {
        INTERRUPT_W::new(self, 0)
    }
}
#[doc = "LP GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
