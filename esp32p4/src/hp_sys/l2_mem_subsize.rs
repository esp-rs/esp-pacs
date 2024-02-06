#[doc = "Register `L2_MEM_SUBSIZE` reader"]
pub type R = crate::R<L2_MEM_SUBSIZE_SPEC>;
#[doc = "Register `L2_MEM_SUBSIZE` writer"]
pub type W = crate::W<L2_MEM_SUBSIZE_SPEC>;
#[doc = "Field `REG_L2_MEM_SUB_BLKSIZE` reader - l2mem sub block size 00=>32 01=>64 10=>128 11=>256"]
pub type REG_L2_MEM_SUB_BLKSIZE_R = crate::FieldReader;
#[doc = "Field `REG_L2_MEM_SUB_BLKSIZE` writer - l2mem sub block size 00=>32 01=>64 10=>128 11=>256"]
pub type REG_L2_MEM_SUB_BLKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - l2mem sub block size 00=>32 01=>64 10=>128 11=>256"]
    #[inline(always)]
    pub fn reg_l2_mem_sub_blksize(&self) -> REG_L2_MEM_SUB_BLKSIZE_R {
        REG_L2_MEM_SUB_BLKSIZE_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_SUBSIZE")
            .field(
                "reg_l2_mem_sub_blksize",
                &format_args!("{}", self.reg_l2_mem_sub_blksize().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_MEM_SUBSIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - l2mem sub block size 00=>32 01=>64 10=>128 11=>256"]
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_sub_blksize(&mut self) -> REG_L2_MEM_SUB_BLKSIZE_W<L2_MEM_SUBSIZE_SPEC> {
        REG_L2_MEM_SUB_BLKSIZE_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_subsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_subsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_MEM_SUBSIZE_SPEC;
impl crate::RegisterSpec for L2_MEM_SUBSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_mem_subsize::R`](R) reader structure"]
impl crate::Readable for L2_MEM_SUBSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_mem_subsize::W`](W) writer structure"]
impl crate::Writable for L2_MEM_SUBSIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_MEM_SUBSIZE to value 0"]
impl crate::Resettable for L2_MEM_SUBSIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
