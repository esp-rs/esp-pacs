#[doc = "Register `INTERNAL_SRAM_USAGE_3` reader"]
pub type R = crate::R<INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "Register `INTERNAL_SRAM_USAGE_3` writer"]
pub type W = crate::W<INTERNAL_SRAM_USAGE_3_SPEC>;
#[doc = "Field `INTERNAL_SRAM_MAC_DUMP_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
pub type INTERNAL_SRAM_MAC_DUMP_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_MAC_DUMP_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
pub type INTERNAL_SRAM_MAC_DUMP_USAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
    #[inline(always)]
    pub fn internal_sram_mac_dump_usage(&self) -> INTERNAL_SRAM_MAC_DUMP_USAGE_R {
        INTERNAL_SRAM_MAC_DUMP_USAGE_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_3")
            .field(
                "internal_sram_mac_dump_usage",
                &self.internal_sram_mac_dump_usage(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
    #[inline(always)]
    pub fn internal_sram_mac_dump_usage(
        &mut self,
    ) -> INTERNAL_SRAM_MAC_DUMP_USAGE_W<'_, INTERNAL_SRAM_USAGE_3_SPEC> {
        INTERNAL_SRAM_MAC_DUMP_USAGE_W::new(self, 0)
    }
}
#[doc = "Internal SRAM configuration register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`internal_sram_usage_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`internal_sram_usage_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERNAL_SRAM_USAGE_3_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`internal_sram_usage_3::R`](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`internal_sram_usage_3::W`](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_3 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_3_SPEC {}
