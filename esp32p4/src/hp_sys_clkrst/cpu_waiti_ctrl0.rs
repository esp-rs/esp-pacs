#[doc = "Register `CPU_WAITI_CTRL0` reader"]
pub type R = crate::R<CPU_WAITI_CTRL0_SPEC>;
#[doc = "Register `CPU_WAITI_CTRL0` writer"]
pub type W = crate::W<CPU_WAITI_CTRL0_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WAITI_CTRL0")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, CPU_WAITI_CTRL0_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_waiti_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_waiti_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_WAITI_CTRL0_SPEC;
impl crate::RegisterSpec for CPU_WAITI_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_waiti_ctrl0::R`](R) reader structure"]
impl crate::Readable for CPU_WAITI_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_waiti_ctrl0::W`](W) writer structure"]
impl crate::Writable for CPU_WAITI_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_WAITI_CTRL0 to value 0"]
impl crate::Resettable for CPU_WAITI_CTRL0_SPEC {}
