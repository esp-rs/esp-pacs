#[doc = "Register `DIG_PAD_HOLD` reader"]
pub type R = crate::R<DIG_PAD_HOLD_SPEC>;
#[doc = "Register `DIG_PAD_HOLD` writer"]
pub type W = crate::W<DIG_PAD_HOLD_SPEC>;
#[doc = "Field `DIG_PAD_HOLD` reader - the configure of digital pad"]
pub type DIG_PAD_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `DIG_PAD_HOLD` writer - the configure of digital pad"]
pub type DIG_PAD_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - the configure of digital pad"]
    #[inline(always)]
    pub fn dig_pad_hold(&self) -> DIG_PAD_HOLD_R {
        DIG_PAD_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIG_PAD_HOLD")
            .field(
                "dig_pad_hold",
                &format_args!("{}", self.dig_pad_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIG_PAD_HOLD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the configure of digital pad"]
    #[inline(always)]
    #[must_use]
    pub fn dig_pad_hold(&mut self) -> DIG_PAD_HOLD_W<DIG_PAD_HOLD_SPEC> {
        DIG_PAD_HOLD_W::new(self, 0)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dig_pad_hold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dig_pad_hold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIG_PAD_HOLD_SPEC;
impl crate::RegisterSpec for DIG_PAD_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dig_pad_hold::R`](R) reader structure"]
impl crate::Readable for DIG_PAD_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dig_pad_hold::W`](W) writer structure"]
impl crate::Writable for DIG_PAD_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIG_PAD_HOLD to value 0"]
impl crate::Resettable for DIG_PAD_HOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
