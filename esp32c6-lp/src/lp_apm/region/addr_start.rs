#[doc = "Register `ADDR_START` reader"]
pub type R = crate::R<ADDR_START_SPEC>;
#[doc = "Register `ADDR_START` writer"]
pub type W = crate::W<ADDR_START_SPEC>;
#[doc = "Field `ADDR_START` reader - Start address of region0"]
pub type ADDR_START_R = crate::FieldReader<u32>;
#[doc = "Field `ADDR_START` writer - Start address of region0"]
pub type ADDR_START_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of region0"]
    #[inline(always)]
    pub fn addr_start(&self) -> ADDR_START_R {
        ADDR_START_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR_START")
            .field("addr_start", &self.addr_start().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ADDR_START_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of region0"]
    #[inline(always)]
    #[must_use]
    pub fn addr_start(&mut self) -> ADDR_START_W<ADDR_START_SPEC> {
        ADDR_START_W::new(self, 0)
    }
}
#[doc = "Region address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr_start::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_start::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_START_SPEC;
impl crate::RegisterSpec for ADDR_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_start::R`](R) reader structure"]
impl crate::Readable for ADDR_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_start::W`](W) writer structure"]
impl crate::Writable for ADDR_START_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR_START to value 0"]
impl crate::Resettable for ADDR_START_SPEC {
    const RESET_VALUE: u32 = 0;
}
