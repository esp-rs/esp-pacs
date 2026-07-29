#[doc = "Register `DBG0_CLK_CTRL0` reader"]
pub type R = crate::R<DBG0_CLK_CTRL0_SPEC>;
#[doc = "Register `DBG0_CLK_CTRL0` writer"]
pub type W = crate::W<DBG0_CLK_CTRL0_SPEC>;
#[doc = "Field `SEL` reader - need_des"]
pub type SEL_R = crate::FieldReader;
#[doc = "Field `SEL` writer - need_des"]
pub type SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DIV_NUM` reader - need_des"]
pub type DIV_NUM_R = crate::FieldReader;
#[doc = "Field `DIV_NUM` writer - need_des"]
pub type DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EN` reader - need_des"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - need_des"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn div_num(&self) -> DIV_NUM_R {
        DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG0_CLK_CTRL0")
            .field("sel", &self.sel())
            .field("div_num", &self.div_num())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W<'_, DBG0_CLK_CTRL0_SPEC> {
        SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn div_num(&mut self) -> DIV_NUM_W<'_, DBG0_CLK_CTRL0_SPEC> {
        DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, DBG0_CLK_CTRL0_SPEC> {
        EN_W::new(self, 16)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`dbg0_clk_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbg0_clk_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG0_CLK_CTRL0_SPEC;
impl crate::RegisterSpec for DBG0_CLK_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg0_clk_ctrl0::R`](R) reader structure"]
impl crate::Readable for DBG0_CLK_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg0_clk_ctrl0::W`](W) writer structure"]
impl crate::Writable for DBG0_CLK_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBG0_CLK_CTRL0 to value 0x03ff"]
impl crate::Resettable for DBG0_CLK_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
