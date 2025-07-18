#[doc = "Register `CK_BG_LOW` reader"]
pub type R = crate::R<CK_BG_LOW_SPEC>;
#[doc = "Register `CK_BG_LOW` writer"]
pub type W = crate::W<CK_BG_LOW_SPEC>;
#[doc = "Field `COLORKEY_BG_B_LOW` reader - color key lower threshold of background b channel"]
pub type COLORKEY_BG_B_LOW_R = crate::FieldReader;
#[doc = "Field `COLORKEY_BG_B_LOW` writer - color key lower threshold of background b channel"]
pub type COLORKEY_BG_B_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_BG_G_LOW` reader - color key lower threshold of background g channel"]
pub type COLORKEY_BG_G_LOW_R = crate::FieldReader;
#[doc = "Field `COLORKEY_BG_G_LOW` writer - color key lower threshold of background g channel"]
pub type COLORKEY_BG_G_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLORKEY_BG_R_LOW` reader - color key lower threshold of background r channel"]
pub type COLORKEY_BG_R_LOW_R = crate::FieldReader;
#[doc = "Field `COLORKEY_BG_R_LOW` writer - color key lower threshold of background r channel"]
pub type COLORKEY_BG_R_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - color key lower threshold of background b channel"]
    #[inline(always)]
    pub fn colorkey_bg_b_low(&self) -> COLORKEY_BG_B_LOW_R {
        COLORKEY_BG_B_LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - color key lower threshold of background g channel"]
    #[inline(always)]
    pub fn colorkey_bg_g_low(&self) -> COLORKEY_BG_G_LOW_R {
        COLORKEY_BG_G_LOW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - color key lower threshold of background r channel"]
    #[inline(always)]
    pub fn colorkey_bg_r_low(&self) -> COLORKEY_BG_R_LOW_R {
        COLORKEY_BG_R_LOW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK_BG_LOW")
            .field("colorkey_bg_b_low", &self.colorkey_bg_b_low())
            .field("colorkey_bg_g_low", &self.colorkey_bg_g_low())
            .field("colorkey_bg_r_low", &self.colorkey_bg_r_low())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - color key lower threshold of background b channel"]
    #[inline(always)]
    pub fn colorkey_bg_b_low(&mut self) -> COLORKEY_BG_B_LOW_W<CK_BG_LOW_SPEC> {
        COLORKEY_BG_B_LOW_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - color key lower threshold of background g channel"]
    #[inline(always)]
    pub fn colorkey_bg_g_low(&mut self) -> COLORKEY_BG_G_LOW_W<CK_BG_LOW_SPEC> {
        COLORKEY_BG_G_LOW_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - color key lower threshold of background r channel"]
    #[inline(always)]
    pub fn colorkey_bg_r_low(&mut self) -> COLORKEY_BG_R_LOW_W<CK_BG_LOW_SPEC> {
        COLORKEY_BG_R_LOW_W::new(self, 16)
    }
}
#[doc = "background color key lower threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`ck_bg_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_bg_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CK_BG_LOW_SPEC;
impl crate::RegisterSpec for CK_BG_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck_bg_low::R`](R) reader structure"]
impl crate::Readable for CK_BG_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ck_bg_low::W`](W) writer structure"]
impl crate::Writable for CK_BG_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CK_BG_LOW to value 0x00ff_ffff"]
impl crate::Resettable for CK_BG_LOW_SPEC {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
