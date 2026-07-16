#[doc = "Register `REF_25M_CTRL0` reader"]
pub type R = crate::R<REF_25M_CTRL0_SPEC>;
#[doc = "Register `REF_25M_CTRL0` writer"]
pub type W = crate::W<REF_25M_CTRL0_SPEC>;
#[doc = "Field `REF_25M_CLK_DIV_NUM` reader - need_des"]
pub type REF_25M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REF_25M_CLK_DIV_NUM` writer - need_des"]
pub type REF_25M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REF_25M_CLK_EN` reader - need_des"]
pub type REF_25M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_25M_CLK_EN` writer - need_des"]
pub type REF_25M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn ref_25m_clk_div_num(&self) -> REF_25M_CLK_DIV_NUM_R {
        REF_25M_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn ref_25m_clk_en(&self) -> REF_25M_CLK_EN_R {
        REF_25M_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_25M_CTRL0")
            .field("ref_25m_clk_div_num", &self.ref_25m_clk_div_num())
            .field("ref_25m_clk_en", &self.ref_25m_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn ref_25m_clk_div_num(&mut self) -> REF_25M_CLK_DIV_NUM_W<'_, REF_25M_CTRL0_SPEC> {
        REF_25M_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn ref_25m_clk_en(&mut self) -> REF_25M_CLK_EN_W<'_, REF_25M_CTRL0_SPEC> {
        REF_25M_CLK_EN_W::new(self, 8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_25m_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_25m_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_25M_CTRL0_SPEC;
impl crate::RegisterSpec for REF_25M_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_25m_ctrl0::R`](R) reader structure"]
impl crate::Readable for REF_25M_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_25m_ctrl0::W`](W) writer structure"]
impl crate::Writable for REF_25M_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_25M_CTRL0 to value 0x010b"]
impl crate::Resettable for REF_25M_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x010b;
}
