///Register `CK_FG_HIGH` reader
pub type R = crate::R<CK_FG_HIGH_SPEC>;
///Register `CK_FG_HIGH` writer
pub type W = crate::W<CK_FG_HIGH_SPEC>;
///Field `COLORKEY_FG_B_HIGH` reader - color key higher threshold of foreground b channel
pub type COLORKEY_FG_B_HIGH_R = crate::FieldReader;
///Field `COLORKEY_FG_B_HIGH` writer - color key higher threshold of foreground b channel
pub type COLORKEY_FG_B_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLORKEY_FG_G_HIGH` reader - color key higher threshold of foreground g channel
pub type COLORKEY_FG_G_HIGH_R = crate::FieldReader;
///Field `COLORKEY_FG_G_HIGH` writer - color key higher threshold of foreground g channel
pub type COLORKEY_FG_G_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `COLORKEY_FG_R_HIGH` reader - color key higher threshold of foreground r channel
pub type COLORKEY_FG_R_HIGH_R = crate::FieldReader;
///Field `COLORKEY_FG_R_HIGH` writer - color key higher threshold of foreground r channel
pub type COLORKEY_FG_R_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - color key higher threshold of foreground b channel
    #[inline(always)]
    pub fn colorkey_fg_b_high(&self) -> COLORKEY_FG_B_HIGH_R {
        COLORKEY_FG_B_HIGH_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - color key higher threshold of foreground g channel
    #[inline(always)]
    pub fn colorkey_fg_g_high(&self) -> COLORKEY_FG_G_HIGH_R {
        COLORKEY_FG_G_HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - color key higher threshold of foreground r channel
    #[inline(always)]
    pub fn colorkey_fg_r_high(&self) -> COLORKEY_FG_R_HIGH_R {
        COLORKEY_FG_R_HIGH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK_FG_HIGH")
            .field("colorkey_fg_b_high", &self.colorkey_fg_b_high())
            .field("colorkey_fg_g_high", &self.colorkey_fg_g_high())
            .field("colorkey_fg_r_high", &self.colorkey_fg_r_high())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - color key higher threshold of foreground b channel
    #[inline(always)]
    #[must_use]
    pub fn colorkey_fg_b_high(&mut self) -> COLORKEY_FG_B_HIGH_W<CK_FG_HIGH_SPEC> {
        COLORKEY_FG_B_HIGH_W::new(self, 0)
    }
    ///Bits 8:15 - color key higher threshold of foreground g channel
    #[inline(always)]
    #[must_use]
    pub fn colorkey_fg_g_high(&mut self) -> COLORKEY_FG_G_HIGH_W<CK_FG_HIGH_SPEC> {
        COLORKEY_FG_G_HIGH_W::new(self, 8)
    }
    ///Bits 16:23 - color key higher threshold of foreground r channel
    #[inline(always)]
    #[must_use]
    pub fn colorkey_fg_r_high(&mut self) -> COLORKEY_FG_R_HIGH_W<CK_FG_HIGH_SPEC> {
        COLORKEY_FG_R_HIGH_W::new(self, 16)
    }
}
/**foreground color key higher threshold

You can [`read`](crate::generic::Reg::read) this register and get [`ck_fg_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck_fg_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CK_FG_HIGH_SPEC;
impl crate::RegisterSpec for CK_FG_HIGH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ck_fg_high::R`](R) reader structure
impl crate::Readable for CK_FG_HIGH_SPEC {}
///`write(|w| ..)` method takes [`ck_fg_high::W`](W) writer structure
impl crate::Writable for CK_FG_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CK_FG_HIGH to value 0
impl crate::Resettable for CK_FG_HIGH_SPEC {
    const RESET_VALUE: u32 = 0;
}
