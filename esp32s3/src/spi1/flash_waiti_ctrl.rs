#[doc = "Register `FLASH_WAITI_CTRL` reader"]
pub struct R(crate::R<FLASH_WAITI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_WAITI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_WAITI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_WAITI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_WAITI_CTRL` writer"]
pub struct W(crate::W<FLASH_WAITI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_WAITI_CTRL_SPEC>;
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
impl From<crate::W<FLASH_WAITI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_WAITI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAITI_EN` reader - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
pub type WAITI_EN_R = crate::BitReader;
#[doc = "Field `WAITI_EN` writer - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
pub type WAITI_EN_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_WAITI_CTRL_SPEC, O>;
#[doc = "Field `WAITI_DUMMY` reader - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
pub type WAITI_DUMMY_R = crate::BitReader;
#[doc = "Field `WAITI_DUMMY` writer - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
pub type WAITI_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_WAITI_CTRL_SPEC, O>;
#[doc = "Field `WAITI_CMD` reader - The command value of auto wait flash idle transfer(RDSR)."]
pub type WAITI_CMD_R = crate::FieldReader;
#[doc = "Field `WAITI_CMD` writer - The command value of auto wait flash idle transfer(RDSR)."]
pub type WAITI_CMD_W<'a, const O: u8> = crate::FieldWriter<'a, FLASH_WAITI_CTRL_SPEC, 8, O>;
#[doc = "Field `WAITI_DUMMY_CYCLELEN` reader - The dummy cycle length when wait flash idle(RDSR)."]
pub type WAITI_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `WAITI_DUMMY_CYCLELEN` writer - The dummy cycle length when wait flash idle(RDSR)."]
pub type WAITI_DUMMY_CYCLELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, FLASH_WAITI_CTRL_SPEC, 6, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
    #[inline(always)]
    pub fn waiti_en(&self) -> WAITI_EN_R {
        WAITI_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    pub fn waiti_dummy(&self) -> WAITI_DUMMY_R {
        WAITI_DUMMY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:9 - The command value of auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    pub fn waiti_cmd(&self) -> WAITI_CMD_R {
        WAITI_CMD_R::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:15 - The dummy cycle length when wait flash idle(RDSR)."]
    #[inline(always)]
    pub fn waiti_dummy_cyclelen(&self) -> WAITI_DUMMY_CYCLELEN_R {
        WAITI_DUMMY_CYCLELEN_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_WAITI_CTRL")
            .field("waiti_en", &format_args!("{}", self.waiti_en().bit()))
            .field("waiti_dummy", &format_args!("{}", self.waiti_dummy().bit()))
            .field("waiti_cmd", &format_args!("{}", self.waiti_cmd().bits()))
            .field(
                "waiti_dummy_cyclelen",
                &format_args!("{}", self.waiti_dummy_cyclelen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASH_WAITI_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable auto-waiting flash idle operation when PP/SE/BE/CE/WRSR/PES command is sent."]
    #[inline(always)]
    #[must_use]
    pub fn waiti_en(&mut self) -> WAITI_EN_W<0> {
        WAITI_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to enable DUMMY phase in auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    #[must_use]
    pub fn waiti_dummy(&mut self) -> WAITI_DUMMY_W<1> {
        WAITI_DUMMY_W::new(self)
    }
    #[doc = "Bits 2:9 - The command value of auto wait flash idle transfer(RDSR)."]
    #[inline(always)]
    #[must_use]
    pub fn waiti_cmd(&mut self) -> WAITI_CMD_W<2> {
        WAITI_CMD_W::new(self)
    }
    #[doc = "Bits 10:15 - The dummy cycle length when wait flash idle(RDSR)."]
    #[inline(always)]
    #[must_use]
    pub fn waiti_dummy_cyclelen(&mut self) -> WAITI_DUMMY_CYCLELEN_W<10> {
        WAITI_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 wait idle control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_waiti_ctrl](index.html) module"]
pub struct FLASH_WAITI_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_WAITI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_waiti_ctrl::R](R) reader structure"]
impl crate::Readable for FLASH_WAITI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_waiti_ctrl::W](W) writer structure"]
impl crate::Writable for FLASH_WAITI_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_WAITI_CTRL to value 0x14"]
impl crate::Resettable for FLASH_WAITI_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x14;
}
