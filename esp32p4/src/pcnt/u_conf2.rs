#[doc = "Register `U%s_CONF2` reader"]
pub type R = crate::R<U_CONF2_SPEC>;
#[doc = "Register `U%s_CONF2` writer"]
pub type W = crate::W<U_CONF2_SPEC>;
#[doc = "Field `CNT_H_LIM_U` reader - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CNT_H_LIM_U_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_H_LIM_U` writer - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CNT_H_LIM_U_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_L_LIM_U` reader - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CNT_L_LIM_U_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_L_LIM_U` writer - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
pub type CNT_L_LIM_U_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_h_lim_u(&self) -> CNT_H_LIM_U_R {
        CNT_H_LIM_U_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    pub fn cnt_l_lim_u(&self) -> CNT_L_LIM_U_R {
        CNT_L_LIM_U_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CONF2")
            .field(
                "cnt_h_lim_u",
                &format_args!("{}", self.cnt_h_lim_u().bits()),
            )
            .field(
                "cnt_l_lim_u",
                &format_args!("{}", self.cnt_l_lim_u().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U_CONF2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is used to configure the thr_h_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_h_lim_u(&mut self) -> CNT_H_LIM_U_W<U_CONF2_SPEC> {
        CNT_H_LIM_U_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - This register is used to configure the thr_l_lim value for unit %s. When pcnt reaches this value, the counter will be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_l_lim_u(&mut self) -> CNT_L_LIM_U_W<U_CONF2_SPEC> {
        CNT_L_LIM_U_W::new(self, 16)
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
#[doc = "Configuration register 2 for unit %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`u_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`u_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U_CONF2_SPEC;
impl crate::RegisterSpec for U_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u_conf2::R`](R) reader structure"]
impl crate::Readable for U_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u_conf2::W`](W) writer structure"]
impl crate::Writable for U_CONF2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets U%s_CONF2 to value 0"]
impl crate::Resettable for U_CONF2_SPEC {
    const RESET_VALUE: u32 = 0;
}
