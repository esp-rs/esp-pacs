#[doc = "Register `HOLD_FORCE` reader"]
pub type R = crate::R<HOLD_FORCE_SPEC>;
#[doc = "Register `HOLD_FORCE` writer"]
pub type W = crate::W<HOLD_FORCE_SPEC>;
#[doc = "Field `ADC1` reader - "]
pub type ADC1_R = crate::BitReader;
#[doc = "Field `ADC1` writer - "]
pub type ADC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2` reader - "]
pub type ADC2_R = crate::BitReader;
#[doc = "Field `ADC2` writer - "]
pub type ADC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC1` reader - "]
pub type PDAC1_R = crate::BitReader;
#[doc = "Field `PDAC1` writer - "]
pub type PDAC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDAC2` reader - "]
pub type PDAC2_R = crate::BitReader;
#[doc = "Field `PDAC2` writer - "]
pub type PDAC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE1` reader - "]
pub type SENSE1_R = crate::BitReader;
#[doc = "Field `SENSE1` writer - "]
pub type SENSE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE2` reader - "]
pub type SENSE2_R = crate::BitReader;
#[doc = "Field `SENSE2` writer - "]
pub type SENSE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE3` reader - "]
pub type SENSE3_R = crate::BitReader;
#[doc = "Field `SENSE3` writer - "]
pub type SENSE3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENSE4` reader - "]
pub type SENSE4_R = crate::BitReader;
#[doc = "Field `SENSE4` writer - "]
pub type SENSE4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD0` reader - "]
pub type TOUCH_PAD0_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD0` writer - "]
pub type TOUCH_PAD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD1` reader - "]
pub type TOUCH_PAD1_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD1` writer - "]
pub type TOUCH_PAD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD2` reader - "]
pub type TOUCH_PAD2_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD2` writer - "]
pub type TOUCH_PAD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD3` reader - "]
pub type TOUCH_PAD3_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD3` writer - "]
pub type TOUCH_PAD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD4` reader - "]
pub type TOUCH_PAD4_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD4` writer - "]
pub type TOUCH_PAD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD5` reader - "]
pub type TOUCH_PAD5_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD5` writer - "]
pub type TOUCH_PAD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD6` reader - "]
pub type TOUCH_PAD6_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD6` writer - "]
pub type TOUCH_PAD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_PAD7` reader - "]
pub type TOUCH_PAD7_R = crate::BitReader;
#[doc = "Field `TOUCH_PAD7` writer - "]
pub type TOUCH_PAD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32P` reader - "]
pub type X32P_R = crate::BitReader;
#[doc = "Field `X32P` writer - "]
pub type X32P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `X32N` reader - "]
pub type X32N_R = crate::BitReader;
#[doc = "Field `X32N` writer - "]
pub type X32N_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2(&self) -> ADC2_R {
        ADC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1(&self) -> PDAC1_R {
        PDAC1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2(&self) -> PDAC2_R {
        PDAC2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1(&self) -> SENSE1_R {
        SENSE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2(&self) -> SENSE2_R {
        SENSE2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3(&self) -> SENSE3_R {
        SENSE3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4(&self) -> SENSE4_R {
        SENSE4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0(&self) -> TOUCH_PAD0_R {
        TOUCH_PAD0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1(&self) -> TOUCH_PAD1_R {
        TOUCH_PAD1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2(&self) -> TOUCH_PAD2_R {
        TOUCH_PAD2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3(&self) -> TOUCH_PAD3_R {
        TOUCH_PAD3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4(&self) -> TOUCH_PAD4_R {
        TOUCH_PAD4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5(&self) -> TOUCH_PAD5_R {
        TOUCH_PAD5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6(&self) -> TOUCH_PAD6_R {
        TOUCH_PAD6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7(&self) -> TOUCH_PAD7_R {
        TOUCH_PAD7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p(&self) -> X32P_R {
        X32P_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n(&self) -> X32N_R {
        X32N_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOLD_FORCE")
            .field("adc1", &self.adc1())
            .field("adc2", &self.adc2())
            .field("pdac1", &self.pdac1())
            .field("pdac2", &self.pdac2())
            .field("sense1", &self.sense1())
            .field("sense2", &self.sense2())
            .field("sense3", &self.sense3())
            .field("sense4", &self.sense4())
            .field("touch_pad0", &self.touch_pad0())
            .field("touch_pad1", &self.touch_pad1())
            .field("touch_pad2", &self.touch_pad2())
            .field("touch_pad3", &self.touch_pad3())
            .field("touch_pad4", &self.touch_pad4())
            .field("touch_pad5", &self.touch_pad5())
            .field("touch_pad6", &self.touch_pad6())
            .field("touch_pad7", &self.touch_pad7())
            .field("x32p", &self.x32p())
            .field("x32n", &self.x32n())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1(&mut self) -> ADC1_W<'_, HOLD_FORCE_SPEC> {
        ADC1_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2(&mut self) -> ADC2_W<'_, HOLD_FORCE_SPEC> {
        ADC2_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1(&mut self) -> PDAC1_W<'_, HOLD_FORCE_SPEC> {
        PDAC1_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2(&mut self) -> PDAC2_W<'_, HOLD_FORCE_SPEC> {
        PDAC2_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1(&mut self) -> SENSE1_W<'_, HOLD_FORCE_SPEC> {
        SENSE1_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2(&mut self) -> SENSE2_W<'_, HOLD_FORCE_SPEC> {
        SENSE2_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3(&mut self) -> SENSE3_W<'_, HOLD_FORCE_SPEC> {
        SENSE3_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4(&mut self) -> SENSE4_W<'_, HOLD_FORCE_SPEC> {
        SENSE4_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0(&mut self) -> TOUCH_PAD0_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD0_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1(&mut self) -> TOUCH_PAD1_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD1_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2(&mut self) -> TOUCH_PAD2_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD2_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3(&mut self) -> TOUCH_PAD3_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD3_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4(&mut self) -> TOUCH_PAD4_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD4_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5(&mut self) -> TOUCH_PAD5_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD5_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6(&mut self) -> TOUCH_PAD6_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD6_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7(&mut self) -> TOUCH_PAD7_W<'_, HOLD_FORCE_SPEC> {
        TOUCH_PAD7_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p(&mut self) -> X32P_W<'_, HOLD_FORCE_SPEC> {
        X32P_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n(&mut self) -> X32N_W<'_, HOLD_FORCE_SPEC> {
        X32N_W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hold_force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hold_force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOLD_FORCE_SPEC;
impl crate::RegisterSpec for HOLD_FORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hold_force::R`](R) reader structure"]
impl crate::Readable for HOLD_FORCE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hold_force::W`](W) writer structure"]
impl crate::Writable for HOLD_FORCE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOLD_FORCE to value 0"]
impl crate::Resettable for HOLD_FORCE_SPEC {}
