#[doc = "Register `INTERNAL_SRAM_USAGE_3` reader"]
pub type R = crate::R<INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "Register `INTERNAL_SRAM_USAGE_3` writer"]
pub type W = crate::W<INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "Field `INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM` reader - internal_sram_usage_mac_dump_sram"]
pub type INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM` writer - internal_sram_usage_mac_dump_sram"]
pub type INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INTERNAL_SRAM_ALLOC_MAC_DUMP` reader - internal_sram_alloc_mac_dump"]
pub type INTERNAL_SRAM_ALLOC_MAC_DUMP_R = crate::BitReader;
#[doc = "Field `INTERNAL_SRAM_ALLOC_MAC_DUMP` writer - internal_sram_alloc_mac_dump"]
pub type INTERNAL_SRAM_ALLOC_MAC_DUMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - internal_sram_usage_mac_dump_sram"]
    #[inline(always)]
    pub fn internal_sram_usage_mac_dump_sram(&self) -> INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R {
        INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - internal_sram_alloc_mac_dump"]
    #[inline(always)]
    pub fn internal_sram_alloc_mac_dump(&self) -> INTERNAL_SRAM_ALLOC_MAC_DUMP_R {
        INTERNAL_SRAM_ALLOC_MAC_DUMP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_3")
            .field(
                "internal_sram_usage_mac_dump_sram",
                &format_args!("{}", self.internal_sram_usage_mac_dump_sram().bits()),
            )
            .field(
                "internal_sram_alloc_mac_dump",
                &format_args!("{}", self.internal_sram_alloc_mac_dump().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERNAL_SRAM_USAGE_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - internal_sram_usage_mac_dump_sram"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_usage_mac_dump_sram(
        &mut self,
    ) -> INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W<INTERNAL_SRAM_USAGE_3_SPEC> {
        INTERNAL_SRAM_USAGE_MAC_DUMP_SRAM_W::new(self, 0)
    }
    #[doc = "Bit 3 - internal_sram_alloc_mac_dump"]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_alloc_mac_dump(
        &mut self,
    ) -> INTERNAL_SRAM_ALLOC_MAC_DUMP_W<INTERNAL_SRAM_USAGE_3_SPEC> {
        INTERNAL_SRAM_ALLOC_MAC_DUMP_W::new(self, 3)
    }
}
#[doc = "SENSITIVE_INTERNAL_SRAM_USAGE_3_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`internal_sram_usage_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`internal_sram_usage_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERNAL_SRAM_USAGE_3_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`internal_sram_usage_3::R`](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`internal_sram_usage_3::W`](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_3 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
