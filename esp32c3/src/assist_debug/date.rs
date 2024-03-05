#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `ASSIST_DEBUG_DATE` reader - reg_assist_debug_date"]
pub type ASSIST_DEBUG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `ASSIST_DEBUG_DATE` writer - reg_assist_debug_date"]
pub type ASSIST_DEBUG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - reg_assist_debug_date"]
    #[inline(always)]
    pub fn assist_debug_date(&self) -> ASSIST_DEBUG_DATE_R {
        ASSIST_DEBUG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE")
            .field(
                "assist_debug_date",
                &format_args!("{}", self.assist_debug_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - reg_assist_debug_date"]
    #[inline(always)]
    #[must_use]
    pub fn assist_debug_date(&mut self) -> ASSIST_DEBUG_DATE_W<DATE_SPEC> {
        ASSIST_DEBUG_DATE_W::new(self, 0)
    }
}
#[doc = "ASSIST_DEBUG_DATE_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0x0200_8010"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0200_8010;
}
