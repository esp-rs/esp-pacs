#[doc = "Register `HP_L2_MEM_AHB_BUFFER_CTRL` reader"]
pub type R = crate::R<HP_L2_MEM_AHB_BUFFER_CTRL_SPEC>;
#[doc = "Register `HP_L2_MEM_AHB_BUFFER_CTRL` writer"]
pub type W = crate::W<HP_L2_MEM_AHB_BUFFER_CTRL_SPEC>;
#[doc = "Field `HP_L2_MEM_AHB_WRBUFFER_EN` reader - Set 1 to turn on l2mem ahb wr buffer"]
pub type HP_L2_MEM_AHB_WRBUFFER_EN_R = crate::BitReader;
#[doc = "Field `HP_L2_MEM_AHB_WRBUFFER_EN` writer - Set 1 to turn on l2mem ahb wr buffer"]
pub type HP_L2_MEM_AHB_WRBUFFER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_L2_MEM_AHB_RDBUFFER_EN` reader - Set 1 to turn on l2mem ahb rd buffer"]
pub type HP_L2_MEM_AHB_RDBUFFER_EN_R = crate::BitReader;
#[doc = "Field `HP_L2_MEM_AHB_RDBUFFER_EN` writer - Set 1 to turn on l2mem ahb rd buffer"]
pub type HP_L2_MEM_AHB_RDBUFFER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to turn on l2mem ahb wr buffer"]
    #[inline(always)]
    pub fn hp_l2_mem_ahb_wrbuffer_en(&self) -> HP_L2_MEM_AHB_WRBUFFER_EN_R {
        HP_L2_MEM_AHB_WRBUFFER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to turn on l2mem ahb rd buffer"]
    #[inline(always)]
    pub fn hp_l2_mem_ahb_rdbuffer_en(&self) -> HP_L2_MEM_AHB_RDBUFFER_EN_R {
        HP_L2_MEM_AHB_RDBUFFER_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_L2_MEM_AHB_BUFFER_CTRL")
            .field(
                "hp_l2_mem_ahb_wrbuffer_en",
                &format_args!("{}", self.hp_l2_mem_ahb_wrbuffer_en().bit()),
            )
            .field(
                "hp_l2_mem_ahb_rdbuffer_en",
                &format_args!("{}", self.hp_l2_mem_ahb_rdbuffer_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_L2_MEM_AHB_BUFFER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to turn on l2mem ahb wr buffer"]
    #[inline(always)]
    #[must_use]
    pub fn hp_l2_mem_ahb_wrbuffer_en(
        &mut self,
    ) -> HP_L2_MEM_AHB_WRBUFFER_EN_W<HP_L2_MEM_AHB_BUFFER_CTRL_SPEC> {
        HP_L2_MEM_AHB_WRBUFFER_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to turn on l2mem ahb rd buffer"]
    #[inline(always)]
    #[must_use]
    pub fn hp_l2_mem_ahb_rdbuffer_en(
        &mut self,
    ) -> HP_L2_MEM_AHB_RDBUFFER_EN_W<HP_L2_MEM_AHB_BUFFER_CTRL_SPEC> {
        HP_L2_MEM_AHB_RDBUFFER_EN_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_l2_mem_ahb_buffer_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_l2_mem_ahb_buffer_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_L2_MEM_AHB_BUFFER_CTRL_SPEC;
impl crate::RegisterSpec for HP_L2_MEM_AHB_BUFFER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_l2_mem_ahb_buffer_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_L2_MEM_AHB_BUFFER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_l2_mem_ahb_buffer_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_L2_MEM_AHB_BUFFER_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_L2_MEM_AHB_BUFFER_CTRL to value 0"]
impl crate::Resettable for HP_L2_MEM_AHB_BUFFER_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
