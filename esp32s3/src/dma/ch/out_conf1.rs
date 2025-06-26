#[doc = "Register `OUT_CONF1` reader"]
pub type R = crate::R<OUT_CONF1_SPEC>;
#[doc = "Register `OUT_CONF1` writer"]
pub type W = crate::W<OUT_CONF1_SPEC>;
#[doc = "Field `OUT_CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `OUT_CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type OUT_CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EXT_MEM_BK_SIZE` reader - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type OUT_EXT_MEM_BK_SIZE_R = crate::FieldReader;
#[doc = "Field `OUT_EXT_MEM_BK_SIZE` writer - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
pub type OUT_EXT_MEM_BK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&self) -> OUT_CHECK_OWNER_R {
        OUT_CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn out_ext_mem_bk_size(&self) -> OUT_EXT_MEM_BK_SIZE_R {
        OUT_EXT_MEM_BK_SIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CONF1")
            .field("out_check_owner", &self.out_check_owner())
            .field("out_ext_mem_bk_size", &self.out_ext_mem_bk_size())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn out_check_owner(&mut self) -> OUT_CHECK_OWNER_W<OUT_CONF1_SPEC> {
        OUT_CHECK_OWNER_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - Block size of Tx channel 0 when DMA access external SRAM. 0: 16 bytes 1: 32 bytes 2/3:reserved"]
    #[inline(always)]
    pub fn out_ext_mem_bk_size(&mut self) -> OUT_EXT_MEM_BK_SIZE_W<OUT_CONF1_SPEC> {
        OUT_EXT_MEM_BK_SIZE_W::new(self, 13)
    }
}
#[doc = "Configure 1 register of Tx channel 0\n\nYou can [`read`](crate::Reg::read) this register and get [`out_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CONF1_SPEC;
impl crate::RegisterSpec for OUT_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_conf1::R`](R) reader structure"]
impl crate::Readable for OUT_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_conf1::W`](W) writer structure"]
impl crate::Writable for OUT_CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_CONF1 to value 0"]
impl crate::Resettable for OUT_CONF1_SPEC {}
