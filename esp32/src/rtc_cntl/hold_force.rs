#[doc = "Register `HOLD_FORCE` reader"]
pub type R = crate::R<HOLD_FORCE_SPEC>;
#[doc = "Register `HOLD_FORCE` writer"]
pub type W = crate::W<HOLD_FORCE_SPEC>;
#[doc = "Field `ADC1_HOLD_FORCE` reader - "]
pub type ADC1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `ADC1_HOLD_FORCE` writer - "]
pub type ADC1_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_HOLD_FORCE` reader - "]
pub type ADC2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `ADC2_HOLD_FORCE` writer - "]
pub type ADC2_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1_HOLD_FORCE` reader - "]
pub type PDAC1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC1_HOLD_FORCE` writer - "]
pub type PDAC1_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2_HOLD_FORCE` reader - "]
pub type PDAC2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `PDAC2_HOLD_FORCE` writer - "]
pub type PDAC2_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE1_HOLD_FORCE` reader - "]
pub type SENSE1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE1_HOLD_FORCE` writer - "]
pub type SENSE1_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE2_HOLD_FORCE` reader - "]
pub type SENSE2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE2_HOLD_FORCE` writer - "]
pub type SENSE2_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE3_HOLD_FORCE` reader - "]
pub type SENSE3_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE3_HOLD_FORCE` writer - "]
pub type SENSE3_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE4_HOLD_FORCE` reader - "]
pub type SENSE4_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `SENSE4_HOLD_FORCE` writer - "]
pub type SENSE4_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD0_HOLD_FORCE` reader - "]
pub type TOUCH_PAD0_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0_HOLD_FORCE` writer - "]
pub type TOUCH_PAD0_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD1_HOLD_FORCE` reader - "]
pub type TOUCH_PAD1_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD1_HOLD_FORCE` writer - "]
pub type TOUCH_PAD1_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD2_HOLD_FORCE` reader - "]
pub type TOUCH_PAD2_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD2_HOLD_FORCE` writer - "]
pub type TOUCH_PAD2_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD3_HOLD_FORCE` reader - "]
pub type TOUCH_PAD3_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD3_HOLD_FORCE` writer - "]
pub type TOUCH_PAD3_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD4_HOLD_FORCE` reader - "]
pub type TOUCH_PAD4_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD4_HOLD_FORCE` writer - "]
pub type TOUCH_PAD4_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD5_HOLD_FORCE` reader - "]
pub type TOUCH_PAD5_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD5_HOLD_FORCE` writer - "]
pub type TOUCH_PAD5_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD6_HOLD_FORCE` reader - "]
pub type TOUCH_PAD6_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD6_HOLD_FORCE` writer - "]
pub type TOUCH_PAD6_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD7_HOLD_FORCE` reader - "]
pub type TOUCH_PAD7_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD7_HOLD_FORCE` writer - "]
pub type TOUCH_PAD7_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32P_HOLD_FORCE` reader - "]
pub type X32P_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `X32P_HOLD_FORCE` writer - "]
pub type X32P_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N_HOLD_FORCE` reader - "]
pub type X32N_HOLD_FORCE_R = crate::BitReader;
#[doc = "Field `X32N_HOLD_FORCE` writer - "]
pub type X32N_HOLD_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_hold_force(&self) -> ADC1_HOLD_FORCE_R {
        ADC1_HOLD_FORCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_hold_force(&self) -> ADC2_HOLD_FORCE_R {
        ADC2_HOLD_FORCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1_hold_force(&self) -> PDAC1_HOLD_FORCE_R {
        PDAC1_HOLD_FORCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2_hold_force(&self) -> PDAC2_HOLD_FORCE_R {
        PDAC2_HOLD_FORCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1_hold_force(&self) -> SENSE1_HOLD_FORCE_R {
        SENSE1_HOLD_FORCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2_hold_force(&self) -> SENSE2_HOLD_FORCE_R {
        SENSE2_HOLD_FORCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3_hold_force(&self) -> SENSE3_HOLD_FORCE_R {
        SENSE3_HOLD_FORCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4_hold_force(&self) -> SENSE4_HOLD_FORCE_R {
        SENSE4_HOLD_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0_hold_force(&self) -> TOUCH_PAD0_HOLD_FORCE_R {
        TOUCH_PAD0_HOLD_FORCE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1_hold_force(&self) -> TOUCH_PAD1_HOLD_FORCE_R {
        TOUCH_PAD1_HOLD_FORCE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2_hold_force(&self) -> TOUCH_PAD2_HOLD_FORCE_R {
        TOUCH_PAD2_HOLD_FORCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3_hold_force(&self) -> TOUCH_PAD3_HOLD_FORCE_R {
        TOUCH_PAD3_HOLD_FORCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4_hold_force(&self) -> TOUCH_PAD4_HOLD_FORCE_R {
        TOUCH_PAD4_HOLD_FORCE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5_hold_force(&self) -> TOUCH_PAD5_HOLD_FORCE_R {
        TOUCH_PAD5_HOLD_FORCE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6_hold_force(&self) -> TOUCH_PAD6_HOLD_FORCE_R {
        TOUCH_PAD6_HOLD_FORCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7_hold_force(&self) -> TOUCH_PAD7_HOLD_FORCE_R {
        TOUCH_PAD7_HOLD_FORCE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p_hold_force(&self) -> X32P_HOLD_FORCE_R {
        X32P_HOLD_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n_hold_force(&self) -> X32N_HOLD_FORCE_R {
        X32N_HOLD_FORCE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOLD_FORCE")
            .field("adc1_hold_force", &self.adc1_hold_force())
            .field("adc2_hold_force", &self.adc2_hold_force())
            .field("pdac1_hold_force", &self.pdac1_hold_force())
            .field("pdac2_hold_force", &self.pdac2_hold_force())
            .field("sense1_hold_force", &self.sense1_hold_force())
            .field("sense2_hold_force", &self.sense2_hold_force())
            .field("sense3_hold_force", &self.sense3_hold_force())
            .field("sense4_hold_force", &self.sense4_hold_force())
            .field("touch_pad0_hold_force", &self.touch_pad0_hold_force())
            .field("touch_pad1_hold_force", &self.touch_pad1_hold_force())
            .field("touch_pad2_hold_force", &self.touch_pad2_hold_force())
            .field("touch_pad3_hold_force", &self.touch_pad3_hold_force())
            .field("touch_pad4_hold_force", &self.touch_pad4_hold_force())
            .field("touch_pad5_hold_force", &self.touch_pad5_hold_force())
            .field("touch_pad6_hold_force", &self.touch_pad6_hold_force())
            .field("touch_pad7_hold_force", &self.touch_pad7_hold_force())
            .field("x32p_hold_force", &self.x32p_hold_force())
            .field("x32n_hold_force", &self.x32n_hold_force())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_hold_force(&mut self) -> ADC1_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        ADC1_HOLD_FORCE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_hold_force(&mut self) -> ADC2_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        ADC2_HOLD_FORCE_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn pdac1_hold_force(&mut self) -> PDAC1_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        PDAC1_HOLD_FORCE_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn pdac2_hold_force(&mut self) -> PDAC2_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        PDAC2_HOLD_FORCE_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sense1_hold_force(&mut self) -> SENSE1_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        SENSE1_HOLD_FORCE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sense2_hold_force(&mut self) -> SENSE2_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        SENSE2_HOLD_FORCE_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sense3_hold_force(&mut self) -> SENSE3_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        SENSE3_HOLD_FORCE_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sense4_hold_force(&mut self) -> SENSE4_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        SENSE4_HOLD_FORCE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_hold_force(&mut self) -> TOUCH_PAD0_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD0_HOLD_FORCE_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad1_hold_force(&mut self) -> TOUCH_PAD1_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD1_HOLD_FORCE_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad2_hold_force(&mut self) -> TOUCH_PAD2_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD2_HOLD_FORCE_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad3_hold_force(&mut self) -> TOUCH_PAD3_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD3_HOLD_FORCE_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad4_hold_force(&mut self) -> TOUCH_PAD4_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD4_HOLD_FORCE_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad5_hold_force(&mut self) -> TOUCH_PAD5_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD5_HOLD_FORCE_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad6_hold_force(&mut self) -> TOUCH_PAD6_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD6_HOLD_FORCE_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad7_hold_force(&mut self) -> TOUCH_PAD7_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        TOUCH_PAD7_HOLD_FORCE_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn x32p_hold_force(&mut self) -> X32P_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        X32P_HOLD_FORCE_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn x32n_hold_force(&mut self) -> X32N_HOLD_FORCE_W<HOLD_FORCE_SPEC> {
        X32N_HOLD_FORCE_W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hold_force::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hold_force::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOLD_FORCE_SPEC;
impl crate::RegisterSpec for HOLD_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hold_force::R`](R) reader structure"]
impl crate::Readable for HOLD_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hold_force::W`](W) writer structure"]
impl crate::Writable for HOLD_FORCE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOLD_FORCE to value 0"]
impl crate::Resettable for HOLD_FORCE_SPEC {
    const RESET_VALUE: u32 = 0;
}
