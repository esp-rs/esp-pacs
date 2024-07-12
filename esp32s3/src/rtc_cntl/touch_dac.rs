#[doc = "Register `TOUCH_DAC` reader"]
pub type R = crate::R<TOUCH_DAC_SPEC>;
#[doc = "Register `TOUCH_DAC` writer"]
pub type W = crate::W<TOUCH_DAC_SPEC>;
#[doc = "Field `TOUCH_PAD9_DAC` reader - configure touch pad dac9"]
pub type TOUCH_PAD9_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD9_DAC` writer - configure touch pad dac9"]
pub type TOUCH_PAD9_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD8_DAC` reader - configure touch pad dac8"]
pub type TOUCH_PAD8_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD8_DAC` writer - configure touch pad dac8"]
pub type TOUCH_PAD8_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD7_DAC` reader - configure touch pad dac7"]
pub type TOUCH_PAD7_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD7_DAC` writer - configure touch pad dac7"]
pub type TOUCH_PAD7_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD6_DAC` reader - configure touch pad dac6"]
pub type TOUCH_PAD6_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD6_DAC` writer - configure touch pad dac6"]
pub type TOUCH_PAD6_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD5_DAC` reader - configure touch pad dac5"]
pub type TOUCH_PAD5_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD5_DAC` writer - configure touch pad dac5"]
pub type TOUCH_PAD5_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD4_DAC` reader - configure touch pad dac4"]
pub type TOUCH_PAD4_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD4_DAC` writer - configure touch pad dac4"]
pub type TOUCH_PAD4_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD3_DAC` reader - configure touch pad dac3"]
pub type TOUCH_PAD3_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD3_DAC` writer - configure touch pad dac3"]
pub type TOUCH_PAD3_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD2_DAC` reader - configure touch pad dac2"]
pub type TOUCH_PAD2_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD2_DAC` writer - configure touch pad dac2"]
pub type TOUCH_PAD2_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD1_DAC` reader - configure touch pad dac1"]
pub type TOUCH_PAD1_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD1_DAC` writer - configure touch pad dac1"]
pub type TOUCH_PAD1_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_PAD0_DAC` reader - configure touch pad dac0"]
pub type TOUCH_PAD0_DAC_R = crate::FieldReader;
#[doc = "Field `TOUCH_PAD0_DAC` writer - configure touch pad dac0"]
pub type TOUCH_PAD0_DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 2:4 - configure touch pad dac9"]
    #[inline(always)]
    pub fn touch_pad9_dac(&self) -> TOUCH_PAD9_DAC_R {
        TOUCH_PAD9_DAC_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - configure touch pad dac8"]
    #[inline(always)]
    pub fn touch_pad8_dac(&self) -> TOUCH_PAD8_DAC_R {
        TOUCH_PAD8_DAC_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - configure touch pad dac7"]
    #[inline(always)]
    pub fn touch_pad7_dac(&self) -> TOUCH_PAD7_DAC_R {
        TOUCH_PAD7_DAC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - configure touch pad dac6"]
    #[inline(always)]
    pub fn touch_pad6_dac(&self) -> TOUCH_PAD6_DAC_R {
        TOUCH_PAD6_DAC_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - configure touch pad dac5"]
    #[inline(always)]
    pub fn touch_pad5_dac(&self) -> TOUCH_PAD5_DAC_R {
        TOUCH_PAD5_DAC_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - configure touch pad dac4"]
    #[inline(always)]
    pub fn touch_pad4_dac(&self) -> TOUCH_PAD4_DAC_R {
        TOUCH_PAD4_DAC_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - configure touch pad dac3"]
    #[inline(always)]
    pub fn touch_pad3_dac(&self) -> TOUCH_PAD3_DAC_R {
        TOUCH_PAD3_DAC_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - configure touch pad dac2"]
    #[inline(always)]
    pub fn touch_pad2_dac(&self) -> TOUCH_PAD2_DAC_R {
        TOUCH_PAD2_DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - configure touch pad dac1"]
    #[inline(always)]
    pub fn touch_pad1_dac(&self) -> TOUCH_PAD1_DAC_R {
        TOUCH_PAD1_DAC_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - configure touch pad dac0"]
    #[inline(always)]
    pub fn touch_pad0_dac(&self) -> TOUCH_PAD0_DAC_R {
        TOUCH_PAD0_DAC_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_DAC")
            .field("touch_pad9_dac", &self.touch_pad9_dac())
            .field("touch_pad8_dac", &self.touch_pad8_dac())
            .field("touch_pad7_dac", &self.touch_pad7_dac())
            .field("touch_pad6_dac", &self.touch_pad6_dac())
            .field("touch_pad5_dac", &self.touch_pad5_dac())
            .field("touch_pad4_dac", &self.touch_pad4_dac())
            .field("touch_pad3_dac", &self.touch_pad3_dac())
            .field("touch_pad2_dac", &self.touch_pad2_dac())
            .field("touch_pad1_dac", &self.touch_pad1_dac())
            .field("touch_pad0_dac", &self.touch_pad0_dac())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:4 - configure touch pad dac9"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad9_dac(&mut self) -> TOUCH_PAD9_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD9_DAC_W::new(self, 2)
    }
    #[doc = "Bits 5:7 - configure touch pad dac8"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad8_dac(&mut self) -> TOUCH_PAD8_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD8_DAC_W::new(self, 5)
    }
    #[doc = "Bits 8:10 - configure touch pad dac7"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad7_dac(&mut self) -> TOUCH_PAD7_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD7_DAC_W::new(self, 8)
    }
    #[doc = "Bits 11:13 - configure touch pad dac6"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad6_dac(&mut self) -> TOUCH_PAD6_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD6_DAC_W::new(self, 11)
    }
    #[doc = "Bits 14:16 - configure touch pad dac5"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad5_dac(&mut self) -> TOUCH_PAD5_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD5_DAC_W::new(self, 14)
    }
    #[doc = "Bits 17:19 - configure touch pad dac4"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad4_dac(&mut self) -> TOUCH_PAD4_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD4_DAC_W::new(self, 17)
    }
    #[doc = "Bits 20:22 - configure touch pad dac3"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad3_dac(&mut self) -> TOUCH_PAD3_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD3_DAC_W::new(self, 20)
    }
    #[doc = "Bits 23:25 - configure touch pad dac2"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad2_dac(&mut self) -> TOUCH_PAD2_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD2_DAC_W::new(self, 23)
    }
    #[doc = "Bits 26:28 - configure touch pad dac1"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad1_dac(&mut self) -> TOUCH_PAD1_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD1_DAC_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - configure touch pad dac0"]
    #[inline(always)]
    #[must_use]
    pub fn touch_pad0_dac(&mut self) -> TOUCH_PAD0_DAC_W<TOUCH_DAC_SPEC> {
        TOUCH_PAD0_DAC_W::new(self, 29)
    }
}
#[doc = "configure touch dac\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_dac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_dac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_DAC_SPEC;
impl crate::RegisterSpec for TOUCH_DAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_dac::R`](R) reader structure"]
impl crate::Readable for TOUCH_DAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_dac::W`](W) writer structure"]
impl crate::Writable for TOUCH_DAC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_DAC to value 0"]
impl crate::Resettable for TOUCH_DAC_SPEC {
    const RESET_VALUE: u32 = 0;
}
