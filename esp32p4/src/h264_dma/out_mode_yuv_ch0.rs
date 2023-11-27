#[doc = "Register `OUT_MODE_YUV_CH0` reader"]
pub type R = crate::R<OUT_MODE_YUV_CH0_SPEC>;
#[doc = "Register `OUT_MODE_YUV_CH0` writer"]
pub type W = crate::W<OUT_MODE_YUV_CH0_SPEC>;
#[doc = "Field `OUT_TEST_Y_VALUE_CH0` reader - tx CH0 test mode y value"]
pub type OUT_TEST_Y_VALUE_CH0_R = crate::FieldReader;
#[doc = "Field `OUT_TEST_Y_VALUE_CH0` writer - tx CH0 test mode y value"]
pub type OUT_TEST_Y_VALUE_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUT_TEST_U_VALUE_CH0` reader - tx CH0 test mode u value"]
pub type OUT_TEST_U_VALUE_CH0_R = crate::FieldReader;
#[doc = "Field `OUT_TEST_U_VALUE_CH0` writer - tx CH0 test mode u value"]
pub type OUT_TEST_U_VALUE_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUT_TEST_V_VALUE_CH0` reader - tx CH0 test mode v value"]
pub type OUT_TEST_V_VALUE_CH0_R = crate::FieldReader;
#[doc = "Field `OUT_TEST_V_VALUE_CH0` writer - tx CH0 test mode v value"]
pub type OUT_TEST_V_VALUE_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - tx CH0 test mode y value"]
    #[inline(always)]
    pub fn out_test_y_value_ch0(&self) -> OUT_TEST_Y_VALUE_CH0_R {
        OUT_TEST_Y_VALUE_CH0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tx CH0 test mode u value"]
    #[inline(always)]
    pub fn out_test_u_value_ch0(&self) -> OUT_TEST_U_VALUE_CH0_R {
        OUT_TEST_U_VALUE_CH0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tx CH0 test mode v value"]
    #[inline(always)]
    pub fn out_test_v_value_ch0(&self) -> OUT_TEST_V_VALUE_CH0_R {
        OUT_TEST_V_VALUE_CH0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_MODE_YUV_CH0")
            .field(
                "out_test_y_value_ch0",
                &format_args!("{}", self.out_test_y_value_ch0().bits()),
            )
            .field(
                "out_test_u_value_ch0",
                &format_args!("{}", self.out_test_u_value_ch0().bits()),
            )
            .field(
                "out_test_v_value_ch0",
                &format_args!("{}", self.out_test_v_value_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_MODE_YUV_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - tx CH0 test mode y value"]
    #[inline(always)]
    #[must_use]
    pub fn out_test_y_value_ch0(&mut self) -> OUT_TEST_Y_VALUE_CH0_W<OUT_MODE_YUV_CH0_SPEC> {
        OUT_TEST_Y_VALUE_CH0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - tx CH0 test mode u value"]
    #[inline(always)]
    #[must_use]
    pub fn out_test_u_value_ch0(&mut self) -> OUT_TEST_U_VALUE_CH0_W<OUT_MODE_YUV_CH0_SPEC> {
        OUT_TEST_U_VALUE_CH0_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - tx CH0 test mode v value"]
    #[inline(always)]
    #[must_use]
    pub fn out_test_v_value_ch0(&mut self) -> OUT_TEST_V_VALUE_CH0_W<OUT_MODE_YUV_CH0_SPEC> {
        OUT_TEST_V_VALUE_CH0_W::new(self, 16)
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
#[doc = "tx CH0 test mode yuv value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_mode_yuv_ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_mode_yuv_ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_MODE_YUV_CH0_SPEC;
impl crate::RegisterSpec for OUT_MODE_YUV_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_mode_yuv_ch0::R`](R) reader structure"]
impl crate::Readable for OUT_MODE_YUV_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_mode_yuv_ch0::W`](W) writer structure"]
impl crate::Writable for OUT_MODE_YUV_CH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_MODE_YUV_CH0 to value 0"]
impl crate::Resettable for OUT_MODE_YUV_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
