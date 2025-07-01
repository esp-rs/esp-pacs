#[doc = "Register `DPI_COLOR_CODING_ACT` reader"]
pub type R = crate::R<DPI_COLOR_CODING_ACT_SPEC>;
#[doc = "Field `DPI_COLOR_CODING_ACT` reader - NA"]
pub type DPI_COLOR_CODING_ACT_R = crate::FieldReader;
#[doc = "Field `LOOSELY18_EN_ACT` reader - NA"]
pub type LOOSELY18_EN_ACT_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn dpi_color_coding_act(&self) -> DPI_COLOR_CODING_ACT_R {
        DPI_COLOR_CODING_ACT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn loosely18_en_act(&self) -> LOOSELY18_EN_ACT_R {
        LOOSELY18_EN_ACT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPI_COLOR_CODING_ACT")
            .field("dpi_color_coding_act", &self.dpi_color_coding_act())
            .field("loosely18_en_act", &self.loosely18_en_act())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dpi_color_coding_act::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPI_COLOR_CODING_ACT_SPEC;
impl crate::RegisterSpec for DPI_COLOR_CODING_ACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi_color_coding_act::R`](R) reader structure"]
impl crate::Readable for DPI_COLOR_CODING_ACT_SPEC {}
#[doc = "`reset()` method sets DPI_COLOR_CODING_ACT to value 0"]
impl crate::Resettable for DPI_COLOR_CODING_ACT_SPEC {}
