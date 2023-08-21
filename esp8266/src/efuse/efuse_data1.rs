#[doc = "Register `EFUSE_DATA1` reader"]
pub type R = crate::R<EFUSE_DATA1_SPEC>;
#[doc = "Register `EFUSE_DATA1` writer"]
pub type W = crate::W<EFUSE_DATA1_SPEC>;
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE_DATA1")
            .field("register", &format_args!("{}", self.register().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EFUSE_DATA1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<EFUSE_DATA1_SPEC, 0> {
        REGISTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "EFUSE_DATA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuse_data1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuse_data1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFUSE_DATA1_SPEC;
impl crate::RegisterSpec for EFUSE_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_data1::R`](R) reader structure"]
impl crate::Readable for EFUSE_DATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`efuse_data1::W`](W) writer structure"]
impl crate::Writable for EFUSE_DATA1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EFUSE_DATA1 to value 0"]
impl crate::Resettable for EFUSE_DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
