#[doc = "Register `L2_CACHE_OBJECT_CTRL` reader"]
pub type R = crate::R<L2_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_OBJECT_CTRL` writer"]
pub type W = crate::W<L2_CACHE_OBJECT_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_TAG_OBJECT` reader - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L2_CACHE_TAG_OBJECT_R = crate::BitReader;
#[doc = "Field `L2_CACHE_TAG_OBJECT` writer - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
pub type L2_CACHE_TAG_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_MEM_OBJECT` reader - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L2_CACHE_MEM_OBJECT_R = crate::BitReader;
#[doc = "Field `L2_CACHE_MEM_OBJECT` writer - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
pub type L2_CACHE_MEM_OBJECT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l2_cache_tag_object(&self) -> L2_CACHE_TAG_OBJECT_R {
        L2_CACHE_TAG_OBJECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    pub fn l2_cache_mem_object(&self) -> L2_CACHE_MEM_OBJECT_R {
        L2_CACHE_MEM_OBJECT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_OBJECT_CTRL")
            .field(
                "l2_cache_tag_object",
                &format_args!("{}", self.l2_cache_tag_object().bit()),
            )
            .field(
                "l2_cache_mem_object",
                &format_args!("{}", self.l2_cache_mem_object().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_OBJECT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 5 - Set this bit to set L2-Cache tag memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_tag_object(&mut self) -> L2_CACHE_TAG_OBJECT_W<L2_CACHE_OBJECT_CTRL_SPEC> {
        L2_CACHE_TAG_OBJECT_W::new(self, 5)
    }
    #[doc = "Bit 11 - Set this bit to set L2-Cache data memory as object. This bit should be onehot with the others fields inside this register."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_mem_object(&mut self) -> L2_CACHE_MEM_OBJECT_W<L2_CACHE_OBJECT_CTRL_SPEC> {
        L2_CACHE_MEM_OBJECT_W::new(self, 11)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Cache Tag and Data memory Object control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_object_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_object_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_OBJECT_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_OBJECT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_object_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_OBJECT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_object_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_OBJECT_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_OBJECT_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_OBJECT_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
