///Register `SET_LINE_CODE_W1` reader
pub type R = crate::R<SET_LINE_CODE_W1_SPEC>;
///Field `BCHAR_FORMAT` reader - The value of bCharFormat set by host through SET_LINE_CODING command.
pub type BCHAR_FORMAT_R = crate::FieldReader;
///Field `BPARITY_TYPE` reader - The value of bParityTpye set by host through SET_LINE_CODING command.
pub type BPARITY_TYPE_R = crate::FieldReader;
///Field `BDATA_BITS` reader - The value of bDataBits set by host through SET_LINE_CODING command.
pub type BDATA_BITS_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - The value of bCharFormat set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub fn bchar_format(&self) -> BCHAR_FORMAT_R {
        BCHAR_FORMAT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - The value of bParityTpye set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub fn bparity_type(&self) -> BPARITY_TYPE_R {
        BPARITY_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - The value of bDataBits set by host through SET_LINE_CODING command.
    #[inline(always)]
    pub fn bdata_bits(&self) -> BDATA_BITS_R {
        BDATA_BITS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SET_LINE_CODE_W1")
            .field("bchar_format", &self.bchar_format())
            .field("bparity_type", &self.bparity_type())
            .field("bdata_bits", &self.bdata_bits())
            .finish()
    }
}
/**W1 of SET_LINE_CODING command.

You can [`read`](crate::generic::Reg::read) this register and get [`set_line_code_w1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SET_LINE_CODE_W1_SPEC;
impl crate::RegisterSpec for SET_LINE_CODE_W1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`set_line_code_w1::R`](R) reader structure
impl crate::Readable for SET_LINE_CODE_W1_SPEC {}
///`reset()` method sets SET_LINE_CODE_W1 to value 0
impl crate::Resettable for SET_LINE_CODE_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
