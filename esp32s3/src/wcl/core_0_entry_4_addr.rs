#[doc = "Register `Core_0_ENTRY_4_ADDR` reader"]
pub type R = crate::R<CORE_0_ENTRY_4_ADDR_SPEC>;
#[doc = "Register `Core_0_ENTRY_4_ADDR` writer"]
pub type W = crate::W<CORE_0_ENTRY_4_ADDR_SPEC>;
#[doc = "Field `CORE_0_ENTRY_4_ADDR` reader - Core_0 Entry 4 address from WORLD1 to WORLD0"]
pub type CORE_0_ENTRY_4_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_ENTRY_4_ADDR` writer - Core_0 Entry 4 address from WORLD1 to WORLD0"]
pub type CORE_0_ENTRY_4_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core_0 Entry 4 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn core_0_entry_4_addr(&self) -> CORE_0_ENTRY_4_ADDR_R {
        CORE_0_ENTRY_4_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_ENTRY_4_ADDR")
            .field("core_0_entry_4_addr", &self.core_0_entry_4_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Core_0 Entry 4 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn core_0_entry_4_addr(&mut self) -> CORE_0_ENTRY_4_ADDR_W<CORE_0_ENTRY_4_ADDR_SPEC> {
        CORE_0_ENTRY_4_ADDR_W::new(self, 0)
    }
}
#[doc = "Core_0 Entry 4 address configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_entry_4_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_entry_4_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_ENTRY_4_ADDR_SPEC;
impl crate::RegisterSpec for CORE_0_ENTRY_4_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_entry_4_addr::R`](R) reader structure"]
impl crate::Readable for CORE_0_ENTRY_4_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_entry_4_addr::W`](W) writer structure"]
impl crate::Writable for CORE_0_ENTRY_4_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Core_0_ENTRY_4_ADDR to value 0"]
impl crate::Resettable for CORE_0_ENTRY_4_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
