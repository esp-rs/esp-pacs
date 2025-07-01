#[doc = "Register `LP_RST_CTRL` reader"]
pub type R = crate::R<LP_RST_CTRL_SPEC>;
#[doc = "Register `LP_RST_CTRL` writer"]
pub type W = crate::W<LP_RST_CTRL_SPEC>;
#[doc = "Field `ANA_RST_BYPASS` reader - analog source reset bypass : wdt,brown out,super wdt,glitch"]
pub type ANA_RST_BYPASS_R = crate::BitReader;
#[doc = "Field `ANA_RST_BYPASS` writer - analog source reset bypass : wdt,brown out,super wdt,glitch"]
pub type ANA_RST_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_RST_BYPASS` reader - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
pub type SYS_RST_BYPASS_R = crate::BitReader;
#[doc = "Field `SYS_RST_BYPASS` writer - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
pub type SYS_RST_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_FORCE_NORST` reader - efuse force no reset control"]
pub type EFUSE_FORCE_NORST_R = crate::BitReader;
#[doc = "Field `EFUSE_FORCE_NORST` writer - efuse force no reset control"]
pub type EFUSE_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - analog source reset bypass : wdt,brown out,super wdt,glitch"]
    #[inline(always)]
    pub fn ana_rst_bypass(&self) -> ANA_RST_BYPASS_R {
        ANA_RST_BYPASS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
    #[inline(always)]
    pub fn sys_rst_bypass(&self) -> SYS_RST_BYPASS_R {
        SYS_RST_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - efuse force no reset control"]
    #[inline(always)]
    pub fn efuse_force_norst(&self) -> EFUSE_FORCE_NORST_R {
        EFUSE_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RST_CTRL")
            .field("ana_rst_bypass", &self.ana_rst_bypass())
            .field("sys_rst_bypass", &self.sys_rst_bypass())
            .field("efuse_force_norst", &self.efuse_force_norst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - analog source reset bypass : wdt,brown out,super wdt,glitch"]
    #[inline(always)]
    pub fn ana_rst_bypass(&mut self) -> ANA_RST_BYPASS_W<LP_RST_CTRL_SPEC> {
        ANA_RST_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 1 - system source reset bypass : software reset,hp wdt,lp wdt,efuse"]
    #[inline(always)]
    pub fn sys_rst_bypass(&mut self) -> SYS_RST_BYPASS_W<LP_RST_CTRL_SPEC> {
        SYS_RST_BYPASS_W::new(self, 1)
    }
    #[doc = "Bit 2 - efuse force no reset control"]
    #[inline(always)]
    pub fn efuse_force_norst(&mut self) -> EFUSE_FORCE_NORST_W<LP_RST_CTRL_SPEC> {
        EFUSE_FORCE_NORST_W::new(self, 2)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_rst_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_rst_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_RST_CTRL_SPEC;
impl crate::RegisterSpec for LP_RST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_rst_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_RST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_rst_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_RST_CTRL to value 0x03"]
impl crate::Resettable for LP_RST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
