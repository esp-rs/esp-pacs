#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_ADDR` reader"]
pub type R = crate::R<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
#[doc = "Register `PRO_ICACHE_AUTOLOAD_SECTION1_ADDR` writer"]
pub type W = crate::W<PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ADDR` reader - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_ICACHE_AUTOLOAD_SCT1_ADDR` writer - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
pub type PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_addr(&self) -> PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R {
        PRO_ICACHE_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_AUTOLOAD_SECTION1_ADDR")
            .field(
                "pro_icache_autoload_sct1_addr",
                &self.pro_icache_autoload_sct1_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for conditional pre-load operation. It should be combined with pro_icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_addr(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W<'_, PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC> {
        PRO_ICACHE_AUTOLOAD_SCT1_ADDR_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_icache_autoload_section1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_icache_autoload_section1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_icache_autoload_section1_addr::R`](R) reader structure"]
impl crate::Readable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_icache_autoload_section1_addr::W`](W) writer structure"]
impl crate::Writable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_ICACHE_AUTOLOAD_SECTION1_ADDR to value 0"]
impl crate::Resettable for PRO_ICACHE_AUTOLOAD_SECTION1_ADDR_SPEC {}
