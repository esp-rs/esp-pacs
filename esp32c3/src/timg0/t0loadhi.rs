#[doc = "Register `T0LOADHI` reader"]
pub type R = crate::R<T0LOADHI_SPEC>;
#[doc = "Register `T0LOADHI` writer"]
pub type W = crate::W<T0LOADHI_SPEC>;
#[doc = "Field `LOAD_HI` reader - reg_t0_load_hi."]
pub type LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_HI` writer - reg_t0_load_hi."]
pub type LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - reg_t0_load_hi."]
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0LOADHI")
            .field("load_hi", &format_args!("{}", self.load_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0LOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - reg_t0_load_hi."]
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<T0LOADHI_SPEC> {
        LOAD_HI_W::new(self, 0)
    }
}
#[doc = "TIMG_T0LOADHI_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0loadhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0loadhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0LOADHI_SPEC;
impl crate::RegisterSpec for T0LOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0loadhi::R`](R) reader structure"]
impl crate::Readable for T0LOADHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`t0loadhi::W`](W) writer structure"]
impl crate::Writable for T0LOADHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T0LOADHI to value 0"]
impl crate::Resettable for T0LOADHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
