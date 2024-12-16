#[doc = "Register `CONF1` reader"]
pub type R = crate::R<CONF1_SPEC>;
#[doc = "Register `CONF1` writer"]
pub type W = crate::W<CONF1_SPEC>;
#[doc = "Field `INFIFO_FULL_THRS` reader - This register is used to generate the CRYPTO_DMA_INFIFO_FULL_WM_INT interrupt when the byte number is up to the value of the register."]
pub type INFIFO_FULL_THRS_R = crate::FieldReader<u16>;
#[doc = "Field `INFIFO_FULL_THRS` writer - This register is used to generate the CRYPTO_DMA_INFIFO_FULL_WM_INT interrupt when the byte number is up to the value of the register."]
pub type INFIFO_FULL_THRS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CHECK_OWNER` reader - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type CHECK_OWNER_R = crate::BitReader;
#[doc = "Field `CHECK_OWNER` writer - Set this bit to enable checking the owner attribute of the link descriptor."]
pub type CHECK_OWNER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA access external memory block size. 0: 16 bytes; 1: 32 bytes; 2: 64 bytes; 3:Reserved.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXT_MEM_BK_SIZE {
    #[doc = "0: DMA access external memory block size is 16 bytes"]
    Size16 = 0,
    #[doc = "1: DMA access external memory block size is 32 bytes"]
    Size32 = 1,
    #[doc = "2: DMA access external memory block size is 64 bytes"]
    Size64 = 2,
}
impl From<EXT_MEM_BK_SIZE> for u8 {
    #[inline(always)]
    fn from(variant: EXT_MEM_BK_SIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXT_MEM_BK_SIZE {
    type Ux = u8;
}
impl crate::IsEnum for EXT_MEM_BK_SIZE {}
#[doc = "Field `EXT_MEM_BK_SIZE` reader - DMA access external memory block size. 0: 16 bytes; 1: 32 bytes; 2: 64 bytes; 3:Reserved."]
pub type EXT_MEM_BK_SIZE_R = crate::FieldReader<EXT_MEM_BK_SIZE>;
impl EXT_MEM_BK_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXT_MEM_BK_SIZE> {
        match self.bits {
            0 => Some(EXT_MEM_BK_SIZE::Size16),
            1 => Some(EXT_MEM_BK_SIZE::Size32),
            2 => Some(EXT_MEM_BK_SIZE::Size64),
            _ => None,
        }
    }
    #[doc = "DMA access external memory block size is 16 bytes"]
    #[inline(always)]
    pub fn is_size_16(&self) -> bool {
        *self == EXT_MEM_BK_SIZE::Size16
    }
    #[doc = "DMA access external memory block size is 32 bytes"]
    #[inline(always)]
    pub fn is_size_32(&self) -> bool {
        *self == EXT_MEM_BK_SIZE::Size32
    }
    #[doc = "DMA access external memory block size is 64 bytes"]
    #[inline(always)]
    pub fn is_size_64(&self) -> bool {
        *self == EXT_MEM_BK_SIZE::Size64
    }
}
#[doc = "Field `EXT_MEM_BK_SIZE` writer - DMA access external memory block size. 0: 16 bytes; 1: 32 bytes; 2: 64 bytes; 3:Reserved."]
pub type EXT_MEM_BK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, EXT_MEM_BK_SIZE>;
impl<'a, REG> EXT_MEM_BK_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA access external memory block size is 16 bytes"]
    #[inline(always)]
    pub fn size_16(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_MEM_BK_SIZE::Size16)
    }
    #[doc = "DMA access external memory block size is 32 bytes"]
    #[inline(always)]
    pub fn size_32(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_MEM_BK_SIZE::Size32)
    }
    #[doc = "DMA access external memory block size is 64 bytes"]
    #[inline(always)]
    pub fn size_64(self) -> &'a mut crate::W<REG> {
        self.variant(EXT_MEM_BK_SIZE::Size64)
    }
}
impl R {
    #[doc = "Bits 0:11 - This register is used to generate the CRYPTO_DMA_INFIFO_FULL_WM_INT interrupt when the byte number is up to the value of the register."]
    #[inline(always)]
    pub fn infifo_full_thrs(&self) -> INFIFO_FULL_THRS_R {
        INFIFO_FULL_THRS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn check_owner(&self) -> CHECK_OWNER_R {
        CHECK_OWNER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - DMA access external memory block size. 0: 16 bytes; 1: 32 bytes; 2: 64 bytes; 3:Reserved."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&self) -> EXT_MEM_BK_SIZE_R {
        EXT_MEM_BK_SIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF1")
            .field("infifo_full_thrs", &self.infifo_full_thrs())
            .field("check_owner", &self.check_owner())
            .field("ext_mem_bk_size", &self.ext_mem_bk_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - This register is used to generate the CRYPTO_DMA_INFIFO_FULL_WM_INT interrupt when the byte number is up to the value of the register."]
    #[inline(always)]
    pub fn infifo_full_thrs(&mut self) -> INFIFO_FULL_THRS_W<CONF1_SPEC> {
        INFIFO_FULL_THRS_W::new(self, 0)
    }
    #[doc = "Bit 12 - Set this bit to enable checking the owner attribute of the link descriptor."]
    #[inline(always)]
    pub fn check_owner(&mut self) -> CHECK_OWNER_W<CONF1_SPEC> {
        CHECK_OWNER_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - DMA access external memory block size. 0: 16 bytes; 1: 32 bytes; 2: 64 bytes; 3:Reserved."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&mut self) -> EXT_MEM_BK_SIZE_W<CONF1_SPEC> {
        EXT_MEM_BK_SIZE_W::new(self, 13)
    }
}
#[doc = "DMA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf1::R`](R) reader structure"]
impl crate::Readable for CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf1::W`](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
