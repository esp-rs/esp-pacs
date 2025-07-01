#[doc = "Register `FRAME_CFG` reader"]
pub type R = crate::R<FRAME_CFG_SPEC>;
#[doc = "Register `FRAME_CFG` writer"]
pub type W = crate::W<FRAME_CFG_SPEC>;
#[doc = "Field `VADR_NUM` reader - this field configures input image size in y-direction, image row number - 1"]
pub type VADR_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `VADR_NUM` writer - this field configures input image size in y-direction, image row number - 1"]
pub type VADR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HADR_NUM` reader - this field configures input image size in x-direction, image line number - 1"]
pub type HADR_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `HADR_NUM` writer - this field configures input image size in x-direction, image line number - 1"]
pub type HADR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BAYER_MODE` reader - this field configures the bayer mode of input pixel. 00 : BG/GR 01 : GB/RG 10 : GR/BG 11 : RG/GB"]
pub type BAYER_MODE_R = crate::FieldReader;
#[doc = "Field `BAYER_MODE` writer - this field configures the bayer mode of input pixel. 00 : BG/GR 01 : GB/RG 10 : GR/BG 11 : RG/GB"]
pub type BAYER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSYNC_START_EXIST` reader - this bit configures the line end packet exist or not. 0: not exist, 1: exist"]
pub type HSYNC_START_EXIST_R = crate::BitReader;
#[doc = "Field `HSYNC_START_EXIST` writer - this bit configures the line end packet exist or not. 0: not exist, 1: exist"]
pub type HSYNC_START_EXIST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNC_END_EXIST` reader - this bit configures the line start packet exist or not. 0: not exist, 1: exist"]
pub type HSYNC_END_EXIST_R = crate::BitReader;
#[doc = "Field `HSYNC_END_EXIST` writer - this bit configures the line start packet exist or not. 0: not exist, 1: exist"]
pub type HSYNC_END_EXIST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - this field configures input image size in y-direction, image row number - 1"]
    #[inline(always)]
    pub fn vadr_num(&self) -> VADR_NUM_R {
        VADR_NUM_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - this field configures input image size in x-direction, image line number - 1"]
    #[inline(always)]
    pub fn hadr_num(&self) -> HADR_NUM_R {
        HADR_NUM_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bits 27:28 - this field configures the bayer mode of input pixel. 00 : BG/GR 01 : GB/RG 10 : GR/BG 11 : RG/GB"]
    #[inline(always)]
    pub fn bayer_mode(&self) -> BAYER_MODE_R {
        BAYER_MODE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - this bit configures the line end packet exist or not. 0: not exist, 1: exist"]
    #[inline(always)]
    pub fn hsync_start_exist(&self) -> HSYNC_START_EXIST_R {
        HSYNC_START_EXIST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - this bit configures the line start packet exist or not. 0: not exist, 1: exist"]
    #[inline(always)]
    pub fn hsync_end_exist(&self) -> HSYNC_END_EXIST_R {
        HSYNC_END_EXIST_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRAME_CFG")
            .field("vadr_num", &self.vadr_num())
            .field("hadr_num", &self.hadr_num())
            .field("bayer_mode", &self.bayer_mode())
            .field("hsync_start_exist", &self.hsync_start_exist())
            .field("hsync_end_exist", &self.hsync_end_exist())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures input image size in y-direction, image row number - 1"]
    #[inline(always)]
    pub fn vadr_num(&mut self) -> VADR_NUM_W<FRAME_CFG_SPEC> {
        VADR_NUM_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - this field configures input image size in x-direction, image line number - 1"]
    #[inline(always)]
    pub fn hadr_num(&mut self) -> HADR_NUM_W<FRAME_CFG_SPEC> {
        HADR_NUM_W::new(self, 12)
    }
    #[doc = "Bits 27:28 - this field configures the bayer mode of input pixel. 00 : BG/GR 01 : GB/RG 10 : GR/BG 11 : RG/GB"]
    #[inline(always)]
    pub fn bayer_mode(&mut self) -> BAYER_MODE_W<FRAME_CFG_SPEC> {
        BAYER_MODE_W::new(self, 27)
    }
    #[doc = "Bit 29 - this bit configures the line end packet exist or not. 0: not exist, 1: exist"]
    #[inline(always)]
    pub fn hsync_start_exist(&mut self) -> HSYNC_START_EXIST_W<FRAME_CFG_SPEC> {
        HSYNC_START_EXIST_W::new(self, 29)
    }
    #[doc = "Bit 30 - this bit configures the line start packet exist or not. 0: not exist, 1: exist"]
    #[inline(always)]
    pub fn hsync_end_exist(&mut self) -> HSYNC_END_EXIST_W<FRAME_CFG_SPEC> {
        HSYNC_END_EXIST_W::new(self, 30)
    }
}
#[doc = "frame control parameter register\n\nYou can [`read`](crate::Reg::read) this register and get [`frame_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRAME_CFG_SPEC;
impl crate::RegisterSpec for FRAME_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame_cfg::R`](R) reader structure"]
impl crate::Readable for FRAME_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frame_cfg::W`](W) writer structure"]
impl crate::Writable for FRAME_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME_CFG to value 0x601e_01e0"]
impl crate::Resettable for FRAME_CFG_SPEC {
    const RESET_VALUE: u32 = 0x601e_01e0;
}
