#[doc = "Register `TIMESTAMP_PRESCALER` reader"]
pub type R = crate::R<TIMESTAMP_PRESCALER_SPEC>;
#[doc = "Register `TIMESTAMP_PRESCALER` writer"]
pub type W = crate::W<TIMESTAMP_PRESCALER_SPEC>;
#[doc = "Field `TS_DIV_NUM` reader - Configures the clock division number of timestamp counter."]
pub type TS_DIV_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TS_DIV_NUM` writer - Configures the clock division number of timestamp counter."]
pub type TS_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the clock division number of timestamp counter."]
    #[inline(always)]
    pub fn ts_div_num(&self) -> TS_DIV_NUM_R {
        TS_DIV_NUM_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMESTAMP_PRESCALER")
            .field("ts_div_num", &format_args!("{}", self.ts_div_num().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMESTAMP_PRESCALER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the clock division number of timestamp counter."]
    #[inline(always)]
    #[must_use]
    pub fn ts_div_num(&mut self) -> TS_DIV_NUM_W<TIMESTAMP_PRESCALER_SPEC> {
        TS_DIV_NUM_W::new(self, 0)
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
#[doc = "Timestamp configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_prescaler::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_prescaler::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_PRESCALER_SPEC;
impl crate::RegisterSpec for TIMESTAMP_PRESCALER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_prescaler::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_PRESCALER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timestamp_prescaler::W`](W) writer structure"]
impl crate::Writable for TIMESTAMP_PRESCALER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMESTAMP_PRESCALER to value 0x1f"]
impl crate::Resettable for TIMESTAMP_PRESCALER_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
