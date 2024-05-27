///Register `L2_MEM_SW_ECC_BWE_MASK` reader
pub type R = crate::R<L2_MEM_SW_ECC_BWE_MASK_SPEC>;
///Register `L2_MEM_SW_ECC_BWE_MASK` writer
pub type W = crate::W<L2_MEM_SW_ECC_BWE_MASK_SPEC>;
///Field `REG_L2_MEM_SW_ECC_BWE_MASK_CTRL` reader - Set 1 to mask bwe hamming code bit
pub type REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_R = crate::BitReader;
///Field `REG_L2_MEM_SW_ECC_BWE_MASK_CTRL` writer - Set 1 to mask bwe hamming code bit
pub type REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to mask bwe hamming code bit
    #[inline(always)]
    pub fn reg_l2_mem_sw_ecc_bwe_mask_ctrl(&self) -> REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_R {
        REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_SW_ECC_BWE_MASK")
            .field(
                "reg_l2_mem_sw_ecc_bwe_mask_ctrl",
                &self.reg_l2_mem_sw_ecc_bwe_mask_ctrl(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to mask bwe hamming code bit
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_mem_sw_ecc_bwe_mask_ctrl(
        &mut self,
    ) -> REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_W<L2_MEM_SW_ECC_BWE_MASK_SPEC> {
        REG_L2_MEM_SW_ECC_BWE_MASK_CTRL_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_sw_ecc_bwe_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_sw_ecc_bwe_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_MEM_SW_ECC_BWE_MASK_SPEC;
impl crate::RegisterSpec for L2_MEM_SW_ECC_BWE_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_mem_sw_ecc_bwe_mask::R`](R) reader structure
impl crate::Readable for L2_MEM_SW_ECC_BWE_MASK_SPEC {}
///`write(|w| ..)` method takes [`l2_mem_sw_ecc_bwe_mask::W`](W) writer structure
impl crate::Writable for L2_MEM_SW_ECC_BWE_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_MEM_SW_ECC_BWE_MASK to value 0
impl crate::Resettable for L2_MEM_SW_ECC_BWE_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
