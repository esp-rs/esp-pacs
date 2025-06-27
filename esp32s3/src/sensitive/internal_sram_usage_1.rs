#[doc = "Register `INTERNAL_SRAM_USAGE_1` reader"]
pub type R = crate::R<INTERNAL_SRAM_USAGE_1_SPEC>;
#[doc = "Register `INTERNAL_SRAM_USAGE_1` writer"]
pub type W = crate::W<INTERNAL_SRAM_USAGE_1_SPEC>;
#[doc = "Field `INTERNAL_SRAM_ICACHE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
pub type INTERNAL_SRAM_ICACHE_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_ICACHE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
pub type INTERNAL_SRAM_ICACHE_USAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTERNAL_SRAM_DCACHE_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
pub type INTERNAL_SRAM_DCACHE_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_DCACHE_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
pub type INTERNAL_SRAM_DCACHE_USAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTERNAL_SRAM_CPU_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
pub type INTERNAL_SRAM_CPU_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_CPU_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
pub type INTERNAL_SRAM_CPU_USAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
    #[inline(always)]
    pub fn internal_sram_icache_usage(&self) -> INTERNAL_SRAM_ICACHE_USAGE_R {
        INTERNAL_SRAM_ICACHE_USAGE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
    #[inline(always)]
    pub fn internal_sram_dcache_usage(&self) -> INTERNAL_SRAM_DCACHE_USAGE_R {
        INTERNAL_SRAM_DCACHE_USAGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:10 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
    #[inline(always)]
    pub fn internal_sram_cpu_usage(&self) -> INTERNAL_SRAM_CPU_USAGE_R {
        INTERNAL_SRAM_CPU_USAGE_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_1")
            .field(
                "internal_sram_icache_usage",
                &self.internal_sram_icache_usage(),
            )
            .field(
                "internal_sram_dcache_usage",
                &self.internal_sram_dcache_usage(),
            )
            .field("internal_sram_cpu_usage", &self.internal_sram_cpu_usage())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by icache."]
    #[inline(always)]
    pub fn internal_sram_icache_usage(
        &mut self,
    ) -> INTERNAL_SRAM_ICACHE_USAGE_W<INTERNAL_SRAM_USAGE_1_SPEC> {
        INTERNAL_SRAM_ICACHE_USAGE_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by dcache."]
    #[inline(always)]
    pub fn internal_sram_dcache_usage(
        &mut self,
    ) -> INTERNAL_SRAM_DCACHE_USAGE_W<INTERNAL_SRAM_USAGE_1_SPEC> {
        INTERNAL_SRAM_DCACHE_USAGE_W::new(self, 2)
    }
    #[doc = "Bits 4:10 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by cpu."]
    #[inline(always)]
    pub fn internal_sram_cpu_usage(
        &mut self,
    ) -> INTERNAL_SRAM_CPU_USAGE_W<INTERNAL_SRAM_USAGE_1_SPEC> {
        INTERNAL_SRAM_CPU_USAGE_W::new(self, 4)
    }
}
#[doc = "Internal SRAM configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`internal_sram_usage_1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`internal_sram_usage_1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERNAL_SRAM_USAGE_1_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`internal_sram_usage_1::R`](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`internal_sram_usage_1::W`](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_1 to value 0x07ff"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_1_SPEC {
    const RESET_VALUE: u32 = 0x07ff;
}
