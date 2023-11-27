#[doc = "Register `SHARP_CTRL0` reader"]
pub type R = crate::R<SHARP_CTRL0_SPEC>;
#[doc = "Register `SHARP_CTRL0` writer"]
pub type W = crate::W<SHARP_CTRL0_SPEC>;
#[doc = "Field `SHARP_THRESHOLD_LOW` reader - this field configures sharpen threshold for detail"]
pub type SHARP_THRESHOLD_LOW_R = crate::FieldReader;
#[doc = "Field `SHARP_THRESHOLD_LOW` writer - this field configures sharpen threshold for detail"]
pub type SHARP_THRESHOLD_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_THRESHOLD_HIGH` reader - this field configures sharpen threshold for edge"]
pub type SHARP_THRESHOLD_HIGH_R = crate::FieldReader;
#[doc = "Field `SHARP_THRESHOLD_HIGH` writer - this field configures sharpen threshold for edge"]
pub type SHARP_THRESHOLD_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_AMOUNT_LOW` reader - this field configures sharpen amount for detail"]
pub type SHARP_AMOUNT_LOW_R = crate::FieldReader;
#[doc = "Field `SHARP_AMOUNT_LOW` writer - this field configures sharpen amount for detail"]
pub type SHARP_AMOUNT_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SHARP_AMOUNT_HIGH` reader - this field configures sharpen amount for edge"]
pub type SHARP_AMOUNT_HIGH_R = crate::FieldReader;
#[doc = "Field `SHARP_AMOUNT_HIGH` writer - this field configures sharpen amount for edge"]
pub type SHARP_AMOUNT_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures sharpen threshold for detail"]
    #[inline(always)]
    pub fn sharp_threshold_low(&self) -> SHARP_THRESHOLD_LOW_R {
        SHARP_THRESHOLD_LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures sharpen threshold for edge"]
    #[inline(always)]
    pub fn sharp_threshold_high(&self) -> SHARP_THRESHOLD_HIGH_R {
        SHARP_THRESHOLD_HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures sharpen amount for detail"]
    #[inline(always)]
    pub fn sharp_amount_low(&self) -> SHARP_AMOUNT_LOW_R {
        SHARP_AMOUNT_LOW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures sharpen amount for edge"]
    #[inline(always)]
    pub fn sharp_amount_high(&self) -> SHARP_AMOUNT_HIGH_R {
        SHARP_AMOUNT_HIGH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHARP_CTRL0")
            .field(
                "sharp_threshold_low",
                &format_args!("{}", self.sharp_threshold_low().bits()),
            )
            .field(
                "sharp_threshold_high",
                &format_args!("{}", self.sharp_threshold_high().bits()),
            )
            .field(
                "sharp_amount_low",
                &format_args!("{}", self.sharp_amount_low().bits()),
            )
            .field(
                "sharp_amount_high",
                &format_args!("{}", self.sharp_amount_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SHARP_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures sharpen threshold for detail"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_threshold_low(&mut self) -> SHARP_THRESHOLD_LOW_W<SHARP_CTRL0_SPEC> {
        SHARP_THRESHOLD_LOW_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures sharpen threshold for edge"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_threshold_high(&mut self) -> SHARP_THRESHOLD_HIGH_W<SHARP_CTRL0_SPEC> {
        SHARP_THRESHOLD_HIGH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures sharpen amount for detail"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_amount_low(&mut self) -> SHARP_AMOUNT_LOW_W<SHARP_CTRL0_SPEC> {
        SHARP_AMOUNT_LOW_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures sharpen amount for edge"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_amount_high(&mut self) -> SHARP_AMOUNT_HIGH_W<SHARP_CTRL0_SPEC> {
        SHARP_AMOUNT_HIGH_W::new(self, 24)
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
#[doc = "sharp control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sharp_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHARP_CTRL0_SPEC;
impl crate::RegisterSpec for SHARP_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharp_ctrl0::R`](R) reader structure"]
impl crate::Readable for SHARP_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sharp_ctrl0::W`](W) writer structure"]
impl crate::Writable for SHARP_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHARP_CTRL0 to value 0"]
impl crate::Resettable for SHARP_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
