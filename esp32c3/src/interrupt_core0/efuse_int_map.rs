#[doc = "Register `EFUSE_INT_MAP` reader"]
pub type R = crate::R<EFUSE_INT_MAP_SPEC>;
#[doc = "Register `EFUSE_INT_MAP` writer"]
pub type W = crate::W<EFUSE_INT_MAP_SPEC>;
#[doc = "Field `EFUSE_INT_MAP` reader - reg_core0_efuse_int_map"]
pub type EFUSE_INT_MAP_R = crate::FieldReader;
#[doc = "Field `EFUSE_INT_MAP` writer - reg_core0_efuse_int_map"]
pub type EFUSE_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_efuse_int_map"]
    #[inline(always)]
    pub fn efuse_int_map(&self) -> EFUSE_INT_MAP_R {
        EFUSE_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE_INT_MAP")
            .field(
                "efuse_int_map",
                &format_args!("{}", self.efuse_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EFUSE_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_efuse_int_map"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_int_map(&mut self) -> EFUSE_INT_MAP_W<EFUSE_INT_MAP_SPEC> {
        EFUSE_INT_MAP_W::new(self, 0)
    }
}
#[doc = "efuse intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFUSE_INT_MAP_SPEC;
impl crate::RegisterSpec for EFUSE_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_int_map::R`](R) reader structure"]
impl crate::Readable for EFUSE_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`efuse_int_map::W`](W) writer structure"]
impl crate::Writable for EFUSE_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSE_INT_MAP to value 0"]
impl crate::Resettable for EFUSE_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
