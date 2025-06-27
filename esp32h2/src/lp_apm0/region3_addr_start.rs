#[doc = "Register `REGION3_ADDR_START` reader"]
pub type R = crate::R<REGION3_ADDR_START_SPEC>;
#[doc = "Register `REGION3_ADDR_START` writer"]
pub type W = crate::W<REGION3_ADDR_START_SPEC>;
#[doc = "Field `REGION3_ADDR_START` reader - Start address of region3"]
pub type REGION3_ADDR_START_R = crate::FieldReader<u32>;
#[doc = "Field `REGION3_ADDR_START` writer - Start address of region3"]
pub type REGION3_ADDR_START_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of region3"]
    #[inline(always)]
    pub fn region3_addr_start(&self) -> REGION3_ADDR_START_R {
        REGION3_ADDR_START_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGION3_ADDR_START")
            .field("region3_addr_start", &self.region3_addr_start())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of region3"]
    #[inline(always)]
    pub fn region3_addr_start(&mut self) -> REGION3_ADDR_START_W<REGION3_ADDR_START_SPEC> {
        REGION3_ADDR_START_W::new(self, 0)
    }
}
#[doc = "Region address register\n\nYou can [`read`](crate::Reg::read) this register and get [`region3_addr_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`region3_addr_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGION3_ADDR_START_SPEC;
impl crate::RegisterSpec for REGION3_ADDR_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`region3_addr_start::R`](R) reader structure"]
impl crate::Readable for REGION3_ADDR_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`region3_addr_start::W`](W) writer structure"]
impl crate::Writable for REGION3_ADDR_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGION3_ADDR_START to value 0"]
impl crate::Resettable for REGION3_ADDR_START_SPEC {}
