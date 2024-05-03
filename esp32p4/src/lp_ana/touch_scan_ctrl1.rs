#[doc = "Register `TOUCH_SCAN_CTRL1` reader"]
pub type R = crate::R<TOUCH_SCAN_CTRL1_SPEC>;
#[doc = "Register `TOUCH_SCAN_CTRL1` writer"]
pub type W = crate::W<TOUCH_SCAN_CTRL1_SPEC>;
#[doc = "Field `TOUCH_SHIELD_PAD_EN` reader - need_des"]
pub type TOUCH_SHIELD_PAD_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SHIELD_PAD_EN` writer - need_des"]
pub type TOUCH_SHIELD_PAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_INACTIVE_CONNECTION` reader - need_des"]
pub type TOUCH_INACTIVE_CONNECTION_R = crate::BitReader;
#[doc = "Field `TOUCH_INACTIVE_CONNECTION` writer - need_des"]
pub type TOUCH_INACTIVE_CONNECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SCAN_PAD_MAP` reader - need_des"]
pub type TOUCH_SCAN_PAD_MAP_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_SCAN_PAD_MAP` writer - need_des"]
pub type TOUCH_SCAN_PAD_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TOUCH_XPD_WAIT` reader - need_des"]
pub type TOUCH_XPD_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_XPD_WAIT` writer - need_des"]
pub type TOUCH_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn touch_shield_pad_en(&self) -> TOUCH_SHIELD_PAD_EN_R {
        TOUCH_SHIELD_PAD_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn touch_inactive_connection(&self) -> TOUCH_INACTIVE_CONNECTION_R {
        TOUCH_INACTIVE_CONNECTION_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:16 - need_des"]
    #[inline(always)]
    pub fn touch_scan_pad_map(&self) -> TOUCH_SCAN_PAD_MAP_R {
        TOUCH_SCAN_PAD_MAP_R::new(((self.bits >> 2) & 0x7fff) as u16)
    }
    #[doc = "Bits 17:31 - need_des"]
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_SCAN_CTRL1")
            .field("touch_shield_pad_en", &self.touch_shield_pad_en().bit())
            .field(
                "touch_inactive_connection",
                &self.touch_inactive_connection().bit(),
            )
            .field("touch_scan_pad_map", &self.touch_scan_pad_map().bits())
            .field("touch_xpd_wait", &self.touch_xpd_wait().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_SCAN_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_shield_pad_en(&mut self) -> TOUCH_SHIELD_PAD_EN_W<TOUCH_SCAN_CTRL1_SPEC> {
        TOUCH_SHIELD_PAD_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive_connection(
        &mut self,
    ) -> TOUCH_INACTIVE_CONNECTION_W<TOUCH_SCAN_CTRL1_SPEC> {
        TOUCH_INACTIVE_CONNECTION_W::new(self, 1)
    }
    #[doc = "Bits 2:16 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_pad_map(&mut self) -> TOUCH_SCAN_PAD_MAP_W<TOUCH_SCAN_CTRL1_SPEC> {
        TOUCH_SCAN_PAD_MAP_W::new(self, 2)
    }
    #[doc = "Bits 17:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W<TOUCH_SCAN_CTRL1_SPEC> {
        TOUCH_XPD_WAIT_W::new(self, 17)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_SCAN_CTRL1_SPEC;
impl crate::RegisterSpec for TOUCH_SCAN_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_scan_ctrl1::R`](R) reader structure"]
impl crate::Readable for TOUCH_SCAN_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_scan_ctrl1::W`](W) writer structure"]
impl crate::Writable for TOUCH_SCAN_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_SCAN_CTRL1 to value 0x0008_0000"]
impl crate::Resettable for TOUCH_SCAN_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0008_0000;
}
