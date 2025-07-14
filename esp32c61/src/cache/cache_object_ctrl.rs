#[doc = "Register `CACHE_OBJECT_CTRL` reader"]
pub type R = crate::R<CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Register `CACHE_OBJECT_CTRL` writer"]
pub type W = crate::W<CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Field `ICACHE0_TAG_OBJECT` reader - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE0_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE0_TAG_OBJECT` writer - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE0_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_TAG_OBJECT` reader - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE1_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE1_TAG_OBJECT` writer - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE1_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_TAG_OBJECT` reader - Reserved"]
pub type ICACHE2_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE2_TAG_OBJECT` writer - Reserved"]
pub type ICACHE2_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE3_TAG_OBJECT` reader - Reserved"]
pub type ICACHE3_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE3_TAG_OBJECT` writer - Reserved"]
pub type ICACHE3_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_TAG_OBJECT` reader - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type CACHE_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `CACHE_TAG_OBJECT` writer - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type CACHE_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0_MEM_OBJECT` reader - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE0_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE0_MEM_OBJECT` writer - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE0_MEM_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE1_MEM_OBJECT` reader - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE1_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE1_MEM_OBJECT` writer - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
pub type ICACHE1_MEM_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_MEM_OBJECT` reader - Reserved"]
pub type ICACHE2_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE2_MEM_OBJECT` writer - Reserved"]
pub type ICACHE2_MEM_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE3_MEM_OBJECT` reader - Reserved"]
pub type ICACHE3_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `ICACHE3_MEM_OBJECT` writer - Reserved"]
pub type ICACHE3_MEM_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_MEM_OBJECT` reader - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type CACHE_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `CACHE_MEM_OBJECT` writer - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type CACHE_MEM_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache0_tag_object(&self) -> ICACHE0_TAG_OBJECT_R {
        ICACHE0_TAG_OBJECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache1_tag_object(&self) -> ICACHE1_TAG_OBJECT_R {
        ICACHE1_TAG_OBJECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_tag_object(&self) -> ICACHE2_TAG_OBJECT_R {
        ICACHE2_TAG_OBJECT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_tag_object(&self) -> ICACHE3_TAG_OBJECT_R {
        ICACHE3_TAG_OBJECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn cache_tag_object(&self) -> CACHE_TAG_OBJECT_R {
        CACHE_TAG_OBJECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache0_mem_object(&self) -> ICACHE0_MEM_OBJECT_R {
        ICACHE0_MEM_OBJECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache1_mem_object(&self) -> ICACHE1_MEM_OBJECT_R {
        ICACHE1_MEM_OBJECT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn icache2_mem_object(&self) -> ICACHE2_MEM_OBJECT_R {
        ICACHE2_MEM_OBJECT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache3_mem_object(&self) -> ICACHE3_MEM_OBJECT_R {
        ICACHE3_MEM_OBJECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn cache_mem_object(&self) -> CACHE_MEM_OBJECT_R {
        CACHE_MEM_OBJECT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_OBJECT_CTRL")
            .field("icache0_tag_object", &self.icache0_tag_object())
            .field("icache1_tag_object", &self.icache1_tag_object())
            .field("icache2_tag_object", &self.icache2_tag_object())
            .field("icache3_tag_object", &self.icache3_tag_object())
            .field("cache_tag_object", &self.cache_tag_object())
            .field("icache0_mem_object", &self.icache0_mem_object())
            .field("icache1_mem_object", &self.icache1_mem_object())
            .field("icache2_mem_object", &self.icache2_mem_object())
            .field("icache3_mem_object", &self.icache3_mem_object())
            .field("cache_mem_object", &self.cache_mem_object())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to set L1-ICache0 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache0_tag_object(&mut self) -> ICACHE0_TAG_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE0_TAG_OBJECT_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to set L1-ICache1 tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache1_tag_object(&mut self) -> ICACHE1_TAG_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE1_TAG_OBJECT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn icache2_tag_object(&mut self) -> ICACHE2_TAG_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE2_TAG_OBJECT_W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn icache3_tag_object(&mut self) -> ICACHE3_TAG_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE3_TAG_OBJECT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to set L1-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn cache_tag_object(&mut self) -> CACHE_TAG_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        CACHE_TAG_OBJECT_W::new(self, 4)
    }
    #[doc = "Bit 6 - Set this bit to set L1-ICache0 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache0_mem_object(&mut self) -> ICACHE0_MEM_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE0_MEM_OBJECT_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to set L1-ICache1 data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn icache1_mem_object(&mut self) -> ICACHE1_MEM_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE1_MEM_OBJECT_W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn icache2_mem_object(&mut self) -> ICACHE2_MEM_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE2_MEM_OBJECT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache3_mem_object(&mut self) -> ICACHE3_MEM_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        ICACHE3_MEM_OBJECT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to set L1-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn cache_mem_object(&mut self) -> CACHE_MEM_OBJECT_W<CACHE_OBJECT_CTRL_SPEC> {
        CACHE_MEM_OBJECT_W::new(self, 10)
    }
}
#[doc = "Cache Tag and Data memory Object control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_object_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_object_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_object_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_OBJECT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_object_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_OBJECT_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_OBJECT_CTRL to value 0"]
impl crate::Resettable for CACHE_OBJECT_CTRL_SPEC {}
