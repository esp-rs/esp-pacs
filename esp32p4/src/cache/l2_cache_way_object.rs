#[doc = "Register `L2_CACHE_WAY_OBJECT` reader"]
pub type R = crate::R<L2_CACHE_WAY_OBJECT_SPEC>;
#[doc = "Register `L2_CACHE_WAY_OBJECT` writer"]
pub type W = crate::W<L2_CACHE_WAY_OBJECT_SPEC>;
#[doc = "Field `L2_CACHE_WAY_OBJECT` reader - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
pub type L2_CACHE_WAY_OBJECT_R = crate::FieldReader;
#[doc = "Field `L2_CACHE_WAY_OBJECT` writer - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
pub type L2_CACHE_WAY_OBJECT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
    #[inline(always)]
    pub fn l2_cache_way_object(&self) -> L2_CACHE_WAY_OBJECT_R {
        L2_CACHE_WAY_OBJECT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_WAY_OBJECT")
            .field("l2_cache_way_object", &self.l2_cache_way_object())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7."]
    #[inline(always)]
    pub fn l2_cache_way_object(&mut self) -> L2_CACHE_WAY_OBJECT_W<L2_CACHE_WAY_OBJECT_SPEC> {
        L2_CACHE_WAY_OBJECT_W::new(self, 0)
    }
}
#[doc = "Cache Tag and Data memory way register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_way_object::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_way_object::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_WAY_OBJECT_SPEC;
impl crate::RegisterSpec for L2_CACHE_WAY_OBJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_way_object::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_WAY_OBJECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_way_object::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_WAY_OBJECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_WAY_OBJECT to value 0"]
impl crate::Resettable for L2_CACHE_WAY_OBJECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
