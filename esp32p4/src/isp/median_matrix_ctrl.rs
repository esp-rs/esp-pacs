#[doc = "Register `MEDIAN_MATRIX_CTRL` reader"]
pub type R = crate::R<MEDIAN_MATRIX_CTRL_SPEC>;
#[doc = "Register `MEDIAN_MATRIX_CTRL` writer"]
pub type W = crate::W<MEDIAN_MATRIX_CTRL_SPEC>;
#[doc = "Field `MEDIAN_PADDING_DATA` reader - this field configures median matrix padding data"]
pub type MEDIAN_PADDING_DATA_R = crate::FieldReader;
#[doc = "Field `MEDIAN_PADDING_DATA` writer - this field configures median matrix padding data"]
pub type MEDIAN_PADDING_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MEDIAN_PADDING_MODE` reader - this bit configures the padding mode of median matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type MEDIAN_PADDING_MODE_R = crate::BitReader;
#[doc = "Field `MEDIAN_PADDING_MODE` writer - this bit configures the padding mode of median matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
pub type MEDIAN_PADDING_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - this field configures median matrix padding data"]
    #[inline(always)]
    pub fn median_padding_data(&self) -> MEDIAN_PADDING_DATA_R {
        MEDIAN_PADDING_DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - this bit configures the padding mode of median matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    pub fn median_padding_mode(&self) -> MEDIAN_PADDING_MODE_R {
        MEDIAN_PADDING_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEDIAN_MATRIX_CTRL")
            .field("median_padding_data", &self.median_padding_data().bits())
            .field("median_padding_mode", &self.median_padding_mode().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEDIAN_MATRIX_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures median matrix padding data"]
    #[inline(always)]
    #[must_use]
    pub fn median_padding_data(&mut self) -> MEDIAN_PADDING_DATA_W<MEDIAN_MATRIX_CTRL_SPEC> {
        MEDIAN_PADDING_DATA_W::new(self, 0)
    }
    #[doc = "Bit 8 - this bit configures the padding mode of median matrix. 0: use pixel in image to do padding 1: use reg_padding_data to do padding"]
    #[inline(always)]
    #[must_use]
    pub fn median_padding_mode(&mut self) -> MEDIAN_PADDING_MODE_W<MEDIAN_MATRIX_CTRL_SPEC> {
        MEDIAN_PADDING_MODE_W::new(self, 8)
    }
}
#[doc = "median pix2matrix ctrl\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`median_matrix_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`median_matrix_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEDIAN_MATRIX_CTRL_SPEC;
impl crate::RegisterSpec for MEDIAN_MATRIX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`median_matrix_ctrl::R`](R) reader structure"]
impl crate::Readable for MEDIAN_MATRIX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`median_matrix_ctrl::W`](W) writer structure"]
impl crate::Writable for MEDIAN_MATRIX_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEDIAN_MATRIX_CTRL to value 0"]
impl crate::Resettable for MEDIAN_MATRIX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
