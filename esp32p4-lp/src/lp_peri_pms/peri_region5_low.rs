#[doc = "Register `PERI_REGION5_LOW` reader"]
pub type R = crate::R<PERI_REGION5_LOW_SPEC>;
#[doc = "Register `PERI_REGION5_LOW` writer"]
pub type W = crate::W<PERI_REGION5_LOW_SPEC>;
#[doc = "Field `REG_PERI_REGION5_LOW` reader - NA"]
pub type REG_PERI_REGION5_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `REG_PERI_REGION5_LOW` writer - NA"]
pub type REG_PERI_REGION5_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - NA"]
    #[inline(always)]
    pub fn reg_peri_region5_low(&self) -> REG_PERI_REGION5_LOW_R {
        REG_PERI_REGION5_LOW_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_REGION5_LOW")
            .field("reg_peri_region5_low", &self.reg_peri_region5_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:31 - NA"]
    #[inline(always)]
    pub fn reg_peri_region5_low(&mut self) -> REG_PERI_REGION5_LOW_W<'_, PERI_REGION5_LOW_SPEC> {
        REG_PERI_REGION5_LOW_W::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_region5_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_region5_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_REGION5_LOW_SPEC;
impl crate::RegisterSpec for PERI_REGION5_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_region5_low::R`](R) reader structure"]
impl crate::Readable for PERI_REGION5_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_region5_low::W`](W) writer structure"]
impl crate::Writable for PERI_REGION5_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_REGION5_LOW to value 0"]
impl crate::Resettable for PERI_REGION5_LOW_SPEC {}
