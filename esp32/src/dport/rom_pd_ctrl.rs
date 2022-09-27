#[doc = "Register `ROM_PD_CTRL` reader"]
pub struct R(crate::R<ROM_PD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROM_PD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROM_PD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROM_PD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROM_PD_CTRL` writer"]
pub struct W(crate::W<ROM_PD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROM_PD_CTRL_SPEC>;
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
impl From<crate::W<ROM_PD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROM_PD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_ROM_PD` reader - "]
pub type PRO_ROM_PD_R = crate::BitReader<bool>;
#[doc = "Field `PRO_ROM_PD` writer - "]
pub type PRO_ROM_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROM_PD_CTRL_SPEC, bool, O>;
#[doc = "Field `APP_ROM_PD` reader - "]
pub type APP_ROM_PD_R = crate::BitReader<bool>;
#[doc = "Field `APP_ROM_PD` writer - "]
pub type APP_ROM_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROM_PD_CTRL_SPEC, bool, O>;
#[doc = "Field `SHARE_ROM_PD` reader - "]
pub type SHARE_ROM_PD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHARE_ROM_PD` writer - "]
pub type SHARE_ROM_PD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROM_PD_CTRL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_pd(&self) -> PRO_ROM_PD_R {
        PRO_ROM_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_pd(&self) -> APP_ROM_PD_R {
        APP_ROM_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_pd(&self) -> SHARE_ROM_PD_R {
        SHARE_ROM_PD_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_pd(&mut self) -> PRO_ROM_PD_W<0> {
        PRO_ROM_PD_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_pd(&mut self) -> APP_ROM_PD_W<1> {
        APP_ROM_PD_W::new(self)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_pd(&mut self) -> SHARE_ROM_PD_W<2> {
        SHARE_ROM_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rom_pd_ctrl](index.html) module"]
pub struct ROM_PD_CTRL_SPEC;
impl crate::RegisterSpec for ROM_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rom_pd_ctrl::R](R) reader structure"]
impl crate::Readable for ROM_PD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rom_pd_ctrl::W](W) writer structure"]
impl crate::Writable for ROM_PD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROM_PD_CTRL to value 0"]
impl crate::Resettable for ROM_PD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
