#[doc = "Register `CARRIER0_CFG` reader"]
pub type R = crate::R<CARRIER0_CFG_SPEC>;
#[doc = "Register `CARRIER0_CFG` writer"]
pub type W = crate::W<CARRIER0_CFG_SPEC>;
#[doc = "Field `CARRIER0_EN` reader - "]
pub type CARRIER0_EN_R = crate::BitReader;
#[doc = "Field `CARRIER0_EN` writer - "]
pub type CARRIER0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER0_PRESCALE` reader - "]
pub type CARRIER0_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CARRIER0_PRESCALE` writer - "]
pub type CARRIER0_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CARRIER0_DUTY` reader - "]
pub type CARRIER0_DUTY_R = crate::FieldReader;
#[doc = "Field `CARRIER0_DUTY` writer - "]
pub type CARRIER0_DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CARRIER0_OSHTWTH` reader - "]
pub type CARRIER0_OSHTWTH_R = crate::FieldReader;
#[doc = "Field `CARRIER0_OSHTWTH` writer - "]
pub type CARRIER0_OSHTWTH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CARRIER0_OUT_INVERT` reader - "]
pub type CARRIER0_OUT_INVERT_R = crate::BitReader;
#[doc = "Field `CARRIER0_OUT_INVERT` writer - "]
pub type CARRIER0_OUT_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARRIER0_IN_INVERT` reader - "]
pub type CARRIER0_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CARRIER0_IN_INVERT` writer - "]
pub type CARRIER0_IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn carrier0_en(&self) -> CARRIER0_EN_R {
        CARRIER0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    pub fn carrier0_prescale(&self) -> CARRIER0_PRESCALE_R {
        CARRIER0_PRESCALE_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    pub fn carrier0_duty(&self) -> CARRIER0_DUTY_R {
        CARRIER0_DUTY_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn carrier0_oshtwth(&self) -> CARRIER0_OSHTWTH_R {
        CARRIER0_OSHTWTH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn carrier0_out_invert(&self) -> CARRIER0_OUT_INVERT_R {
        CARRIER0_OUT_INVERT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn carrier0_in_invert(&self) -> CARRIER0_IN_INVERT_R {
        CARRIER0_IN_INVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CARRIER0_CFG")
            .field("carrier0_en", &format_args!("{}", self.carrier0_en().bit()))
            .field(
                "carrier0_prescale",
                &format_args!("{}", self.carrier0_prescale().bits()),
            )
            .field(
                "carrier0_duty",
                &format_args!("{}", self.carrier0_duty().bits()),
            )
            .field(
                "carrier0_oshtwth",
                &format_args!("{}", self.carrier0_oshtwth().bits()),
            )
            .field(
                "carrier0_out_invert",
                &format_args!("{}", self.carrier0_out_invert().bit()),
            )
            .field(
                "carrier0_in_invert",
                &format_args!("{}", self.carrier0_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CARRIER0_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn carrier0_en(&mut self) -> CARRIER0_EN_W<CARRIER0_CFG_SPEC> {
        CARRIER0_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:4"]
    #[inline(always)]
    #[must_use]
    pub fn carrier0_prescale(&mut self) -> CARRIER0_PRESCALE_W<CARRIER0_CFG_SPEC> {
        CARRIER0_PRESCALE_W::new(self, 1)
    }
    #[doc = "Bits 5:7"]
    #[inline(always)]
    #[must_use]
    pub fn carrier0_duty(&mut self) -> CARRIER0_DUTY_W<CARRIER0_CFG_SPEC> {
        CARRIER0_DUTY_W::new(self, 5)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn carrier0_oshtwth(&mut self) -> CARRIER0_OSHTWTH_W<CARRIER0_CFG_SPEC> {
        CARRIER0_OSHTWTH_W::new(self, 8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn carrier0_out_invert(&mut self) -> CARRIER0_OUT_INVERT_W<CARRIER0_CFG_SPEC> {
        CARRIER0_OUT_INVERT_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn carrier0_in_invert(&mut self) -> CARRIER0_IN_INVERT_W<CARRIER0_CFG_SPEC> {
        CARRIER0_IN_INVERT_W::new(self, 13)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier0_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier0_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CARRIER0_CFG_SPEC;
impl crate::RegisterSpec for CARRIER0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`carrier0_cfg::R`](R) reader structure"]
impl crate::Readable for CARRIER0_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`carrier0_cfg::W`](W) writer structure"]
impl crate::Writable for CARRIER0_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CARRIER0_CFG to value 0"]
impl crate::Resettable for CARRIER0_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
