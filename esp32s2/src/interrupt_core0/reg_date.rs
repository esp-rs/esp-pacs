#[doc = "Register `REG_DATE` reader"]
pub type R = crate::R<REG_DATE_SPEC>;
#[doc = "Register `REG_DATE` writer"]
pub type W = crate::W<REG_DATE_SPEC>;
#[doc = "Field `INTERRUPT_REG_DATE` reader - This is the version register."]
pub type INTERRUPT_REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `INTERRUPT_REG_DATE` writer - This is the version register."]
pub type INTERRUPT_REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - This is the version register."]
    #[inline(always)]
    pub fn interrupt_reg_date(&self) -> INTERRUPT_REG_DATE_R {
        INTERRUPT_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_DATE")
            .field("interrupt_reg_date", &self.interrupt_reg_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - This is the version register."]
    #[inline(always)]
    pub fn interrupt_reg_date(&mut self) -> INTERRUPT_REG_DATE_W<REG_DATE_SPEC> {
        INTERRUPT_REG_DATE_W::new(self, 0)
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_DATE_SPEC;
impl crate::RegisterSpec for REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_date::R`](R) reader structure"]
impl crate::Readable for REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_date::W`](W) writer structure"]
impl crate::Writable for REG_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DATE to value 0x0190_4180"]
impl crate::Resettable for REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0190_4180;
}
