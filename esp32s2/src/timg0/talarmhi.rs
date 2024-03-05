#[doc = "Register `T%sALARMHI` reader"]
pub type R = crate::R<TALARMHI_SPEC>;
#[doc = "Register `T%sALARMHI` writer"]
pub type W = crate::W<TALARMHI_SPEC>;
#[doc = "Field `ALARM_HI` reader - Timer %s alarm trigger time-base counter value, high 32 bits."]
pub type ALARM_HI_R = crate::FieldReader<u32>;
#[doc = "Field `ALARM_HI` writer - Timer %s alarm trigger time-base counter value, high 32 bits."]
pub type ALARM_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, high 32 bits."]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TALARMHI")
            .field("alarm_hi", &format_args!("{}", self.alarm_hi().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TALARMHI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, high 32 bits."]
    #[inline(always)]
    #[must_use]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W<TALARMHI_SPEC> {
        ALARM_HI_W::new(self, 0)
    }
}
#[doc = "Timer %s alarm value, high bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`talarmhi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`talarmhi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TALARMHI_SPEC;
impl crate::RegisterSpec for TALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`talarmhi::R`](R) reader structure"]
impl crate::Readable for TALARMHI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`talarmhi::W`](W) writer structure"]
impl crate::Writable for TALARMHI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T%sALARMHI to value 0"]
impl crate::Resettable for TALARMHI_SPEC {
    const RESET_VALUE: u32 = 0;
}
