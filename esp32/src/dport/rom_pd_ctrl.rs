#[doc = "Register `ROM_PD_CTRL` reader"]
pub type R = crate::R<ROM_PD_CTRL_SPEC>;
#[doc = "Register `ROM_PD_CTRL` writer"]
pub type W = crate::W<ROM_PD_CTRL_SPEC>;
#[doc = "Field `PRO_ROM_PD` reader - "]
pub type PRO_ROM_PD_R = crate::BitReader;
#[doc = "Field `PRO_ROM_PD` writer - "]
pub type PRO_ROM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_ROM_PD` reader - "]
pub type APP_ROM_PD_R = crate::BitReader;
#[doc = "Field `APP_ROM_PD` writer - "]
pub type APP_ROM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHARE_ROM_PD` reader - "]
pub type SHARE_ROM_PD_R = crate::FieldReader;
#[doc = "Field `SHARE_ROM_PD` writer - "]
pub type SHARE_ROM_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_PD_CTRL")
            .field("pro_rom_pd", &self.pro_rom_pd())
            .field("app_rom_pd", &self.app_rom_pd())
            .field("share_rom_pd", &self.share_rom_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_rom_pd(&mut self) -> PRO_ROM_PD_W<'_, ROM_PD_CTRL_SPEC> {
        PRO_ROM_PD_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn app_rom_pd(&mut self) -> APP_ROM_PD_W<'_, ROM_PD_CTRL_SPEC> {
        APP_ROM_PD_W::new(self, 1)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn share_rom_pd(&mut self) -> SHARE_ROM_PD_W<'_, ROM_PD_CTRL_SPEC> {
        SHARE_ROM_PD_W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_PD_CTRL_SPEC;
impl crate::RegisterSpec for ROM_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for ROM_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for ROM_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_PD_CTRL to value 0"]
impl crate::Resettable for ROM_PD_CTRL_SPEC {}
