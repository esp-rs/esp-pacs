#[doc = "Register `SRAM_USAGE_CONF` reader"]
pub type R = crate::R<SRAM_USAGE_CONF_SPEC>;
#[doc = "Register `SRAM_USAGE_CONF` writer"]
pub type W = crate::W<SRAM_USAGE_CONF_SPEC>;
#[doc = "Field `SRAM_USAGE` reader - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
pub type SRAM_USAGE_R = crate::FieldReader;
#[doc = "Field `SRAM_USAGE` writer - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
pub type SRAM_USAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MAC_DUMP_ALLOC` reader - reserved."]
pub type MAC_DUMP_ALLOC_R = crate::FieldReader;
#[doc = "Field `MAC_DUMP_ALLOC` writer - reserved."]
pub type MAC_DUMP_ALLOC_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CACHE_USAGE` reader - reserved"]
pub type CACHE_USAGE_R = crate::BitReader;
impl R {
    #[doc = "Bits 10:14 - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
    #[inline(always)]
    pub fn sram_usage(&self) -> SRAM_USAGE_R {
        SRAM_USAGE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - reserved."]
    #[inline(always)]
    pub fn mac_dump_alloc(&self) -> MAC_DUMP_ALLOC_R {
        MAC_DUMP_ALLOC_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn cache_usage(&self) -> CACHE_USAGE_R {
        CACHE_USAGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_USAGE_CONF")
            .field("sram_usage", &self.sram_usage())
            .field("mac_dump_alloc", &self.mac_dump_alloc())
            .field("cache_usage", &self.cache_usage())
            .finish()
    }
}
impl W {
    #[doc = "Bits 10:14 - 0: cpu use hp-memory. 1: mac-dump accessing hp-memory."]
    #[inline(always)]
    #[must_use]
    pub fn sram_usage(&mut self) -> SRAM_USAGE_W<SRAM_USAGE_CONF_SPEC> {
        SRAM_USAGE_W::new(self, 10)
    }
    #[doc = "Bits 20:24 - reserved."]
    #[inline(always)]
    #[must_use]
    pub fn mac_dump_alloc(&mut self) -> MAC_DUMP_ALLOC_W<SRAM_USAGE_CONF_SPEC> {
        MAC_DUMP_ALLOC_W::new(self, 20)
    }
}
#[doc = "HP memory usage configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_usage_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_usage_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_USAGE_CONF_SPEC;
impl crate::RegisterSpec for SRAM_USAGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_usage_conf::R`](R) reader structure"]
impl crate::Readable for SRAM_USAGE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_usage_conf::W`](W) writer structure"]
impl crate::Writable for SRAM_USAGE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_USAGE_CONF to value 0"]
impl crate::Resettable for SRAM_USAGE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
