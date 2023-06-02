#[doc = "Register `ROM_FO_CTRL` reader"]
pub struct R(crate::R<ROM_FO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_FO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_FO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_FO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_FO_CTRL` writer"]
pub struct W(crate::W<ROM_FO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_FO_CTRL_SPEC>;
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
impl From<crate::W<ROM_FO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_FO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ROM_FO` reader - "]
pub type PRO_ROM_FO_R = crate::BitReader;
#[doc = "Field `PRO_ROM_FO` writer - "]
pub type PRO_ROM_FO_W<'a, const O: u8> = crate::BitWriter<'a, ROM_FO_CTRL_SPEC, O>;
#[doc = "Field `APP_ROM_FO` reader - "]
pub type APP_ROM_FO_R = crate::BitReader;
#[doc = "Field `APP_ROM_FO` writer - "]
pub type APP_ROM_FO_W<'a, const O: u8> = crate::BitWriter<'a, ROM_FO_CTRL_SPEC, O>;
#[doc = "Field `SHARE_ROM_FO` reader - "]
pub type SHARE_ROM_FO_R = crate::FieldReader;
#[doc = "Field `SHARE_ROM_FO` writer - "]
pub type SHARE_ROM_FO_W<'a, const O: u8> = crate::FieldWriter<'a, ROM_FO_CTRL_SPEC, 6, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_fo(&self) -> PRO_ROM_FO_R {
        PRO_ROM_FO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_fo(&self) -> APP_ROM_FO_R {
        APP_ROM_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_fo(&self) -> SHARE_ROM_FO_R {
        SHARE_ROM_FO_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_FO_CTRL")
            .field("pro_rom_fo", &format_args!("{}", self.pro_rom_fo().bit()))
            .field("app_rom_fo", &format_args!("{}", self.app_rom_fo().bit()))
            .field(
                "share_rom_fo",
                &format_args!("{}", self.share_rom_fo().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROM_FO_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pro_rom_fo(&mut self) -> PRO_ROM_FO_W<0> {
        PRO_ROM_FO_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn app_rom_fo(&mut self) -> APP_ROM_FO_W<1> {
        APP_ROM_FO_W::new(self)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    #[must_use]
    pub fn share_rom_fo(&mut self) -> SHARE_ROM_FO_W<2> {
        SHARE_ROM_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_fo_ctrl](index.html) module"]
pub struct ROM_FO_CTRL_SPEC;
impl crate::RegisterSpec for ROM_FO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_fo_ctrl::R](R) reader structure"]
impl crate::Readable for ROM_FO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_fo_ctrl::W](W) writer structure"]
impl crate::Writable for ROM_FO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROM_FO_CTRL to value 0x03"]
impl crate::Resettable for ROM_FO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
