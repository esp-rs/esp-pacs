#[doc = "Register `T%sLOADHI` reader"]
pub type R = crate::R<TLOADHI_SPEC>;
#[doc = "Register `T%sLOADHI` writer"]
pub type W = crate::W<TLOADHI_SPEC>;
#[doc = "Field `LOAD_HI` reader - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
pub type LOAD_HI_R = crate::FieldReader<u32>;
#[doc = "Field `LOAD_HI` writer - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
pub type LOAD_HI_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 22, O, u32>;
impl R {
    #[doc = "Bits 0:21 - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TLOADHI")
            .field("load_hi", &format_args!("{}", self.load_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TLOADHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:21 - High 22 bits of the value that a reload will load onto timer %s time-base counter."]
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<TLOADHI_SPEC, 0> {
        LOAD_HI_W::new(self)
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
#[doc = "Timer %s reload value, high 22 bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tloadhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tloadhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TLOADHI_SPEC;
impl crate::RegisterSpec for TLOADHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tloadhi::R`](R) reader structure"]
impl crate::Readable for TLOADHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tloadhi::W`](W) writer structure"]
impl crate::Writable for TLOADHI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sLOADHI to value 0"]
impl crate::Resettable for TLOADHI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
