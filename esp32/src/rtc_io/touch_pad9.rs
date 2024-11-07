#[doc = "Register `TOUCH_PAD9` reader"]
pub type R = crate::R<TOUCH_PAD9_SPEC>;
#[doc = "Register `TOUCH_PAD9` writer"]
pub type W = crate::W<TOUCH_PAD9_SPEC>;
#[doc = "Field `TO_GPIO` reader - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
pub type TO_GPIO_R = crate::BitReader;
#[doc = "Field `TO_GPIO` writer - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
pub type TO_GPIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD` reader - touch sensor power on."]
pub type XPD_R = crate::BitReader;
#[doc = "Field `XPD` writer - touch sensor power on."]
pub type XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE_OPT` reader - default touch sensor tie option. 0: tie low 1: tie high."]
pub type TIE_OPT_R = crate::BitReader;
#[doc = "Field `TIE_OPT` writer - default touch sensor tie option. 0: tie low 1: tie high."]
pub type TIE_OPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - start touch sensor."]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - start touch sensor."]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC` reader - touch sensor slope control. 3-bit for each touch panel default 100."]
pub type DAC_R = crate::FieldReader;
#[doc = "Field `DAC` writer - touch sensor slope control. 3-bit for each touch panel default 100."]
pub type DAC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
    #[inline(always)]
    pub fn to_gpio(&self) -> TO_GPIO_R {
        TO_GPIO_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&self) -> TIE_OPT_R {
        TIE_OPT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn dac(&self) -> DAC_R {
        DAC_R::new(((self.bits >> 23) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD9")
            .field("to_gpio", &self.to_gpio())
            .field("xpd", &self.xpd())
            .field("tie_opt", &self.tie_opt())
            .field("start", &self.start())
            .field("dac", &self.dac())
            .finish()
    }
}
impl W {
    #[doc = "Bit 19 - connect the rtc pad input to digital pad input Ó0Ó is availbale"]
    #[inline(always)]
    pub fn to_gpio(&mut self) -> TO_GPIO_W<TOUCH_PAD9_SPEC> {
        TO_GPIO_W::new(self, 19)
    }
    #[doc = "Bit 20 - touch sensor power on."]
    #[inline(always)]
    pub fn xpd(&mut self) -> XPD_W<TOUCH_PAD9_SPEC> {
        XPD_W::new(self, 20)
    }
    #[doc = "Bit 21 - default touch sensor tie option. 0: tie low 1: tie high."]
    #[inline(always)]
    pub fn tie_opt(&mut self) -> TIE_OPT_W<TOUCH_PAD9_SPEC> {
        TIE_OPT_W::new(self, 21)
    }
    #[doc = "Bit 22 - start touch sensor."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<TOUCH_PAD9_SPEC> {
        START_W::new(self, 22)
    }
    #[doc = "Bits 23:25 - touch sensor slope control. 3-bit for each touch panel default 100."]
    #[inline(always)]
    pub fn dac(&mut self) -> DAC_W<TOUCH_PAD9_SPEC> {
        DAC_W::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pad9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pad9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_PAD9_SPEC;
impl crate::RegisterSpec for TOUCH_PAD9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_pad9::R`](R) reader structure"]
impl crate::Readable for TOUCH_PAD9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_pad9::W`](W) writer structure"]
impl crate::Writable for TOUCH_PAD9_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_PAD9 to value 0x0200_0000"]
impl crate::Resettable for TOUCH_PAD9_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
