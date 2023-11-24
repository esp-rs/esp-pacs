#[doc = "Register `TOUCH_SCAN_CTRL` reader"]
pub type R = crate::R<TOUCH_SCAN_CTRL_SPEC>;
#[doc = "Register `TOUCH_SCAN_CTRL` writer"]
pub type W = crate::W<TOUCH_SCAN_CTRL_SPEC>;
#[doc = "Field `TOUCH_DENOISE_RES` reader - De-noise resolution: 12/10/8/4 bit"]
pub type TOUCH_DENOISE_RES_R = crate::FieldReader;
#[doc = "Field `TOUCH_DENOISE_RES` writer - De-noise resolution: 12/10/8/4 bit"]
pub type TOUCH_DENOISE_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DENOISE_EN` reader - touch pad0 will be used to de-noise"]
pub type TOUCH_DENOISE_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_DENOISE_EN` writer - touch pad0 will be used to de-noise"]
pub type TOUCH_DENOISE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_INACTIVE_CONNECTION` reader - inactive touch pads connect to 1: gnd 0: HighZ"]
pub type TOUCH_INACTIVE_CONNECTION_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE_CONNECTION` writer - inactive touch pads connect to 1: gnd 0: HighZ"]
pub type TOUCH_INACTIVE_CONNECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SHIELD_PAD_EN` reader - touch pad14 will be used as shield"]
pub type TOUCH_SHIELD_PAD_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SHIELD_PAD_EN` writer - touch pad14 will be used as shield"]
pub type TOUCH_SHIELD_PAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SCAN_PAD_MAP` reader - touch scan mode pad enable map"]
pub type TOUCH_SCAN_PAD_MAP_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SCAN_PAD_MAP` writer - touch scan mode pad enable map"]
pub type TOUCH_SCAN_PAD_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TOUCH_BUFDRV` reader - touch7 buffer driver strength"]
pub type TOUCH_BUFDRV_R = crate::FieldReader;
#[doc = "Field `TOUCH_BUFDRV` writer - touch7 buffer driver strength"]
pub type TOUCH_BUFDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_OUT_RING` reader - select out ring pad"]
pub type TOUCH_OUT_RING_R = crate::FieldReader;
#[doc = "Field `TOUCH_OUT_RING` writer - select out ring pad"]
pub type TOUCH_OUT_RING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - De-noise resolution: 12/10/8/4 bit"]
    #[inline(always)]
    pub fn touch_denoise_res(&self) -> TOUCH_DENOISE_RES_R {
        TOUCH_DENOISE_RES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - touch pad0 will be used to de-noise"]
    #[inline(always)]
    pub fn touch_denoise_en(&self) -> TOUCH_DENOISE_EN_R {
        TOUCH_DENOISE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - inactive touch pads connect to 1: gnd 0: HighZ"]
    #[inline(always)]
    pub fn touch_inactive_connection(&self) -> TOUCH_INACTIVE_CONNECTION_R {
        TOUCH_INACTIVE_CONNECTION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - touch pad14 will be used as shield"]
    #[inline(always)]
    pub fn touch_shield_pad_en(&self) -> TOUCH_SHIELD_PAD_EN_R {
        TOUCH_SHIELD_PAD_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:24 - touch scan mode pad enable map"]
    #[inline(always)]
    pub fn touch_scan_pad_map(&self) -> TOUCH_SCAN_PAD_MAP_R {
        TOUCH_SCAN_PAD_MAP_R::new(((self.bits >> 10) & 0x7fff) as u16)
    }
    #[doc = "Bits 25:27 - touch7 buffer driver strength"]
    #[inline(always)]
    pub fn touch_bufdrv(&self) -> TOUCH_BUFDRV_R {
        TOUCH_BUFDRV_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:31 - select out ring pad"]
    #[inline(always)]
    pub fn touch_out_ring(&self) -> TOUCH_OUT_RING_R {
        TOUCH_OUT_RING_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_SCAN_CTRL")
            .field(
                "touch_denoise_res",
                &format_args!("{}", self.touch_denoise_res().bits()),
            )
            .field(
                "touch_denoise_en",
                &format_args!("{}", self.touch_denoise_en().bit()),
            )
            .field(
                "touch_inactive_connection",
                &format_args!("{}", self.touch_inactive_connection().bit()),
            )
            .field(
                "touch_shield_pad_en",
                &format_args!("{}", self.touch_shield_pad_en().bit()),
            )
            .field(
                "touch_scan_pad_map",
                &format_args!("{}", self.touch_scan_pad_map().bits()),
            )
            .field(
                "touch_bufdrv",
                &format_args!("{}", self.touch_bufdrv().bits()),
            )
            .field(
                "touch_out_ring",
                &format_args!("{}", self.touch_out_ring().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_SCAN_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - De-noise resolution: 12/10/8/4 bit"]
    #[inline(always)]
    #[must_use]
    pub fn touch_denoise_res(&mut self) -> TOUCH_DENOISE_RES_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_DENOISE_RES_W::new(self, 0)
    }
    #[doc = "Bit 2 - touch pad0 will be used to de-noise"]
    #[inline(always)]
    #[must_use]
    pub fn touch_denoise_en(&mut self) -> TOUCH_DENOISE_EN_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_DENOISE_EN_W::new(self, 2)
    }
    #[doc = "Bit 8 - inactive touch pads connect to 1: gnd 0: HighZ"]
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive_connection(
        &mut self,
    ) -> TOUCH_INACTIVE_CONNECTION_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_INACTIVE_CONNECTION_W::new(self, 8)
    }
    #[doc = "Bit 9 - touch pad14 will be used as shield"]
    #[inline(always)]
    #[must_use]
    pub fn touch_shield_pad_en(&mut self) -> TOUCH_SHIELD_PAD_EN_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_SHIELD_PAD_EN_W::new(self, 9)
    }
    #[doc = "Bits 10:24 - touch scan mode pad enable map"]
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_pad_map(&mut self) -> TOUCH_SCAN_PAD_MAP_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_SCAN_PAD_MAP_W::new(self, 10)
    }
    #[doc = "Bits 25:27 - touch7 buffer driver strength"]
    #[inline(always)]
    #[must_use]
    pub fn touch_bufdrv(&mut self) -> TOUCH_BUFDRV_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_BUFDRV_W::new(self, 25)
    }
    #[doc = "Bits 28:31 - select out ring pad"]
    #[inline(always)]
    #[must_use]
    pub fn touch_out_ring(&mut self) -> TOUCH_OUT_RING_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_OUT_RING_W::new(self, 28)
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
#[doc = "configure touch controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_SCAN_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_SCAN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_scan_ctrl::R`](R) reader structure"]
impl crate::Readable for TOUCH_SCAN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_scan_ctrl::W`](W) writer structure"]
impl crate::Writable for TOUCH_SCAN_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOUCH_SCAN_CTRL to value 0xf000_0102"]
impl crate::Resettable for TOUCH_SCAN_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xf000_0102;
}
