#[doc = "Register `SINTX_CFG` reader"]
pub type R = crate::R<SINTX_CFG_SPEC>;
#[doc = "Register `SINTX_CFG` writer"]
pub type W = crate::W<SINTX_CFG_SPEC>;
#[doc = "Field `SW_TONE` reader - 1:enable software adjusting current sintx output data 0:disable software adjustment"]
pub type SW_TONE_R = crate::BitReader;
#[doc = "Field `SW_TONE` writer - 1:enable software adjusting current sintx output data 0:disable software adjustment"]
pub type SW_TONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_FSTEP` reader - software adjust angle velocity foot step for sintx LUT input"]
pub type SW_FSTEP_R = crate::FieldReader<u16>;
#[doc = "Field `SW_FSTEP` writer - software adjust angle velocity foot step for sintx LUT input"]
pub type SW_FSTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SCALE_1` reader - right shift scaling config for LUT output of sintx for pad 0"]
pub type SCALE_1_R = crate::FieldReader;
#[doc = "Field `SCALE_1` writer - right shift scaling config for LUT output of sintx for pad 0"]
pub type SCALE_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_2` reader - software adjust angle velocity foot step for sintx LUT input"]
pub type SCALE_2_R = crate::FieldReader;
#[doc = "Field `SCALE_2` writer - software adjust angle velocity foot step for sintx LUT input"]
pub type SCALE_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INV_1` reader - bit 1: invert the highest bit of output for pad 0 Bit 0: invert the entire output for pad 0"]
pub type INV_1_R = crate::FieldReader;
#[doc = "Field `INV_1` writer - bit 1: invert the highest bit of output for pad 0 Bit 0: invert the entire output for pad 0"]
pub type INV_1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INV_2` reader - bit 1: invert the highest bit of output for pad 1 Bit 0: invert the entire output for pad 1"]
pub type INV_2_R = crate::FieldReader;
#[doc = "Field `INV_2` writer - bit 1: invert the highest bit of output for pad 1 Bit 0: invert the entire output for pad 1"]
pub type INV_2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 1:enable software adjusting current sintx output data 0:disable software adjustment"]
    #[inline(always)]
    pub fn sw_tone(&self) -> SW_TONE_R {
        SW_TONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - software adjust angle velocity foot step for sintx LUT input"]
    #[inline(always)]
    pub fn sw_fstep(&self) -> SW_FSTEP_R {
        SW_FSTEP_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
    #[doc = "Bits 17:18 - right shift scaling config for LUT output of sintx for pad 0"]
    #[inline(always)]
    pub fn scale_1(&self) -> SCALE_1_R {
        SCALE_1_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - software adjust angle velocity foot step for sintx LUT input"]
    #[inline(always)]
    pub fn scale_2(&self) -> SCALE_2_R {
        SCALE_2_R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:22 - bit 1: invert the highest bit of output for pad 0 Bit 0: invert the entire output for pad 0"]
    #[inline(always)]
    pub fn inv_1(&self) -> INV_1_R {
        INV_1_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - bit 1: invert the highest bit of output for pad 1 Bit 0: invert the entire output for pad 1"]
    #[inline(always)]
    pub fn inv_2(&self) -> INV_2_R {
        INV_2_R::new(((self.bits >> 23) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SINTX_CFG")
            .field("sw_tone", &self.sw_tone())
            .field("sw_fstep", &self.sw_fstep())
            .field("scale_1", &self.scale_1())
            .field("scale_2", &self.scale_2())
            .field("inv_1", &self.inv_1())
            .field("inv_2", &self.inv_2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1:enable software adjusting current sintx output data 0:disable software adjustment"]
    #[inline(always)]
    pub fn sw_tone(&mut self) -> SW_TONE_W<'_, SINTX_CFG_SPEC> {
        SW_TONE_W::new(self, 0)
    }
    #[doc = "Bits 1:16 - software adjust angle velocity foot step for sintx LUT input"]
    #[inline(always)]
    pub fn sw_fstep(&mut self) -> SW_FSTEP_W<'_, SINTX_CFG_SPEC> {
        SW_FSTEP_W::new(self, 1)
    }
    #[doc = "Bits 17:18 - right shift scaling config for LUT output of sintx for pad 0"]
    #[inline(always)]
    pub fn scale_1(&mut self) -> SCALE_1_W<'_, SINTX_CFG_SPEC> {
        SCALE_1_W::new(self, 17)
    }
    #[doc = "Bits 19:20 - software adjust angle velocity foot step for sintx LUT input"]
    #[inline(always)]
    pub fn scale_2(&mut self) -> SCALE_2_W<'_, SINTX_CFG_SPEC> {
        SCALE_2_W::new(self, 19)
    }
    #[doc = "Bits 21:22 - bit 1: invert the highest bit of output for pad 0 Bit 0: invert the entire output for pad 0"]
    #[inline(always)]
    pub fn inv_1(&mut self) -> INV_1_W<'_, SINTX_CFG_SPEC> {
        INV_1_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - bit 1: invert the highest bit of output for pad 1 Bit 0: invert the entire output for pad 1"]
    #[inline(always)]
    pub fn inv_2(&mut self) -> INV_2_W<'_, SINTX_CFG_SPEC> {
        INV_2_W::new(self, 23)
    }
}
#[doc = "dac rstn register\n\nYou can [`read`](crate::Reg::read) this register and get [`sintx_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sintx_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SINTX_CFG_SPEC;
impl crate::RegisterSpec for SINTX_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sintx_cfg::R`](R) reader structure"]
impl crate::Readable for SINTX_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sintx_cfg::W`](W) writer structure"]
impl crate::Writable for SINTX_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINTX_CFG to value 0x02"]
impl crate::Resettable for SINTX_CFG_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
