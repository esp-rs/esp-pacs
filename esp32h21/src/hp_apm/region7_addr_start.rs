#[doc = "Register `REGION7_ADDR_START` reader"]
pub type R = crate::R<REGION7_ADDR_START_SPEC>;
#[doc = "Register `REGION7_ADDR_START` writer"]
pub type W = crate::W<REGION7_ADDR_START_SPEC>;
#[doc = "Field `REGION7_ADDR_START` reader - Start address of region7"]
pub type REGION7_ADDR_START_R = crate::FieldReader<u32>;
#[doc = "Field `REGION7_ADDR_START` writer - Start address of region7"]
pub type REGION7_ADDR_START_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of region7"]
    #[inline(always)]
    pub fn region7_addr_start(&self) -> REGION7_ADDR_START_R {
        REGION7_ADDR_START_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION7_ADDR_START")
            .field("region7_addr_start", &self.region7_addr_start())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of region7"]
    #[inline(always)]
    pub fn region7_addr_start(&mut self) -> REGION7_ADDR_START_W<'_, REGION7_ADDR_START_SPEC> {
        REGION7_ADDR_START_W::new(self, 0)
    }
}
#[doc = "Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region7_addr_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region7_addr_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION7_ADDR_START_SPEC;
impl crate::RegisterSpec for REGION7_ADDR_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region7_addr_start::R`](R) reader structure"]
impl crate::Readable for REGION7_ADDR_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region7_addr_start::W`](W) writer structure"]
impl crate::Writable for REGION7_ADDR_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGION7_ADDR_START to value 0"]
impl crate::Resettable for REGION7_ADDR_START_SPEC {}
