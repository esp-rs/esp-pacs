#[doc = "Register `SAR_TOUCH_CONF` reader"]
pub type R = crate::R<SAR_TOUCH_CONF_SPEC>;
#[doc = "Register `SAR_TOUCH_CONF` writer"]
pub type W = crate::W<SAR_TOUCH_CONF_SPEC>;
#[doc = "Field `TOUCH_OUTEN` reader - Enable touch controller output."]
pub type TOUCH_OUTEN_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_OUTEN` writer - Enable touch controller output."]
pub type TOUCH_OUTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TOUCH_STATUS_CLR` writer - Clear all touch active status."]
pub type TOUCH_STATUS_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_DATA_SEL` reader - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
pub type TOUCH_DATA_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_DATA_SEL` writer - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
pub type TOUCH_DATA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DENOISE_END` reader - Touch denoise done."]
pub type TOUCH_DENOISE_END_R = crate::BitReader;
#[doc = "Field `TOUCH_UNIT_END` reader - Indicate the completion of sampling."]
pub type TOUCH_UNIT_END_R = crate::BitReader;
#[doc = "Field `TOUCH_APPROACH_PAD2` reader - Indicate which pad is selected as proximity pad2"]
pub type TOUCH_APPROACH_PAD2_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD2` writer - Indicate which pad is selected as proximity pad2"]
pub type TOUCH_APPROACH_PAD2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_APPROACH_PAD1` reader - Indicate which pad is selected as proximity pad1"]
pub type TOUCH_APPROACH_PAD1_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD1` writer - Indicate which pad is selected as proximity pad1"]
pub type TOUCH_APPROACH_PAD1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TOUCH_APPROACH_PAD0` reader - Indicate which pad is selected as proximity pad0"]
pub type TOUCH_APPROACH_PAD0_R = crate::FieldReader;
#[doc = "Field `TOUCH_APPROACH_PAD0` writer - Indicate which pad is selected as proximity pad0"]
pub type TOUCH_APPROACH_PAD0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Enable touch controller output."]
    #[inline(always)]
    pub fn touch_outen(&self) -> TOUCH_OUTEN_R {
        TOUCH_OUTEN_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:17 - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
    #[inline(always)]
    pub fn touch_data_sel(&self) -> TOUCH_DATA_SEL_R {
        TOUCH_DATA_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Touch denoise done."]
    #[inline(always)]
    pub fn touch_denoise_end(&self) -> TOUCH_DENOISE_END_R {
        TOUCH_DENOISE_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicate the completion of sampling."]
    #[inline(always)]
    pub fn touch_unit_end(&self) -> TOUCH_UNIT_END_R {
        TOUCH_UNIT_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Indicate which pad is selected as proximity pad2"]
    #[inline(always)]
    pub fn touch_approach_pad2(&self) -> TOUCH_APPROACH_PAD2_R {
        TOUCH_APPROACH_PAD2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Indicate which pad is selected as proximity pad1"]
    #[inline(always)]
    pub fn touch_approach_pad1(&self) -> TOUCH_APPROACH_PAD1_R {
        TOUCH_APPROACH_PAD1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Indicate which pad is selected as proximity pad0"]
    #[inline(always)]
    pub fn touch_approach_pad0(&self) -> TOUCH_APPROACH_PAD0_R {
        TOUCH_APPROACH_PAD0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_CONF")
            .field("touch_outen", &self.touch_outen())
            .field("touch_data_sel", &self.touch_data_sel())
            .field("touch_denoise_end", &self.touch_denoise_end())
            .field("touch_unit_end", &self.touch_unit_end())
            .field("touch_approach_pad2", &self.touch_approach_pad2())
            .field("touch_approach_pad1", &self.touch_approach_pad1())
            .field("touch_approach_pad0", &self.touch_approach_pad0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - Enable touch controller output."]
    #[inline(always)]
    pub fn touch_outen(&mut self) -> TOUCH_OUTEN_W<SAR_TOUCH_CONF_SPEC> {
        TOUCH_OUTEN_W::new(self, 0)
    }
    #[doc = "Bit 15 - Clear all touch active status."]
    #[inline(always)]
    pub fn touch_status_clr(&mut self) -> TOUCH_STATUS_CLR_W<SAR_TOUCH_CONF_SPEC> {
        TOUCH_STATUS_CLR_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - 0 and 1: touch_raw_data; 2: base_line; 3: touch_smooth_data."]
    #[inline(always)]
    pub fn touch_data_sel(&mut self) -> TOUCH_DATA_SEL_W<SAR_TOUCH_CONF_SPEC> {
        TOUCH_DATA_SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Indicate which pad is selected as proximity pad2"]
    #[inline(always)]
    pub fn touch_approach_pad2(&mut self) -> TOUCH_APPROACH_PAD2_W<SAR_TOUCH_CONF_SPEC> {
        TOUCH_APPROACH_PAD2_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Indicate which pad is selected as proximity pad1"]
    #[inline(always)]
    pub fn touch_approach_pad1(&mut self) -> TOUCH_APPROACH_PAD1_W<SAR_TOUCH_CONF_SPEC> {
        TOUCH_APPROACH_PAD1_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Indicate which pad is selected as proximity pad0"]
    #[inline(always)]
    pub fn touch_approach_pad0(&mut self) -> TOUCH_APPROACH_PAD0_W<SAR_TOUCH_CONF_SPEC> {
        TOUCH_APPROACH_PAD0_W::new(self, 28)
    }
}
#[doc = "Touch sensor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_touch_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_CONF_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_conf::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_touch_conf::W`](W) writer structure"]
impl crate::Writable for SAR_TOUCH_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_TOUCH_CONF to value 0xfff0_7fff"]
impl crate::Resettable for SAR_TOUCH_CONF_SPEC {
    const RESET_VALUE: u32 = 0xfff0_7fff;
}
