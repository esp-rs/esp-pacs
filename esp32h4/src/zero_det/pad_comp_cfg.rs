#[doc = "Register `PAD_COMP_CFG` reader"]
pub type R = crate::R<PAD_COMP_CFG_SPEC>;
#[doc = "Register `PAD_COMP_CFG` writer"]
pub type W = crate::W<PAD_COMP_CFG_SPEC>;
#[doc = "Field `PAD_COMP_HYS` reader - hys cfg singal"]
pub type PAD_COMP_HYS_R = crate::FieldReader;
#[doc = "Field `PAD_COMP_HYS` writer - hys cfg singal"]
pub type PAD_COMP_HYS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_COMP_HYS_EN` reader - enable hys function,only works while pad comp mode = 0"]
pub type PAD_COMP_HYS_EN_R = crate::BitReader;
#[doc = "Field `PAD_COMP_HYS_EN` writer - enable hys function,only works while pad comp mode = 0"]
pub type PAD_COMP_HYS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_DREF` reader - internal vref cfg singal,0~2.3v step is 330mv"]
pub type PAD_COMP_DREF_R = crate::FieldReader;
#[doc = "Field `PAD_COMP_DREF` writer - internal vref cfg singal,0~2.3v step is 330mv"]
pub type PAD_COMP_DREF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAD_COMP_MODE` reader - pad comp mode cfg 1:external pad 0:internal vref"]
pub type PAD_COMP_MODE_R = crate::BitReader;
#[doc = "Field `PAD_COMP_MODE` writer - pad comp mode cfg 1:external pad 0:internal vref"]
pub type PAD_COMP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_COMP_XPD` reader - pad comp xpd"]
pub type PAD_COMP_XPD_R = crate::BitReader;
#[doc = "Field `PAD_COMP_XPD` writer - pad comp xpd"]
pub type PAD_COMP_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - hys cfg singal"]
    #[inline(always)]
    pub fn pad_comp_hys(&self) -> PAD_COMP_HYS_R {
        PAD_COMP_HYS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - enable hys function,only works while pad comp mode = 0"]
    #[inline(always)]
    pub fn pad_comp_hys_en(&self) -> PAD_COMP_HYS_EN_R {
        PAD_COMP_HYS_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - internal vref cfg singal,0~2.3v step is 330mv"]
    #[inline(always)]
    pub fn pad_comp_dref(&self) -> PAD_COMP_DREF_R {
        PAD_COMP_DREF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - pad comp mode cfg 1:external pad 0:internal vref"]
    #[inline(always)]
    pub fn pad_comp_mode(&self) -> PAD_COMP_MODE_R {
        PAD_COMP_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pad comp xpd"]
    #[inline(always)]
    pub fn pad_comp_xpd(&self) -> PAD_COMP_XPD_R {
        PAD_COMP_XPD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP_CFG")
            .field("pad_comp_hys", &self.pad_comp_hys())
            .field("pad_comp_hys_en", &self.pad_comp_hys_en())
            .field("pad_comp_dref", &self.pad_comp_dref())
            .field("pad_comp_mode", &self.pad_comp_mode())
            .field("pad_comp_xpd", &self.pad_comp_xpd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - hys cfg singal"]
    #[inline(always)]
    pub fn pad_comp_hys(&mut self) -> PAD_COMP_HYS_W<'_, PAD_COMP_CFG_SPEC> {
        PAD_COMP_HYS_W::new(self, 0)
    }
    #[doc = "Bit 3 - enable hys function,only works while pad comp mode = 0"]
    #[inline(always)]
    pub fn pad_comp_hys_en(&mut self) -> PAD_COMP_HYS_EN_W<'_, PAD_COMP_CFG_SPEC> {
        PAD_COMP_HYS_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - internal vref cfg singal,0~2.3v step is 330mv"]
    #[inline(always)]
    pub fn pad_comp_dref(&mut self) -> PAD_COMP_DREF_W<'_, PAD_COMP_CFG_SPEC> {
        PAD_COMP_DREF_W::new(self, 4)
    }
    #[doc = "Bit 7 - pad comp mode cfg 1:external pad 0:internal vref"]
    #[inline(always)]
    pub fn pad_comp_mode(&mut self) -> PAD_COMP_MODE_W<'_, PAD_COMP_CFG_SPEC> {
        PAD_COMP_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8 - pad comp xpd"]
    #[inline(always)]
    pub fn pad_comp_xpd(&mut self) -> PAD_COMP_XPD_W<'_, PAD_COMP_CFG_SPEC> {
        PAD_COMP_XPD_W::new(self, 8)
    }
}
#[doc = "pad comp cfg reg\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_COMP_CFG_SPEC;
impl crate::RegisterSpec for PAD_COMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp_cfg::R`](R) reader structure"]
impl crate::Readable for PAD_COMP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_comp_cfg::W`](W) writer structure"]
impl crate::Writable for PAD_COMP_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_COMP_CFG to value 0"]
impl crate::Resettable for PAD_COMP_CFG_SPEC {}
