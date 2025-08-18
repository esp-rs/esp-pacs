#[doc = "Register `CACHE_TAG_OBJECT_CTRL` reader"]
pub type R = crate::R<CACHE_TAG_OBJECT_CTRL_SPEC>;
#[doc = "Register `CACHE_TAG_OBJECT_CTRL` writer"]
pub type W = crate::W<CACHE_TAG_OBJECT_CTRL_SPEC>;
#[doc = "Field `ICACHE_TAG_OBJECT` reader - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE_TAG_OBJECT` writer - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_TAG_OBJECT` reader - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type DCACHE_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `DCACHE_TAG_OBJECT` writer - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type DCACHE_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache_tag_object(&self) -> ICACHE_TAG_OBJECT_R {
        ICACHE_TAG_OBJECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn dcache_tag_object(&self) -> DCACHE_TAG_OBJECT_R {
        DCACHE_TAG_OBJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_TAG_OBJECT_CTRL")
            .field("icache_tag_object", &self.icache_tag_object())
            .field("dcache_tag_object", &self.dcache_tag_object())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to set icache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache_tag_object(&mut self) -> ICACHE_TAG_OBJECT_W<'_, CACHE_TAG_OBJECT_CTRL_SPEC> {
        ICACHE_TAG_OBJECT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to set dcache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn dcache_tag_object(&mut self) -> DCACHE_TAG_OBJECT_W<'_, CACHE_TAG_OBJECT_CTRL_SPEC> {
        DCACHE_TAG_OBJECT_W::new(self, 1)
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_tag_object_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_tag_object_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_TAG_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_TAG_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_tag_object_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_TAG_OBJECT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_tag_object_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_TAG_OBJECT_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_TAG_OBJECT_CTRL to value 0"]
impl crate::Resettable for CACHE_TAG_OBJECT_CTRL_SPEC {}
