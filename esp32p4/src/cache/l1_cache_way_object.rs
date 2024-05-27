///Register `L1_CACHE_WAY_OBJECT` reader
pub type R = crate::R<L1_CACHE_WAY_OBJECT_SPEC>;
///Register `L1_CACHE_WAY_OBJECT` writer
pub type W = crate::W<L1_CACHE_WAY_OBJECT_SPEC>;
///Field `L1_CACHE_WAY_OBJECT` reader - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7.
pub type L1_CACHE_WAY_OBJECT_R = crate::FieldReader;
///Field `L1_CACHE_WAY_OBJECT` writer - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7.
pub type L1_CACHE_WAY_OBJECT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7.
    #[inline(always)]
    pub fn l1_cache_way_object(&self) -> L1_CACHE_WAY_OBJECT_R {
        L1_CACHE_WAY_OBJECT_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_WAY_OBJECT")
            .field("l1_cache_way_object", &self.l1_cache_way_object())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Set this bits to select which way of the tag-object will be accessed. 0: way0, 1: way1, 2: way2, 3: way3, ?, 7: way7.
    #[inline(always)]
    #[must_use]
    pub fn l1_cache_way_object(&mut self) -> L1_CACHE_WAY_OBJECT_W<L1_CACHE_WAY_OBJECT_SPEC> {
        L1_CACHE_WAY_OBJECT_W::new(self, 0)
    }
}
/**Cache Tag and Data memory way register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_way_object::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_way_object::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_WAY_OBJECT_SPEC;
impl crate::RegisterSpec for L1_CACHE_WAY_OBJECT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_way_object::R`](R) reader structure
impl crate::Readable for L1_CACHE_WAY_OBJECT_SPEC {}
///`write(|w| ..)` method takes [`l1_cache_way_object::W`](W) writer structure
impl crate::Writable for L1_CACHE_WAY_OBJECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L1_CACHE_WAY_OBJECT to value 0
impl crate::Resettable for L1_CACHE_WAY_OBJECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
