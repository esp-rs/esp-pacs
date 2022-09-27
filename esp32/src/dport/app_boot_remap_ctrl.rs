#[doc = "Register `APP_BOOT_REMAP_CTRL` reader"]
pub struct R(crate::R<APP_BOOT_REMAP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_BOOT_REMAP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_BOOT_REMAP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_BOOT_REMAP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_BOOT_REMAP_CTRL` writer"]
pub struct W(crate::W<APP_BOOT_REMAP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_BOOT_REMAP_CTRL_SPEC>;
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
impl From<crate::W<APP_BOOT_REMAP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_BOOT_REMAP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_BOOT_REMAP` reader - "]
pub type APP_BOOT_REMAP_R = crate::BitReader<bool>;
#[doc = "Field `APP_BOOT_REMAP` writer - "]
pub type APP_BOOT_REMAP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APP_BOOT_REMAP_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&self) -> APP_BOOT_REMAP_R {
        APP_BOOT_REMAP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_boot_remap(&mut self) -> APP_BOOT_REMAP_W<0> {
        APP_BOOT_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_boot_remap_ctrl](index.html) module"]
pub struct APP_BOOT_REMAP_CTRL_SPEC;
impl crate::RegisterSpec for APP_BOOT_REMAP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_boot_remap_ctrl::R](R) reader structure"]
impl crate::Readable for APP_BOOT_REMAP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_boot_remap_ctrl::W](W) writer structure"]
impl crate::Writable for APP_BOOT_REMAP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APP_BOOT_REMAP_CTRL to value 0"]
impl crate::Resettable for APP_BOOT_REMAP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
