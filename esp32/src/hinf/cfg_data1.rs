#[doc = "Register `CFG_DATA1` reader"]
pub struct R(crate::R<CFG_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_DATA1` writer"]
pub struct W(crate::W<CFG_DATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFG_DATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_DATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_ENABLE` reader - "]
pub type SDIO_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_ENABLE` writer - "]
pub type SDIO_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_DATA1_SPEC, bool, O>;
#[doc = "Field `SDIO_IOREADY1` reader - "]
pub type SDIO_IOREADY1_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_IOREADY1` writer - "]
pub type SDIO_IOREADY1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_DATA1_SPEC, bool, O>;
#[doc = "Field `HIGHSPEED_ENABLE` reader - "]
pub type HIGHSPEED_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `HIGHSPEED_ENABLE` writer - "]
pub type HIGHSPEED_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_DATA1_SPEC, bool, O>;
#[doc = "Field `HIGHSPEED_MODE` reader - "]
pub type HIGHSPEED_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_CD_ENABLE` reader - "]
pub type SDIO_CD_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_CD_ENABLE` writer - "]
pub type SDIO_CD_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_DATA1_SPEC, bool, O>;
#[doc = "Field `SDIO_IOREADY2` reader - "]
pub type SDIO_IOREADY2_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_IOREADY2` writer - "]
pub type SDIO_IOREADY2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_DATA1_SPEC, bool, O>;
#[doc = "Field `SDIO_INT_MASK` reader - "]
pub type SDIO_INT_MASK_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_INT_MASK` writer - "]
pub type SDIO_INT_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_DATA1_SPEC, bool, O>;
#[doc = "Field `IOENABLE2` reader - "]
pub type IOENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `CD_DISABLE` reader - "]
pub type CD_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `FUNC1_EPS` reader - "]
pub type FUNC1_EPS_R = crate::BitReader<bool>;
#[doc = "Field `EMP` reader - "]
pub type EMP_R = crate::BitReader<bool>;
#[doc = "Field `IOENABLE1` reader - "]
pub type IOENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `SDIO20_CONF0` reader - "]
pub type SDIO20_CONF0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDIO20_CONF0` writer - "]
pub type SDIO20_CONF0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DATA1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SDIO_VER` reader - "]
pub type SDIO_VER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDIO_VER` writer - "]
pub type SDIO_VER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_DATA1_SPEC, u16, u16, 12, O>;
#[doc = "Field `FUNC2_EPS` reader - "]
pub type FUNC2_EPS_R = crate::BitReader<bool>;
#[doc = "Field `SDIO20_CONF1` reader - "]
pub type SDIO20_CONF1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDIO20_CONF1` writer - "]
pub type SDIO20_CONF1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_DATA1_SPEC, u8, u8, 3, O>;
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
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_enable(&mut self) -> SDIO_ENABLE_W<0> {
        SDIO_ENABLE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready1(&mut self) -> SDIO_IOREADY1_W<1> {
        SDIO_IOREADY1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn highspeed_enable(&mut self) -> HIGHSPEED_ENABLE_W<2> {
        HIGHSPEED_ENABLE_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_cd_enable(&mut self) -> SDIO_CD_ENABLE_W<4> {
        SDIO_CD_ENABLE_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ioready2(&mut self) -> SDIO_IOREADY2_W<5> {
        SDIO_IOREADY2_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_mask(&mut self) -> SDIO_INT_MASK_W<6> {
        SDIO_INT_MASK_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn sdio20_conf0(&mut self) -> SDIO20_CONF0_W<12> {
        SDIO20_CONF0_W::new(self)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_ver(&mut self) -> SDIO_VER_W<16> {
        SDIO_VER_W::new(self)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    #[must_use]
    pub fn sdio20_conf1(&mut self) -> SDIO20_CONF1_W<29> {
        SDIO20_CONF1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_data1](index.html) module"]
pub struct CFG_DATA1_SPEC;
impl crate::RegisterSpec for CFG_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_data1::R](R) reader structure"]
impl crate::Readable for CFG_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_data1::W](W) writer structure"]
impl crate::Writable for CFG_DATA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_DATA1 to value 0x0111_0011"]
impl crate::Resettable for CFG_DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0111_0011;
}
