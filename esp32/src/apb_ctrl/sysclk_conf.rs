#[doc = "Register `SYSCLK_CONF` reader"]
pub type R = crate::R<SYSCLK_CONF_SPEC>;
#[doc = "Register `SYSCLK_CONF` writer"]
pub type W = crate::W<SYSCLK_CONF_SPEC>;
#[doc = "Field `PRE_DIV_CNT` reader - "]
pub type PRE_DIV_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `PRE_DIV_CNT` writer - "]
pub type PRE_DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
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
#[doc = "Field `QUICK_CLK_CHNG` reader - "]
pub type QUICK_CLK_CHNG_R = crate::BitReader;
#[doc = "Field `QUICK_CLK_CHNG` writer - "]
pub type QUICK_CLK_CHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn pre_div_cnt(&self) -> PRE_DIV_CNT_R {
        PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
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
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn quick_clk_chng(&self) -> QUICK_CLK_CHNG_R {
        QUICK_CLK_CHNG_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_CONF")
            .field("pre_div_cnt", &self.pre_div_cnt())
            .field("clk_320m_en", &self.clk_320m_en())
            .field("clk_en", &self.clk_en())
            .field("rst_tick_cnt", &self.rst_tick_cnt())
            .field("quick_clk_chng", &self.quick_clk_chng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    #[must_use]
    pub fn pre_div_cnt(&mut self) -> PRE_DIV_CNT_W<SYSCLK_CONF_SPEC> {
        PRE_DIV_CNT_W::new(self, 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_320m_en(&mut self) -> CLK_320M_EN_W<SYSCLK_CONF_SPEC> {
        CLK_320M_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<SYSCLK_CONF_SPEC> {
        CLK_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rst_tick_cnt(&mut self) -> RST_TICK_CNT_W<SYSCLK_CONF_SPEC> {
        RST_TICK_CNT_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn quick_clk_chng(&mut self) -> QUICK_CLK_CHNG_W<SYSCLK_CONF_SPEC> {
        QUICK_CLK_CHNG_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCLK_CONF_SPEC;
impl crate::RegisterSpec for SYSCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclk_conf::R`](R) reader structure"]
impl crate::Readable for SYSCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysclk_conf::W`](W) writer structure"]
impl crate::Writable for SYSCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSCLK_CONF to value 0x2000"]
impl crate::Resettable for SYSCLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x2000;
}
