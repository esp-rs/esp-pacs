#[doc = "Register `EXT2` reader"]
pub type R = crate::R<EXT2_SPEC>;
#[doc = "Field `ST` reader - The status of spi state machine ."]
pub type ST_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - The status of spi state machine ."]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT2")
            .field("st", &format_args!("{}", self.st().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT2_SPEC;
impl crate::RegisterSpec for EXT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext2::R`](R) reader structure"]
impl crate::Readable for EXT2_SPEC {}
#[doc = "`reset()` method sets EXT2 to value 0"]
impl crate::Resettable for EXT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
