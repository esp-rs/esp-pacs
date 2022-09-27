#[doc = "Register `SECURE_BOOT_CTRL` reader"]
pub struct R(crate::R<SECURE_BOOT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURE_BOOT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURE_BOOT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURE_BOOT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURE_BOOT_CTRL` writer"]
pub struct W(crate::W<SECURE_BOOT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURE_BOOT_CTRL_SPEC>;
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
impl From<crate::W<SECURE_BOOT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURE_BOOT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_BOOTLOADER_SEL` reader - "]
pub type SW_BOOTLOADER_SEL_R = crate::BitReader<bool>;
#[doc = "Field `SW_BOOTLOADER_SEL` writer - "]
pub type SW_BOOTLOADER_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SECURE_BOOT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_bootloader_sel(&self) -> SW_BOOTLOADER_SEL_R {
        SW_BOOTLOADER_SEL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_bootloader_sel(&mut self) -> SW_BOOTLOADER_SEL_W<0> {
        SW_BOOTLOADER_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [secure_boot_ctrl](index.html) module"]
pub struct SECURE_BOOT_CTRL_SPEC;
impl crate::RegisterSpec for SECURE_BOOT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [secure_boot_ctrl::R](R) reader structure"]
impl crate::Readable for SECURE_BOOT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [secure_boot_ctrl::W](W) writer structure"]
impl crate::Writable for SECURE_BOOT_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECURE_BOOT_CTRL to value 0"]
impl crate::Resettable for SECURE_BOOT_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
