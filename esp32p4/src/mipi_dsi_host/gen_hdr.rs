///Register `GEN_HDR` reader
pub type R = crate::R<GEN_HDR_SPEC>;
///Register `GEN_HDR` writer
pub type W = crate::W<GEN_HDR_SPEC>;
///Field `GEN_DT` reader - NA
pub type GEN_DT_R = crate::FieldReader;
///Field `GEN_DT` writer - NA
pub type GEN_DT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `GEN_VC` reader - NA
pub type GEN_VC_R = crate::FieldReader;
///Field `GEN_VC` writer - NA
pub type GEN_VC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `GEN_WC_LSBYTE` reader - NA
pub type GEN_WC_LSBYTE_R = crate::FieldReader;
///Field `GEN_WC_LSBYTE` writer - NA
pub type GEN_WC_LSBYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GEN_WC_MSBYTE` reader - NA
pub type GEN_WC_MSBYTE_R = crate::FieldReader;
///Field `GEN_WC_MSBYTE` writer - NA
pub type GEN_WC_MSBYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:5 - NA
    #[inline(always)]
    pub fn gen_dt(&self) -> GEN_DT_R {
        GEN_DT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - NA
    #[inline(always)]
    pub fn gen_vc(&self) -> GEN_VC_R {
        GEN_VC_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:15 - NA
    #[inline(always)]
    pub fn gen_wc_lsbyte(&self) -> GEN_WC_LSBYTE_R {
        GEN_WC_LSBYTE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - NA
    #[inline(always)]
    pub fn gen_wc_msbyte(&self) -> GEN_WC_MSBYTE_R {
        GEN_WC_MSBYTE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_HDR")
            .field("gen_dt", &self.gen_dt())
            .field("gen_vc", &self.gen_vc())
            .field("gen_wc_lsbyte", &self.gen_wc_lsbyte())
            .field("gen_wc_msbyte", &self.gen_wc_msbyte())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - NA
    #[inline(always)]
    #[must_use]
    pub fn gen_dt(&mut self) -> GEN_DT_W<GEN_HDR_SPEC> {
        GEN_DT_W::new(self, 0)
    }
    ///Bits 6:7 - NA
    #[inline(always)]
    #[must_use]
    pub fn gen_vc(&mut self) -> GEN_VC_W<GEN_HDR_SPEC> {
        GEN_VC_W::new(self, 6)
    }
    ///Bits 8:15 - NA
    #[inline(always)]
    #[must_use]
    pub fn gen_wc_lsbyte(&mut self) -> GEN_WC_LSBYTE_W<GEN_HDR_SPEC> {
        GEN_WC_LSBYTE_W::new(self, 8)
    }
    ///Bits 16:23 - NA
    #[inline(always)]
    #[must_use]
    pub fn gen_wc_msbyte(&mut self) -> GEN_WC_MSBYTE_W<GEN_HDR_SPEC> {
        GEN_WC_MSBYTE_W::new(self, 16)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`gen_hdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_hdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GEN_HDR_SPEC;
impl crate::RegisterSpec for GEN_HDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gen_hdr::R`](R) reader structure
impl crate::Readable for GEN_HDR_SPEC {}
///`write(|w| ..)` method takes [`gen_hdr::W`](W) writer structure
impl crate::Writable for GEN_HDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GEN_HDR to value 0
impl crate::Resettable for GEN_HDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
