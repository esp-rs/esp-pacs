#[doc = "Register `Core_1_ENTRY_1_ADDR` reader"]
pub type R = crate::R<CORE_1_ENTRY_1_ADDR_SPEC>;
#[doc = "Register `Core_1_ENTRY_1_ADDR` writer"]
pub type W = crate::W<CORE_1_ENTRY_1_ADDR_SPEC>;
#[doc = "Field `CORE_1_ENTRY_1_ADDR` reader - Core_1 Entry 1 address from WORLD1 to WORLD0"]
pub type CORE_1_ENTRY_1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_1_ENTRY_1_ADDR` writer - Core_1 Entry 1 address from WORLD1 to WORLD0"]
pub type CORE_1_ENTRY_1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Core_1 Entry 1 address from WORLD1 to WORLD0"]
    #[inline(always)]
    pub fn core_1_entry_1_addr(&self) -> CORE_1_ENTRY_1_ADDR_R {
        CORE_1_ENTRY_1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_ENTRY_1_ADDR")
            .field(
                "core_1_entry_1_addr",
                &format_args!("{}", self.core_1_entry_1_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_ENTRY_1_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Core_1 Entry 1 address from WORLD1 to WORLD0"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_entry_1_addr(&mut self) -> CORE_1_ENTRY_1_ADDR_W<CORE_1_ENTRY_1_ADDR_SPEC> {
        CORE_1_ENTRY_1_ADDR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Core_1 Entry 1 address configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_entry_1_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_entry_1_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_ENTRY_1_ADDR_SPEC;
impl crate::RegisterSpec for CORE_1_ENTRY_1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_entry_1_addr::R`](R) reader structure"]
impl crate::Readable for CORE_1_ENTRY_1_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_entry_1_addr::W`](W) writer structure"]
impl crate::Writable for CORE_1_ENTRY_1_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_ENTRY_1_ADDR to value 0"]
impl crate::Resettable for CORE_1_ENTRY_1_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
