#[doc = "Register `ROM_CTRL` reader"]
pub type R = crate::R<ROM_CTRL_SPEC>;
#[doc = "Register `ROM_CTRL` writer"]
pub type W = crate::W<ROM_CTRL_SPEC>;
#[doc = "Field `LP_ROM_FORCE_ON` reader - need_des"]
pub type LP_ROM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `LP_ROM_FORCE_ON` writer - need_des"]
pub type LP_ROM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ROM_RST_EN` reader - need_des"]
pub type LP_ROM_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_ROM_RST_EN` writer - need_des"]
pub type LP_ROM_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_rom_force_on(&self) -> LP_ROM_FORCE_ON_R {
        LP_ROM_FORCE_ON_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_rom_rst_en(&self) -> LP_ROM_RST_EN_R {
        LP_ROM_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_CTRL")
            .field("lp_rom_force_on", &self.lp_rom_force_on())
            .field("lp_rom_rst_en", &self.lp_rom_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_rom_force_on(&mut self) -> LP_ROM_FORCE_ON_W<'_, ROM_CTRL_SPEC> {
        LP_ROM_FORCE_ON_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_rom_rst_en(&mut self) -> LP_ROM_RST_EN_W<'_, ROM_CTRL_SPEC> {
        LP_ROM_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_CTRL_SPEC;
impl crate::RegisterSpec for ROM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_ctrl::R`](R) reader structure"]
impl crate::Readable for ROM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_ctrl::W`](W) writer structure"]
impl crate::Writable for ROM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROM_CTRL to value 0"]
impl crate::Resettable for ROM_CTRL_SPEC {}
