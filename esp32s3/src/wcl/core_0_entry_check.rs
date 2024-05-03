#[doc = "Register `Core_0_ENTRY_CHECK` reader"]
pub type R = crate::R<CORE_0_ENTRY_CHECK_SPEC>;
#[doc = "Register `Core_0_ENTRY_CHECK` writer"]
pub type W = crate::W<CORE_0_ENTRY_CHECK_SPEC>;
#[doc = "Field `CORE_0_ENTRY_CHECK` reader - This filed is used to enable entry address check"]
pub type CORE_0_ENTRY_CHECK_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_0_ENTRY_CHECK` writer - This filed is used to enable entry address check"]
pub type CORE_0_ENTRY_CHECK_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 1:13 - This filed is used to enable entry address check"]
    #[inline(always)]
    pub fn core_0_entry_check(&self) -> CORE_0_ENTRY_CHECK_R {
        CORE_0_ENTRY_CHECK_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_ENTRY_CHECK")
            .field("core_0_entry_check", &self.core_0_entry_check().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_ENTRY_CHECK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 1:13 - This filed is used to enable entry address check"]
    #[inline(always)]
    #[must_use]
    pub fn core_0_entry_check(&mut self) -> CORE_0_ENTRY_CHECK_W<CORE_0_ENTRY_CHECK_SPEC> {
        CORE_0_ENTRY_CHECK_W::new(self, 1)
    }
}
#[doc = "Core_0 Entry check configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_entry_check::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_entry_check::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_ENTRY_CHECK_SPEC;
impl crate::RegisterSpec for CORE_0_ENTRY_CHECK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_entry_check::R`](R) reader structure"]
impl crate::Readable for CORE_0_ENTRY_CHECK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_entry_check::W`](W) writer structure"]
impl crate::Writable for CORE_0_ENTRY_CHECK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Core_0_ENTRY_CHECK to value 0x02"]
impl crate::Resettable for CORE_0_ENTRY_CHECK_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
