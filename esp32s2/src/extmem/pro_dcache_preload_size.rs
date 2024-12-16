#[doc = "Register `PRO_DCACHE_PRELOAD_SIZE` reader"]
pub type R = crate::R<PRO_DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Register `PRO_DCACHE_PRELOAD_SIZE` writer"]
pub type W = crate::W<PRO_DCACHE_PRELOAD_SIZE_SPEC>;
#[doc = "Field `PRO_DCACHE_PRELOAD_SIZE` reader - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
pub type PRO_DCACHE_PRELOAD_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `PRO_DCACHE_PRELOAD_SIZE` writer - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
pub type PRO_DCACHE_PRELOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PRO_DCACHE_PRELOAD_ORDER` reader - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_PRELOAD_ORDER_R = crate::BitReader;
#[doc = "Field `PRO_DCACHE_PRELOAD_ORDER` writer - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
pub type PRO_DCACHE_PRELOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
    #[inline(always)]
    pub fn pro_dcache_preload_size(&self) -> PRO_DCACHE_PRELOAD_SIZE_R {
        PRO_DCACHE_PRELOAD_SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_preload_order(&self) -> PRO_DCACHE_PRELOAD_ORDER_R {
        PRO_DCACHE_PRELOAD_ORDER_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_PRELOAD_SIZE")
            .field("pro_dcache_preload_size", &self.pro_dcache_preload_size())
            .field("pro_dcache_preload_order", &self.pro_dcache_preload_order())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - The bits are used to configure the length for manual pre-load operation. It should be combined with PRO_DCACHE_PRELOAD_ADDR_REG.."]
    #[inline(always)]
    pub fn pro_dcache_preload_size(
        &mut self,
    ) -> PRO_DCACHE_PRELOAD_SIZE_W<PRO_DCACHE_PRELOAD_SIZE_SPEC> {
        PRO_DCACHE_PRELOAD_SIZE_W::new(self, 0)
    }
    #[doc = "Bit 10 - The bits are used to configure the direction of manual pre-load operation. 1: descending, 0: ascending."]
    #[inline(always)]
    pub fn pro_dcache_preload_order(
        &mut self,
    ) -> PRO_DCACHE_PRELOAD_ORDER_W<PRO_DCACHE_PRELOAD_SIZE_SPEC> {
        PRO_DCACHE_PRELOAD_ORDER_W::new(self, 10)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_dcache_preload_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_dcache_preload_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_PRELOAD_SIZE_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_PRELOAD_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_preload_size::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_PRELOAD_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dcache_preload_size::W`](W) writer structure"]
impl crate::Writable for PRO_DCACHE_PRELOAD_SIZE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_DCACHE_PRELOAD_SIZE to value 0x0200"]
impl crate::Resettable for PRO_DCACHE_PRELOAD_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
