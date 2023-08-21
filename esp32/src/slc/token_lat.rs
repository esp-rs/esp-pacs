#[doc = "Register `TOKEN_LAT` reader"]
pub type R = crate::R<TOKEN_LAT_SPEC>;
#[doc = "Field `SLC0_TOKEN` reader - "]
pub type SLC0_TOKEN_R = crate::FieldReader<u16>;
#[doc = "Field `SLC1_TOKEN` reader - "]
pub type SLC1_TOKEN_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc0_token(&self) -> SLC0_TOKEN_R {
        SLC0_TOKEN_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn slc1_token(&self) -> SLC1_TOKEN_R {
        SLC1_TOKEN_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOKEN_LAT")
            .field("slc0_token", &format_args!("{}", self.slc0_token().bits()))
            .field("slc1_token", &format_args!("{}", self.slc1_token().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOKEN_LAT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`token_lat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOKEN_LAT_SPEC;
impl crate::RegisterSpec for TOKEN_LAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`token_lat::R`](R) reader structure"]
impl crate::Readable for TOKEN_LAT_SPEC {}
#[doc = "`reset()` method sets TOKEN_LAT to value 0"]
impl crate::Resettable for TOKEN_LAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
