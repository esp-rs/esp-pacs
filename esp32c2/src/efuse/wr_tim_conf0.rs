#[doc = "Register `WR_TIM_CONF0` reader"]
pub type R = crate::R<WR_TIM_CONF0_SPEC>;
#[doc = "Register `WR_TIM_CONF0` writer"]
pub type W = crate::W<WR_TIM_CONF0_SPEC>;
#[doc = "Field `THP_A` reader - Configures hold time for efuse program."]
pub type THP_A_R = crate::FieldReader;
#[doc = "Field `THP_A` writer - Configures hold time for efuse program."]
pub type THP_A_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPGM_INACTIVE` reader - Configures pulse time for burning '0' bit."]
pub type TPGM_INACTIVE_R = crate::FieldReader;
#[doc = "Field `TPGM_INACTIVE` writer - Configures pulse time for burning '0' bit."]
pub type TPGM_INACTIVE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPGM` reader - Configures pulse time for burning '1' bit."]
pub type TPGM_R = crate::FieldReader<u16>;
#[doc = "Field `TPGM` writer - Configures pulse time for burning '1' bit."]
pub type TPGM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Configures hold time for efuse program."]
    #[inline(always)]
    pub fn thp_a(&self) -> THP_A_R {
        THP_A_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures pulse time for burning '0' bit."]
    #[inline(always)]
    pub fn tpgm_inactive(&self) -> TPGM_INACTIVE_R {
        TPGM_INACTIVE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Configures pulse time for burning '1' bit."]
    #[inline(always)]
    pub fn tpgm(&self) -> TPGM_R {
        TPGM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WR_TIM_CONF0")
            .field("thp_a", &self.thp_a())
            .field("tpgm_inactive", &self.tpgm_inactive())
            .field("tpgm", &self.tpgm())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures hold time for efuse program."]
    #[inline(always)]
    #[must_use]
    pub fn thp_a(&mut self) -> THP_A_W<WR_TIM_CONF0_SPEC> {
        THP_A_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures pulse time for burning '0' bit."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm_inactive(&mut self) -> TPGM_INACTIVE_W<WR_TIM_CONF0_SPEC> {
        TPGM_INACTIVE_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Configures pulse time for burning '1' bit."]
    #[inline(always)]
    #[must_use]
    pub fn tpgm(&mut self) -> TPGM_W<WR_TIM_CONF0_SPEC> {
        TPGM_W::new(self, 16)
    }
}
#[doc = "Configurarion register 0 of eFuse programming timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_TIM_CONF0_SPEC;
impl crate::RegisterSpec for WR_TIM_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_tim_conf0::R`](R) reader structure"]
impl crate::Readable for WR_TIM_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wr_tim_conf0::W`](W) writer structure"]
impl crate::Writable for WR_TIM_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_TIM_CONF0 to value 0x00c8_0101"]
impl crate::Resettable for WR_TIM_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x00c8_0101;
}
