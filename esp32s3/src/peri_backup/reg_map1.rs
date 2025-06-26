#[doc = "Register `REG_MAP1` reader"]
pub type R = crate::R<REG_MAP1_SPEC>;
#[doc = "Register `REG_MAP1` writer"]
pub type W = crate::W<REG_MAP1_SPEC>;
#[doc = "Field `MAP1` reader - x"]
pub type MAP1_R = crate::FieldReader<u32>;
#[doc = "Field `MAP1` writer - x"]
pub type MAP1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map1(&self) -> MAP1_R {
        MAP1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP1")
            .field("map1", &self.map1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map1(&mut self) -> MAP1_W<REG_MAP1_SPEC> {
        MAP1_W::new(self, 0)
    }
}
#[doc = "x\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_map1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_map1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_MAP1_SPEC;
impl crate::RegisterSpec for REG_MAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_map1::R`](R) reader structure"]
impl crate::Readable for REG_MAP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_map1::W`](W) writer structure"]
impl crate::Writable for REG_MAP1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MAP1 to value 0"]
impl crate::Resettable for REG_MAP1_SPEC {}
