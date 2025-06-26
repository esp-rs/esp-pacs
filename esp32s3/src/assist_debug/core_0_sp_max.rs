#[doc = "Register `CORE_0_SP_MAX` reader"]
pub type R = crate::R<CORE_0_SP_MAX_SPEC>;
#[doc = "Register `CORE_0_SP_MAX` writer"]
pub type W = crate::W<CORE_0_SP_MAX_SPEC>;
#[doc = "Field `CORE_0_SP_MAX` reader - stack max value"]
pub type CORE_0_SP_MAX_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_SP_MAX` writer - stack max value"]
pub type CORE_0_SP_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - stack max value"]
    #[inline(always)]
    pub fn core_0_sp_max(&self) -> CORE_0_SP_MAX_R {
        CORE_0_SP_MAX_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_SP_MAX")
            .field("core_0_sp_max", &self.core_0_sp_max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - stack max value"]
    #[inline(always)]
    pub fn core_0_sp_max(&mut self) -> CORE_0_SP_MAX_W<CORE_0_SP_MAX_SPEC> {
        CORE_0_SP_MAX_W::new(self, 0)
    }
}
#[doc = "core0 sp region configuration regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_sp_max::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_sp_max::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_SP_MAX_SPEC;
impl crate::RegisterSpec for CORE_0_SP_MAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_sp_max::R`](R) reader structure"]
impl crate::Readable for CORE_0_SP_MAX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_sp_max::W`](W) writer structure"]
impl crate::Writable for CORE_0_SP_MAX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_0_SP_MAX to value 0xffff_ffff"]
impl crate::Resettable for CORE_0_SP_MAX_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
