#[doc = "Register `UXINT_CONF` reader"]
pub type R = crate::R<UXINT_CONF_SPEC>;
#[doc = "Register `UXINT_CONF` writer"]
pub type W = crate::W<UXINT_CONF_SPEC>;
#[doc = "Field `CPU_UXINT_CONF` reader - "]
pub type CPU_UXINT_CONF_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_UXINT_CONF` writer - "]
pub type CPU_UXINT_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu_uxint_conf(&self) -> CPU_UXINT_CONF_R {
        CPU_UXINT_CONF_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UXINT_CONF")
            .field("cpu_uxint_conf", &self.cpu_uxint_conf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu_uxint_conf(&mut self) -> CPU_UXINT_CONF_W<'_, UXINT_CONF_SPEC> {
        CPU_UXINT_CONF_W::new(self, 0)
    }
}
#[doc = "PLIC UX Interrupt Configuration Register (PLIC_UXINT_CONF_REG)\n\nYou can [`read`](crate::Reg::read) this register and get [`uxint_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uxint_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UXINT_CONF_SPEC;
impl crate::RegisterSpec for UXINT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uxint_conf::R`](R) reader structure"]
impl crate::Readable for UXINT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uxint_conf::W`](W) writer structure"]
impl crate::Writable for UXINT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UXINT_CONF to value 0"]
impl crate::Resettable for UXINT_CONF_SPEC {}
