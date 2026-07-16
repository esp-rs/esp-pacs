#[doc = "Register `DECODER_STATUS5` reader"]
pub type R = crate::R<DECODER_STATUS5_SPEC>;
#[doc = "Register `DECODER_STATUS5` writer"]
pub type W = crate::W<DECODER_STATUS5_SPEC>;
#[doc = "Field `IDCT_HFM_DATA` reader - Reserved"]
pub type IDCT_HFM_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `NS0` reader - Reserved"]
pub type NS0_R = crate::FieldReader;
#[doc = "Field `NS1` reader - Reserved"]
pub type NS1_R = crate::FieldReader;
#[doc = "Field `NS2` reader - Reserved"]
pub type NS2_R = crate::FieldReader;
#[doc = "Field `NS3` reader - Reserved"]
pub type NS3_R = crate::FieldReader;
#[doc = "Field `DATA_LAST_O` reader - Reserved"]
pub type DATA_LAST_O_R = crate::BitReader;
#[doc = "Field `RDN_RESULT` reader - redundant registers for jpeg"]
pub type RDN_RESULT_R = crate::BitReader;
#[doc = "Field `RDN_ENA` reader - redundant control registers for jpeg"]
pub type RDN_ENA_R = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - redundant control registers for jpeg"]
pub type RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn idct_hfm_data(&self) -> IDCT_HFM_DATA_R {
        IDCT_HFM_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Reserved"]
    #[inline(always)]
    pub fn ns0(&self) -> NS0_R {
        NS0_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Reserved"]
    #[inline(always)]
    pub fn ns1(&self) -> NS1_R {
        NS1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Reserved"]
    #[inline(always)]
    pub fn ns2(&self) -> NS2_R {
        NS2_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Reserved"]
    #[inline(always)]
    pub fn ns3(&self) -> NS3_R {
        NS3_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn data_last_o(&self) -> DATA_LAST_O_R {
        DATA_LAST_O_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - redundant registers for jpeg"]
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - redundant control registers for jpeg"]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECODER_STATUS5")
            .field("idct_hfm_data", &self.idct_hfm_data())
            .field("ns0", &self.ns0())
            .field("ns1", &self.ns1())
            .field("ns2", &self.ns2())
            .field("ns3", &self.ns3())
            .field("data_last_o", &self.data_last_o())
            .field("rdn_result", &self.rdn_result())
            .field("rdn_ena", &self.rdn_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - redundant control registers for jpeg"]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<'_, DECODER_STATUS5_SPEC> {
        RDN_ENA_W::new(self, 30)
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder_status5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decoder_status5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DECODER_STATUS5_SPEC;
impl crate::RegisterSpec for DECODER_STATUS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder_status5::R`](R) reader structure"]
impl crate::Readable for DECODER_STATUS5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`decoder_status5::W`](W) writer structure"]
impl crate::Writable for DECODER_STATUS5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DECODER_STATUS5 to value 0"]
impl crate::Resettable for DECODER_STATUS5_SPEC {}
