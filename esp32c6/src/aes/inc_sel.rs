#[doc = "Register `INC_SEL` reader"]
pub type R = crate::R<INC_SEL_SPEC>;
#[doc = "Register `INC_SEL` writer"]
pub type W = crate::W<INC_SEL_SPEC>;
#[doc = "Field `INC_SEL` reader - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
pub type INC_SEL_R = crate::BitReader;
#[doc = "Field `INC_SEL` writer - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
pub type INC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
    #[inline(always)]
    pub fn inc_sel(&self) -> INC_SEL_R {
        INC_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INC_SEL")
            .field("inc_sel", &format_args!("{}", self.inc_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INC_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit decides the standard incrementing function. 0: INC32. 1: INC128."]
    #[inline(always)]
    #[must_use]
    pub fn inc_sel(&mut self) -> INC_SEL_W<INC_SEL_SPEC> {
        INC_SEL_W::new(self, 0)
    }
}
#[doc = "Standard incrementing function configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inc_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inc_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INC_SEL_SPEC;
impl crate::RegisterSpec for INC_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inc_sel::R`](R) reader structure"]
impl crate::Readable for INC_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inc_sel::W`](W) writer structure"]
impl crate::Writable for INC_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INC_SEL to value 0"]
impl crate::Resettable for INC_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
