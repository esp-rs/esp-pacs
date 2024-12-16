#[doc = "Register `UXINT_ENABLE` reader"]
pub type R = crate::R<UXINT_ENABLE_SPEC>;
#[doc = "Register `UXINT_ENABLE` writer"]
pub type W = crate::W<UXINT_ENABLE_SPEC>;
#[doc = "Field `CPU_UXINT_ENABLE` reader - "]
pub type CPU_UXINT_ENABLE_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_UXINT_ENABLE` writer - "]
pub type CPU_UXINT_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu_uxint_enable(&self) -> CPU_UXINT_ENABLE_R {
        CPU_UXINT_ENABLE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UXINT_ENABLE")
            .field("cpu_uxint_enable", &self.cpu_uxint_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu_uxint_enable(&mut self) -> CPU_UXINT_ENABLE_W<UXINT_ENABLE_SPEC> {
        CPU_UXINT_ENABLE_W::new(self, 0)
    }
}
#[doc = "PLIC UX Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uxint_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uxint_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UXINT_ENABLE_SPEC;
impl crate::RegisterSpec for UXINT_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uxint_enable::R`](R) reader structure"]
impl crate::Readable for UXINT_ENABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uxint_enable::W`](W) writer structure"]
impl crate::Writable for UXINT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UXINT_ENABLE to value 0"]
impl crate::Resettable for UXINT_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
