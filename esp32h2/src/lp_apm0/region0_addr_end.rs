#[doc = "Register `REGION0_ADDR_END` reader"]
pub type R = crate::R<REGION0_ADDR_END_SPEC>;
#[doc = "Register `REGION0_ADDR_END` writer"]
pub type W = crate::W<REGION0_ADDR_END_SPEC>;
#[doc = "Field `REGION0_ADDR_END` reader - End address of region0"]
pub type REGION0_ADDR_END_R = crate::FieldReader<u32>;
#[doc = "Field `REGION0_ADDR_END` writer - End address of region0"]
pub type REGION0_ADDR_END_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - End address of region0"]
    #[inline(always)]
    pub fn region0_addr_end(&self) -> REGION0_ADDR_END_R {
        REGION0_ADDR_END_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION0_ADDR_END")
            .field("region0_addr_end", &self.region0_addr_end())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - End address of region0"]
    #[inline(always)]
    pub fn region0_addr_end(&mut self) -> REGION0_ADDR_END_W<REGION0_ADDR_END_SPEC> {
        REGION0_ADDR_END_W::new(self, 0)
    }
}
#[doc = "Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region0_addr_end::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region0_addr_end::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION0_ADDR_END_SPEC;
impl crate::RegisterSpec for REGION0_ADDR_END_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region0_addr_end::R`](R) reader structure"]
impl crate::Readable for REGION0_ADDR_END_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region0_addr_end::W`](W) writer structure"]
impl crate::Writable for REGION0_ADDR_END_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGION0_ADDR_END to value 0xffff_ffff"]
impl crate::Resettable for REGION0_ADDR_END_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
