#[doc = "Register `REG_MAP0` reader"]
pub type R = crate::R<REG_MAP0_SPEC>;
#[doc = "Register `REG_MAP0` writer"]
pub type W = crate::W<REG_MAP0_SPEC>;
#[doc = "Field `MAP0` reader - x"]
pub type MAP0_R = crate::FieldReader<u32>;
#[doc = "Field `MAP0` writer - x"]
pub type MAP0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map0(&self) -> MAP0_R {
        MAP0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP0")
            .field("map0", &self.map0().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_MAP0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    #[must_use]
    pub fn map0(&mut self) -> MAP0_W<REG_MAP0_SPEC> {
        MAP0_W::new(self, 0)
    }
}
#[doc = "x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_map0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_MAP0_SPEC;
impl crate::RegisterSpec for REG_MAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_map0::R`](R) reader structure"]
impl crate::Readable for REG_MAP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_map0::W`](W) writer structure"]
impl crate::Writable for REG_MAP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_MAP0 to value 0"]
impl crate::Resettable for REG_MAP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
