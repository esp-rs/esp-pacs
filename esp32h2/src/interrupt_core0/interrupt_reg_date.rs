#[doc = "Register `INTERRUPT_REG_DATE` reader"]
pub type R = crate::R<INTERRUPT_REG_DATE_SPEC>;
#[doc = "Register `INTERRUPT_REG_DATE` writer"]
pub type W = crate::W<INTERRUPT_REG_DATE_SPEC>;
#[doc = "Field `INTERRUPT_REG_DATE` reader - Version control register"]
pub type INTERRUPT_REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `INTERRUPT_REG_DATE` writer - Version control register"]
pub type INTERRUPT_REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Version control register"]
    #[inline(always)]
    pub fn interrupt_reg_date(&self) -> INTERRUPT_REG_DATE_R {
        INTERRUPT_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_REG_DATE")
            .field("interrupt_reg_date", &self.interrupt_reg_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register"]
    #[inline(always)]
    pub fn interrupt_reg_date(&mut self) -> INTERRUPT_REG_DATE_W<INTERRUPT_REG_DATE_SPEC> {
        INTERRUPT_REG_DATE_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_reg_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_reg_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_REG_DATE_SPEC;
impl crate::RegisterSpec for INTERRUPT_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_reg_date::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_reg_date::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_REG_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_REG_DATE to value 0x0220_9150"]
impl crate::Resettable for INTERRUPT_REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0220_9150;
}
