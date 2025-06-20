#[doc = "Register `U%s_CONF2` reader"]
pub type R = crate::R<U_CONF2_SPEC>;
#[doc = "Register `U%s_CONF2` writer"]
pub type W = crate::W<U_CONF2_SPEC>;
#[doc = "Field `CNT_H_LIM_U` reader - Configures the thr_h_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
pub type CNT_H_LIM_U_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_H_LIM_U` writer - Configures the thr_h_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
pub type CNT_H_LIM_U_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_L_LIM_U` reader - Configures the thr_l_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
pub type CNT_L_LIM_U_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_L_LIM_U` writer - Configures the thr_l_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
pub type CNT_L_LIM_U_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the thr_h_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_h_lim_u(&self) -> CNT_H_LIM_U_R {
        CNT_H_LIM_U_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the thr_l_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_l_lim_u(&self) -> CNT_L_LIM_U_R {
        CNT_L_LIM_U_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CONF2")
            .field("cnt_h_lim_u", &self.cnt_h_lim_u())
            .field("cnt_l_lim_u", &self.cnt_l_lim_u())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the thr_h_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_h_lim_u(&mut self) -> CNT_H_LIM_U_W<U_CONF2_SPEC> {
        CNT_H_LIM_U_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures the thr_l_lim value for unit %s. When pulse_cnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_l_lim_u(&mut self) -> CNT_L_LIM_U_W<U_CONF2_SPEC> {
        CNT_L_LIM_U_W::new(self, 16)
    }
}
#[doc = "Configuration register 2 for unit %s\n\nYou can [`read`](crate::Reg::read) this register and get [`u_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_CONF2_SPEC;
impl crate::RegisterSpec for U_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_conf2::R`](R) reader structure"]
impl crate::Readable for U_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u_conf2::W`](W) writer structure"]
impl crate::Writable for U_CONF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets U%s_CONF2 to value 0"]
impl crate::Resettable for U_CONF2_SPEC {
    const RESET_VALUE: u32 = 0;
}
