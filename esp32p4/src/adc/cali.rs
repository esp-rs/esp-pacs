#[doc = "Register `CALI` reader"]
pub type R = crate::R<CALI_SPEC>;
#[doc = "Register `CALI` writer"]
pub type W = crate::W<CALI_SPEC>;
#[doc = "Field `CFG` reader - need_des"]
pub type CFG_R = crate::FieldReader<u32>;
#[doc = "Field `CFG` writer - need_des"]
pub type CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALI")
            .field("cfg", &format_args!("{}", self.cfg().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CALI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CFG_W<CALI_SPEC> {
        CFG_W::new(self, 0)
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
#[doc = "Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cali::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cali::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CALI_SPEC;
impl crate::RegisterSpec for CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cali::R`](R) reader structure"]
impl crate::Readable for CALI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cali::W`](W) writer structure"]
impl crate::Writable for CALI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALI to value 0x8000"]
impl crate::Resettable for CALI_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
