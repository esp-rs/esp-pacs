#[doc = "Register `SYSCLK_CONF` reader"]
pub type R = crate::R<SYSCLK_CONF_SPEC>;
#[doc = "Register `SYSCLK_CONF` writer"]
pub type W = crate::W<SYSCLK_CONF_SPEC>;
#[doc = "Field `CLK_320M_EN` reader - "]
pub type CLK_320M_EN_R = crate::BitReader;
#[doc = "Field `CLK_320M_EN` writer - "]
pub type CLK_320M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EN` reader - "]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - "]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_TICK_CNT` reader - "]
pub type RST_TICK_CNT_R = crate::BitReader;
#[doc = "Field `RST_TICK_CNT` writer - "]
pub type RST_TICK_CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_320m_en(&self) -> CLK_320M_EN_R {
        CLK_320M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rst_tick_cnt(&self) -> RST_TICK_CNT_R {
        RST_TICK_CNT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_CONF")
            .field("clk_320m_en", &self.clk_320m_en())
            .field("clk_en", &self.clk_en())
            .field("rst_tick_cnt", &self.rst_tick_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clk_320m_en(&mut self) -> CLK_320M_EN_W<SYSCLK_CONF_SPEC> {
        CLK_320M_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<SYSCLK_CONF_SPEC> {
        CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rst_tick_cnt(&mut self) -> RST_TICK_CNT_W<SYSCLK_CONF_SPEC> {
        RST_TICK_CNT_W::new(self, 12)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCLK_CONF_SPEC;
impl crate::RegisterSpec for SYSCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclk_conf::R`](R) reader structure"]
impl crate::Readable for SYSCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysclk_conf::W`](W) writer structure"]
impl crate::Writable for SYSCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCLK_CONF to value 0"]
impl crate::Resettable for SYSCLK_CONF_SPEC {}
