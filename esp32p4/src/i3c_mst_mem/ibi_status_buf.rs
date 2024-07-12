#[doc = "Register `IBI_STATUS_BUF` reader"]
pub type R = crate::R<IBI_STATUS_BUF_SPEC>;
#[doc = "Field `DATA_LENGTH` reader - This field represents the length of data received along with IBI, in bytes."]
pub type DATA_LENGTH_R = crate::FieldReader;
#[doc = "Field `IBI_ID` reader - IBI Identifier. The byte received after START which includes the address the R/W bit: Device address and R/W bit in case of Slave Interrupt or Master Request."]
pub type IBI_ID_R = crate::FieldReader;
#[doc = "Field `IBI_STS` reader - IBI received data/status. IBI Data register is mapped to the IBI Buffer. The IBI Data is always packed in4-byte aligned and put to the IBI Buffer. This register When read from, reads the data from the IBI buffer. IBI Status register when read from, returns the data from the IBI Buffer and indicates how the controller responded to incoming IBI(SIR, MR and HJ)."]
pub type IBI_STS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - This field represents the length of data received along with IBI, in bytes."]
    #[inline(always)]
    pub fn data_length(&self) -> DATA_LENGTH_R {
        DATA_LENGTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - IBI Identifier. The byte received after START which includes the address the R/W bit: Device address and R/W bit in case of Slave Interrupt or Master Request."]
    #[inline(always)]
    pub fn ibi_id(&self) -> IBI_ID_R {
        IBI_ID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 28 - IBI received data/status. IBI Data register is mapped to the IBI Buffer. The IBI Data is always packed in4-byte aligned and put to the IBI Buffer. This register When read from, reads the data from the IBI buffer. IBI Status register when read from, returns the data from the IBI Buffer and indicates how the controller responded to incoming IBI(SIR, MR and HJ)."]
    #[inline(always)]
    pub fn ibi_sts(&self) -> IBI_STS_R {
        IBI_STS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBI_STATUS_BUF")
            .field("data_length", &self.data_length())
            .field("ibi_id", &self.ibi_id())
            .field("ibi_sts", &self.ibi_sts())
            .finish()
    }
}
#[doc = "In-Band Interrupt Buffer Status/Data Register. When receiving an IBI, IBI_PORT is used to both: Read the IBI Status Read the IBI Data(which is raw/opaque data)\n\nYou can [`read`](crate::Reg::read) this register and get [`ibi_status_buf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBI_STATUS_BUF_SPEC;
impl crate::RegisterSpec for IBI_STATUS_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibi_status_buf::R`](R) reader structure"]
impl crate::Readable for IBI_STATUS_BUF_SPEC {}
#[doc = "`reset()` method sets IBI_STATUS_BUF to value 0"]
impl crate::Resettable for IBI_STATUS_BUF_SPEC {
    const RESET_VALUE: u32 = 0;
}
