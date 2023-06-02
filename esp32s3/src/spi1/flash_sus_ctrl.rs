#[doc = "Register `FLASH_SUS_CTRL` reader"]
pub struct R(crate::R<FLASH_SUS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SUS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SUS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_SUS_CTRL` writer"]
pub struct W(crate::W<FLASH_SUS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SUS_CTRL_SPEC>;
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
impl From<crate::W<FLASH_SUS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SUS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASH_PES_EN` reader - Set this bit to enable auto-suspend function."]
pub type FLASH_PES_EN_R = crate::BitReader;
#[doc = "Field `FLASH_PES_EN` writer - Set this bit to enable auto-suspend function."]
pub type FLASH_PES_EN_W<'a, const O: u8> = crate::BitWriter<'a, FLASH_SUS_CTRL_SPEC, O>;
#[doc = "Field `FLASH_PER_COMMAND` reader - Program/Erase resume command value."]
pub type FLASH_PER_COMMAND_R = crate::FieldReader;
#[doc = "Field `FLASH_PER_COMMAND` writer - Program/Erase resume command value."]
pub type FLASH_PER_COMMAND_W<'a, const O: u8> = crate::FieldWriter<'a, FLASH_SUS_CTRL_SPEC, 8, O>;
#[doc = "Field `FLASH_PES_COMMAND` reader - Program/Erase suspend command value."]
pub type FLASH_PES_COMMAND_R = crate::FieldReader;
#[doc = "Field `FLASH_PES_COMMAND` writer - Program/Erase suspend command value."]
pub type FLASH_PES_COMMAND_W<'a, const O: u8> = crate::FieldWriter<'a, FLASH_SUS_CTRL_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable auto-suspend function."]
    #[inline(always)]
    pub fn flash_pes_en(&self) -> FLASH_PES_EN_R {
        FLASH_PES_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Program/Erase resume command value."]
    #[inline(always)]
    pub fn flash_per_command(&self) -> FLASH_PER_COMMAND_R {
        FLASH_PER_COMMAND_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - Program/Erase suspend command value."]
    #[inline(always)]
    pub fn flash_pes_command(&self) -> FLASH_PES_COMMAND_R {
        FLASH_PES_COMMAND_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SUS_CTRL")
            .field(
                "flash_pes_en",
                &format_args!("{}", self.flash_pes_en().bit()),
            )
            .field(
                "flash_per_command",
                &format_args!("{}", self.flash_per_command().bits()),
            )
            .field(
                "flash_pes_command",
                &format_args!("{}", self.flash_pes_command().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FLASH_SUS_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable auto-suspend function."]
    #[inline(always)]
    #[must_use]
    pub fn flash_pes_en(&mut self) -> FLASH_PES_EN_W<0> {
        FLASH_PES_EN_W::new(self)
    }
    #[doc = "Bits 1:8 - Program/Erase resume command value."]
    #[inline(always)]
    #[must_use]
    pub fn flash_per_command(&mut self) -> FLASH_PER_COMMAND_W<1> {
        FLASH_PER_COMMAND_W::new(self)
    }
    #[doc = "Bits 9:16 - Program/Erase suspend command value."]
    #[inline(always)]
    #[must_use]
    pub fn flash_pes_command(&mut self) -> FLASH_PES_COMMAND_W<9> {
        FLASH_PES_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 flash suspend command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_sus_ctrl](index.html) module"]
pub struct FLASH_SUS_CTRL_SPEC;
impl crate::RegisterSpec for FLASH_SUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_sus_ctrl::R](R) reader structure"]
impl crate::Readable for FLASH_SUS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_sus_ctrl::W](W) writer structure"]
impl crate::Writable for FLASH_SUS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_SUS_CTRL to value 0xeaf4"]
impl crate::Resettable for FLASH_SUS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xeaf4;
}
