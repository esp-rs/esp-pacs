#[doc = "Register `CORE_1_SP_MAX` reader"]
pub type R = crate::R<CORE_1_SP_MAX_SPEC>;
#[doc = "Register `CORE_1_SP_MAX` writer"]
pub type W = crate::W<CORE_1_SP_MAX_SPEC>;
#[doc = "Field `CORE_1_SP_MAX` reader - core1 sp pc status register"]
pub type CORE_1_SP_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_SP_MAX` writer - core1 sp pc status register"]
pub type CORE_1_SP_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - core1 sp pc status register"]
    #[inline(always)]
    pub fn core_1_sp_max(&self) -> CORE_1_SP_MAX_R {
        CORE_1_SP_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_SP_MAX")
            .field("core_1_sp_max", &self.core_1_sp_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - core1 sp pc status register"]
    #[inline(always)]
    pub fn core_1_sp_max(&mut self) -> CORE_1_SP_MAX_W<CORE_1_SP_MAX_SPEC> {
        CORE_1_SP_MAX_W::new(self, 0)
    }
}
#[doc = "stack max value\n\nYou can [`read`](crate::Reg::read) this register and get [`core_1_sp_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_1_sp_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_SP_MAX_SPEC;
impl crate::RegisterSpec for CORE_1_SP_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_sp_max::R`](R) reader structure"]
impl crate::Readable for CORE_1_SP_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_sp_max::W`](W) writer structure"]
impl crate::Writable for CORE_1_SP_MAX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_1_SP_MAX to value 0xffff_ffff"]
impl crate::Resettable for CORE_1_SP_MAX_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
