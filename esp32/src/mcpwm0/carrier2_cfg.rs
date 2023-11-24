#[doc = "Register `CARRIER2_CFG` reader"]
pub type R = crate::R<CARRIER2_CFG_SPEC>;
#[doc = "Register `CARRIER2_CFG` writer"]
pub type W = crate::W<CARRIER2_CFG_SPEC>;
#[doc = "Field `CARRIER2_EN` reader - "]
pub type CARRIER2_EN_R = crate::BitReader;
#[doc = "Field `CARRIER2_EN` writer - "]
pub type CARRIER2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER2_PRESCALE` reader - "]
pub type CARRIER2_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CARRIER2_PRESCALE` writer - "]
pub type CARRIER2_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CARRIER2_DUTY` reader - "]
pub type CARRIER2_DUTY_R = crate::FieldReader;
#[doc = "Field `CARRIER2_DUTY` writer - "]
pub type CARRIER2_DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER2_OSHTWTH` reader - "]
pub type CARRIER2_OSHTWTH_R = crate::FieldReader;
#[doc = "Field `CARRIER2_OSHTWTH` writer - "]
pub type CARRIER2_OSHTWTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CARRIER2_OUT_INVERT` reader - "]
pub type CARRIER2_OUT_INVERT_R = crate::BitReader;
#[doc = "Field `CARRIER2_OUT_INVERT` writer - "]
pub type CARRIER2_OUT_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER2_IN_INVERT` reader - "]
pub type CARRIER2_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CARRIER2_IN_INVERT` writer - "]
pub type CARRIER2_IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn carrier2_en(&self) -> CARRIER2_EN_R {
        CARRIER2_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn carrier2_prescale(&self) -> CARRIER2_PRESCALE_R {
        CARRIER2_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn carrier2_duty(&self) -> CARRIER2_DUTY_R {
        CARRIER2_DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn carrier2_oshtwth(&self) -> CARRIER2_OSHTWTH_R {
        CARRIER2_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn carrier2_out_invert(&self) -> CARRIER2_OUT_INVERT_R {
        CARRIER2_OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn carrier2_in_invert(&self) -> CARRIER2_IN_INVERT_R {
        CARRIER2_IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER2_CFG")
            .field("carrier2_en", &format_args!("{}", self.carrier2_en().bit()))
            .field(
                "carrier2_prescale",
                &format_args!("{}", self.carrier2_prescale().bits()),
            )
            .field(
                "carrier2_duty",
                &format_args!("{}", self.carrier2_duty().bits()),
            )
            .field(
                "carrier2_oshtwth",
                &format_args!("{}", self.carrier2_oshtwth().bits()),
            )
            .field(
                "carrier2_out_invert",
                &format_args!("{}", self.carrier2_out_invert().bit()),
            )
            .field(
                "carrier2_in_invert",
                &format_args!("{}", self.carrier2_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CARRIER2_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn carrier2_en(&mut self) -> CARRIER2_EN_W<CARRIER2_CFG_SPEC> {
        CARRIER2_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    #[must_use]
    pub fn carrier2_prescale(&mut self) -> CARRIER2_PRESCALE_W<CARRIER2_CFG_SPEC> {
        CARRIER2_PRESCALE_W::new(self, 1)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn carrier2_duty(&mut self) -> CARRIER2_DUTY_W<CARRIER2_CFG_SPEC> {
        CARRIER2_DUTY_W::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn carrier2_oshtwth(&mut self) -> CARRIER2_OSHTWTH_W<CARRIER2_CFG_SPEC> {
        CARRIER2_OSHTWTH_W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn carrier2_out_invert(&mut self) -> CARRIER2_OUT_INVERT_W<CARRIER2_CFG_SPEC> {
        CARRIER2_OUT_INVERT_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn carrier2_in_invert(&mut self) -> CARRIER2_IN_INVERT_W<CARRIER2_CFG_SPEC> {
        CARRIER2_IN_INVERT_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier2_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier2_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER2_CFG_SPEC;
impl crate::RegisterSpec for CARRIER2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier2_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER2_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier2_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER2_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CARRIER2_CFG to value 0"]
impl crate::Resettable for CARRIER2_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
