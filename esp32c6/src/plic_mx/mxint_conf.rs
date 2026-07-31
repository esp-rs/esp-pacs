#[doc = "Register `MXINT_CONF` reader"]
pub type R = crate::R<MXINT_CONF_SPEC>;
#[doc = "Register `MXINT_CONF` writer"]
pub type W = crate::W<MXINT_CONF_SPEC>;
#[doc = "Field `CPU_MXINT_CONF` reader - "]
pub type CPU_MXINT_CONF_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_MXINT_CONF` writer - "]
pub type CPU_MXINT_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu_mxint_conf(&self) -> CPU_MXINT_CONF_R {
        CPU_MXINT_CONF_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MXINT_CONF")
            .field("cpu_mxint_conf", &self.cpu_mxint_conf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cpu_mxint_conf(&mut self) -> CPU_MXINT_CONF_W<'_, MXINT_CONF_SPEC> {
        CPU_MXINT_CONF_W::new(self, 0)
    }
}
#[doc = "PLIC MX Interrupt Configuration Register (PLIC_MXINT_CONF_REG)\n\nYou can [`read`](crate::Reg::read) this register and get [`mxint_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mxint_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MXINT_CONF_SPEC;
impl crate::RegisterSpec for MXINT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mxint_conf::R`](R) reader structure"]
impl crate::Readable for MXINT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mxint_conf::W`](W) writer structure"]
impl crate::Writable for MXINT_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MXINT_CONF to value 0"]
impl crate::Resettable for MXINT_CONF_SPEC {}
