#[doc = "Register `REG_MAP2` reader"]
pub type R = crate::R<REG_MAP2_SPEC>;
#[doc = "Register `REG_MAP2` writer"]
pub type W = crate::W<REG_MAP2_SPEC>;
#[doc = "Field `MAP2` reader - x"]
pub type MAP2_R = crate::FieldReader<u32>;
#[doc = "Field `MAP2` writer - x"]
pub type MAP2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    pub fn map2(&self) -> MAP2_R {
        MAP2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_MAP2")
            .field("map2", &format_args!("{}", self.map2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_MAP2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - x"]
    #[inline(always)]
    #[must_use]
    pub fn map2(&mut self) -> MAP2_W<REG_MAP2_SPEC> {
        MAP2_W::new(self, 0)
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
#[doc = "x\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_map2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_map2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_MAP2_SPEC;
impl crate::RegisterSpec for REG_MAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_map2::R`](R) reader structure"]
impl crate::Readable for REG_MAP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_map2::W`](W) writer structure"]
impl crate::Writable for REG_MAP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_MAP2 to value 0"]
impl crate::Resettable for REG_MAP2_SPEC {
    const RESET_VALUE: u32 = 0;
}
