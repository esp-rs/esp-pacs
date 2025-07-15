#[doc = "Register `ENTRY_ADDR%s` reader"]
pub type R = crate::R<ENTRY_ADDR_SPEC>;
#[doc = "Register `ENTRY_ADDR%s` writer"]
pub type W = crate::W<ENTRY_ADDR_SPEC>;
#[doc = "Field `ENTRY_1_ADDR` reader - Core_0 Entry 1 address from WORLD1 to WORLD0"]
pub type ENTRY_1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ENTRY_1_ADDR` writer - Core_0 Entry 1 address from WORLD1 to WORLD0"]
pub type ENTRY_1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core_0 Entry 1 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn entry_1_addr(&self) -> ENTRY_1_ADDR_R {
        ENTRY_1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENTRY_ADDR")
            .field("entry_1_addr", &self.entry_1_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Core_0 Entry 1 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn entry_1_addr(&mut self) -> ENTRY_1_ADDR_W<ENTRY_ADDR_SPEC> {
        ENTRY_1_ADDR_W::new(self, 0)
    }
}
#[doc = "Core_0 Entry %s address configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`entry_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`entry_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENTRY_ADDR_SPEC;
impl crate::RegisterSpec for ENTRY_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry_addr::R`](R) reader structure"]
impl crate::Readable for ENTRY_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`entry_addr::W`](W) writer structure"]
impl crate::Writable for ENTRY_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENTRY_ADDR%s to value 0"]
impl crate::Resettable for ENTRY_ADDR_SPEC {}
