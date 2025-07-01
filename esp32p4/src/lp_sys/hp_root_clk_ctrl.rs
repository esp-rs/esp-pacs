#[doc = "Register `HP_ROOT_CLK_CTRL` reader"]
pub type R = crate::R<HP_ROOT_CLK_CTRL_SPEC>;
#[doc = "Register `HP_ROOT_CLK_CTRL` writer"]
pub type W = crate::W<HP_ROOT_CLK_CTRL_SPEC>;
#[doc = "Field `CPU_CLK_EN` reader - clock gate enable for hp cpu root 400M clk"]
pub type CPU_CLK_EN_R = crate::BitReader;
#[doc = "Field `CPU_CLK_EN` writer - clock gate enable for hp cpu root 400M clk"]
pub type CPU_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_CLK_EN` reader - clock gate enable for hp sys root 480M clk"]
pub type SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_CLK_EN` writer - clock gate enable for hp sys root 480M clk"]
pub type SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clock gate enable for hp cpu root 400M clk"]
    #[inline(always)]
    pub fn cpu_clk_en(&self) -> CPU_CLK_EN_R {
        CPU_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clock gate enable for hp sys root 480M clk"]
    #[inline(always)]
    pub fn sys_clk_en(&self) -> SYS_CLK_EN_R {
        SYS_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_ROOT_CLK_CTRL")
            .field("cpu_clk_en", &self.cpu_clk_en())
            .field("sys_clk_en", &self.sys_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - clock gate enable for hp cpu root 400M clk"]
    #[inline(always)]
    pub fn cpu_clk_en(&mut self) -> CPU_CLK_EN_W<HP_ROOT_CLK_CTRL_SPEC> {
        CPU_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - clock gate enable for hp sys root 480M clk"]
    #[inline(always)]
    pub fn sys_clk_en(&mut self) -> SYS_CLK_EN_W<HP_ROOT_CLK_CTRL_SPEC> {
        SYS_CLK_EN_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_root_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_root_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_ROOT_CLK_CTRL_SPEC;
impl crate::RegisterSpec for HP_ROOT_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_root_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_ROOT_CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_root_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_ROOT_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_ROOT_CLK_CTRL to value 0x03"]
impl crate::Resettable for HP_ROOT_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
