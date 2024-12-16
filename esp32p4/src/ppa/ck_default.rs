#[doc = "Register `CK_DEFAULT` reader"]
pub type R = crate::R<CK_DEFAULT_SPEC>;
#[doc = "Register `CK_DEFAULT` writer"]
pub type W = crate::W<CK_DEFAULT_SPEC>;
#[doc = "Field `COLORKEY_DEFAULT_B` reader - default B channle value of color key"]
pub type COLORKEY_DEFAULT_B_R = crate::FieldReader;
#[doc = "Field `COLORKEY_DEFAULT_B` writer - default B channle value of color key"]
pub type COLORKEY_DEFAULT_B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_DEFAULT_G` reader - default G channle value of color key"]
pub type COLORKEY_DEFAULT_G_R = crate::FieldReader;
#[doc = "Field `COLORKEY_DEFAULT_G` writer - default G channle value of color key"]
pub type COLORKEY_DEFAULT_G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_DEFAULT_R` reader - default R channle value of color key"]
pub type COLORKEY_DEFAULT_R_R = crate::FieldReader;
#[doc = "Field `COLORKEY_DEFAULT_R` writer - default R channle value of color key"]
pub type COLORKEY_DEFAULT_R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_FG_BG_REVERSE` reader - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
pub type COLORKEY_FG_BG_REVERSE_R = crate::BitReader;
#[doc = "Field `COLORKEY_FG_BG_REVERSE` writer - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
pub type COLORKEY_FG_BG_REVERSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - default B channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_b(&self) -> COLORKEY_DEFAULT_B_R {
        COLORKEY_DEFAULT_B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - default G channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_g(&self) -> COLORKEY_DEFAULT_G_R {
        COLORKEY_DEFAULT_G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - default R channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_r(&self) -> COLORKEY_DEFAULT_R_R {
        COLORKEY_DEFAULT_R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
    #[inline(always)]
    pub fn colorkey_fg_bg_reverse(&self) -> COLORKEY_FG_BG_REVERSE_R {
        COLORKEY_FG_BG_REVERSE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK_DEFAULT")
            .field("colorkey_default_b", &self.colorkey_default_b())
            .field("colorkey_default_g", &self.colorkey_default_g())
            .field("colorkey_default_r", &self.colorkey_default_r())
            .field("colorkey_fg_bg_reverse", &self.colorkey_fg_bg_reverse())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - default B channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_b(&mut self) -> COLORKEY_DEFAULT_B_W<CK_DEFAULT_SPEC> {
        COLORKEY_DEFAULT_B_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - default G channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_g(&mut self) -> COLORKEY_DEFAULT_G_W<CK_DEFAULT_SPEC> {
        COLORKEY_DEFAULT_G_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - default R channle value of color key"]
    #[inline(always)]
    pub fn colorkey_default_r(&mut self) -> COLORKEY_DEFAULT_R_W<CK_DEFAULT_SPEC> {
        COLORKEY_DEFAULT_R_W::new(self, 16)
    }
    #[doc = "Bit 24 - when pixel in bg ck range but not in fg ck range, 0: the result is bg, 1: the result is fg"]
    #[inline(always)]
    pub fn colorkey_fg_bg_reverse(&mut self) -> COLORKEY_FG_BG_REVERSE_W<CK_DEFAULT_SPEC> {
        COLORKEY_FG_BG_REVERSE_W::new(self, 24)
    }
}
#[doc = "default value when foreground and background both in color key range\n\nYou can [`read`](crate::Reg::read) this register and get [`ck_default::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_default::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CK_DEFAULT_SPEC;
impl crate::RegisterSpec for CK_DEFAULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck_default::R`](R) reader structure"]
impl crate::Readable for CK_DEFAULT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ck_default::W`](W) writer structure"]
impl crate::Writable for CK_DEFAULT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CK_DEFAULT to value 0"]
impl crate::Resettable for CK_DEFAULT_SPEC {
    const RESET_VALUE: u32 = 0;
}
