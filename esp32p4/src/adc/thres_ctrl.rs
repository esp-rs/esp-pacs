#[doc = "Register `THRES_CTRL` reader"]
pub type R = crate::R<THRES_CTRL_SPEC>;
#[doc = "Register `THRES_CTRL` writer"]
pub type W = crate::W<THRES_CTRL_SPEC>;
#[doc = "Field `THRES_ALL_EN` reader - need_des"]
pub type THRES_ALL_EN_R = crate::BitReader;
#[doc = "Field `THRES_ALL_EN` writer - need_des"]
pub type THRES_ALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES3_EN` reader - need_des"]
pub type THRES3_EN_R = crate::BitReader;
#[doc = "Field `THRES3_EN` writer - need_des"]
pub type THRES3_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES2_EN` reader - need_des"]
pub type THRES2_EN_R = crate::BitReader;
#[doc = "Field `THRES2_EN` writer - need_des"]
pub type THRES2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES1_EN` reader - need_des"]
pub type THRES1_EN_R = crate::BitReader;
#[doc = "Field `THRES1_EN` writer - need_des"]
pub type THRES1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES0_EN` reader - need_des"]
pub type THRES0_EN_R = crate::BitReader;
#[doc = "Field `THRES0_EN` writer - need_des"]
pub type THRES0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres_all_en(&self) -> THRES_ALL_EN_R {
        THRES_ALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres3_en(&self) -> THRES3_EN_R {
        THRES3_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres2_en(&self) -> THRES2_EN_R {
        THRES2_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn thres1_en(&self) -> THRES1_EN_R {
        THRES1_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn thres0_en(&self) -> THRES0_EN_R {
        THRES0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES_CTRL")
            .field("thres_all_en", &self.thres_all_en())
            .field("thres3_en", &self.thres3_en())
            .field("thres2_en", &self.thres2_en())
            .field("thres1_en", &self.thres1_en())
            .field("thres0_en", &self.thres0_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn thres_all_en(&mut self) -> THRES_ALL_EN_W<THRES_CTRL_SPEC> {
        THRES_ALL_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn thres3_en(&mut self) -> THRES3_EN_W<THRES_CTRL_SPEC> {
        THRES3_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn thres2_en(&mut self) -> THRES2_EN_W<THRES_CTRL_SPEC> {
        THRES2_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn thres1_en(&mut self) -> THRES1_EN_W<THRES_CTRL_SPEC> {
        THRES1_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn thres0_en(&mut self) -> THRES0_EN_W<THRES_CTRL_SPEC> {
        THRES0_EN_W::new(self, 31)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thres_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thres_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thres_ctrl::R`](R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`thres_ctrl::W`](W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
