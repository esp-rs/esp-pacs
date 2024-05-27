#[doc = "Register `CFG_DATA1` reader"]
pub type R = crate::R<CFG_DATA1_SPEC>;
#[doc = "Register `CFG_DATA1` writer"]
pub type W = crate::W<CFG_DATA1_SPEC>;
#[doc = "Field `SDIO_ENABLE` reader - "]
pub type SDIO_ENABLE_R = crate::BitReader;
#[doc = "Field `SDIO_ENABLE` writer - "]
pub type SDIO_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IOREADY1` reader - "]
pub type SDIO_IOREADY1_R = crate::BitReader;
#[doc = "Field `SDIO_IOREADY1` writer - "]
pub type SDIO_IOREADY1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHSPEED_ENABLE` reader - "]
pub type HIGHSPEED_ENABLE_R = crate::BitReader;
#[doc = "Field `HIGHSPEED_ENABLE` writer - "]
pub type HIGHSPEED_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHSPEED_MODE` reader - "]
pub type HIGHSPEED_MODE_R = crate::BitReader;
#[doc = "Field `SDIO_CD_ENABLE` reader - "]
pub type SDIO_CD_ENABLE_R = crate::BitReader;
#[doc = "Field `SDIO_CD_ENABLE` writer - "]
pub type SDIO_CD_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IOREADY2` reader - "]
pub type SDIO_IOREADY2_R = crate::BitReader;
#[doc = "Field `SDIO_IOREADY2` writer - "]
pub type SDIO_IOREADY2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_INT_MASK` reader - "]
pub type SDIO_INT_MASK_R = crate::BitReader;
#[doc = "Field `SDIO_INT_MASK` writer - "]
pub type SDIO_INT_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOENABLE2` reader - "]
pub type IOENABLE2_R = crate::BitReader;
#[doc = "Field `CD_DISABLE` reader - "]
pub type CD_DISABLE_R = crate::BitReader;
#[doc = "Field `FUNC1_EPS` reader - "]
pub type FUNC1_EPS_R = crate::BitReader;
#[doc = "Field `EMP` reader - "]
pub type EMP_R = crate::BitReader;
#[doc = "Field `IOENABLE1` reader - "]
pub type IOENABLE1_R = crate::BitReader;
#[doc = "Field `SDIO20_CONF0` reader - "]
pub type SDIO20_CONF0_R = crate::FieldReader;
#[doc = "Field `SDIO20_CONF0` writer - "]
pub type SDIO20_CONF0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO_VER` reader - "]
pub type SDIO_VER_R = crate::FieldReader<u16>;
#[doc = "Field `SDIO_VER` writer - "]
pub type SDIO_VER_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FUNC2_EPS` reader - "]
pub type FUNC2_EPS_R = crate::BitReader;
#[doc = "Field `SDIO20_CONF1` reader - "]
pub type SDIO20_CONF1_R = crate::FieldReader;
#[doc = "Field `SDIO20_CONF1` writer - "]
pub type SDIO20_CONF1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sdio_enable(&self) -> SDIO_ENABLE_R {
        SDIO_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sdio_ioready1(&self) -> SDIO_IOREADY1_R {
        SDIO_IOREADY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn highspeed_enable(&self) -> HIGHSPEED_ENABLE_R {
        HIGHSPEED_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn highspeed_mode(&self) -> HIGHSPEED_MODE_R {
        HIGHSPEED_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sdio_cd_enable(&self) -> SDIO_CD_ENABLE_R {
        SDIO_CD_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdio_ioready2(&self) -> SDIO_IOREADY2_R {
        SDIO_IOREADY2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SDIO_INT_MASK_R {
        SDIO_INT_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ioenable2(&self) -> IOENABLE2_R {
        IOENABLE2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cd_disable(&self) -> CD_DISABLE_R {
        CD_DISABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn func1_eps(&self) -> FUNC1_EPS_R {
        FUNC1_EPS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn emp(&self) -> EMP_R {
        EMP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ioenable1(&self) -> IOENABLE1_R {
        IOENABLE1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn sdio20_conf0(&self) -> SDIO20_CONF0_R {
        SDIO20_CONF0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn sdio_ver(&self) -> SDIO_VER_R {
        SDIO_VER_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn func2_eps(&self) -> FUNC2_EPS_R {
        FUNC2_EPS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn sdio20_conf1(&self) -> SDIO20_CONF1_R {
        SDIO20_CONF1_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG_DATA1")
            .field("sdio_enable", &self.sdio_enable())
            .field("sdio_ioready1", &self.sdio_ioready1())
            .field("highspeed_enable", &self.highspeed_enable())
            .field("highspeed_mode", &self.highspeed_mode())
            .field("sdio_cd_enable", &self.sdio_cd_enable())
            .field("sdio_ioready2", &self.sdio_ioready2())
            .field("sdio_int_mask", &self.sdio_int_mask())
            .field("ioenable2", &self.ioenable2())
            .field("cd_disable", &self.cd_disable())
            .field("func1_eps", &self.func1_eps())
            .field("emp", &self.emp())
            .field("ioenable1", &self.ioenable1())
            .field("sdio20_conf0", &self.sdio20_conf0())
            .field("sdio_ver", &self.sdio_ver())
            .field("func2_eps", &self.func2_eps())
            .field("sdio20_conf1", &self.sdio20_conf1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_enable(&mut self) -> SDIO_ENABLE_W<CFG_DATA1_SPEC> {
        SDIO_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready1(&mut self) -> SDIO_IOREADY1_W<CFG_DATA1_SPEC> {
        SDIO_IOREADY1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn highspeed_enable(&mut self) -> HIGHSPEED_ENABLE_W<CFG_DATA1_SPEC> {
        HIGHSPEED_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cd_enable(&mut self) -> SDIO_CD_ENABLE_W<CFG_DATA1_SPEC> {
        SDIO_CD_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready2(&mut self) -> SDIO_IOREADY2_W<CFG_DATA1_SPEC> {
        SDIO_IOREADY2_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W<CFG_DATA1_SPEC> {
        SDIO_INT_MASK_W::new(self, 6)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn sdio20_conf0(&mut self) -> SDIO20_CONF0_W<CFG_DATA1_SPEC> {
        SDIO20_CONF0_W::new(self, 12)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ver(&mut self) -> SDIO_VER_W<CFG_DATA1_SPEC> {
        SDIO_VER_W::new(self, 16)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    #[must_use]
    pub fn sdio20_conf1(&mut self) -> SDIO20_CONF1_W<CFG_DATA1_SPEC> {
        SDIO20_CONF1_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_data1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_data1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_DATA1_SPEC;
impl crate::RegisterSpec for CFG_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_data1::R`](R) reader structure"]
impl crate::Readable for CFG_DATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg_data1::W`](W) writer structure"]
impl crate::Writable for CFG_DATA1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_DATA1 to value 0x0111_0011"]
impl crate::Resettable for CFG_DATA1_SPEC {
    const RESET_VALUE: u32 = 0x0111_0011;
}
