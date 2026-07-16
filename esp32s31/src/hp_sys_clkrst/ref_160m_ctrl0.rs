#[doc = "Register `REF_160M_CTRL0` reader"]
pub type R = crate::R<REF_160M_CTRL0_SPEC>;
#[doc = "Register `REF_160M_CTRL0` writer"]
pub type W = crate::W<REF_160M_CTRL0_SPEC>;
#[doc = "Field `REF_160M_CLK_DIV_NUM` reader - need_des"]
pub type REF_160M_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `REF_160M_CLK_DIV_NUM` writer - need_des"]
pub type REF_160M_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REF_160M_CLK_EN` reader - need_des"]
pub type REF_160M_CLK_EN_R = crate::BitReader;
#[doc = "Field `REF_160M_CLK_EN` writer - need_des"]
pub type REF_160M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn ref_160m_clk_div_num(&self) -> REF_160M_CLK_DIV_NUM_R {
        REF_160M_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn ref_160m_clk_en(&self) -> REF_160M_CLK_EN_R {
        REF_160M_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_160M_CTRL0")
            .field("ref_160m_clk_div_num", &self.ref_160m_clk_div_num())
            .field("ref_160m_clk_en", &self.ref_160m_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn ref_160m_clk_div_num(&mut self) -> REF_160M_CLK_DIV_NUM_W<'_, REF_160M_CTRL0_SPEC> {
        REF_160M_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn ref_160m_clk_en(&mut self) -> REF_160M_CLK_EN_W<'_, REF_160M_CTRL0_SPEC> {
        REF_160M_CLK_EN_W::new(self, 8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_160m_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_160m_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_160M_CTRL0_SPEC;
impl crate::RegisterSpec for REF_160M_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_160m_ctrl0::R`](R) reader structure"]
impl crate::Readable for REF_160M_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_160m_ctrl0::W`](W) writer structure"]
impl crate::Writable for REF_160M_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_160M_CTRL0 to value 0x0102"]
impl crate::Resettable for REF_160M_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0102;
}
