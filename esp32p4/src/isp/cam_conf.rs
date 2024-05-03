#[doc = "Register `CAM_CONF` reader"]
pub type R = crate::R<CAM_CONF_SPEC>;
#[doc = "Register `CAM_CONF` writer"]
pub type W = crate::W<CAM_CONF_SPEC>;
#[doc = "Field `CAM_DATA_ORDER` reader - this field configures data order of cam port, 0: cam_data_in, 1:{cam_data_in\\[7:0\\], cam_data_in\\[15:8\\]}"]
pub type CAM_DATA_ORDER_R = crate::BitReader;
#[doc = "Field `CAM_DATA_ORDER` writer - this field configures data order of cam port, 0: cam_data_in, 1:{cam_data_in\\[7:0\\], cam_data_in\\[15:8\\]}"]
pub type CAM_DATA_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_2BYTE_MODE` reader - this field configures enable of cam 2 byte mode(input 2 bytes each clock). 0: disable, 1: enable"]
pub type CAM_2BYTE_MODE_R = crate::BitReader;
#[doc = "Field `CAM_2BYTE_MODE` writer - this field configures enable of cam 2 byte mode(input 2 bytes each clock). 0: disable, 1: enable"]
pub type CAM_2BYTE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_DATA_TYPE` reader - this field configures idi data type for image data, 0x2a: RAW8, 0x2b: RAW10, 0x2c: RAW12"]
pub type CAM_DATA_TYPE_R = crate::FieldReader;
#[doc = "Field `CAM_DATA_TYPE` writer - this field configures idi data type for image data, 0x2a: RAW8, 0x2b: RAW10, 0x2c: RAW12"]
pub type CAM_DATA_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CAM_DE_INV` reader - this bit configures cam data enable invert. 0: not invert, 1: invert"]
pub type CAM_DE_INV_R = crate::BitReader;
#[doc = "Field `CAM_DE_INV` writer - this bit configures cam data enable invert. 0: not invert, 1: invert"]
pub type CAM_DE_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_HSYNC_INV` reader - this bit configures cam hsync invert. 0: not invert, 1: invert"]
pub type CAM_HSYNC_INV_R = crate::BitReader;
#[doc = "Field `CAM_HSYNC_INV` writer - this bit configures cam hsync invert. 0: not invert, 1: invert"]
pub type CAM_HSYNC_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VSYNC_INV` reader - this bit configures cam vsync invert. 0: not invert, 1: invert"]
pub type CAM_VSYNC_INV_R = crate::BitReader;
#[doc = "Field `CAM_VSYNC_INV` writer - this bit configures cam vsync invert. 0: not invert, 1: invert"]
pub type CAM_VSYNC_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAM_VSYNC_FILTER_THRES` reader - this bit configures the number of clock of vsync filter length"]
pub type CAM_VSYNC_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `CAM_VSYNC_FILTER_THRES` writer - this bit configures the number of clock of vsync filter length"]
pub type CAM_VSYNC_FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAM_VSYNC_FILTER_EN` reader - this bit configures vsync filter en"]
pub type CAM_VSYNC_FILTER_EN_R = crate::BitReader;
#[doc = "Field `CAM_VSYNC_FILTER_EN` writer - this bit configures vsync filter en"]
pub type CAM_VSYNC_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this field configures data order of cam port, 0: cam_data_in, 1:{cam_data_in\\[7:0\\], cam_data_in\\[15:8\\]}"]
    #[inline(always)]
    pub fn cam_data_order(&self) -> CAM_DATA_ORDER_R {
        CAM_DATA_ORDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this field configures enable of cam 2 byte mode(input 2 bytes each clock). 0: disable, 1: enable"]
    #[inline(always)]
    pub fn cam_2byte_mode(&self) -> CAM_2BYTE_MODE_R {
        CAM_2BYTE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - this field configures idi data type for image data, 0x2a: RAW8, 0x2b: RAW10, 0x2c: RAW12"]
    #[inline(always)]
    pub fn cam_data_type(&self) -> CAM_DATA_TYPE_R {
        CAM_DATA_TYPE_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - this bit configures cam data enable invert. 0: not invert, 1: invert"]
    #[inline(always)]
    pub fn cam_de_inv(&self) -> CAM_DE_INV_R {
        CAM_DE_INV_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - this bit configures cam hsync invert. 0: not invert, 1: invert"]
    #[inline(always)]
    pub fn cam_hsync_inv(&self) -> CAM_HSYNC_INV_R {
        CAM_HSYNC_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - this bit configures cam vsync invert. 0: not invert, 1: invert"]
    #[inline(always)]
    pub fn cam_vsync_inv(&self) -> CAM_VSYNC_INV_R {
        CAM_VSYNC_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - this bit configures the number of clock of vsync filter length"]
    #[inline(always)]
    pub fn cam_vsync_filter_thres(&self) -> CAM_VSYNC_FILTER_THRES_R {
        CAM_VSYNC_FILTER_THRES_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - this bit configures vsync filter en"]
    #[inline(always)]
    pub fn cam_vsync_filter_en(&self) -> CAM_VSYNC_FILTER_EN_R {
        CAM_VSYNC_FILTER_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAM_CONF")
            .field("cam_data_order", &self.cam_data_order().bit())
            .field("cam_2byte_mode", &self.cam_2byte_mode().bit())
            .field("cam_data_type", &self.cam_data_type().bits())
            .field("cam_de_inv", &self.cam_de_inv().bit())
            .field("cam_hsync_inv", &self.cam_hsync_inv().bit())
            .field("cam_vsync_inv", &self.cam_vsync_inv().bit())
            .field(
                "cam_vsync_filter_thres",
                &self.cam_vsync_filter_thres().bits(),
            )
            .field("cam_vsync_filter_en", &self.cam_vsync_filter_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - this field configures data order of cam port, 0: cam_data_in, 1:{cam_data_in\\[7:0\\], cam_data_in\\[15:8\\]}"]
    #[inline(always)]
    #[must_use]
    pub fn cam_data_order(&mut self) -> CAM_DATA_ORDER_W<CAM_CONF_SPEC> {
        CAM_DATA_ORDER_W::new(self, 0)
    }
    #[doc = "Bit 1 - this field configures enable of cam 2 byte mode(input 2 bytes each clock). 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn cam_2byte_mode(&mut self) -> CAM_2BYTE_MODE_W<CAM_CONF_SPEC> {
        CAM_2BYTE_MODE_W::new(self, 1)
    }
    #[doc = "Bits 2:7 - this field configures idi data type for image data, 0x2a: RAW8, 0x2b: RAW10, 0x2c: RAW12"]
    #[inline(always)]
    #[must_use]
    pub fn cam_data_type(&mut self) -> CAM_DATA_TYPE_W<CAM_CONF_SPEC> {
        CAM_DATA_TYPE_W::new(self, 2)
    }
    #[doc = "Bit 8 - this bit configures cam data enable invert. 0: not invert, 1: invert"]
    #[inline(always)]
    #[must_use]
    pub fn cam_de_inv(&mut self) -> CAM_DE_INV_W<CAM_CONF_SPEC> {
        CAM_DE_INV_W::new(self, 8)
    }
    #[doc = "Bit 9 - this bit configures cam hsync invert. 0: not invert, 1: invert"]
    #[inline(always)]
    #[must_use]
    pub fn cam_hsync_inv(&mut self) -> CAM_HSYNC_INV_W<CAM_CONF_SPEC> {
        CAM_HSYNC_INV_W::new(self, 9)
    }
    #[doc = "Bit 10 - this bit configures cam vsync invert. 0: not invert, 1: invert"]
    #[inline(always)]
    #[must_use]
    pub fn cam_vsync_inv(&mut self) -> CAM_VSYNC_INV_W<CAM_CONF_SPEC> {
        CAM_VSYNC_INV_W::new(self, 10)
    }
    #[doc = "Bits 11:13 - this bit configures the number of clock of vsync filter length"]
    #[inline(always)]
    #[must_use]
    pub fn cam_vsync_filter_thres(&mut self) -> CAM_VSYNC_FILTER_THRES_W<CAM_CONF_SPEC> {
        CAM_VSYNC_FILTER_THRES_W::new(self, 11)
    }
    #[doc = "Bit 14 - this bit configures vsync filter en"]
    #[inline(always)]
    #[must_use]
    pub fn cam_vsync_filter_en(&mut self) -> CAM_VSYNC_FILTER_EN_W<CAM_CONF_SPEC> {
        CAM_VSYNC_FILTER_EN_W::new(self, 14)
    }
}
#[doc = "isp cam source config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cam_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cam_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAM_CONF_SPEC;
impl crate::RegisterSpec for CAM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cam_conf::R`](R) reader structure"]
impl crate::Readable for CAM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cam_conf::W`](W) writer structure"]
impl crate::Writable for CAM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAM_CONF to value 0xa8"]
impl crate::Resettable for CAM_CONF_SPEC {
    const RESET_VALUE: u32 = 0xa8;
}
