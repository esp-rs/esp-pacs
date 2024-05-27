#[doc = "Register `PRO_CACHE_LOCK_0_ADDR` reader"]
pub type R = crate::R<PRO_CACHE_LOCK_0_ADDR_SPEC>;
#[doc = "Register `PRO_CACHE_LOCK_0_ADDR` writer"]
pub type W = crate::W<PRO_CACHE_LOCK_0_ADDR_SPEC>;
#[doc = "Field `PRE` reader - "]
pub type PRE_R = crate::FieldReader<u16>;
#[doc = "Field `PRE` writer - "]
pub type PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MIN` reader - "]
pub type MIN_R = crate::FieldReader;
#[doc = "Field `MIN` writer - "]
pub type MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAX` reader - "]
pub type MAX_R = crate::FieldReader;
#[doc = "Field `MAX` writer - "]
pub type MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pre(&self) -> PRE_R {
        PRE_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_LOCK_0_ADDR")
            .field("pre", &self.pre())
            .field("min", &self.min())
            .field("max", &self.max())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PRE_W<PRO_CACHE_LOCK_0_ADDR_SPEC> {
        PRE_W::new(self, 0)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<PRO_CACHE_LOCK_0_ADDR_SPEC> {
        MIN_W::new(self, 14)
    }
    #[doc = "Bits 18:21"]
    #[inline(always)]
    #[must_use]
    pub fn max(&mut self) -> MAX_W<PRO_CACHE_LOCK_0_ADDR_SPEC> {
        MAX_W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_lock_0_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_lock_0_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_LOCK_0_ADDR_SPEC;
impl crate::RegisterSpec for PRO_CACHE_LOCK_0_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_lock_0_addr::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_LOCK_0_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_lock_0_addr::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_LOCK_0_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_LOCK_0_ADDR to value 0"]
impl crate::Resettable for PRO_CACHE_LOCK_0_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
