#[doc = "Register `RD_TIM_CONF` reader"]
pub type R = crate::R<RD_TIM_CONF_SPEC>;
#[doc = "Register `RD_TIM_CONF` writer"]
pub type W = crate::W<RD_TIM_CONF_SPEC>;
#[doc = "Field `READ_INIT_NUM` reader - Configures the initial read time of eFuse."]
pub type READ_INIT_NUM_R = crate::FieldReader;
#[doc = "Field `READ_INIT_NUM` writer - Configures the initial read time of eFuse."]
pub type READ_INIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31 - Configures the initial read time of eFuse."]
    #[inline(always)]
    pub fn read_init_num(&self) -> READ_INIT_NUM_R {
        READ_INIT_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_TIM_CONF")
            .field(
                "read_init_num",
                &format_args!("{}", self.read_init_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_TIM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 24:31 - Configures the initial read time of eFuse."]
    #[inline(always)]
    #[must_use]
    pub fn read_init_num(&mut self) -> READ_INIT_NUM_W<RD_TIM_CONF_SPEC> {
        READ_INIT_NUM_W::new(self, 24)
    }
}
#[doc = "Configures read timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_tim_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_tim_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_TIM_CONF_SPEC;
impl crate::RegisterSpec for RD_TIM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_tim_conf::R`](R) reader structure"]
impl crate::Readable for RD_TIM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_tim_conf::W`](W) writer structure"]
impl crate::Writable for RD_TIM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD_TIM_CONF to value 0x1200_0000"]
impl crate::Resettable for RD_TIM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1200_0000;
}
