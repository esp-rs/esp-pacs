#[doc = "Register `SRAM_USAGE_CONF` reader"]
pub type R = crate::R<SRAM_USAGE_CONF_SPEC>;
#[doc = "Register `SRAM_USAGE_CONF` writer"]
pub type W = crate::W<SRAM_USAGE_CONF_SPEC>;
#[doc = "Field `DCACHE_USAGE` reader - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the first layer(Layer0) usage. 0: cpu use hp-memory. 1: dcache use hp-mmory. By default, dcache is closed, and typically users can enable dcache after boot-loader, but before user's BIN start runing."]
pub type DCACHE_USAGE_R = crate::BitReader;
#[doc = "Field `DCACHE_USAGE` writer - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the first layer(Layer0) usage. 0: cpu use hp-memory. 1: dcache use hp-mmory. By default, dcache is closed, and typically users can enable dcache after boot-loader, but before user's BIN start runing."]
pub type DCACHE_USAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_USAGE` reader - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the last layer(Layer6) usage. 0: cpu use hp-memory. 1: icache1 use hp-mmory. \\\\ By default, icache1 is not disabled, and the last layer memory blongs to icache1. Typically users can set this bit to 0 to disable icache1 in boot-loader."]
pub type ICACHE1_USAGE_R = crate::BitReader;
#[doc = "Field `ICACHE1_USAGE` writer - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the last layer(Layer6) usage. 0: cpu use hp-memory. 1: icache1 use hp-mmory. \\\\ By default, icache1 is not disabled, and the last layer memory blongs to icache1. Typically users can set this bit to 0 to disable icache1 in boot-loader."]
pub type ICACHE1_USAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the first layer(Layer0) usage. 0: cpu use hp-memory. 1: dcache use hp-mmory. By default, dcache is closed, and typically users can enable dcache after boot-loader, but before user's BIN start runing."]
    #[inline(always)]
    pub fn dcache_usage(&self) -> DCACHE_USAGE_R {
        DCACHE_USAGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the last layer(Layer6) usage. 0: cpu use hp-memory. 1: icache1 use hp-mmory. \\\\ By default, icache1 is not disabled, and the last layer memory blongs to icache1. Typically users can set this bit to 0 to disable icache1 in boot-loader."]
    #[inline(always)]
    pub fn icache1_usage(&self) -> ICACHE1_USAGE_R {
        ICACHE1_USAGE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_USAGE_CONF")
            .field("dcache_usage", &self.dcache_usage())
            .field("icache1_usage", &self.icache1_usage())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the first layer(Layer0) usage. 0: cpu use hp-memory. 1: dcache use hp-mmory. By default, dcache is closed, and typically users can enable dcache after boot-loader, but before user's BIN start runing."]
    #[inline(always)]
    pub fn dcache_usage(&mut self) -> DCACHE_USAGE_W<'_, SRAM_USAGE_CONF_SPEC> {
        DCACHE_USAGE_W::new(self, 0)
    }
    #[doc = "Bit 1 - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total, this field is used to control the last layer(Layer6) usage. 0: cpu use hp-memory. 1: icache1 use hp-mmory. \\\\ By default, icache1 is not disabled, and the last layer memory blongs to icache1. Typically users can set this bit to 0 to disable icache1 in boot-loader."]
    #[inline(always)]
    pub fn icache1_usage(&mut self) -> ICACHE1_USAGE_W<'_, SRAM_USAGE_CONF_SPEC> {
        ICACHE1_USAGE_W::new(self, 1)
    }
}
#[doc = "HP memory usage configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_usage_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_usage_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_USAGE_CONF_SPEC;
impl crate::RegisterSpec for SRAM_USAGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_usage_conf::R`](R) reader structure"]
impl crate::Readable for SRAM_USAGE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_usage_conf::W`](W) writer structure"]
impl crate::Writable for SRAM_USAGE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_USAGE_CONF to value 0x02"]
impl crate::Resettable for SRAM_USAGE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
