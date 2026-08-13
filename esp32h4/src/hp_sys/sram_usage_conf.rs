#[doc = "Register `SRAM_USAGE_CONF` reader"]
pub type R = crate::R<SRAM_USAGE_CONF_SPEC>;
#[doc = "Register `SRAM_USAGE_CONF` writer"]
pub type W = crate::W<SRAM_USAGE_CONF_SPEC>;
#[doc = "Field `SRAM_USAGE` reader - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total. Mac-dump can occupy one layer within Layer1~Layer5. \\\\ This field is used to select which one layer will be used by mac-dump. This field SHALL/MUST be one-hot. LSB-bit controls Layer1, MSB-bit controls Layer5. For each bit, 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
pub type SRAM_USAGE_R = crate::FieldReader;
#[doc = "Field `SRAM_USAGE` writer - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total. Mac-dump can occupy one layer within Layer1~Layer5. \\\\ This field is used to select which one layer will be used by mac-dump. This field SHALL/MUST be one-hot. LSB-bit controls Layer1, MSB-bit controls Layer5. For each bit, 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
pub type SRAM_USAGE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MAC_DUMP_ALLOC` reader - reserved"]
pub type MAC_DUMP_ALLOC_R = crate::BitReader;
impl R {
    #[doc = "Bits 8:12 - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total. Mac-dump can occupy one layer within Layer1~Layer5. \\\\ This field is used to select which one layer will be used by mac-dump. This field SHALL/MUST be one-hot. LSB-bit controls Layer1, MSB-bit controls Layer5. For each bit, 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
    #[inline(always)]
    pub fn sram_usage(&self) -> SRAM_USAGE_R {
        SRAM_USAGE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    pub fn mac_dump_alloc(&self) -> MAC_DUMP_ALLOC_R {
        MAC_DUMP_ALLOC_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_USAGE_CONF")
            .field("sram_usage", &self.sram_usage())
            .field("mac_dump_alloc", &self.mac_dump_alloc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:12 - hp system memory is splited to 7 layers(Layer0 ~ Layer6) in total. Mac-dump can occupy one layer within Layer1~Layer5. \\\\ This field is used to select which one layer will be used by mac-dump. This field SHALL/MUST be one-hot. LSB-bit controls Layer1, MSB-bit controls Layer5. For each bit, 0: cpu use hp-memory. 1:mac-dump accessing hp-memory."]
    #[inline(always)]
    pub fn sram_usage(&mut self) -> SRAM_USAGE_W<'_, SRAM_USAGE_CONF_SPEC> {
        SRAM_USAGE_W::new(self, 8)
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
#[doc = "`reset()` method sets SRAM_USAGE_CONF to value 0"]
impl crate::Resettable for SRAM_USAGE_CONF_SPEC {}
