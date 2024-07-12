#[doc = "Register `MODE_YUV` reader"]
pub type R = crate::R<MODE_YUV_SPEC>;
#[doc = "Register `MODE_YUV` writer"]
pub type W = crate::W<MODE_YUV_SPEC>;
#[doc = "Field `OUT_TEST_Y_VALUE` reader - tx CH0 test mode y value"]
pub type OUT_TEST_Y_VALUE_R = crate::FieldReader;
#[doc = "Field `OUT_TEST_Y_VALUE` writer - tx CH0 test mode y value"]
pub type OUT_TEST_Y_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUT_TEST_U_VALUE` reader - tx CH0 test mode u value"]
pub type OUT_TEST_U_VALUE_R = crate::FieldReader;
#[doc = "Field `OUT_TEST_U_VALUE` writer - tx CH0 test mode u value"]
pub type OUT_TEST_U_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OUT_TEST_V_VALUE` reader - tx CH0 test mode v value"]
pub type OUT_TEST_V_VALUE_R = crate::FieldReader;
#[doc = "Field `OUT_TEST_V_VALUE` writer - tx CH0 test mode v value"]
pub type OUT_TEST_V_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - tx CH0 test mode y value"]
    #[inline(always)]
    pub fn out_test_y_value(&self) -> OUT_TEST_Y_VALUE_R {
        OUT_TEST_Y_VALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - tx CH0 test mode u value"]
    #[inline(always)]
    pub fn out_test_u_value(&self) -> OUT_TEST_U_VALUE_R {
        OUT_TEST_U_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - tx CH0 test mode v value"]
    #[inline(always)]
    pub fn out_test_v_value(&self) -> OUT_TEST_V_VALUE_R {
        OUT_TEST_V_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE_YUV")
            .field("out_test_y_value", &self.out_test_y_value())
            .field("out_test_u_value", &self.out_test_u_value())
            .field("out_test_v_value", &self.out_test_v_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - tx CH0 test mode y value"]
    #[inline(always)]
    #[must_use]
    pub fn out_test_y_value(&mut self) -> OUT_TEST_Y_VALUE_W<MODE_YUV_SPEC> {
        OUT_TEST_Y_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - tx CH0 test mode u value"]
    #[inline(always)]
    #[must_use]
    pub fn out_test_u_value(&mut self) -> OUT_TEST_U_VALUE_W<MODE_YUV_SPEC> {
        OUT_TEST_U_VALUE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - tx CH0 test mode v value"]
    #[inline(always)]
    #[must_use]
    pub fn out_test_v_value(&mut self) -> OUT_TEST_V_VALUE_W<MODE_YUV_SPEC> {
        OUT_TEST_V_VALUE_W::new(self, 16)
    }
}
#[doc = "TX CHx test mode yuv value register. Available on CH0\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_yuv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_yuv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE_YUV_SPEC;
impl crate::RegisterSpec for MODE_YUV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_yuv::R`](R) reader structure"]
impl crate::Readable for MODE_YUV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode_yuv::W`](W) writer structure"]
impl crate::Writable for MODE_YUV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE_YUV to value 0"]
impl crate::Resettable for MODE_YUV_SPEC {
    const RESET_VALUE: u32 = 0;
}
