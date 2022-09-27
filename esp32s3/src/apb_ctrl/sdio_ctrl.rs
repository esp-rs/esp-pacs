#[doc = "Register `SDIO_CTRL` reader"]
pub struct R(crate::R<SDIO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDIO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDIO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDIO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDIO_CTRL` writer"]
pub struct W(crate::W<SDIO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIO_CTRL_SPEC>;
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
impl From<crate::W<SDIO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDIO_WIN_ACCESS_EN` reader - ******* Description ***********"]
pub type SDIO_WIN_ACCESS_EN_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_WIN_ACCESS_EN` writer - ******* Description ***********"]
pub type SDIO_WIN_ACCESS_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDIO_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ******* Description ***********"]
    #[inline(always)]
    pub fn sdio_win_access_en(&self) -> SDIO_WIN_ACCESS_EN_R {
        SDIO_WIN_ACCESS_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ******* Description ***********"]
    #[inline(always)]
    pub fn sdio_win_access_en(&mut self) -> SDIO_WIN_ACCESS_EN_W<0> {
        SDIO_WIN_ACCESS_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdio_ctrl](index.html) module"]
pub struct SDIO_CTRL_SPEC;
impl crate::RegisterSpec for SDIO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdio_ctrl::R](R) reader structure"]
impl crate::Readable for SDIO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdio_ctrl::W](W) writer structure"]
impl crate::Writable for SDIO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIO_CTRL to value 0"]
impl crate::Resettable for SDIO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
