#[doc = "Register `PRO_VECBASE_SET` reader"]
pub type R = crate::R<PRO_VECBASE_SET_SPEC>;
#[doc = "Register `PRO_VECBASE_SET` writer"]
pub type W = crate::W<PRO_VECBASE_SET_SPEC>;
#[doc = "Field `PRO_OUT_VECBASE` reader - "]
pub type PRO_OUT_VECBASE_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_OUT_VECBASE` writer - "]
pub type PRO_OUT_VECBASE_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    pub fn pro_out_vecbase(&self) -> PRO_OUT_VECBASE_R {
        PRO_OUT_VECBASE_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_VECBASE_SET")
            .field(
                "pro_out_vecbase",
                &format_args!("{}", self.pro_out_vecbase().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_VECBASE_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21"]
    #[inline(always)]
    #[must_use]
    pub fn pro_out_vecbase(&mut self) -> PRO_OUT_VECBASE_W<PRO_VECBASE_SET_SPEC> {
        PRO_OUT_VECBASE_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_vecbase_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_vecbase_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_VECBASE_SET_SPEC;
impl crate::RegisterSpec for PRO_VECBASE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_vecbase_set::R`](R) reader structure"]
impl crate::Readable for PRO_VECBASE_SET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_vecbase_set::W`](W) writer structure"]
impl crate::Writable for PRO_VECBASE_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_VECBASE_SET to value 0"]
impl crate::Resettable for PRO_VECBASE_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
