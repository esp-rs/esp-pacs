#[doc = "Register `STATUS1` reader"]
pub type R = crate::R<STATUS1_SPEC>;
#[doc = "Register `STATUS1` writer"]
pub type W = crate::W<STATUS1_SPEC>;
#[doc = "Field `INTERRUPT` reader - The interrupt status of GPIO32 ~ GPIO32, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
pub type INTERRUPT_R = crate::BitReader;
#[doc = "Field `INTERRUPT` writer - The interrupt status of GPIO32 ~ GPIO32, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
pub type INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt status of GPIO32 ~ GPIO32, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new((self.bits & 1) != 0)
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
    #[doc = "Bit 0 - The interrupt status of GPIO32 ~ GPIO32, can be configured by the software. - Each bit represents the status of its corresponding GPIO: - 0: Represents the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 0 by the software. - 1: Represents the GPIO generates the interrupt configured by GPIO_PIN0_INT_TYPE, or this bit is configured to 1 by the software."]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W<'_, STATUS1_SPEC> {
        INTERRUPT_W::new(self, 0)
    }
}
#[doc = "GPIO interrupt status register for GPIO32-32\n\nYou can [`read`](crate::Reg::read) this register and get [`status1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS1_SPEC;
impl crate::RegisterSpec for STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status1::R`](R) reader structure"]
impl crate::Readable for STATUS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status1::W`](W) writer structure"]
impl crate::Writable for STATUS1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS1 to value 0"]
impl crate::Resettable for STATUS1_SPEC {}
