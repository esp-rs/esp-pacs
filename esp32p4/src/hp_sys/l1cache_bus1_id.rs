#[doc = "Register `L1CACHE_BUS1_ID` reader"]
pub type R = crate::R<L1CACHE_BUS1_ID_SPEC>;
#[doc = "Register `L1CACHE_BUS1_ID` writer"]
pub type W = crate::W<L1CACHE_BUS1_ID_SPEC>;
#[doc = "Field `REG_L1_CACHE_BUS1_ID` reader - NA"]
pub type REG_L1_CACHE_BUS1_ID_R = crate::FieldReader;
#[doc = "Field `REG_L1_CACHE_BUS1_ID` writer - NA"]
pub type REG_L1_CACHE_BUS1_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn reg_l1_cache_bus1_id(&self) -> REG_L1_CACHE_BUS1_ID_R {
        REG_L1_CACHE_BUS1_ID_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1CACHE_BUS1_ID")
            .field("reg_l1_cache_bus1_id", &self.reg_l1_cache_bus1_id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn reg_l1_cache_bus1_id(&mut self) -> REG_L1_CACHE_BUS1_ID_W<'_, L1CACHE_BUS1_ID_SPEC> {
        REG_L1_CACHE_BUS1_ID_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`l1cache_bus1_id::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cache_bus1_id::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1CACHE_BUS1_ID_SPEC;
impl crate::RegisterSpec for L1CACHE_BUS1_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cache_bus1_id::R`](R) reader structure"]
impl crate::Readable for L1CACHE_BUS1_ID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l1cache_bus1_id::W`](W) writer structure"]
impl crate::Writable for L1CACHE_BUS1_ID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1CACHE_BUS1_ID to value 0"]
impl crate::Resettable for L1CACHE_BUS1_ID_SPEC {}
